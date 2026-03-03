#!/usr/bin/env bash
set -euo pipefail

API_BASE="${API_BASE:-http://127.0.0.1:3068}"
BASE_JSON_INPUT="${BASE_JSON_INPUT:-/tmp/stress-rootcause-20260302-145152/extract-once.json}"
STREAM_CONCURRENCY="${STREAM_CONCURRENCY:-140}"
STREAM_TOTAL="${STREAM_TOTAL:-4000}"
STREAM_TIMEOUT_SECS="${STREAM_TIMEOUT_SECS:-15}"
MUXED_CONCURRENCY="${MUXED_CONCURRENCY:-60}"
MUXED_TOTAL="${MUXED_TOTAL:-1200}"
MUXED_TIMEOUT_SECS="${MUXED_TIMEOUT_SECS:-30}"

RUN_ID="$(date +%Y%m%d-%H%M%S)"
OUT_DIR="${OUT_DIR:-/tmp/stress-refresh-probe-${RUN_ID}}"
mkdir -p "$OUT_DIR"
SUMMARY_FILE="$OUT_DIR/summary.tsv"
echo -e "case\tendpoint\tconcurrency\ttotal\ttimeout_s\thttp200\thttp5xx\thttp4xx\thttp000\trc0\trc28\trc18\tp50_s\tp95_s\tp99_s\txargs_rc" > "$SUMMARY_FILE"

if [[ ! -f "$BASE_JSON_INPUT" ]]; then
  echo "Missing BASE_JSON_INPUT=$BASE_JSON_INPUT" >&2
  exit 1
fi

percentile() {
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

read -r SOURCE_URL STREAM_URL STREAM_FORMAT_ID VIDEO_URL VIDEO_FORMAT_ID AUDIO_URL AUDIO_FORMAT_ID < <(node - "$BASE_JSON_INPUT" <<'NODE'
const fs=require('fs');
const d=JSON.parse(fs.readFileSync(process.argv[2],'utf8'));
const f=(d?.metadata?.formats)||[];
const score=(q='')=>{const s=String(q).toLowerCase();const m=s.match(/(\d{3,4})p/);return m?Number(m[1]):0};
const stream=f.find(x=>x.ext==='mp4'&&x.has_audio)||f.find(x=>x.ext==='mp4')||f[0];
const video=[...f].filter(x=>!x.is_audio_only&&!x.has_audio&&x.ext==='mp4').sort((a,b)=>score(b.quality)-score(a.quality))[0]||stream;
const audio=[...f].filter(x=>x.is_audio_only&&x.ext==='m4a').sort((a,b)=>(b.bitrate||0)-(a.bitrate||0))[0]||stream;
console.log([
  d?.metadata?.original_url||'',
  stream?.url||'',
  stream?.format_id||'',
  video?.url||'',
  video?.format_id||'',
  audio?.url||'',
  audio?.format_id||''
].join(' '));
NODE
)

if [[ -z "$SOURCE_URL" || -z "$STREAM_URL" || -z "$VIDEO_URL" || -z "$AUDIO_URL" ]]; then
  echo "Failed to parse URLs from BASE_JSON_INPUT" >&2
  exit 1
fi

export API_BASE SOURCE_URL STREAM_URL STREAM_FORMAT_ID VIDEO_URL VIDEO_FORMAT_ID AUDIO_URL AUDIO_FORMAT_ID

run_http_case() {
  local case_name="$1"
  local endpoint="$2"
  local concurrency="$3"
  local total="$4"
  local timeout_s="$5"
  local runner="$6"

  local out="$OUT_DIR/${case_name}.tsv"
  : > "$out"
  local xrc
  set +e
  seq "$total" | xargs -P "$concurrency" -I{} bash -lc "$runner" >> "$out"
  xrc=$?
  set -e

  local http200 http5xx http4xx http000 rc0 rc28 rc18 p50 p95 p99
  http200=$(awk -F'\t' '$1==200{c++} END{print c+0}' "$out")
  http5xx=$(awk -F'\t' '$1>=500 && $1<600{c++} END{print c+0}' "$out")
  http4xx=$(awk -F'\t' '$1>=400 && $1<500{c++} END{print c+0}' "$out")
  http000=$(awk -F'\t' '$1==0 || $1=="000"{c++} END{print c+0}' "$out")
  rc0=$(awk -F'\t' '$4==0{c++} END{print c+0}' "$out")
  rc28=$(awk -F'\t' '$4==28{c++} END{print c+0}' "$out")
  rc18=$(awk -F'\t' '$4==18{c++} END{print c+0}' "$out")
  p50=$(percentile "$out" 2 0.50)
  p95=$(percentile "$out" 2 0.95)
  p99=$(percentile "$out" 2 0.99)

  echo -e "${case_name}\t${endpoint}\t${concurrency}\t${total}\t${timeout_s}\t${http200}\t${http5xx}\t${http4xx}\t${http000}\t${rc0}\t${rc28}\t${rc18}\t${p50}\t${p95}\t${p99}\t${xrc}" >> "$SUMMARY_FILE"
  echo "[DONE] ${case_name}: ok=${http200}/${total} 5xx=${http5xx} 4xx=${http4xx} 000=${http000} rc28=${rc28} p95=${p95}s"
}

run_http_case "stream4k-refresh" "/api/stream" "$STREAM_CONCURRENCY" "$STREAM_TOTAL" "$STREAM_TIMEOUT_SECS" 'set +e; m=$(curl -sS --max-time "'"$STREAM_TIMEOUT_SECS"'" -o /dev/null -w "%{http_code}\t%{time_total}\t%{size_download}" -G "$API_BASE/api/stream" -H "Range: bytes=0-4095" --data-urlencode "url=$STREAM_URL" --data-urlencode "source_url=$SOURCE_URL" --data-urlencode "format_id=$STREAM_FORMAT_ID" --data-urlencode "title=stress" --data-urlencode "format=mp4"); rc=$?; set -e; c=$(echo "$m"|cut -f1); t=$(echo "$m"|cut -f2); s=$(echo "$m"|cut -f3); [ -z "$c" ]&&c=000; [ -z "$t" ]&&t=0; [ -z "$s" ]&&s=0; echo -e "$c\t$t\t$s\t$rc"'
run_http_case "muxed-refresh" "/api/stream/muxed" "$MUXED_CONCURRENCY" "$MUXED_TOTAL" "$MUXED_TIMEOUT_SECS" 'set +e; m=$(curl -sS --max-time "'"$MUXED_TIMEOUT_SECS"'" -o /dev/null -w "%{http_code}\t%{time_total}\t%{size_download}" -G "$API_BASE/api/stream/muxed" --data-urlencode "video_url=$VIDEO_URL" --data-urlencode "audio_url=$AUDIO_URL" --data-urlencode "source_url=$SOURCE_URL" --data-urlencode "video_format_id=$VIDEO_FORMAT_ID" --data-urlencode "audio_format_id=$AUDIO_FORMAT_ID" --data-urlencode "title=stress"); rc=$?; set -e; c=$(echo "$m"|cut -f1); t=$(echo "$m"|cut -f2); s=$(echo "$m"|cut -f3); [ -z "$c" ]&&c=000; [ -z "$t" ]&&t=0; [ -z "$s" ]&&s=0; echo -e "$c\t$t\t$s\t$rc"'

echo "SUMMARY_FILE=$SUMMARY_FILE"
