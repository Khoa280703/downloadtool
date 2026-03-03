#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 2 ]]; then
  echo "Usage: $0 <new_commit> <old_commit> [video_url] [playlist_url]"
  exit 1
fi

NEW_COMMIT="$1"
OLD_COMMIT="$2"
VIDEO_URL="${3:-https://www.youtube.com/watch?v=dQw4w9WgXcQ}"
PLAYLIST_URL="${4:-https://www.youtube.com/playlist?list=PLgIS1MvgCaOVR07APOp8zfBLHbh7VGllX}"
LEVELS="${LEVELS:-smoke medium heavy extreme}"

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
RUN_ID="$(date +%Y%m%d-%H%M%S)"
OUT_BASE="${OUT_BASE:-/tmp/stress-compare-commits-${RUN_ID}}"
TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/downloadtool-compare-target}"
mkdir -p "$OUT_BASE" "$TARGET_DIR"

NEW_WORKTREE="$OUT_BASE/worktree-new"
OLD_WORKTREE="$OUT_BASE/worktree-old"
NEW_OUT="$OUT_BASE/new"
OLD_OUT="$OUT_BASE/old"
COMPARE_TSV="$OUT_BASE/compare.tsv"

resolve_env_value() {
  local key="$1"
  local fallback="${2:-}"
  local value=""
  value="$(printenv "$key" || true)"
  if [[ -z "$value" && -f "$ROOT_DIR/.env" ]]; then
    value=$(awk -F= -v key="$key" '$1==key{sub("^"key"=",""); print; exit}' "$ROOT_DIR/.env")
  fi
  if [[ -z "$value" ]]; then
    value="$fallback"
  fi
  echo "$value"
}

DATABASE_URL_RESOLVED="$(resolve_env_value "DATABASE_URL" "")"
if [[ -z "$DATABASE_URL_RESOLVED" ]]; then
  echo "DATABASE_URL is required. Set env DATABASE_URL or define it in $ROOT_DIR/.env"
  exit 1
fi
BETTER_AUTH_SECRET_RESOLVED="$(resolve_env_value "BETTER_AUTH_SECRET" "bench-better-auth-secret")"
WHOP_WEBHOOK_SECRET_RESOLVED="$(resolve_env_value "WHOP_WEBHOOK_SECRET" "bench-whop-webhook-secret")"

cleanup_worktrees() {
  set +e
  git -C "$ROOT_DIR" worktree remove --force "$NEW_WORKTREE" >/dev/null 2>&1 || true
  git -C "$ROOT_DIR" worktree remove --force "$OLD_WORKTREE" >/dev/null 2>&1 || true
  set -e
}
trap cleanup_worktrees EXIT

kill_port() {
  local port="$1"
  local pids
  pids=$(lsof -tiTCP:"$port" -sTCP:LISTEN 2>/dev/null || true)
  if [[ -n "$pids" ]]; then
    kill $pids || true
    sleep 1
    pids=$(lsof -tiTCP:"$port" -sTCP:LISTEN 2>/dev/null || true)
    [[ -n "$pids" ]] && kill -9 $pids || true
  fi
}

wait_port() {
  local port="$1"
  for _ in $(seq 1 90); do
    if ss -ltn "( sport = :$port )" | tail -n +2 | grep -q .; then
      return 0
    fi
    sleep 1
  done
  return 1
}

run_one_commit() {
  local label="$1"
  local commit="$2"
  local worktree="$3"
  local outdir="$4"
  local port="$5"
  local backend_log="$outdir/backend.log"
  local suite_log="$outdir/suite.log"
  mkdir -p "$outdir"

  git -C "$ROOT_DIR" worktree add --detach "$worktree" "$commit" >/dev/null

  kill_port "$port"
  (
    cd "$worktree"
    DATABASE_URL="$DATABASE_URL_RESOLVED" \
    BETTER_AUTH_SECRET="$BETTER_AUTH_SECRET_RESOLVED" \
    WHOP_WEBHOOK_SECRET="$WHOP_WEBHOOK_SECRET_RESOLVED" \
    CARGO_TARGET_DIR="$TARGET_DIR" \
    PORT="$port" \
    cargo run -p api --bin api-server >"$backend_log" 2>&1
  ) &
  local backend_pid=$!

  if ! wait_port "$port"; then
    echo "Failed to start backend for $label ($commit) on port $port"
    tail -n 120 "$backend_log" || true
    kill "$backend_pid" || true
    wait "$backend_pid" || true
    exit 1
  fi

  API_BASE="http://127.0.0.1:$port" \
  VIDEO_URL="$VIDEO_URL" \
  PLAYLIST_URL="$PLAYLIST_URL" \
  LEVELS="$LEVELS" \
  OUT_DIR="$outdir/suite" \
  "$ROOT_DIR/scripts/stress-test-comprehensive-suite.sh" >"$suite_log" 2>&1

  kill "$backend_pid" || true
  wait "$backend_pid" || true
  kill_port "$port"
}

build_compare() {
  local new_dir="$1"
  local old_dir="$2"
  local out_file="$3"
  node - "$new_dir" "$old_dir" "$out_file" <<'NODE'
const fs = require('fs');
const path = require('path');

const [newDir, oldDir, outFile] = process.argv.slice(2);

function readTsv(file) {
  if (!fs.existsSync(file)) return [];
  const lines = fs.readFileSync(file, 'utf8').trim().split('\n');
  if (lines.length < 2) return [];
  const headers = lines[0].split('\t');
  return lines.slice(1).map((line) => {
    const cols = line.split('\t');
    return Object.fromEntries(headers.map((h, i) => [h, cols[i] ?? '']));
  });
}

function keyHttp(r) { return r.case || `${r.level}|${r.endpoint}`; }
function keyAsync(r) { return r.case || `${r.level}|${r.endpoint}`; }
function keyBatch(r) { return r.case || `${r.level}|${r.endpoint}`; }

const newHttp = readTsv(path.join(newDir, 'suite', 'http-summary.tsv'));
const oldHttp = readTsv(path.join(oldDir, 'suite', 'http-summary.tsv'));
const newAsync = readTsv(path.join(newDir, 'suite', 'async-summary.tsv'));
const oldAsync = readTsv(path.join(oldDir, 'suite', 'async-summary.tsv'));
const newBatch = readTsv(path.join(newDir, 'suite', 'batch-summary.tsv'));
const oldBatch = readTsv(path.join(oldDir, 'suite', 'batch-summary.tsv'));

const rows = [];

const oldHttpMap = new Map(oldHttp.map((r) => [keyHttp(r), r]));
for (const n of newHttp) {
  const k = keyHttp(n);
  const o = oldHttpMap.get(k);
  rows.push({
    section: 'http',
    key: k,
    metric: 'success_rate_pct',
    old: o ? Number(o.success_rate_pct || 0) : 0,
    now: Number(n.success_rate_pct || 0),
    delta: Number((Number(n.success_rate_pct || 0) - (o ? Number(o.success_rate_pct || 0) : 0)).toFixed(2)),
  });
}

const oldBatchMap = new Map(oldBatch.map((r) => [keyBatch(r), r]));
for (const n of newBatch) {
  const k = keyBatch(n);
  const o = oldBatchMap.get(k);
  rows.push({
    section: 'batch',
    key: k,
    metric: 'done_rate_pct',
    old: o ? Number(o.done_rate_pct || 0) : 0,
    now: Number(n.done_rate_pct || 0),
    delta: Number((Number(n.done_rate_pct || 0) - (o ? Number(o.done_rate_pct || 0) : 0)).toFixed(2)),
  });
}

const oldAsyncMap = new Map(oldAsync.map((r) => [keyAsync(r), r]));
for (const n of newAsync) {
  const k = keyAsync(n);
  const o = oldAsyncMap.get(k);
  const metrics = [
    'accept_pct',
    'ready_pct_of_accepted',
    'ready_within_5m_pct',
    'eventual_ready_pct',
  ];
  for (const metric of metrics) {
    const oldVal = o ? Number(o[metric] || 0) : 0;
    const newVal = Number(n[metric] || 0);
    rows.push({
      section: 'async',
      key: k,
      metric,
      old: oldVal,
      now: newVal,
      delta: Number((newVal - oldVal).toFixed(2)),
    });
  }
}

const header = ['section', 'key', 'metric', 'old', 'now', 'delta'];
const tsv = [header.join('\t'), ...rows.map((r) => header.map((h) => r[h]).join('\t'))].join('\n');
fs.writeFileSync(outFile, tsv);
NODE
}

run_one_commit "new" "$NEW_COMMIT" "$NEW_WORKTREE" "$NEW_OUT" 4068
run_one_commit "old" "$OLD_COMMIT" "$OLD_WORKTREE" "$OLD_OUT" 4069
build_compare "$NEW_OUT" "$OLD_OUT" "$COMPARE_TSV"

echo "OUT_BASE=$OUT_BASE"
echo "NEW_HTTP=$NEW_OUT/suite/http-summary.tsv"
echo "NEW_BATCH=$NEW_OUT/suite/batch-summary.tsv"
echo "NEW_ASYNC=$NEW_OUT/suite/async-summary.tsv"
echo "OLD_HTTP=$OLD_OUT/suite/http-summary.tsv"
echo "OLD_BATCH=$OLD_OUT/suite/batch-summary.tsv"
echo "OLD_ASYNC=$OLD_OUT/suite/async-summary.tsv"
echo "COMPARE_TSV=$COMPARE_TSV"
echo "---COMPARE---"
cat "$COMPARE_TSV"
