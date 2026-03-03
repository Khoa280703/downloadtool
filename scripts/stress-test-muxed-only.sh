#!/usr/bin/env bash
set -euo pipefail

API_BASE="${API_BASE:-http://127.0.0.1:3068}"
VIDEO_URL="${VIDEO_URL:-https://www.youtube.com/watch?v=dQw4w9WgXcQ}"
RUN_ID="$(date +%Y%m%d-%H%M%S)"
OUT_DIR="${OUT_DIR:-/tmp/stress-muxed-only-${RUN_ID}}"
mkdir -p "$OUT_DIR"

# Format: create_concurrency:total_requests:poll_concurrency:poll_timeout_ms
CASES="${CASES:-24:300:96:180000 48:600:128:240000 80:1200:256:300000 120:2000:256:300000}"

PAYLOAD_FILE="$OUT_DIR/payload.json"
SUMMARY_FILE="$OUT_DIR/summary.tsv"

echo -e "case\tapi\ttotal\tcreate_conc\tpoll_conc\tpoll_timeout_ms\taccepted\trejected_503\tready\tfailed\ttimeout\taccept_pct\tready_pct_of_accepted\tcreate_p95_ms\tpoll_p95_ms" > "$SUMMARY_FILE"

echo "[INFO] Preparing payload from /api/extract ..."
extract_resp="$OUT_DIR/extract-response.json"
curl -sS -X POST "${API_BASE}/api/extract" \
  -H "content-type: application/json" \
  --data "{\"url\":\"${VIDEO_URL}\"}" > "$extract_resp"

node - "$extract_resp" "$PAYLOAD_FILE" <<'NODE'
const fs = require('fs');
const inFile = process.argv[2];
const outFile = process.argv[3];
const data = JSON.parse(fs.readFileSync(inFile, 'utf8'));
const formats = data?.metadata?.formats || [];
const score = (q = '') => {
  const m = String(q).toLowerCase().match(/(\d{3,4})p/);
  return m ? Number(m[1]) : 0;
};
const video = [...formats]
  .filter((f) => f && f.url && f.is_audio_only === false && f.has_audio === false && String(f.ext).toLowerCase() === 'mp4')
  .sort((a, b) => score(b.quality) - score(a.quality))[0];
const audio = [...formats]
  .filter((f) => f && f.url && f.is_audio_only === true && String(f.ext).toLowerCase() === 'm4a')
  .sort((a, b) => (b.bitrate || 0) - (a.bitrate || 0))[0];
if (!video || !audio) {
  console.error('Cannot prepare MP4+M4A payload');
  process.exit(1);
}
const payload = {
  video_url: video.url,
  audio_url: audio.url,
  source_url: data?.metadata?.original_url || data?.metadata?.webpage_url || null,
  video_format_id: video.format_id || null,
  audio_format_id: audio.format_id || null,
  title: data?.metadata?.title || 'stress-muxed',
};
fs.writeFileSync(outFile, JSON.stringify(payload));
console.log(JSON.stringify({
  selected_video_format_id: video.format_id,
  selected_video_quality: video.quality,
  selected_audio_format_id: audio.format_id,
  selected_audio_bitrate: audio.bitrate || 0,
}, null, 2));
NODE

echo "[INFO] Running muxed async matrix ..."
case_no=0
for item in $CASES; do
  case_no=$((case_no + 1))
  IFS=":" read -r create_conc total poll_conc poll_timeout <<< "$item"
  case_name="muxed-async-case-${case_no}"
  out_json="$OUT_DIR/${case_name}.json"

  echo "[RUN] ${case_name} total=${total} create=${create_conc} poll=${poll_conc} timeout=${poll_timeout}"
  API_BASE="$API_BASE" \
  PAYLOAD_FILE="$PAYLOAD_FILE" \
  CREATE_TOTAL="$total" \
  CREATE_CONC="$create_conc" \
  POLL_CONC="$poll_conc" \
  POLL_TIMEOUT_MS="$poll_timeout" \
  node scripts/stress-test-muxed-jobs.mjs > "$out_json"

  node - "$out_json" "$SUMMARY_FILE" "$case_name" "$API_BASE" "$total" "$create_conc" "$poll_conc" "$poll_timeout" <<'NODE'
const fs = require('fs');
const [outJson, summaryFile, caseName, apiBase, total, createConc, pollConc, pollTimeout] = process.argv.slice(2);
const d = JSON.parse(fs.readFileSync(outJson, 'utf8'));
const accepted = d?.create?.accepted || 0;
const rejected503 = d?.create?.codeCounts?.['503'] || 0;
const ready = d?.poll?.finalCounts?.ready || 0;
const failed = d?.poll?.finalCounts?.failed || 0;
const timeout = d?.poll?.finalCounts?.timeout || 0;
const totalNum = Number(total) || 0;
const acceptPct = totalNum > 0 ? ((accepted / totalNum) * 100).toFixed(2) : '0.00';
const readyPctAccepted = accepted > 0 ? ((ready / accepted) * 100).toFixed(2) : '0.00';
const row = [
  caseName,
  apiBase,
  total,
  createConc,
  pollConc,
  pollTimeout,
  accepted,
  rejected503,
  ready,
  failed,
  timeout,
  acceptPct,
  readyPctAccepted,
  d?.create?.p95Ms || 0,
  d?.poll?.p95Ms || 0,
].join('\t');
fs.appendFileSync(summaryFile, row + '\n');
NODE
done

echo "SUMMARY_FILE=$SUMMARY_FILE"
cat "$SUMMARY_FILE"
