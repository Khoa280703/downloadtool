#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  scripts/diagnose-extract-local-vs-deploy.sh --deploy <DEPLOY_API_BASE> [options]

Options:
  --deploy <url>        Deploy API base URL, ví dụ: https://api.example.com
  --local <url>         Local API base URL (default: http://127.0.0.1:3068)
  --url <video_url>     Video URL để test (default: Rickroll)
  --timeout <seconds>   Curl max-time cho mỗi request (default: 25)
  --out <dir>           Output directory (default: /tmp/extract-diagnose-<timestamp>)
  -h, --help            Show help

Output files:
  summary.tsv
  report.md
  local-headers.txt, local-body.json, deploy-headers.txt, deploy-body.json
  local-ytdlp.err, local-ytdlp.json
EOF
}

LOCAL_API_BASE="http://127.0.0.1:3068"
DEPLOY_API_BASE="${DEPLOY_API_BASE:-}"
VIDEO_URL="https://www.youtube.com/watch?v=dQw4w9WgXcQ"
TIMEOUT_S=25
RUN_ID="$(date +%Y%m%d-%H%M%S)"
OUT_DIR="/tmp/extract-diagnose-${RUN_ID}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --deploy)
      DEPLOY_API_BASE="$2"
      shift 2
      ;;
    --local)
      LOCAL_API_BASE="$2"
      shift 2
      ;;
    --url)
      VIDEO_URL="$2"
      shift 2
      ;;
    --timeout)
      TIMEOUT_S="$2"
      shift 2
      ;;
    --out)
      OUT_DIR="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown arg: $1"
      usage
      exit 1
      ;;
  esac
done

if [[ -z "$DEPLOY_API_BASE" ]]; then
  echo "--deploy is required"
  usage
  exit 1
fi

mkdir -p "$OUT_DIR"
SUMMARY_TSV="$OUT_DIR/summary.tsv"
REPORT_MD="$OUT_DIR/report.md"

mask_env() {
  local var_name="$1"
  local val
  val="$(printenv "$var_name" || true)"
  if [[ -z "$val" ]]; then
    echo "<unset>"
    return
  fi
  echo "<set,len=${#val}>"
}

header_value() {
  local headers_file="$1"
  local key="$2"
  if [[ ! -f "$headers_file" ]]; then
    echo ""
    return
  fi
  awk -F': ' -v k="$key" 'tolower($1)==tolower(k){print $2; exit}' "$headers_file" | tr -d '\r'
}

json_field() {
  local file="$1"
  local field="$2"
  node -e '
    const fs = require("fs");
    const file = process.argv[1];
    const field = process.argv[2];
    let raw = "";
    try { raw = fs.readFileSync(file, "utf8"); } catch { process.stdout.write(""); process.exit(0); }
    let data;
    try { data = JSON.parse(raw); } catch { process.stdout.write(""); process.exit(0); }
    const val =
      field === "error" ? (data?.error ?? data?.message ?? "") :
      field === "title" ? (data?.metadata?.title ?? "") :
      field === "formats" ? ((data?.metadata?.formats ?? []).length) :
      "";
    process.stdout.write(String(val));
  ' "$file" "$field" 2>/dev/null || true
}

run_extract_once() {
  local label="$1"
  local base="$2"
  local headers="$OUT_DIR/${label}-headers.txt"
  local body="$OUT_DIR/${label}-body.json"

  local meta rc
  set +e
  meta=$(curl -sS --max-time "$TIMEOUT_S" \
    -D "$headers" \
    -o "$body" \
    -w "%{http_code}\t%{time_total}\t%{remote_ip}\t%{size_download}" \
    -X POST "$base/api/extract" \
    -H "content-type: application/json" \
    --data "{\"url\":\"$VIDEO_URL\"}")
  rc=$?
  set -e

  local http_code time_total remote_ip size_download
  IFS=$'\t' read -r http_code time_total remote_ip size_download <<<"$meta"
  http_code="${http_code:-000}"
  time_total="${time_total:-0}"
  remote_ip="${remote_ip:-}"
  size_download="${size_download:-0}"

  local error_msg title formats
  error_msg="$(json_field "$body" error)"
  title="$(json_field "$body" title)"
  formats="$(json_field "$body" formats)"

  echo -e "${label}\t${base}\t${http_code}\t${time_total}\t${rc}\t${remote_ip}\t${size_download}\t${formats}\t${title}\t${error_msg}" >> "$SUMMARY_TSV"
}

run_local_ytdlp_probe() {
  local y_json="$OUT_DIR/local-ytdlp.json"
  local y_err="$OUT_DIR/local-ytdlp.err"
  local y_ver_file="$OUT_DIR/local-ytdlp-version.txt"
  local y_path_file="$OUT_DIR/local-ytdlp-path.txt"

  {
    command -v yt-dlp || true
  } > "$y_path_file"

  {
    yt-dlp --version || true
  } > "$y_ver_file"

  set +e
  yt-dlp -J --no-playlist --no-warnings --socket-timeout 15 --no-check-certificates \
    "$VIDEO_URL" >"$y_json" 2>"$y_err"
  local rc=$?
  set -e
  echo "$rc" > "$OUT_DIR/local-ytdlp-exit-code.txt"
}

write_report() {
  local local_headers="$OUT_DIR/local-headers.txt"
  local deploy_headers="$OUT_DIR/deploy-headers.txt"
  local local_server deploy_server local_cf_ray deploy_cf_ray local_ct deploy_ct
  local_server="$(header_value "$local_headers" "server")"
  deploy_server="$(header_value "$deploy_headers" "server")"
  local_cf_ray="$(header_value "$local_headers" "cf-ray")"
  deploy_cf_ray="$(header_value "$deploy_headers" "cf-ray")"
  local_ct="$(header_value "$local_headers" "content-type")"
  deploy_ct="$(header_value "$deploy_headers" "content-type")"

  local ytdlp_path ytdlp_ver ytdlp_rc ytdlp_err
  ytdlp_path="$(cat "$OUT_DIR/local-ytdlp-path.txt" 2>/dev/null || true)"
  ytdlp_ver="$(cat "$OUT_DIR/local-ytdlp-version.txt" 2>/dev/null || true)"
  ytdlp_rc="$(cat "$OUT_DIR/local-ytdlp-exit-code.txt" 2>/dev/null || true)"
  ytdlp_err="$(head -n 2 "$OUT_DIR/local-ytdlp.err" 2>/dev/null | tr '\n' ' ' | sed 's/[[:space:]]\+/ /g')"

  {
    echo "# Extract Diagnose: Local vs Deploy"
    echo
    echo "Generated: $(date -Iseconds)"
    echo
    echo "## Inputs"
    echo "- VIDEO_URL: $VIDEO_URL"
    echo "- LOCAL_API_BASE: $LOCAL_API_BASE"
    echo "- DEPLOY_API_BASE: $DEPLOY_API_BASE"
    echo "- TIMEOUT_S: $TIMEOUT_S"
    echo
    echo "## Summary"
    echo
    echo '| target | base | http_code | time_total_s | curl_rc | remote_ip | size_download | formats | title | error |'
    echo '|---|---|---:|---:|---:|---|---:|---:|---|---|'
    awk -F'\t' 'NR>1{
      gsub(/\|/, "\\|", $9); gsub(/\|/, "\\|", $10);
      printf("| %s | %s | %s | %s | %s | %s | %s | %s | %s | %s |\n",$1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
    }' "$SUMMARY_TSV"
    echo
    echo "## Header Highlights"
    echo "- local server: ${local_server:-<none>}"
    echo "- deploy server: ${deploy_server:-<none>}"
    echo "- local cf-ray: ${local_cf_ray:-<none>}"
    echo "- deploy cf-ray: ${deploy_cf_ray:-<none>}"
    echo "- local content-type: ${local_ct:-<none>}"
    echo "- deploy content-type: ${deploy_ct:-<none>}"
    echo
    echo "## Local Runtime Probe"
    echo "- yt-dlp path: ${ytdlp_path:-<missing>}"
    echo "- yt-dlp version: ${ytdlp_ver:-<missing>}"
    echo "- yt-dlp exit code (direct -J): ${ytdlp_rc:-<missing>}"
    echo "- yt-dlp stderr (first lines): ${ytdlp_err:-<none>}"
    echo
    echo "## Local Env (masked)"
    echo "- YTDLP_PATH: $(mask_env YTDLP_PATH)"
    echo "- SOCKS5_PROXY_URL: $(mask_env SOCKS5_PROXY_URL)"
    echo "- HTTP_PROXY: $(mask_env HTTP_PROXY)"
    echo "- HTTPS_PROXY: $(mask_env HTTPS_PROXY)"
    echo "- ALL_PROXY: $(mask_env ALL_PROXY)"
    echo "- NO_PROXY: $(mask_env NO_PROXY)"
    echo "- EXTRACT_RATE_LIMIT_ENABLED: $(mask_env EXTRACT_RATE_LIMIT_ENABLED)"
    echo
    echo "## Artifacts"
    echo "- $SUMMARY_TSV"
    echo "- $OUT_DIR/local-headers.txt"
    echo "- $OUT_DIR/deploy-headers.txt"
    echo "- $OUT_DIR/local-body.json"
    echo "- $OUT_DIR/deploy-body.json"
    echo "- $OUT_DIR/local-ytdlp.err"
  } > "$REPORT_MD"
}

echo -e "target\tbase\thttp_code\ttime_total_s\tcurl_rc\tremote_ip\tsize_download\tformats\ttitle\terror" > "$SUMMARY_TSV"

run_local_ytdlp_probe
run_extract_once "local" "$LOCAL_API_BASE"
run_extract_once "deploy" "$DEPLOY_API_BASE"
write_report

echo "OUT_DIR=$OUT_DIR"
echo "SUMMARY_TSV=$SUMMARY_TSV"
echo "REPORT_MD=$REPORT_MD"
echo "--- SUMMARY ---"
if command -v column >/dev/null 2>&1; then
  column -t -s $'\t' "$SUMMARY_TSV"
else
  cat "$SUMMARY_TSV"
fi
