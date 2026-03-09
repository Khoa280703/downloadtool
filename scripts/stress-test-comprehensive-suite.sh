#!/usr/bin/env bash
set -euo pipefail

API_BASE="${API_BASE:-http://127.0.0.1:3068}"
VIDEO_URL="${VIDEO_URL:-https://www.youtube.com/watch?v=dQw4w9WgXcQ}"
PLAYLIST_URL="${PLAYLIST_URL:-https://www.youtube.com/playlist?list=PLgIS1MvgCaOVR07APOp8zfBLHbh7VGllX}"
LEVELS="${LEVELS:-smoke medium heavy extreme}"
LOAD_MULTIPLIER="${LOAD_MULTIPLIER:-1}"
ASYNC_LOAD_MULTIPLIER="${ASYNC_LOAD_MULTIPLIER:-$LOAD_MULTIPLIER}"
SCALE_POLL_TIMEOUT="${SCALE_POLL_TIMEOUT:-0}"

RUN_ID="$(date +%Y%m%d-%H%M%S)"
OUT_DIR="${OUT_DIR:-/tmp/stress-suite-${RUN_ID}}"
mkdir -p "$OUT_DIR"

if ! [[ "$LOAD_MULTIPLIER" =~ ^[0-9]+$ ]] || [[ "$LOAD_MULTIPLIER" -lt 1 ]]; then
  echo "LOAD_MULTIPLIER must be a positive integer (current: $LOAD_MULTIPLIER)" >&2
  exit 1
fi

if ! [[ "$ASYNC_LOAD_MULTIPLIER" =~ ^[0-9]+$ ]] || [[ "$ASYNC_LOAD_MULTIPLIER" -lt 1 ]]; then
  echo "ASYNC_LOAD_MULTIPLIER must be a positive integer (current: $ASYNC_LOAD_MULTIPLIER)" >&2
  exit 1
fi

if ! [[ "$SCALE_POLL_TIMEOUT" =~ ^[01]$ ]]; then
  echo "SCALE_POLL_TIMEOUT must be 0 or 1 (current: $SCALE_POLL_TIMEOUT)" >&2
  exit 1
fi

HTTP_SUMMARY="$OUT_DIR/http-summary.tsv"
BATCH_SUMMARY="$OUT_DIR/batch-summary.tsv"
ASYNC_SUMMARY="$OUT_DIR/async-summary.tsv"
PAYLOAD_FILE="$OUT_DIR/mux-payload.json"
EXTRACT_JSON="$OUT_DIR/extract-once.json"

echo -e "case\tlevel\tendpoint\tconcurrency\ttotal\ttimeout_s\thttp200\thttp403\thttp429\thttp5xx\thttp4xx_other\thttp000\tsuccess_rate_pct\tp50_s\tp95_s\tp99_s\txargs_rc" > "$HTTP_SUMMARY"
echo -e "case\tlevel\tendpoint\tconcurrency\ttotal\ttimeout_s\thttp200\thttp_non_200\tdone_ok\terror_event\tno_done\tsuccess_rate_pct\tdone_rate_pct\tp50_s\tp95_s\tp99_s\txargs_rc" > "$BATCH_SUMMARY"
echo -e "case\tlevel\tendpoint\tcreate_conc\tpoll_conc\ttotal\taccepted\tcode202\tcode503\tcode000\tcode_other\tready\tfailed\ttimeout\tnot_found\tready_within_5m\teventual_ready\taccept_pct\tready_pct_of_accepted\tready_within_5m_pct\teventual_ready_pct\tcreate_p95_ms\tpoll_p95_ms" > "$ASYNC_SUMMARY"

percentile_col() {
  local file="$1"
  local col="$2"
  local p="$3"
  awk -F'\t' -v c="$col" '{if ($c != "") print $c}' "$file" | sort -n | awk -v p="$p" '
    {a[NR]=$1}
    END{
      if (NR==0) { print 0; exit }
      idx=int((NR-1)*p)+1
      print a[idx]
    }
  '
}

prepare_payload() {
  curl -sS -X POST "$API_BASE/api/extract" \
    -H "content-type: application/json" \
    --data "{\"url\":\"$VIDEO_URL\"}" > "$EXTRACT_JSON"

  node - "$EXTRACT_JSON" "$PAYLOAD_FILE" <<'NODE'
const fs = require('fs');
const inFile = process.argv[2];
const outFile = process.argv[3];
const d = JSON.parse(fs.readFileSync(inFile, 'utf8'));
const f = d?.metadata?.formats || [];
const score = (q = '') => {
  const m = String(q).toLowerCase().match(/(\d{3,4})p/);
  return m ? Number(m[1]) : 0;
};
const stream = f.find((x) => x.ext === 'mp4' && x.has_audio) || f.find((x) => x.ext === 'mp4') || f[0];
const video = [...f]
  .filter((x) => !x.is_audio_only && !x.has_audio && String(x.ext).toLowerCase() === 'mp4')
  .sort((a, b) => score(b.quality) - score(a.quality))[0] || stream;
const audio = [...f]
  .filter((x) => x.is_audio_only && String(x.ext).toLowerCase() === 'm4a')
  .sort((a, b) => (b.bitrate || 0) - (a.bitrate || 0))[0] || stream;
if (!stream || !video || !audio || !stream.url || !video.url || !audio.url) {
  console.error('Failed to prepare stream URLs from /api/extract');
  process.exit(1);
}
const payload = {
  video_url: video.url,
  audio_url: audio.url,
  source_url: d?.metadata?.original_url || d?.metadata?.webpage_url || null,
  video_format_id: video.format_id || null,
  audio_format_id: audio.format_id || null,
  title: d?.metadata?.title || 'stress-suite',
  stream_url: stream.url,
  stream_format_id: stream.format_id || '',
};
fs.writeFileSync(outFile, JSON.stringify(payload));
NODE
}

run_http_case() {
  local case_name="$1"
  local level="$2"
  local endpoint="$3"
  local concurrency="$4"
  local total="$5"
  local timeout_s="$6"
  local runner="$7"
  local out="$OUT_DIR/${case_name}.tsv"

  : > "$out"
  local xrc
  set +e
  seq "$total" | xargs -P "$concurrency" -I{} bash -lc "$runner" >> "$out"
  xrc=$?
  set -e

  local http200 http403 http429 http5xx http4xx_other http000 p50 p95 p99 success_rate
  http200=$(awk -F'\t' '$1==200{c++} END{print c+0}' "$out")
  http403=$(awk -F'\t' '$1==403{c++} END{print c+0}' "$out")
  http429=$(awk -F'\t' '$1==429{c++} END{print c+0}' "$out")
  http5xx=$(awk -F'\t' '$1>=500 && $1<600{c++} END{print c+0}' "$out")
  http4xx_other=$(awk -F'\t' '$1>=400 && $1<500 && $1!=403 && $1!=429{c++} END{print c+0}' "$out")
  http000=$(awk -F'\t' '$1==0 || $1=="000"{c++} END{print c+0}' "$out")
  p50=$(percentile_col "$out" 2 0.50)
  p95=$(percentile_col "$out" 2 0.95)
  p99=$(percentile_col "$out" 2 0.99)
  success_rate=$(awk -v ok="$http200" -v total="$total" 'BEGIN{if(total==0){print "0.00"}else{printf "%.2f", (ok/total)*100}}')

  echo -e "${case_name}\t${level}\t${endpoint}\t${concurrency}\t${total}\t${timeout_s}\t${http200}\t${http403}\t${http429}\t${http5xx}\t${http4xx_other}\t${http000}\t${success_rate}\t${p50}\t${p95}\t${p99}\t${xrc}" >> "$HTTP_SUMMARY"
}

run_batch_case() {
  local case_name="$1"
  local level="$2"
  local concurrency="$3"
  local total="$4"
  local timeout_s="$5"
  local out="$OUT_DIR/${case_name}.tsv"

  : > "$out"
  local xrc
  set +e
  seq "$total" | xargs -P "$concurrency" -I{} bash -lc '
    tmp=$(mktemp)
    set +e
    meta=$(curl -sS -N --max-time "'"$timeout_s"'" -G "'"$API_BASE"'/api/batch" --data-urlencode "url='"$PLAYLIST_URL"'" -o "$tmp" -w "%{http_code}\t%{time_total}")
    rc=$?
    set -e
    code=$(echo "$meta" | cut -f1)
    t=$(echo "$meta" | cut -f2)
    [ -z "$code" ] && code=000
    [ -z "$t" ] && t=0
    done_n=$( (grep -o "\"type\":\"done\"" "$tmp" || true) | wc -l | tr -d " " )
    err_n=$( (grep -o "\"type\":\"error\"" "$tmp" || true) | wc -l | tr -d " " )
    echo -e "${code}\t${t}\t${rc}\t${done_n}\t${err_n}"
    rm -f "$tmp"
  ' >> "$out"
  xrc=$?
  set -e

  local http200 http_non_200 done_ok error_event no_done p50 p95 p99 success_rate done_rate
  http200=$(awk -F'\t' '$1==200{c++} END{print c+0}' "$out")
  http_non_200=$(awk -F'\t' '$1!=200{c++} END{print c+0}' "$out")
  done_ok=$(awk -F'\t' '$4>0{c++} END{print c+0}' "$out")
  error_event=$(awk -F'\t' '$5>0{c++} END{print c+0}' "$out")
  no_done=$(awk -F'\t' '$4==0{c++} END{print c+0}' "$out")
  p50=$(percentile_col "$out" 2 0.50)
  p95=$(percentile_col "$out" 2 0.95)
  p99=$(percentile_col "$out" 2 0.99)
  success_rate=$(awk -v ok="$http200" -v total="$total" 'BEGIN{if(total==0){print "0.00"}else{printf "%.2f", (ok/total)*100}}')
  done_rate=$(awk -v ok="$done_ok" -v total="$total" 'BEGIN{if(total==0){print "0.00"}else{printf "%.2f", (ok/total)*100}}')

  echo -e "${case_name}\t${level}\t/api/batch\t${concurrency}\t${total}\t${timeout_s}\t${http200}\t${http_non_200}\t${done_ok}\t${error_event}\t${no_done}\t${success_rate}\t${done_rate}\t${p50}\t${p95}\t${p99}\t${xrc}" >> "$BATCH_SUMMARY"
}

run_async_case() {
  local case_name="$1"
  local level="$2"
  local total="$3"
  local create_conc="$4"
  local poll_conc="$5"
  local poll_timeout_ms="$6"
  local out_json="$OUT_DIR/${case_name}.json"

  API_BASE="$API_BASE" \
  PAYLOAD_FILE="$PAYLOAD_FILE" \
  CREATE_TOTAL="$total" \
  CREATE_CONC="$create_conc" \
  POLL_CONC="$poll_conc" \
  POLL_TIMEOUT_MS="$poll_timeout_ms" \
  node "$(dirname "$0")/stress-test-muxed-jobs.mjs" > "$out_json"

  node - "$out_json" "$ASYNC_SUMMARY" "$case_name" "$level" "$create_conc" "$poll_conc" "$total" <<'NODE'
const fs = require('fs');
const [outJson, summaryFile, caseName, level, createConc, pollConc, total] = process.argv.slice(2);
const d = JSON.parse(fs.readFileSync(outJson, 'utf8'));
const cc = d?.create?.codeCounts || {};
const fc = d?.poll?.finalCounts || {};
const accepted = Number(d?.create?.accepted || 0);
const totalNum = Number(total) || 0;
const acceptPct = totalNum > 0 ? ((accepted / totalNum) * 100).toFixed(2) : '0.00';
const readyPct = accepted > 0 ? (((fc.ready || 0) / accepted) * 100).toFixed(2) : '0.00';
const row = [
  caseName,
  level,
  '/api/jobs',
  createConc,
  pollConc,
  total,
  accepted,
  cc['202'] || 0,
  cc['503'] || 0,
  cc['0'] || 0,
  Math.max(0, Object.entries(cc).filter(([k]) => !['202', '503', '0'].includes(k)).reduce((a, [,v]) => a + Number(v||0), 0)),
  fc.ready || 0,
  fc.failed || 0,
  fc.timeout || 0,
  fc.not_found || 0,
  d?.poll?.readyWithin5m || 0,
  d?.poll?.eventualReady || 0,
  acceptPct,
  readyPct,
  d?.poll?.readyWithin5mPctOfAccepted ?? 0,
  d?.poll?.eventualReadyPctOfAccepted ?? 0,
  d?.create?.p95Ms || 0,
  d?.poll?.p95Ms || 0,
].join('\t');
fs.appendFileSync(summaryFile, row + '\n');
NODE
}

level_params() {
  local level="$1"
  case "$level" in
    smoke)   echo "10 80 10 80 8 40 4 24 3 20 40 8 16 300000" ;;
    medium)  echo "24 240 24 240 16 120 8 60 6 40 120 16 32 300000" ;;
    heavy)   echo "48 600 48 600 32 300 16 150 10 80 300 24 64 300000" ;;
    extreme) echo "80 1200 80 1200 48 600 24 240 14 120 600 48 96 300000" ;;
    *) echo "Unsupported level: $level" >&2; return 1 ;;
  esac
}

prepare_payload
read -r STREAM_URL STREAM_FORMAT_ID VIDEO_URL_ENC VIDEO_FORMAT_ID AUDIO_URL_ENC AUDIO_FORMAT_ID < <(node - "$PAYLOAD_FILE" <<'NODE'
const fs = require('fs');
const p = JSON.parse(fs.readFileSync(process.argv[2], 'utf8'));
console.log([
  p.stream_url || '',
  p.stream_format_id || '',
  p.video_url || '',
  p.video_format_id || '',
  p.audio_url || '',
  p.audio_format_id || '',
].join(' '));
NODE
)

if [[ -z "${STREAM_URL:-}" || -z "${VIDEO_URL_ENC:-}" || -z "${AUDIO_URL_ENC:-}" ]]; then
  echo "Missing stream URLs from payload" >&2
  exit 1
fi

export API_BASE VIDEO_URL PLAYLIST_URL STREAM_URL STREAM_FORMAT_ID VIDEO_URL_ENC VIDEO_FORMAT_ID AUDIO_URL_ENC AUDIO_FORMAT_ID

for level in $LEVELS; do
  read -r EX_C EX_N S4_C S4_N S1_C S1_N MX_C MX_N B_C B_N AX_N AX_C AX_P AX_T <<EOF
$(level_params "$level")
EOF

  if [[ "$LOAD_MULTIPLIER" -gt 1 ]]; then
    EX_C=$((EX_C * LOAD_MULTIPLIER))
    EX_N=$((EX_N * LOAD_MULTIPLIER))
    S4_C=$((S4_C * LOAD_MULTIPLIER))
    S4_N=$((S4_N * LOAD_MULTIPLIER))
    S1_C=$((S1_C * LOAD_MULTIPLIER))
    S1_N=$((S1_N * LOAD_MULTIPLIER))
    MX_C=$((MX_C * LOAD_MULTIPLIER))
    MX_N=$((MX_N * LOAD_MULTIPLIER))
    B_C=$((B_C * LOAD_MULTIPLIER))
    B_N=$((B_N * LOAD_MULTIPLIER))
  fi

  if [[ "$ASYNC_LOAD_MULTIPLIER" -gt 1 ]]; then
    AX_N=$((AX_N * ASYNC_LOAD_MULTIPLIER))
    AX_C=$((AX_C * ASYNC_LOAD_MULTIPLIER))
    AX_P=$((AX_P * ASYNC_LOAD_MULTIPLIER))
    if [[ "$SCALE_POLL_TIMEOUT" == "1" ]]; then
      AX_T=$((AX_T * ASYNC_LOAD_MULTIPLIER))
    fi
  fi

  run_http_case "${level}-extract" "$level" "/api/extract" "$EX_C" "$EX_N" 12 'set +e; m=$(curl -sS --max-time 12 -o /dev/null -w "%{http_code}\t%{time_total}" -X POST "$API_BASE/api/extract" -H "content-type: application/json" --data "{\"url\":\"$VIDEO_URL\"}"); rc=$?; set -e; c=$(echo "$m"|cut -f1); t=$(echo "$m"|cut -f2); [ -z "$c" ]&&c=000; [ -z "$t" ]&&t=0; echo -e "$c\t$t\t0\t$rc"'

  run_http_case "${level}-stream4k" "$level" "/api/stream" "$S4_C" "$S4_N" 20 'set +e; m=$(curl -sS --max-time 20 -o /dev/null -w "%{http_code}\t%{time_total}" -G "$API_BASE/api/stream" -H "Range: bytes=0-4095" --data-urlencode "url=$STREAM_URL" --data-urlencode "format_id=$STREAM_FORMAT_ID" --data-urlencode "title=stress" --data-urlencode "format=mp4"); rc=$?; set -e; c=$(echo "$m"|cut -f1); t=$(echo "$m"|cut -f2); [ -z "$c" ]&&c=000; [ -z "$t" ]&&t=0; echo -e "$c\t$t\t0\t$rc"'

  run_http_case "${level}-stream1m" "$level" "/api/stream" "$S1_C" "$S1_N" 30 'set +e; m=$(curl -sS --max-time 30 -o /dev/null -w "%{http_code}\t%{time_total}" -G "$API_BASE/api/stream" -H "Range: bytes=0-1048575" --data-urlencode "url=$STREAM_URL" --data-urlencode "format_id=$STREAM_FORMAT_ID" --data-urlencode "title=stress" --data-urlencode "format=mp4"); rc=$?; set -e; c=$(echo "$m"|cut -f1); t=$(echo "$m"|cut -f2); [ -z "$c" ]&&c=000; [ -z "$t" ]&&t=0; echo -e "$c\t$t\t0\t$rc"'

  run_batch_case "${level}-batch" "$level" "$B_C" "$B_N" 120
  run_async_case "${level}-muxed-async" "$level" "$AX_N" "$AX_C" "$AX_P" "$AX_T"
done

echo "OUT_DIR=$OUT_DIR"
echo "HTTP_SUMMARY=$HTTP_SUMMARY"
echo "BATCH_SUMMARY=$BATCH_SUMMARY"
echo "ASYNC_SUMMARY=$ASYNC_SUMMARY"
