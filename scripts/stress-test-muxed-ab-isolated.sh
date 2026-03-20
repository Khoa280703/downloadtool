#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

VIDEO_URL="${VIDEO_URL:-https://www.youtube.com/watch?v=dQw4w9WgXcQ}"
# Format: create_conc:total:poll_conc:poll_timeout_ms
CASES="${CASES:-24:300:96:900000}"
RUN_ID="$(date +%Y%m%d-%H%M%S)"
OUT_BASE="${OUT_BASE:-/tmp/stress-muxed-ab-isolated-${RUN_ID}}"
mkdir -p "$OUT_BASE"

SUMMARY_TSV="$OUT_BASE/ab-summary.tsv"
echo -e "mode\tcase\tport\taccepted\trejected_503\tready\tfailed\ttimeout\taccept_pct\tready_pct_of_accepted\tready_within_5m\tready_within_5m_pct\teventual_ready\teventual_ready_pct\tcreate_p95_ms\tpoll_p95_ms\tresult_json\tsummary_tsv" > "$SUMMARY_TSV"

wait_port() {
  local port="$1"
  local retries=60
  for _ in $(seq 1 "$retries"); do
    if ss -ltn "( sport = :${port} )" | tail -n +2 | grep -q .; then
      return 0
    fi
    sleep 1
  done
  return 1
}

run_one_mode() {
  local mode="$1"
  local port="$2"

  local mode_dir="$OUT_BASE/${mode}"
  local backend_log="$mode_dir/backend.log"
  mkdir -p "$mode_dir"

  echo "[INFO] Start backend mode=${mode} port=${port}"
  env \
    PORT="$port" \
    cargo run -p api --bin api-server >"$backend_log" 2>&1 &
  local backend_pid=$!

  if ! wait_port "$port"; then
    echo "[ERROR] Backend not listening on ${port} (mode=${mode})"
    tail -n 120 "$backend_log" || true
    kill "$backend_pid" || true
    wait "$backend_pid" || true
    return 1
  fi

  echo "[INFO] Run cases mode=${mode}"
  local case_index=0
  for item in $CASES; do
    case_index=$((case_index + 1))
    local case_name="case-${case_index}"
    local case_out="$mode_dir/${case_name}"
    mkdir -p "$case_out"

    API_BASE="http://127.0.0.1:${port}" \
    VIDEO_URL="$VIDEO_URL" \
    CASES="$item" \
    OUT_DIR="$case_out" \
    scripts/stress-test-muxed-only.sh >"$case_out/run.log" 2>&1

    local result_json="$case_out/muxed-async-case-1.json"
    local summary_case_tsv="$case_out/summary.tsv"
    if [[ ! -f "$result_json" || ! -f "$summary_case_tsv" ]]; then
      echo "[ERROR] Missing result files for mode=${mode} ${case_name}"
      tail -n 120 "$case_out/run.log" || true
      kill "$backend_pid" || true
      wait "$backend_pid" || true
      return 1
    fi

    node - "$result_json" "$summary_case_tsv" "$SUMMARY_TSV" "$mode" "$case_name" "$port" <<'NODE'
const fs = require('fs');
const [resultJsonPath, caseSummaryPath, outSummaryPath, mode, caseName, port] = process.argv.slice(2);
const result = JSON.parse(fs.readFileSync(resultJsonPath, 'utf8'));
const lines = fs.readFileSync(caseSummaryPath, 'utf8').trim().split('\n');
const head = lines[0].split('\t');
const vals = lines[1].split('\t');
const row = Object.fromEntries(head.map((k, i) => [k, vals[i] ?? '']));

const accepted = Number(row.accepted || 0);
const rejected503 = Number(row.rejected_503 || 0);
const ready = Number(row.ready || 0);
const failed = Number(row.failed || 0);
const timeout = Number(row.timeout || 0);

const append = [
  mode,
  caseName,
  port,
  accepted,
  rejected503,
  ready,
  failed,
  timeout,
  row.accept_pct || '0.00',
  row.ready_pct_of_accepted || '0.00',
  result?.poll?.readyWithin5m || 0,
  result?.poll?.readyWithin5mPctOfAccepted || 0,
  result?.poll?.eventualReady || 0,
  result?.poll?.eventualReadyPctOfAccepted || 0,
  result?.create?.p95Ms || 0,
  result?.poll?.p95Ms || 0,
  resultJsonPath,
  caseSummaryPath
].join('\t');

fs.appendFileSync(outSummaryPath, append + '\n');
NODE
  done

  echo "[INFO] Stop backend mode=${mode}"
  kill "$backend_pid" || true
  wait "$backend_pid" || true
}

# A (baseline defaults)
run_one_mode "baseline" "4070"

# B (repeat)
run_one_mode "repeat" "4071"

echo "AB_SUMMARY_FILE=$SUMMARY_TSV"
cat "$SUMMARY_TSV"
