# Code Reviewer Memory

## Project Patterns
- Server-side modules use `$env/dynamic/private` (SvelteKit pattern), NOT `import.meta.env`
- Client-side uses `import.meta.env.VITE_*` for public env vars
- Audit logs: snake_case in API responses, camelCase in audit payloads
- Signed R2 URLs must NEVER be logged in audit -- mask with `[r2-signed]`
- `appendCacheBuster()` must be skipped for absolute URLs (signed URL + extra param = 403)

## Architecture Notes
- Download flow: file-ticket endpoint resolves delivery mode -> client gets URL + mode
- Fallback chain: Direct R2 -> FSAA proxy fallback -> anchor fallback (3 levels)
- `DOWNLOAD_DELIVERY_MODE` env: `hybrid` (default) / `direct` / `proxy` (kill switch)
- Mux jobs: server-side background processing, client polls for readiness
- Direct stream downloads go through `/api/stream` (same-origin, no R2 involvement)

## Known DRY Violations
- `formatMuxStatus`, `clampProgressPercent`, `resolveMuxPhaseLabel`, `resolveQueuedMuxLabel` duplicated between `DownloadBtn.svelte` and `playlist-download-worker.ts` (~100 LOC)
- `buildFilename` (DownloadBtn) vs `safeFilename` (playlist-download-stream-selection) -- different sanitization approaches

## Review Checklist for Download Features
1. Verify cache-buster skip for absolute URLs
2. Check FSAA-only guard for cross-origin downloads
3. Confirm signed URL masking in all audit/log paths
4. Verify UA detection handles Chrome/Edge "Safari" substring
5. Check server env uses `$env/dynamic/private`
