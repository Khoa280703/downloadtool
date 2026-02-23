# System Architecture

**Last Updated:** 2026-02-23

## High-Level Architecture

```
Internet
   │
   ├─► YouTube CDN (via proxy + n-param transform)
   ├─► YouTube InnerTube API
   └─► Other Platforms
        │
        ▼
   ┌────────────────────────────────────────────────┐
   │   Anti-Bot & Proxy Layer (crates/proxy)        │
   │ • Proxy rotation & health tracking              │
   │ • Cookie persistence per platform               │
   │ • Header randomization                          │
   │ • Domain request throttling                      │
   │ • Retry logic (exponential backoff)              │
   └────────┬─────────────────────────────────────────┘
            │
            ▼
   ┌────────────────────────────────────────────────┐
   │   Extraction Engine (crates/extractor)         │
   │ • Deno runtime for TS/JS extractors            │
   │ • Hot reload support                           │
   │ • Connection pooling                           │
   │ • Returns: streams[], metadata                 │
   └────────┬─────────────────────────────────────────┘
            │
            ├─► YouTube N-Transform (extractors/youtube-n-transform.ts)
            │   • Fetches player.js
            │   • Extracts n-param transform function
            │   • Caches by version
            │   • Applies to CDN URLs
            │
            ▼
   ┌────────────────────────────────────────────────┐
   │   API Server (crates/api)                      │
   │ • HTTP routes:                                 │
   │   - POST /extract → metadata                   │
   │   - POST /batch → schedule downloads           │
   │   - WS /stream → stream data                   │
   │   - POST /transcode → GPU job                  │
   │ • Request validation & error handling          │
   └────────┬─────────────────────────────────────────┘
            │
   ┌────────┴──────┬──────────────────┬──────────────┐
   │               │                  │              │
   ▼               ▼                  ▼              ▼
┌──────┐    ┌──────────┐        ┌──────────┐   ┌──────────┐
│ Muxer│    │GPU       │        │WebSocket │   │Frontend  │
│(fMP4)│    │Pipeline  │        │Stream    │   │(Svelte)  │
└──────┘    └──────────┘        └──────────┘   └──────────┘
   │            │                    │
   ▼            ▼                    ▼
Stored File  Transcoded File    Browser UI
```

## Data Flow Diagrams

### 1. Download & Extraction Flow

```
User Input (URL)
      │
      ▼
┌──────────────────┐
│ Frontend: URLInput│
│ /extract POST    │
└──────┬───────────┘
       │
       ▼
┌─────────────────────────────┐
│ API: extract route          │
│ • Parse video ID            │
│ • Validate platform         │
└──────┬──────────────────────┘
       │
       ▼
┌──────────────────────────────────┐
│ Extractor Engine                │
│ • Load TypeScript extractor     │
│ • Deno runtime execution        │
│ • Platform-specific logic       │
└──────┬───────────────────────────┘
       │
       ▼ (YouTube path)
┌──────────────────────────────────┐
│ YouTube Extractor Pipeline       │
│                                  │
│ ┌─────────────────────────────┐  │
│ │ extractViaInnerTube()       │  │
│ │ • Impersonate iOS client    │  │
│ │ • Get plain stream URLs     │  │
│ │ • Apply n-param transform   │◄─┤──── youtube-n-transform.ts
│ │ • Return streams[]          │  │
│ └─────────────────────────────┘  │
│                │                 │
│            (fails)               │
│                │                 │
│                ▼                 │
│ ┌─────────────────────────────┐  │
│ │ extractViaHTML()            │  │
│ │ • Fetch watch page          │  │
│ │ • Parse ytInitialPlayer     │  │
│ │ • Apply n-param transform   │  │
│ │ • Return streams[]          │  │
│ └─────────────────────────────┘  │
└──────┬───────────────────────────┘
       │
       ▼
┌─────────────────────────────┐
│ Anti-Bot Client             │
│ • Proxy rotation            │
│ • Cookie injection          │
│ • Header randomization      │
│ • Throttle enforcement      │
└──────┬──────────────────────┘
       │
       ▼
┌─────────────────────────────┐
│ YouTube CDN                 │
│ Returns: video data         │
└──────┬──────────────────────┘
       │
       ▼
    Browser ← Streams delivered via WebSocket
```

### 2. YouTube N-Parameter Transform Flow (NEW 2026-02-23)

```
YouTube Stream URL (with throttled n-param)
      │
      ▼
┌──────────────────────────────┐
│ getCachedTransformFn()       │
│ • Check cache hit            │
│ • If miss: getPlayerJsUrl()  │
└──────┬───────────────────────┘
       │
       ├─── cache HIT ──────────────► Use cached function
       │
       └─── cache MISS
            │
            ▼
      ┌─────────────────────┐
      │ getPlayerJsUrl()    │
      │ • Fetch youtube.com │
      │ • Extract JS URL    │
      │ e.g. /s/player/    │
      │   abc123/base.js    │
      └─────┬───────────────┘
            │
            ▼
      ┌──────────────────────┐
      │fetchAndParseTransform│
      │ • GET player.js      │
      │ • parseNTransformFn()│
      └──────┬───────────────┘
             │
             ▼
      ┌──────────────────────────────┐
      │ parseNTransformFn(js_text)  │
      │ 1. Find function name        │
      │    Pattern: .get("n")&&      │
      │    (b=FUNCNAME(b)            │
      │ 2. Extract function body     │
      │ 3. Find helper object        │
      │ 4. Build & test function     │
      │ 5. Return transform fn       │
      └──────┬───────────────────────┘
             │
             ▼
      Cache: { playerUrl, transformFn }
             │
             ▼
      ┌──────────────────────────────┐
      │ transformStreamUrls()        │
      │ For each stream:             │
      │ • Extract n from URL         │
      │ • Apply: n' = fn(n)          │
      │ • Replace URL param          │
      │ • Return transformed URL     │
      └──────┬───────────────────────┘
             │
             ▼
   Full-speed CDN download (~2-3 Mbps)
   (Instead of 100 KB/s throttled)
```

### 3. GPU Transcoding Flow

```
Video Stream (proxy/CDN)
      │
      ▼
┌──────────────────────┐
│ API: transcode route │
│ • Receive job params │
└──────┬───────────────┘
       │
       ▼
┌──────────────────────────────┐
│ GPU Pipeline (crates/gpu-    │
│ pipeline)                    │
│ ┌────────────────────────┐   │
│ │ decoder.rs             │   │
│ │ • Hardware decode      │   │
│ │ • Frame buffering      │   │
│ └────────┬───────────────┘   │
│          │                   │
│          ▼                   │
│ ┌────────────────────────┐   │
│ │ frame_queue.rs         │   │
│ │ • Async buffering      │   │
│ │ • Back-pressure mgmt   │   │
│ └────────┬───────────────┘   │
│          │                   │
│          ▼                   │
│ ┌────────────────────────┐   │
│ │ watermark.rs           │   │
│ │ • Overlay processing   │   │
│ │ • Alpha blending       │   │
│ └────────┬───────────────┘   │
│          │                   │
│          ▼                   │
│ ┌────────────────────────┐   │
│ │ encoder.rs             │   │
│ │ • Hardware encode      │   │
│ │ • Format conversion    │   │
│ │ • Bitrate control      │   │
│ └────────────────────────┘   │
└──────┬───────────────────────┘
       │
       ▼
┌──────────────────────┐
│ GPU Worker (crates/ │
│ gpu-worker)         │
│ • gRPC server       │
│ • Job execution     │
└──────┬───────────────┘
       │
       ▼
┌──────────────────────┐
│ Muxer (crates/muxer)│
│ • fMP4 format       │
│ • Interleaved A/V   │
│ • Seek optimized    │
└──────┬───────────────┘
       │
       ▼
   Output File (MP4, WebM, etc.)
```

## Component Interfaces

### API Routes

```
POST /extract
  Request:  { url: string, cookies?: string }
  Response: {
    streams: Stream[],
    title: string,
    thumbnail?: string,
    duration?: number,
    platform: string
  }

POST /batch
  Request:  { downloads: Download[] }
  Response: { jobId: string }

WS /stream
  Upgrade to WebSocket
  Subscribe to stream events
  Receive chunked video data

POST /transcode
  Request:  { inputUrl: string, codec: string, bitrate: number, ... }
  Response: { jobId: string, outputUrl: string }
```

### Extractor Interface

```typescript
// In extractors/*.ts
export async function extract(
  url: string,
  cookies?: string
): Promise<ExtractionResult>

// ExtractionResult
{
  streams: Stream[],        // Available qualities
  title: string,
  thumbnail?: string,
  duration?: number,
  platform: string
}

// Stream
{
  url: string,              // Direct CDN/server URL
  quality: string,          // "1080p", "720p", "Audio 128kbps"
  format: string,           // "mp4", "webm"
  mime: string,             // "video/mp4; codecs=..."
  bitrate?: number,
  codec?: string,
  width?: number,
  height?: number
}
```

### N-Transform Module (youtube-n-transform.ts)

```typescript
// Get cached transform function (handles versioning)
export async function getCachedTransformFn(): Promise<
  ((n: string) => string) | null
>

// Apply transform to stream URLs
export async function transformStreamUrls(
  streams: Stream[]
): Promise<Stream[]>

// Internal: Parse player.js
function parseNTransformFn(js: string): ((n: string) => string) | null

// Internal: Escape regex strings
function escapeRe(s: string): string
```

### Anti-Bot Client (crates/proxy)

```rust
impl AntiBotClient {
  pub fn new(platform: Platform) -> Result<Self, AntiBotError>
  pub async fn get(&self, url: &str) -> Result<Response, AntiBotError>
  pub async fn get_with_range(
    &self,
    url: &str,
    range: Option<String>
  ) -> Result<Response, AntiBotError>
  pub async fn fetch_stream(
    &self,
    url: &str,
    range: Option<String>
  ) -> Result<impl Stream<Item=Result<Bytes, _>>, AntiBotError>
  pub async fn warm_up(&self) -> Result<(), AntiBotError>
  pub async fn reset_cookies(&self) -> Result<(), AntiBotError>
}
```

## Critical Components Deep-Dive

### 1. Anti-Bot Layer (`crates/proxy/src/anti_bot.rs`)

**Key Fix (2026-02-23):** Timeout configuration

```rust
// BEFORE (line 99): Would timeout mid-transfer
// .timeout(Duration::from_secs(30))

// AFTER (line 99): Only limits connection establishment
.connect_timeout(Duration::from_secs(30))
```

**Why this matters:**
- `.timeout()` - Total request duration timeout (kills long downloads)
- `.connect_timeout()` - Only TCP connection establishment timeout (allows streaming)

**Retry Logic:**
- MAX_RETRIES: 3 attempts
- RETRY_DELAY: 200ms
- Handles 403 (Forbidden) & 429 (Too Many Requests)
- Proxy rotation on failure
- Cookie clearing on 403 (session-related)

### 2. YouTube N-Parameter Transform (`extractors/youtube-n-transform.ts`)

**Purpose:** Bypass YouTube CDN throttling (100 KB/s → 2-3 Mbps)

**Algorithm:**
1. Fetch `https://www.youtube.com/` → extract player.js URL
2. Fetch player.js, find n-parameter transform function via regex
3. Cache by player version
4. For each stream URL: extract `n` query param, apply transform, replace URL

**Key Regex Pattern:**
```javascript
// Finds function that transforms n-param
/\.get\("n"\)\)&&\(b=([a-zA-Z0-9$]{2,}?)(?:\[(\d+)\])?\(b\)/
```

**Why it works:**
- YouTube's player.js contains the actual transform function
- Same technique used by yt-dlp
- Function is minified but structure is consistent
- Caching prevents repeated player.js fetches

### 3. Extraction Engine (`crates/extractor/src/engine.rs`)

**Architecture:**
- Hot-reload TS extractors from `/extractors` directory
- Deno Core runtime (not Node.js)
- Type-safe bridge via `serde`
- Connection pooling

**YouTube Strategy (youtube.ts):**
1. **Primary:** InnerTube API (iOS client)
   - Returns plain URLs (no signature decryption)
   - Works for most videos
   - All resolutions up to 4K
2. **Fallback:** HTML scraping
   - ytInitialPlayerResponse parsing
   - For restricted/age-gated videos

## Data Structures

### Stream Object
```typescript
{
  url: string                 // Direct CDN/server URL
  quality: string             // Human-readable quality
  format: string              // File extension
  mime: string                // MIME type
  bitrate?: number            // bps
  codec?: string              // Video codec
  width?: number              // Pixel width
  height?: number             // Pixel height
}
```

### Platform Enum
```rust
enum Platform {
  YouTube,
  TikTok,
  Instagram,
  Twitter,
  // ...
}
```

### ExtractionError
```typescript
{
  message: string
  platform: string
  timestamp?: number
}
```

## Request/Response Flow with Timeout Fix

### Before (2026-02-22)
```
[Client] ──────────► [HTTP Request] ──────────► [YouTube CDN]
         create           30s timeout           sending 1GB
         request           |                    file...
                           |
                        (TIMEOUT - Connection killed at 30s)
                        ❌ Download fails
```

### After (2026-02-23)
```
[Client] ──────────► [HTTP Request] ──────────► [YouTube CDN]
         create      30s connect_timeout       sending 1GB
         request      (TCP handshake only)     file...
                       |                        |
                   (connected)                  |
                       |                        |
                   [Streaming]  ◄──────────────┘
                   (NO timeout on body transfer)
                   ✅ Download completes
```

## Security & Anti-Detection Measures

| Layer | Technique | Implementation |
|-------|-----------|-----------------|
| **Proxy** | IP rotation | Pool of healthy proxies, failure tracking |
| **Headers** | User-Agent rotation | Random selection from common browsers |
| **Cookies** | Persistence | Per-platform cookie jar |
| **Throttling** | Domain-level rate limiting | Request queuing per domain |
| **Retry** | Exponential backoff | 200ms delays, max 3 retries |
| **N-Parameter** | Anti-throttle | Extracted from player.js, cached |

## Performance Characteristics

| Metric | Value | Notes |
|--------|-------|-------|
| **Extraction** | <1s | InnerTube API, usually 200-500ms |
| **N-Transform Cache** | ~10ms first, <1ms cached | Player.js fetch ~5-10s first |
| **Proxy Rotation** | ~50ms overhead | Per-request proxy selection |
| **CDN Download** | 2-3 Mbps | With n-param (vs 100 KB/s without) |
| **GPU Transcode** | Hardware-dependent | Real-time or faster |

## Deployment Architecture

```
┌─────────────────────┐
│  Frontend (SvelteKit)│ ──── Browser
└──────────┬──────────┘
           │ HTTP/WS
           ▼
┌──────────────────────────────┐
│  API Server (crates/api)     │
│  Listens on :8000            │
└──────┬───────────────┬────────┘
       │               │
       │        (gRPC) │
       │               ▼
       │        ┌──────────────────────┐
   (spawns)    │  GPU Worker Process   │
       │        │  (separate container) │
       ▼        └──────────────────────┘
   ┌────────────────────────────┐
   │ Docker Container           │
   │ • API server               │
   │ • Extractor engine         │
   │ • Proxy & anti-bot         │
   │ • Muxer                    │
   └────────────────────────────┘

Output: Video files → CDN/S3 → Browser download
```

## Error Handling Strategy

| Error | Handling | Recovery |
|-------|----------|----------|
| **InnerTube fails** | Log & fallback | Try HTML scraping |
| **Bot detection (403/429)** | Mark proxy failed | Rotate proxy, retry |
| **Timeout (30s+ connect)** | Log & retry | Use different proxy |
| **Extraction fails** | ExtractionError | Return error to frontend |
| **GPU job fails** | Log error | Return error response |

---

**Version:** 2.0 (Updated with N-Param Transform & Timeout Fix)
