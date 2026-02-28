# Code Standards & Codebase Structure

**Last Updated:** 2026-02-28

## Directory Structure

```
downloadtool/
├── crates/                          # Rust workspace with 6 crates
│   ├── api/                         # HTTP API server (Axum + PostgreSQL)
│   │   ├── src/
│   │   │   ├── main.rs              # Server entry, PostgreSQL pool, route setup
│   │   │   ├── config.rs            # Environment config (JWT_SECRET, DATABASE_URL, etc.)
│   │   │   ├── auth/                # Authentication [NEW 2026-02-28]
│   │   │   │   ├── mod.rs           # Module exports
│   │   │   │   ├── jwt_claims.rs    # JWT payload (user_id, tier, exp)
│   │   │   │   ├── jwt_middleware.rs # Axum middleware for JWT validation
│   │   │   │   └── user_tier.rs     # Enum: Free, Pro, Premium
│   │   │   └── routes/
│   │   │       ├── mod.rs           # Route exports
│   │   │       ├── extract.rs       # POST /extract endpoint (JWT required)
│   │   │       ├── batch.rs         # POST /batch endpoint (SSE, JWT required)
│   │   │       ├── stream.rs        # WS /stream endpoint
│   │   │       ├── transcode.rs     # POST /transcode endpoint
│   │   │       ├── whop_webhook.rs  # POST /whop-webhook (HMAC validation) [NEW]
│   │   │       ├── openapi.rs       # OpenAPI spec generation (utoipa)
│   │   │       └── static_files.rs  # Frontend static file serving
│   │   ├── migrations/              # SQL migrations [NEW]
│   │   │   └── 0001_create_subscriptions.sql
│   │   ├── Cargo.toml
│   │   └── README.md
│   │
│   ├── extractor/                   # Extraction engine (yt-dlp + Deno runtime)
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── lib.rs               # Public interface
│   │   │   ├── engine.rs            # Core orchestrator
│   │   │   ├── runtime.rs           # Deno runtime management
│   │   │   ├── pool.rs              # Connection pooling
│   │   │   ├── ytdlp.rs             # yt-dlp subprocess extractor [NEW 2026-02-28]
│   │   │   ├── hot_reload.rs        # Script hot-reload
│   │   │   └── types.rs             # Shared types
│   │   ├── build.rs                 # Build script for Deno
│   │   ├── Cargo.toml
│   │   └── README.md
│   │
│   ├── gpu-pipeline/                # GPU video encoding
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── pipeline.rs          # Orchestrator (largest: 3,976 tokens)
│   │   │   ├── decoder.rs           # Hardware decode
│   │   │   ├── encoder.rs           # Hardware encode (12,395 tokens)
│   │   │   ├── frame_queue.rs       # Frame buffering
│   │   │   ├── watermark.rs         # Watermark overlay
│   │   │   └── ffi.rs               # GPU driver FFI
│   │   ├── Cargo.toml
│   │   └── README.md
│   │
│   ├── gpu-worker/                  # Standalone GPU worker process
│   │   ├── src/
│   │   │   ├── main.rs              # Entry point
│   │   │   ├── lib.rs
│   │   │   ├── server.rs            # gRPC server
│   │   │   └── transcode.rs         # Job execution
│   │   ├── build.rs
│   │   ├── Cargo.toml
│   │   └── README.md
│   │
│   ├── muxer/                       # Container muxing (fMP4) - dual-traf
│   │   ├── src/
│   │   │   ├── lib.rs               # (101 LOC)
│   │   │   ├── fmp4_remuxer.rs      # Main remuxer (407 LOC)
│   │   │   ├── moov_merger.rs       # Merge video/audio moov (305 LOC)
│   │   │   ├── traf_merger.rs       # Merge track fragments (416 LOC)
│   │   │   ├── box_parser.rs        # BMFF parsing (301 LOC)
│   │   │   ├── fragment_stream.rs   # Fragment streaming (273 LOC)
│   │   │   ├── stream_fetcher.rs    # Fetch & buffer (264 LOC)
│   │   │   ├── mux_router.rs        # Route streams (255 LOC)
│   │   │   └── codec.rs             # Codec config (189 LOC)
│   │   ├── Cargo.toml
│   │   └── README.md
│   │
│   └── proxy/                       # Anti-bot & proxy layer
│       ├── src/
│       │   ├── lib.rs
│       │   ├── anti_bot.rs          # Main client [CRITICAL: timeout fix 2026-02-23]
│       │   ├── proxy_pool.rs        # Proxy rotation & health
│       │   ├── cookie_store.rs      # Per-platform cookies
│       │   ├── header_builder.rs    # Header randomization
│       │   ├── throttle.rs          # Request throttling
│       │   ├── client.rs            # HTTP client wrapper
│       │   └── stream.rs            # Streaming response
│       ├── Cargo.toml
│       └── README.md
│
├── extractors/                      # Dynamic extraction scripts (TypeScript)
│   ├── youtube.ts                   # Main YouTube extractor
│   ├── youtube-innertube.ts         # InnerTube API client
│   ├── youtube-n-transform.ts       # N-param transform [NEW 2026-02-23]
│   ├── youtube-channel.ts           # Channel extraction
│   ├── types.ts                     # Shared TS types
│   ├── dist/                        # Compiled JS output
│   │   └── youtube.js
│   └── deno.json                    # Deno config
│
├── frontend/                        # SvelteKit web UI
│   ├── src/
│   │   ├── app.html                 # HTML template
│   │   ├── routes/
│   │   │   ├── +layout.svelte       # Root layout
│   │   │   ├── +page.svelte         # Home page
│   │   │   └── privacy/
│   │   │       └── +page.svelte
│   │   ├── components/              # Reusable components
│   │   │   ├── UrlInput.svelte      # URL input field
│   │   │   ├── DownloadBtn.svelte   # Download button [UPDATED 2026-02-23]
│   │   │   ├── BatchInput.svelte    # Batch input [UPDATED 2026-02-23]
│   │   │   ├── BatchProgress.svelte # Progress display
│   │   │   ├── FormatPicker.svelte  # Quality selector
│   │   │   ├── CookieConsent.svelte # Privacy banner
│   │   │   ├── AdBanner.svelte      # Ad display
│   │   │   └── InterstitialAd.svelte# Interstitial ads
│   │   └── lib/
│   │       └── assets/
│   │           └── favicon.svg
│   ├── static/
│   │   ├── ads.txt
│   │   └── robots.txt
│   ├── package.json
│   ├── tsconfig.json
│   ├── svelte.config.js
│   ├── .env.example
│   └── README.md
│
├── infra/                           # Infrastructure & deployment
│   └── wireguard/
│       ├── homeserver.conf
│       ├── vps.conf
│       └── README.md
│
├── docker/                          # Container images & composition
│   ├── Dockerfile.gpu-worker        # GPU worker image
│   ├── Dockerfile.api               # API image
│   ├── docker-compose.server.yml
│   └── .dockerignore
│
├── proto/                           # Protocol Buffer definitions
│   └── transcode.proto              # GPU worker IPC protocol
│
├── plans/                           # Development plans & research
│   ├── 260222-1238-video-downloader/
│   │   ├── phase-01-project-scaffold.md
│   │   ├── phase-02-extraction-layer.md
│   │   ├── phase-03-stream-proxy.md
│   │   ├── phase-04-antibot-layer.md
│   │   ├── phase-05-cpu-muxer.md
│   │   ├── phase-06-gpu-pipeline.md
│   │   ├── phase-07-frontend.md
│   │   ├── phase-08-ad-integration.md
│   │   └── plan.md
│   ├── 260223-1345-youtube-download-timeout-and-n-param-fix/
│   │   ├── phase-01-fix-timeout-bug.md
│   │   ├── phase-02-implement-n-param-transform.md
│   │   └── plan.md
│   ├── 260224-1015-quicktime-webm-dual-traf-fixes/
│   │   └── Implementation complete
│   └── reports/                     # Research & implementation reports
│
├── docs/                            # Documentation (THIS FOLDER)
│   ├── codebase-summary.md
│   ├── system-architecture.md
│   ├── code-standards.md            # This file
│   └── project-overview-pdr.md
│
├── target/                          # Rust build artifacts (git ignored)
├── Cargo.toml                       # Workspace root config
├── Cargo.lock
├── Makefile                         # Build commands
├── .gitignore
├── .github/
│   └── workflows/
│       └── ci.yml                   # GitHub Actions CI
├── idea.md                          # Original project concept
└── repomix-output.xml               # Codebase compaction (generated)
```

## Rust Crate Architecture

### Workspace Structure

**Root Cargo.toml:**
```toml
[workspace]
members = [
  "crates/api",
  "crates/extractor",
  "crates/gpu-pipeline",
  "crates/gpu-worker",
  "crates/muxer",
  "crates/proxy"
]
```

### Naming Conventions

#### Rust Files
- **File names:** `snake_case` (e.g., `anti_bot.rs`, `cookie_store.rs`)
- **Module names:** `snake_case` (e.g., `mod proxy_pool`)
- **Struct/Enum names:** `PascalCase` (e.g., `AntiBotClient`, `ExtractionResult`)
- **Function names:** `snake_case` (e.g., `build_client()`, `fetch_stream()`)
- **Constant names:** `SCREAMING_SNAKE_CASE` (e.g., `MAX_RETRIES`, `RETRY_DELAY`)
- **Type aliases:** `PascalCase` (e.g., `type RequestFn = fn(...)`)

#### TypeScript/JavaScript
- **File names:** `kebab-case` or `snake_case` for utilities (e.g., `youtube-n-transform.ts`)
- **Class names:** `PascalCase`
- **Function names:** `camelCase` (e.g., `getCachedTransformFn()`, `transformStreamUrls()`)
- **Constants:** `SCREAMING_SNAKE_CASE` or `PascalCase` for enums
- **Interfaces:** `PascalCase` (e.g., `NTransformCache`, `Stream`)

#### Svelte Components
- **File names:** `PascalCase` (e.g., `UrlInput.svelte`, `BatchProgress.svelte`)
- **Component props:** `camelCase`
- **Event handlers:** `on:eventName`

### Code Organization Principles

#### 1. **Modularity & Single Responsibility**
- Each crate has a single, clear purpose
- Files kept under 300 lines where possible
- Related functionality grouped in modules

#### 2. **Public API Design**
- Explicit public exports in `lib.rs` or `mod.rs`
- Private implementation details hidden
- Error types clearly defined

#### 3. **Error Handling**
All crates use `thiserror` for error definitions:
```rust
#[derive(Debug, thiserror::Error)]
pub enum CrateError {
    #[error("Description: {0}")]
    Variant(#[from] SourceError),
}
```

#### 4. **Async Runtime**
- **Extractor Engine:** Deno Core (isolated TS runtime)
- **All other crates:** Tokio async runtime
- Long operations use `tokio::spawn()` for non-blocking I/O

#### 5. **Logging**
- Use `tracing` crate consistently
- Log levels: `debug!()` (details), `info!()` (status), `warn!()` (issues), `error!()` (failures)
- Example:
  ```rust
  debug!("Making request to {} (attempt {}/{})", url, attempt + 1, MAX_RETRIES);
  warn!("Received {} for {}, rotating proxy", status, url);
  error!("Max retries exceeded for URL: {}", url);
  ```

## Critical Components Walkthrough

### 1. Muxer Architecture (NEW 2026-02-24)

**Core Strategy:** Video-led dual-track fMP4 muxing

**File Sizes (9 modules, 3,205 LOC total):**
- `traf_merger.rs` (416 LOC) - Merge video+audio fragments into single moof
- `fmp4_remuxer.rs` (407 LOC) - Orchestrate dual-traf remuxing pipeline
- `box_parser.rs` (301 LOC) - BMFF box parsing & timescale extraction
- `moov_merger.rs` (305 LOC) - Merge moov boxes, zero mdhd.duration (QuickTime fix)
- `fragment_stream.rs` (273 LOC) - Collect & stream fragments
- `stream_fetcher.rs` (264 LOC) - Fetch video/audio streams via HTTP
- `mux_router.rs` (255 LOC) - Route streams to muxer, detect format
- `codec.rs` (189 LOC) - Codec classification & validation
- `lib.rs` (101 LOC) - Module exports & error types

**Key Fixes (Deployed 2026-02-24):**
1. **QuickTime Double-Duration:** Moov merger now zeros `mdhd.duration` in both trak boxes
2. **WebM Exclusion:** Stream.rs returns 422 for `mime=video/webm` URLs
3. **Brand Patching:** Remuxer changes `ftyp.major_brand` from `dash` to `isom`
4. **Offset Patching:** Traf merger precisely patches `trun.data_offset` for correct sample location

**Data Flow:**
```
Video Stream → Fragment Collection → Moov Merge → Traf Merge → Remux to ftyp→isom
Audio Stream ↘                       (duration=0) (dual traf)   (offset patched)
```

### 2. Anti-Bot Client (`crates/proxy/src/anti_bot.rs`)

**File Size:** ~300 lines | **Complexity:** High (multi-layer protection)

**Key Components:**
- `AntiBotClient` struct: Main API entry point
- `request_with_retry()` method: Core retry loop with proxy rotation
- Constants: `MAX_RETRIES = 3`, `RETRY_DELAY = 200ms`

**Critical Fix (2026-02-23, Line 99):**
```rust
// BEFORE (causes timeout mid-transfer):
// .timeout(Duration::from_secs(30))

// AFTER (only limits connection, not streaming):
.connect_timeout(Duration::from_secs(30))
```

**Why this fix matters:**
- `.timeout()` applies to entire request (headers + body)
- `.connect_timeout()` only applies to TCP handshake
- For video downloads, we need unlimited time for body transfer
- Only the connection establishment should timeout

### 3. YouTube N-Transform Module (`extractors/youtube-n-transform.ts`)

**File Size:** 173 lines | **Purpose:** Bypass CDN throttling

**Key Functions:**
```typescript
export async function getCachedTransformFn(): Promise<((n: string) => string) | null>
export async function transformStreamUrls(streams: Stream[]): Promise<Stream[]>

function parseNTransformFn(js: string): ((n: string) => string) | null
function escapeRe(s: string): string
function getPlayerJsUrl(): Promise<string | null>
function fetchAndParseTransformFn(playerUrl: string): Promise<((n: string) => string) | null>
```

**Algorithm Overview:**
1. Cache keyed by `playerUrl` (auto-updates when YouTube player changes)
2. Parse transform function from minified player.js using regex
3. Build JavaScript function dynamically with `new Function()`
4. Apply to all stream URLs before download

**Regex Pattern for Function Detection:**
```javascript
/\.get\("n"\)\)&&\(b=([a-zA-Z0-9$]{2,}?)(?:\[(\d+)\])?\(b\)/
```
Matches: `.get("n"))&&(b=FUNCNAME(b)` or `.get("n"))&&(b=FUNCNAME[0](b)`

### 4. YouTube Extractor (`extractors/youtube.ts`)

**File Size:** 220 lines | **Strategy:** InnerTube API → HTML fallback

**Primary Strategy (InnerTube API):**
```typescript
// File: extractors/youtube-innertube.ts
export async function extractViaInnerTube(
  videoId: string,
  originalUrl: string,
  cookies?: string
): Promise<ExtractionResult>

// Impersonates iOS client to get plain URLs (no decryption)
// Applies n-parameter transform for full CDN speed (line 218)
```

**Fallback Strategy (HTML Scraping):**
```typescript
async function extractViaHTML(videoId, originalUrl, cookies): Promise<ExtractionResult>

// Fetches watch page, extracts ytInitialPlayerResponse
// Applies n-parameter transform (line 103)
```

**Integration Points:**
- Both strategies call `transformStreamUrls()` for CDN optimization
- Error handling: InnerTube fails → try HTML → throw error

### 5. yt-dlp Subprocess Extractor (`crates/extractor/src/ytdlp.rs`) [NEW 2026-02-28]

**File Size:** 536 LOC

**Architecture:**
```
URL Input
    │
    ├─► Cache Lookup (moka, 500 items, 300s TTL)
    │       │
    │       ├─► Hit: Return Arc<VideoInfo> (~50μs)
    │       │
    │       └─► Miss: Proceed to extraction
    │
    ├─► Semaphore Acquire (max 10 concurrent)
    │
    └─► yt-dlp Subprocess Call
        │
        Command: yt-dlp -J --no-playlist {url}
        │
        ├─► Success: Parse JSON → Cache → Return
        │
        └─► Failure: Retry with alternate player client
            └─► youtube:player_client=android,web
```

**Key Functions:**
```rust
pub async fn extract_via_ytdlp(url: &str) -> Result<Arc<VideoInfo>, ExtractionError>

fn normalize_cache_key(url: &str) -> String  // Canonical YouTube URL
fn extract_video_id(url: &str) -> Option<&str>  // Parse v= or youtu.be/shorts/
fn resolve_ytdlp_binary() -> String  // YTDLP_PATH env or "yt-dlp"
```

**Caching Strategy:**
- Cache key: Normalized YouTube URL (e.g., `https://www.youtube.com/watch?v=...`)
- TTL: 300 seconds (5 minutes)
- Capacity: 500 items (LRU eviction)
- Thread-safe: Using `moka::future::Cache<String, Arc<VideoInfo>>`

**Rate Limiting:**
- Semaphore: `Arc<Semaphore>::new(10)` — prevents resource exhaustion
- Fallback: On first failure, retry with `youtube:player_client=android,web`

**Metrics:**
- `EXTRACT_CACHE_HITS`: AtomicU64 counter for cache hit rate monitoring
- `EXTRACT_CACHE_MISSES`: For cache efficiency analysis
- `EXTRACT_FALLBACK_RETRIES`: Tracks fallback attempts

**Error Handling:**
```rust
match Command::new(binary).args(&args).output().await {
    Ok(output) => {
        // Parse JSON
        let info: VideoInfo = serde_json::from_slice(&output.stdout)?;
        cache.insert(key.clone(), Arc::new(info.clone())).await;
        Ok(Arc::new(info))
    }
    Err(e) => {
        // Retry with fallback args
        if !retried {
            return extract_via_ytdlp_with_fallback(url).await;
        }
        Err(ExtractionError::YtdlpFailed(e))
    }
}
```

### 6. JWT Authentication & Middleware (`crates/api/src/auth/`) [NEW 2026-02-28]

**Files:**
- `jwt_claims.rs` — JWT payload structure
- `jwt_middleware.rs` — Axum extractor & validation (141 LOC)
- `user_tier.rs` — User subscription levels

**JWT Claims Structure:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub tier: UserTier,  // Free | Pro | Premium
    pub exp: u64,  // Unix timestamp
}
```

**Middleware Pattern:**
```rust
// In route handlers:
async fn extract_route(
    State(state): State<AppState>,
    UserClaims(claims): UserClaims,  // Extracted by middleware
) -> Result<Json<ExtractionResult>, ApiError> {
    // claims.user_id, claims.tier available here
    // Rate limiting applied based on tier
}
```

**Signature Verification:**
```rust
let token_data = decode::<Claims>(
    token,
    &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
    &Validation::default(),
)?;
```

**Rate Limit Tiers:**
| Tier | Daily Extractions | Batch Downloads |
|------|------------------|-----------------|
| Free | 5 | 1 |
| Pro | 50 | 10 |
| Premium | Unlimited | Unlimited |

**BFF Proxy Pattern (SvelteKit):**
```typescript
// Frontend calls SvelteKit endpoint (not Rust directly)
POST /api/extract
  │
  └─► SvelteKit backend:
       ├─► Get JWT from secure HTTP-only cookie
       ├─► Proxy request to Rust API with JWT header
       ├─► Inject Authorization: Bearer {jwt}
       └─► Return response to frontend
```

**Security Benefits:**
- JWT never exposed to browser XSS
- Signature validation prevents token tampering
- Short expiration (1 hour) limits token lifespan

### 7. Whop Webhook Handler (`crates/api/src/routes/whop_webhook.rs`) [NEW 2026-02-28]

**File Size:** 187 LOC

**Signature Verification:**
```rust
pub async fn whop_webhook_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<StatusCode, WebhookError> {
    let signature = headers.get("x-whop-signature")?.to_str()?;

    let mut mac = HmacSha256::new_from_slice(WHOP_API_KEY.as_bytes())?;
    mac.update(&body);
    mac.verify_slice(hex::decode(signature)?)  // HMAC-SHA256
        .map_err(|_| WebhookError::InvalidSignature)?;

    // Signature valid, process webhook
}
```

**Webhook Payload:**
```json
{
  "event": "subscription.created",
  "data": {
    "customer": {
      "id": "cus_...",
      "email": "user@example.com"
    },
    "plan": {
      "id": "plan_pro",
      "name": "Pro Plan"
    },
    "custom_data": "user_id=12345"
  }
}
```

**Database Update:**
```rust
sqlx::query(
    "INSERT INTO subscriptions (user_id, tier, created_at, expires_at)
     VALUES ($1, $2, NOW(), NOW() + INTERVAL '1 month')"
)
.bind(user_id)
.bind(tier_from_whop_plan(&plan))
.execute(&state.db)
.await?;
```

### 8. GPU Pipeline (`crates/gpu-pipeline/src/pipeline.rs`)

**File Size:** ~3,976 tokens

**Data Flow:**
```
Input Stream
    ↓
Decoder (hardware)
    ↓
Frame Queue (buffering)
    ↓
Watermark Overlay
    ↓
Encoder (hardware)
    ↓
Output File
```

**Key Pattern:**
- Async processing with `tokio::spawn()`
- Frame rate management
- Error propagation with `Result<T, PipelineError>`

### 9. Legacy Architecture (Removed 2026-02-24)

**Removed:** Old `fmp4_muxer.rs` module

**Reason:** Replaced by modern dual-traf architecture
- Previous approach: Sequential A/V interleaving
- New approach: Video-led grouping with dual traf boxes
- Benefit: QuickTime compatibility, correct duration, WebM filtering

## Testing & Quality Standards

### Rust Testing

**Pattern:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Arrange
        let input = ...;

        // Act
        let result = function(input);

        // Assert
        assert!(result.is_ok());
    }
}
```

**Coverage Expectations:**
- Happy path tests
- Error case tests
- Edge cases (empty, None, default values)

### TypeScript Testing

**Format:** Jest/Deno test suite
```typescript
Deno.test("test name", async () => {
  const result = await functionUnderTest();
  assertEquals(result, expected);
});
```

## Performance Considerations

| Component | Optimization | Technique |
|-----------|-------------|-----------|
| **Proxy Layer** | Connection reuse | HTTP client with connection pool |
| **N-Transform** | Caching by version | Cache keyed by player.js URL |
| **Extraction** | Hot reload | Minimal runtime overhead |
| **GPU Pipeline** | Hardware acceleration | Native GPU APIs (NVIDIA/AMD) |
| **Muxing** | Streaming output | No full-file buffering |

## Security Practices

### Input Validation
- All user URLs validated with `reqwest::Url::parse()`
- Video IDs checked against regex patterns
- Query parameters sanitized

### Error Messages
- No internal paths in error messages
- No credential leakage in logs
- Sensitive data (tokens, cookies) never logged

### Dependency Management
- Rust: cargo audit for vulnerabilities
- TypeScript: npm audit
- Regular updates via dependabot

## Build & Compilation

### Cargo Targets
```bash
# From Makefile:
cargo build --release          # Optimized build
cargo test --all               # Run all tests
cargo clippy --all             # Lint checks
cargo fmt --all -- --check     # Format verification
```

### TypeScript Compilation
```bash
# Extractors are run directly by Deno Core
# No build step needed for .ts files
# Optional: compile to .js for distribution
```

### Docker Build
```bash
# Frontend requires build-time API base URL (injected into client bundle)
docker build \
  --build-arg VITE_API_URL=https://api.example.com \
  -f docker/Dockerfile.frontend .

# Multi-stage build for smaller images
# Extracts release artifacts only
```

**Important:** Do not rely on runtime env for `import.meta.env.VITE_API_URL`. Browser bundle values are fixed at build time.

## Documentation Standards

### Rust Code Comments
```rust
/// Public function documentation (doc comments)
/// Uses standard Rust doc format
///
/// # Arguments
/// * `param` - Description
///
/// # Returns
/// Description of return value
///
/// # Errors
/// When this function returns error
pub fn function(param: Type) -> Result<Output, Error> {
    // Implementation comment: explain non-obvious logic
}
```

### TypeScript Comments
```typescript
/**
 * JSDoc format for public APIs
 *
 * @param url - Video URL to extract
 * @returns Stream array and metadata
 * @throws ExtractionError on failure
 */
export async function extract(url: string): Promise<ExtractionResult>
```

## Git Workflow & Commits

### Commit Message Format
```
feat: add YouTube n-parameter transform for full-speed CDN
fix: change timeout to connect_timeout for streaming
docs: update system architecture documentation
refactor: extract throttle logic into separate module
```

### Branching
- Feature branches: `feature/name-of-feature`
- Bug fixes: `fix/bug-description`
- Release: `release/v1.0.0`

### Before Push
```bash
# Run linting
cargo clippy --all

# Run tests
cargo test --all

# Check formatting
cargo fmt --all -- --check

# Security audit
cargo audit
```

## Common Pitfalls & Solutions

| Pitfall | Issue | Solution |
|---------|-------|----------|
| WebM video streams | EBML container (not BMFF) | Stream.rs returns 422, filter in FormatPicker |
| QuickTime duration | YouTube sums mdhd.duration | Moov merger zeros both trak mdhd.duration |
| `.timeout()` on streams | Kills mid-transfer | Use `.connect_timeout()` instead |
| Blocking in async | Tokio panic | Use `tokio::spawn()` or `tokio::task::block_in_place()` |
| Extractor errors silent | Hard to debug | Check logs with `RUST_LOG=debug` |
| Cookie jar not shared | Per-request cookies | Use `Arc<CookieStore>` + Arc<Client> |
| N-transform cache miss | Slow first request | Cache per player version automatically |
| fMP4 brand mismatch | Dash vs isom | Remuxer patches brand to isom (QuickTime) |

## Deployment Checklist

- [ ] All tests passing (`cargo test --all`)
- [ ] No clippy warnings (`cargo clippy --all`)
- [ ] Code formatted (`cargo fmt --all`)
- [ ] Security audit clean (`cargo audit`)
- [ ] Documentation updated (`docs/*.md`)
- [ ] Docker image builds (`docker build`)
- [ ] Environment variables set (`.env`)
- [ ] Database migrations run (if applicable)

---

**Version:** 1.2
**Last Updated:** 2026-02-24 (Added QuickTime Duration Fix, WebM Exclusion, Dual-Traf Muxer documentation)
