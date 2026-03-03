#!/usr/bin/env bash
set -euo pipefail

API_BASE="${API_BASE:-http://127.0.0.1:3068}"
VIDEO_URL="${VIDEO_URL:-https://www.youtube.com/watch?v=qBsiWmEdj0c&list=PLgIS1MvgCaOVR07APOp8zfBLHbh7VGllX&index=8}"
PLAYLIST_URL="${PLAYLIST_URL:-https://www.youtube.com/playlist?list=PLgIS1MvgCaOVR07APOp8zfBLHbh7VGllX}"
PROFILE="${PROFILE:-strong}"
RUN_BATCH="${RUN_BATCH:-1}"

RUN_ID="$(date +%Y%m%d-%H%M%S)"
OUT_DIR="${OUT_DIR:-/tmp/stress-reusable-${PROFILE}-${RUN_ID}}"
mkdir -p "$OUT_DIR"

SUMMARY_FILE="$OUT_DIR/summary.tsv"
BATCH_FILE="$OUT_DIR/batch-summary.tsv"

echo -e "case\tendpoint\tconcurrency\ttotal\ttimeout_s\thttp200\thttp403\thttp429\thttp5xx\thttp4xx_other\thttp000\trc0\trc28\trc18\tavg_size\tp50_s\tp95_s\tp99_s\txargs_rc" > "$SUMMARY_FILE"
echo -e "case\tendpoint\tconcurrency\ttotal\ttimeout_s\thttp200\thttp_non_200\trc0\trc28\tdone_ok\terror_event\tno_done\tavg_links\tp50_s\tp95_s\tp99_s\txargs_rc" > "$BATCH_FILE"

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

run_http_case() {
  local case_name="$1"
  local endpoint="$2"
  local concurrency="$3"
  local total="$4"
  local timeout_s="$5"
  local runner="$6"

  local out="$OUT_DIR/${case_name}.tsv"
  : > "$out"
  local t0 t1 elapsed xrc
  t0=$(date +%s)
  set +e
  seq "$total" | xargs -P "$concurrency" -I{} bash -lc "$runner" >> "$out"
  xrc=$?
  set -e
  t1=$(date +%s)
  elapsed=$((t1 - t0))

  local http200 http403 http429 http5xx http4xx_other http000 rc0 rc28 rc18
  local avg_size p50 p95 p99
  http200=$(awk -F'\t' '$1==200{c++} END{print c+0}' "$out")
  http403=$(awk -F'\t' '$1==403{c++} END{print c+0}' "$out")
  http429=$(awk -F'\t' '$1==429{c++} END{print c+0}' "$out")
  http5xx=$(awk -F'\t' '$1>=500 && $1<600{c++} END{print c+0}' "$out")
  http4xx_other=$(awk -F'\t' '$1>=400 && $1<500 && $1!=403 && $1!=429{c++} END{print c+0}' "$out")
  http000=$(awk -F'\t' '$1==0 || $1=="000"{c++} END{print c+0}' "$out")
  rc0=$(awk -F'\t' '$4==0{c++} END{print c+0}' "$out")
  rc28=$(awk -F'\t' '$4==28{c++} END{print c+0}' "$out")
  rc18=$(awk -F'\t' '$4==18{c++} END{print c+0}' "$out")
  avg_size=$(awk -F'\t' '{sum+=$3} END{if(NR==0){print 0}else{printf "%.0f", sum/NR}}' "$out")
  p50=$(percentile "$out" 2 0.50)
  p95=$(percentile "$out" 2 0.95)
  p99=$(percentile "$out" 2 0.99)

  echo -e "${case_name}\t${endpoint}\t${concurrency}\t${total}\t${timeout_s}\t${http200}\t${http403}\t${http429}\t${http5xx}\t${http4xx_other}\t${http000}\t${rc0}\t${rc28}\t${rc18}\t${avg_size}\t${p50}\t${p95}\t${p99}\t${xrc}" >> "$SUMMARY_FILE"
  echo "[DONE] ${case_name} ok=${http200}/${total} 5xx=${http5xx} 4xx_other=${http4xx_other} 000=${http000} rc28=${rc28} p95=${p95}s"
}

run_batch_case() {
  local case_name="$1"
  local concurrency="$2"
  local total="$3"
  local timeout_s="$4"
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
    link_n=$( (grep -o "\"type\":\"link\"" "$tmp" || true) | wc -l | tr -d " " )
    echo -e "${code}\t${t}\t${rc}\t${done_n}\t${err_n}\t${link_n}"
    rm -f "$tmp"
  ' >> "$out"
  xrc=$?
  set -e

  local http200 http_non_200 rc0 rc28 done_ok error_event no_done avg_links p50 p95 p99
  http200=$(awk -F'\t' '$1==200{c++} END{print c+0}' "$out")
  http_non_200=$(awk -F'\t' '$1!=200{c++} END{print c+0}' "$out")
  rc0=$(awk -F'\t' '$3==0{c++} END{print c+0}' "$out")
  rc28=$(awk -F'\t' '$3==28{c++} END{print c+0}' "$out")
  done_ok=$(awk -F'\t' '$4>0{c++} END{print c+0}' "$out")
  error_event=$(awk -F'\t' '$5>0{c++} END{print c+0}' "$out")
  no_done=$(awk -F'\t' '$4==0{c++} END{print c+0}' "$out")
  avg_links=$(awk -F'\t' '{sum+=$6} END{if(NR==0){print 0}else{printf "%.2f", sum/NR}}' "$out")
  p50=$(percentile "$out" 2 0.50)
  p95=$(percentile "$out" 2 0.95)
  p99=$(percentile "$out" 2 0.99)

  echo -e "${case_name}\t/api/batch\t${concurrency}\t${total}\t${timeout_s}\t${http200}\t${http_non_200}\t${rc0}\t${rc28}\t${done_ok}\t${error_event}\t${no_done}\t${avg_links}\t${p50}\t${p95}\t${p99}\t${xrc}" >> "$BATCH_FILE"
  echo "[DONE] ${case_name} http200=${http200}/${total} done_ok=${done_ok}/${total} err_evt=${error_event} p95=${p95}s"
}

BASE_JSON="$OUT_DIR/extract-once.json"
if [[ -n "${BASE_JSON_INPUT:-}" && -f "${BASE_JSON_INPUT:-}" ]]; then
  cp "$BASE_JSON_INPUT" "$BASE_JSON"
else
  curl -sS -X POST "$API_BASE/api/extract" -H "content-type: application/json" --data "{\"url\":\"$VIDEO_URL\"}" > "$BASE_JSON"
fi

read -r SOURCE_URL STREAM_URL STREAM_FORMAT_ID VIDEO_ONLY_URL VIDEO_FORMAT_ID AUDIO_M4A_URL AUDIO_FORMAT_ID < <(node - "$BASE_JSON" <<'NODE'
const fs=require('fs');
const d=JSON.parse(fs.readFileSync(process.argv[2],'utf8'));
const f=(d?.metadata?.formats)||[];
const score=(q='')=>{const s=String(q).toLowerCase();const m=s.match(/(\d{3,4})p/);return m?Number(m[1]):0};
const stream=f.find(x=>x.ext==='mp4'&&x.has_audio)||f.find(x=>x.ext==='mp4')||f[0];
const video=[...f].filter(x=>!x.is_audio_only&&!x.has_audio&&x.ext==='mp4').sort((a,b)=>score(b.quality)-score(a.quality))[0]||stream;
const audio=[...f].filter(x=>x.is_audio_only&&x.ext==='m4a').sort((a,b)=>(b.bitrate||0)-(a.bitrate||0))[0]||stream;
const source=d?.metadata?.original_url||'';
console.log([
  source,
  stream?.url||'',
  stream?.format_id||'',
  video?.url||'',
  video?.format_id||'',
  audio?.url||'',
  audio?.format_id||''
].join(' '));
NODE
)

if [[ -z "$SOURCE_URL" || -z "$STREAM_URL" || -z "$VIDEO_ONLY_URL" || -z "$AUDIO_M4A_URL" ]]; then
  echo "Failed to prepare test URLs from /api/extract" >&2
  exit 1
fi

# Make sure xargs worker shells can access all runtime values.
export API_BASE VIDEO_URL PLAYLIST_URL SOURCE_URL STREAM_URL STREAM_FORMAT_ID VIDEO_ONLY_URL VIDEO_FORMAT_ID AUDIO_M4A_URL AUDIO_FORMAT_ID

echo "Run profile: $PROFILE"
echo "Output dir : $OUT_DIR"

if [[ "$PROFILE" == "strong" ]]; then
  run_http_case "extract-c120-n12000" "/api/extract" 120 12000 12 'set +e; m=$(curl -sS --max-time 12 -o /dev/null -w "%{http_code}\t%{time_total}\t%{size_download}" -X POST "$API_BASE/api/extract" -H "content-type: application/json" --data "{\"url\":\"$VIDEO_URL\"}"); rc=$?; set -e; c=$(echo "$m"|cut -f1); t=$(echo "$m"|cut -f2); s=$(echo "$m"|cut -f3); [ -z "$c" ]&&c=000; [ -z "$t" ]&&t=0; [ -z "$s" ]&&s=0; echo -e "$c\t$t\t$s\t$rc"'
  run_http_case "stream4k-c150-n12000" "/api/stream" 150 12000 15 'set +e; m=$(curl -sS --max-time 15 -o /dev/null -w "%{http_code}\t%{time_total}\t%{size_download}" -G "$API_BASE/api/stream" -H "Range: bytes=0-4095" --data-urlencode "url=$STREAM_URL" --data-urlencode "source_url=$SOURCE_URL" --data-urlencode "format_id=$STREAM_FORMAT_ID" --data-urlencode "title=stress" --data-urlencode "format=mp4"); rc=$?; set -e; c=$(echo "$m"|cut -f1); t=$(echo "$m"|cut -f2); s=$(echo "$m"|cut -f3); [ -z "$c" ]&&c=000; [ -z "$t" ]&&t=0; [ -z "$s" ]&&s=0; echo -e "$c\t$t\t$s\t$rc"'
  run_http_case "stream1m-c100-n5000" "/api/stream" 100 5000 30 'set +e; m=$(curl -sS --max-time 30 -o /dev/null -w "%{http_code}\t%{time_total}\t%{size_download}" -G "$API_BASE/api/stream" -H "Range: bytes=0-1048575" --data-urlencode "url=$STREAM_URL" --data-urlencode "source_url=$SOURCE_URL" --data-urlencode "format_id=$STREAM_FORMAT_ID" --data-urlencode "title=stress" --data-urlencode "format=mp4"); rc=$?; set -e; c=$(echo "$m"|cut -f1); t=$(echo "$m"|cut -f2); s=$(echo "$m"|cut -f3); [ -z "$c" ]&&c=000; [ -z "$t" ]&&t=0; [ -z "$s" ]&&s=0; echo -e "$c\t$t\t$s\t$rc"'
  run_http_case "muxed-c40-n2000-t30" "/api/stream/muxed" 40 2000 30 'set +e; m=$(curl -sS --max-time 30 -o /dev/null -w "%{http_code}\t%{time_total}\t%{size_download}" -G "$API_BASE/api/stream/muxed" --data-urlencode "video_url=$VIDEO_ONLY_URL" --data-urlencode "audio_url=$AUDIO_M4A_URL" --data-urlencode "source_url=$SOURCE_URL" --data-urlencode "video_format_id=$VIDEO_FORMAT_ID" --data-urlencode "audio_format_id=$AUDIO_FORMAT_ID" --data-urlencode "title=stress"); rc=$?; set -e; c=$(echo "$m"|cut -f1); t=$(echo "$m"|cut -f2); s=$(echo "$m"|cut -f3); [ -z "$c" ]&&c=000; [ -z "$t" ]&&t=0; [ -z "$s" ]&&s=0; echo -e "$c\t$t\t$s\t$rc"'
  run_http_case "muxed-c80-n2000-t30" "/api/stream/muxed" 80 2000 30 'set +e; m=$(curl -sS --max-time 30 -o /dev/null -w "%{http_code}\t%{time_total}\t%{size_download}" -G "$API_BASE/api/stream/muxed" --data-urlencode "video_url=$VIDEO_ONLY_URL" --data-urlencode "audio_url=$AUDIO_M4A_URL" --data-urlencode "source_url=$SOURCE_URL" --data-urlencode "video_format_id=$VIDEO_FORMAT_ID" --data-urlencode "audio_format_id=$AUDIO_FORMAT_ID" --data-urlencode "title=stress"); rc=$?; set -e; c=$(echo "$m"|cut -f1); t=$(echo "$m"|cut -f2); s=$(echo "$m"|cut -f3); [ -z "$c" ]&&c=000; [ -z "$t" ]&&t=0; [ -z "$s" ]&&s=0; echo -e "$c\t$t\t$s\t$rc"'
  if [[ "$RUN_BATCH" == "1" ]]; then
    run_batch_case "batch-c20-n2000-t120" 20 2000 120
    run_batch_case "batch-c40-n2000-t120" 40 2000 120
  fi
else
  echo "Unsupported PROFILE=$PROFILE (supported: strong)" >&2
  exit 1
fi

echo "SUMMARY_FILE=$SUMMARY_FILE"
echo "BATCH_FILE=$BATCH_FILE"
