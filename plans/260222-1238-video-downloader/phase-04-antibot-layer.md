# Phase 04 — Anti-Bot Layer

## Context
- Plan: [plan.md](plan.md)
- Prev: [phase-03-stream-proxy.md](phase-03-stream-proxy.md)
- Research: [reports/researcher-02-antibot-proxy.md](reports/researcher-02-antibot-proxy.md)

## Overview
- **Priority**: P1
- **Status**: completed
- **Effort**: 1.5d
- Multi-layer anti-bot: residential proxy rotation, cookie persistence, browser-realistic headers, request throttling. JA3 spoofing dropped (no production Rust crate).

## Key Insights
<!-- Updated: Validation Session 1 - Residential proxy removed, VPS IP rotation adopted -->
- **No residential proxy needed**: VPS outbound IP is used for YouTube/TikTok requests (not home IP)
- Multiple cheap VPS instances = natural IP rotation at near-zero cost
- YouTube requires valid session cookies for high-quality streams; TikTok requires device cookies
- Request pattern matters: throttle 50-200ms between requests same-origin
- Home Server never sends requests to platforms directly — VPS is the only exposed endpoint

## Architecture

```
AntiBotClient
  ├── VpsPool (list of VPS IPs for outbound — if multiple VPS deployed)
  ├── CookieStore (per-platform persistent cookies, reqwest CookieStore)
  ├── HeaderBuilder (browser-realistic headers, UA rotation)
  └── ThrottleMap (per-domain rate limiter, tokio::time::sleep)
```
*Note: Single VPS deployment uses its own outbound IP. Multi-VPS deployment adds IP rotation via load balancer.*

## Related Code Files
- `crates/proxy/src/anti_bot.rs` — AntiBotClient
- `crates/proxy/src/proxy_pool.rs` — ProxyPool
- `crates/proxy/src/cookie_store.rs` — persistent cookie jar
- `crates/proxy/src/header_builder.rs` — UA + header rotation
- `crates/proxy/src/throttle.rs` — per-domain rate limiter
- `config/proxies.txt` — proxy list (env-loaded, not committed)

## Implementation Steps

1. **ProxyPool** (`proxy_pool.rs`)
   ```rust
   pub struct ProxyPool { proxies: Vec<String>, current: AtomicUsize }
   impl ProxyPool {
       pub fn next(&self) -> &str  // round-robin
       pub fn mark_failed(&self, proxy: &str)  // skip for 60s
   }
   ```
   - Load from `PROXY_LIST` env var (comma-separated URLs)
   - Format: `http://user:pass@host:port`
   - Health check: remove proxies returning non-2xx 3 times in a row

2. **CookieStore** (`cookie_store.rs`)
   - Use `reqwest::cookie::Jar` per platform
   - Persist to `~/.downloadtool/cookies/{platform}.json` (optional)
   - Warm-up: on startup, fetch platform homepage to seed cookies
   - Rotate: if extraction fails with 403, clear cookies + re-warm

3. **HeaderBuilder** (`header_builder.rs`)
   ```rust
   const USER_AGENTS: &[&str] = &[
       "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 ...",
       "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) ...",
       // 5-10 current Chrome/Firefox UAs
   ];
   pub fn build_headers(platform: Platform) -> HeaderMap
   ```
   - Per-platform headers: YouTube needs `Referer: youtube.com`, TikTok needs `x-tt-params`
   - Rotate UA per request (random selection)
   - Include: Accept, Accept-Language, Accept-Encoding, Sec-Ch-Ua, Sec-Fetch-*

4. **Throttle** (`throttle.rs`)
   ```rust
   pub struct DomainThrottle { last_request: DashMap<String, Instant> }
   impl DomainThrottle {
       pub async fn wait(&self, domain: &str)  // sleep if <100ms since last
   }
   ```
   - Per-domain last-request tracking
   - Minimum 100ms between requests to same domain per proxy
   - Use `dashmap` for concurrent access

5. **AntiBotClient** (`anti_bot.rs`) — wraps ProxyClient from Phase 03
   ```rust
   pub struct AntiBotClient {
       proxy_pool: Arc<ProxyPool>,
       cookie_store: Arc<CookieStore>,
       header_builder: HeaderBuilder,
       throttle: Arc<DomainThrottle>,
   }
   impl AntiBotClient {
       pub async fn get(&self, url: &str, platform: Platform)
           -> Result<reqwest::Response>
       // On 403: rotate proxy + cookies, retry once
   }
   ```

6. **Retry logic**: 403/429 → rotate proxy → retry after 200ms → max 3 attempts

## Todo
- [x] ProxyPool with round-robin + failure tracking
- [x] CookieStore per-platform with warm-up
- [x] HeaderBuilder with UA rotation
- [x] DomainThrottle (100ms minimum per domain)
- [x] AntiBotClient wrapping ProxyClient
- [x] Retry on 403/429 with proxy rotation
- [ ] Integration test: extract with 403-returning mock → verify retry + rotation

## Success Criteria
- <5% 403 rate on YouTube extraction with valid proxy pool
- Proxy rotation transparent to caller
- No hammering: >100ms between requests to same domain

## Risk Assessment
| Risk | Mitigation |
|---|---|
| Proxy provider outage | Multi-provider: Bright Data primary + IPRoyal fallback |
| Proxy cost spikes | Cache extracted URLs for 4h (below YouTube 6h signed URL TTL) |
| TikTok device fingerprint | Accept user-provided cookies via API param |
| IP ban cascade | Rotate proxy on first 403; don't retry same IP |

## Security
- Proxy credentials in env vars only, never in code
- Cookie files stored with 600 permissions, outside web root
