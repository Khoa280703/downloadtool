# Phase 03: R2 CORS Verification + Content-Disposition

## Context Links
- [plan.md](./plan.md)
- Rust presign: `crates/object-store/src/s3_storage_backend.rs:79-105`
- Rust content-disposition builder: `crates/api/src/services/storage_ticket_service.rs:89-110`

## Overview
- **Priority**: P1 (blocker for direct mode)
- **Status**: pending
- **Effort**: 0.5h

Verify and configure R2 bucket CORS rules to allow direct browser downloads from `snapvie.com`. Verify Content-Disposition encoding for Unicode filenames.

## Key Insights

1. R2 presigned URL already includes `response-content-disposition` param (verified in `s3_storage_backend.rs:93-95`)
2. `build_download_content_disposition()` generates `attachment; filename="ascii_name"; filename*=UTF-8''encoded_name` -- correct RFC 6266 format
3. For FSAA (File System Access API) downloads, browser uses `fetch()` which requires CORS. For anchor downloads, CORS is NOT required (navigation, not XHR).
4. FSAA progress needs `Content-Length` header exposed via CORS

## Requirements

### R2 CORS Configuration

Apply to the R2 bucket used for mux artifacts:

```json
[
  {
    "AllowedOrigins": [
      "https://snapvie.com",
      "https://www.snapvie.com",
      "http://localhost:5168"
    ],
    "AllowedMethods": ["GET", "HEAD"],
    "AllowedHeaders": ["*"],
    "ExposeHeaders": [
      "Content-Length",
      "Content-Type",
      "Content-Disposition",
      "ETag"
    ],
    "MaxAgeSeconds": 86400
  }
]
```

### Verification Steps

## Implementation Steps

### Step 1: Configure R2 CORS via Cloudflare Dashboard or Wrangler

Option A -- Cloudflare Dashboard:
1. R2 > Bucket > Settings > CORS Policy
2. Add the rules above

Option B -- Wrangler CLI:
```bash
wrangler r2 bucket cors put <bucket-name> --rules '[...]'
```

### Step 2: Verify presigned URL Content-Disposition

Test with curl:
```bash
# Get a presigned URL from file-ticket endpoint
curl -s "https://snapvie.com/api/proxy/jobs/<test-job-id>/file-ticket" | jq .download_url

# Fetch headers from the presigned URL
curl -I "<presigned-url>"
# Verify: Content-Disposition: attachment; filename="..."; filename*=UTF-8''...
```

### Step 3: Verify CORS headers

```bash
# Preflight check
curl -X OPTIONS "<presigned-url>" \
  -H "Origin: https://snapvie.com" \
  -H "Access-Control-Request-Method: GET" \
  -I

# Verify response includes:
# Access-Control-Allow-Origin: https://snapvie.com
# Access-Control-Expose-Headers: Content-Length, Content-Type, Content-Disposition
```

### Step 4: Verify Unicode filename

Test with a video title containing Unicode characters:
1. Create a mux job with title like "Rick Astley - Never Gonna Give You Up"
2. Check Content-Disposition in response headers
3. Verify browser saves with correct filename

Already handled by `build_download_content_disposition()`:
- ASCII fallback: `filename="Rick Astley - Never Gonna Give You Up.mp4"`
- UTF-8: `filename*=UTF-8''Rick%20Astley%20-%20Never%20Gonna%20Give%20You%20Up.mp4`

## Todo List
- [ ] Configure R2 CORS rules (AllowedOrigins, ExposeHeaders)
- [ ] Test CORS preflight from snapvie.com origin
- [ ] Verify Content-Length is exposed (needed for FSAA progress)
- [ ] Verify Content-Disposition header present on presigned URL response
- [ ] Test Unicode filename encoding
- [ ] Add localhost origin for dev testing

## Success Criteria
- `fetch()` from `snapvie.com` to R2 presigned URL succeeds (no CORS error)
- Response includes `Content-Length` header readable by JavaScript
- File downloads with correct filename including Unicode characters
- Dev environment (`localhost:5168`) also works

## Risk Assessment
- **CORS too permissive**: Only allowing specific origins + GET/HEAD methods. Minimal risk.
- **R2 CORS propagation delay**: Cloudflare CORS changes may take a few minutes. Test before enabling direct mode.
- **MaxAgeSeconds=86400**: Browser caches CORS preflight for 24h. If we need to change rules, users may need to clear cache. Acceptable tradeoff for reduced preflight requests.
