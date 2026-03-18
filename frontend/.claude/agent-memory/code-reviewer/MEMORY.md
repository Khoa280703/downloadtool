# Code Reviewer Memory

## Project Patterns
- SQL: All queries use sqlx parameterized bindings, no string interpolation
- SSE: Rust uses async_stream polling, frontend uses EventSource via SvelteKit proxy
- Auth: dual model -- authenticated users (user_id) or anonymous (session_id from cookie)
- Job IDs: format `{prefix}-{timestamp_ms}-{seq}` (e.g. pl-1742...-0), not UUIDs
- Frontend proxy: SvelteKit routes in `/api/proxy/` forward to Rust API with session headers
- Rate limiting: governor crate, keyed by IP, shared across extract + playlist-jobs create

## Common Issues Found
- SSE proxy routes often miss `signal: request.signal` forwarding -> resource leaks
- Bulk DB operations sometimes lack transactions (insert_items pattern)
- No concurrency limits on tokio::spawn for background tasks
- `from_str` methods silently default instead of logging unknown values

## File Locations
- Backend routes: `crates/api/src/routes/`
- Backend services: `crates/api/src/services/`
- Job system models: `crates/job-system/src/`
- Frontend proxy: `frontend/src/routes/api/proxy/`
- Frontend client APIs: `frontend/src/lib/`
- DB migrations: `crates/api/app-migrations/`
- i18n messages: `frontend/messages/`

## Review Report Location
- Reports go to: `{work_context}/plans/reports/`
- Naming: `code-reviewer-{date}-{time}-{slug}.md`
