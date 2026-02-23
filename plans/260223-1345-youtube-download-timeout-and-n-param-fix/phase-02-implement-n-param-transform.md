# Phase 2: Implement N-Parameter Transform

## Context Links

- [plan.md](plan.md) -- overview
- [youtube-innertube.ts](../../extractors/youtube-innertube.ts) -- primary file to modify
- [youtube.ts](../../extractors/youtube.ts) -- HTML fallback also needs n-param
- [runtime.rs](../../crates/extractor/src/runtime.rs) -- domain whitelist reference

## Overview

- **Priority**: P1 (critical -- downloads throttled to 50-200 KB/s without this)
- **Status**: completed
- **Effort**: 2.5h

YouTube CDN throttles download speed when the `n` parameter in stream URLs is not transformed. The `n` value must be run through an obfuscated JS function extracted from YouTube's player JS. This is the same technique used by yt-dlp.

## Key Insights

- YouTube stream URLs contain `&n=XXXX` parameter; untransformed = throttled to ~100 KB/s
- The transform function is embedded in YouTube's base.js player script (~1.5MB minified)
- Player JS URL pattern: `/s/player/HASH/player_ias.vflset/en_US/base.js`
- The function changes with each player version; must extract dynamically
- IOS/ANDROID clients also return URLs with `n` param that needs transforming
- Module-level caching of extracted function + player URL avoids re-fetching per request
- The transform function is pure JS; can be `eval()`'d in the deno_core runtime

## Requirements

### Functional
- Extract n-transform function from current YouTube player JS
- Apply transform to `n` parameter of all CDN stream URLs
- Cache extracted function at module level (reuse across extractions)
- Invalidate cache when player version changes
- Handle extraction failure gracefully (return untransformed URLs, log warning)

### Non-functional
- Player JS fetch adds ~1-2s to first extraction (cached after)
- Must not break existing extraction if n-param extraction fails
- Must work within deno_core's `fetch` polyfill (uses `op_fetch` under the hood)

## Architecture

```
extractViaInnerTube(videoId)
  |
  v
callInnerTube() --> streamingData with CDN URLs
  |
  v
transformNParams(streams[])   <-- NEW
  |
  +-- getCachedOrFetchPlayerJs()
  |     +-- fetch youtube.com homepage
  |     +-- regex extract player JS URL
  |     +-- fetch player JS
  |     +-- regex extract n-transform function body
  |     +-- cache { playerUrl, transformFn }
  |
  +-- for each stream URL:
        +-- parse URL, get n= param
        +-- run transformFn(n) --> new_n
        +-- replace n= in URL
  |
  v
return transformed streams
```

## Related Code Files

### Files to Modify
- `extractors/youtube-innertube.ts` -- add n-param transform module with:
  - Module-level cache variable
  - `getPlayerJsUrl()` -- fetch youtube.com, extract player JS URL
  - `extractNTransformFunction()` -- regex extract function from player JS
  - `transformNParam()` -- apply transform to single URL
  - `transformAllNParams()` -- apply to all streams
  - Call `transformAllNParams()` in `extractViaInnerTube()` before returning

- `extractors/youtube.ts` -- call same n-param transform in `extractViaHTML()` fallback path

### Files to Verify (no changes expected)
- `crates/extractor/src/runtime.rs` -- domain whitelist already includes `youtube.com` and `googlevideo.com`
- `extractors/types.ts` -- no changes needed

## Implementation Steps

### Step 1: Add module-level cache structure (youtube-innertube.ts)

Add at top of file after imports:

```typescript
// N-parameter transform cache
let nTransformCache: {
  playerUrl: string;
  transformFn: ((n: string) => string) | null;
} | null = null;
```

### Step 2: Implement getPlayerJsUrl()

```typescript
async function getPlayerJsUrl(): Promise<string | null> {
  const resp = await fetch("https://www.youtube.com/", {
    method: "GET",
    headers: { "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36" }
  });
  if (!resp.ok) return null;
  const html = await resp.text();
  // Match: /s/player/HASH/player_ias.vflset/en_US/base.js
  const match = html.match(/\/s\/player\/([a-zA-Z0-9_-]+)\/player_ias\.vflset\/[a-z_]+\/base\.js/);
  return match ? `https://www.youtube.com${match[0]}` : null;
}
```

### Step 3: Implement extractNTransformFunction()

Extract the n-transform function from player JS using yt-dlp-style regex patterns:

```typescript
async function extractNTransformFunction(playerJsUrl: string): Promise<((n: string) => string) | null> {
  const resp = await fetch(playerJsUrl);
  if (!resp.ok) return null;
  const js = await resp.text();

  // Pattern 1: Find function name from n-param usage
  // Look for: .get("n"))&&(b=FUNCNAME(b)  or  &&(b=c[INDEX](b)
  const funcNameMatch = js.match(
    /\.get\("n"\)\)&&\(b=([a-zA-Z0-9$]+)(?:\[(\d+)\])?\(([a-zA-Z0-9])\)/
  );
  if (!funcNameMatch) return null;

  let funcName = funcNameMatch[1];
  const arrayIndex = funcNameMatch[2]; // may be undefined

  // If it's an array access like c[0](b), resolve the actual function name
  if (arrayIndex !== undefined) {
    // Find: var c=[funcA,funcB,...];
    const arrayPattern = new RegExp(
      `var ${funcName.replace(/\$/g, "\\$")}\\s*=\\s*\\[([^\\]]+)\\]`
    );
    const arrayMatch = js.match(arrayPattern);
    if (arrayMatch) {
      const elements = arrayMatch[1].split(",").map(s => s.trim());
      funcName = elements[parseInt(arrayIndex)] || funcName;
    }
  }

  // Extract function body: function FUNCNAME(a){var b=a.split(""),...;return b.join("")}
  // Or: var FUNCNAME=function(a){...}
  const escapedName = funcName.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const funcBodyPatterns = [
    new RegExp(`${escapedName}=function\\(a\\)\\{((?:(?!};).)+)\\}`),
    new RegExp(`function ${escapedName}\\(a\\)\\{((?:(?!};).)+)\\}`),
  ];

  let funcBody: string | null = null;
  for (const pattern of funcBodyPatterns) {
    const m = js.match(pattern);
    if (m) { funcBody = m[1]; break; }
  }

  if (!funcBody) return null;

  // Build executable function
  try {
    const fn = new Function("a", funcBody) as (n: string) => string;
    // Quick sanity test
    const testResult = fn("abc");
    if (typeof testResult !== "string") return null;
    return fn;
  } catch {
    return null;
  }
}
```

**Important**: The regex patterns above are starting points. YouTube frequently obfuscates differently. The implementation should try multiple patterns and log warnings on failure.

### Step 4: Implement getCachedTransformFn()

```typescript
async function getCachedTransformFn(): Promise<((n: string) => string) | null> {
  // Check if cache is still valid
  const playerUrl = await getPlayerJsUrl();
  if (!playerUrl) return null;

  if (nTransformCache && nTransformCache.playerUrl === playerUrl) {
    return nTransformCache.transformFn;
  }

  // Cache miss or player version changed
  const fn = await extractNTransformFunction(playerUrl);
  nTransformCache = { playerUrl, transformFn: fn };
  return fn;
}
```

### Step 5: Implement transformStreamUrls()

```typescript
async function transformStreamUrls(streams: Stream[]): Promise<Stream[]> {
  const transformFn = await getCachedTransformFn();
  if (!transformFn) {
    // Log warning but don't fail -- streams will work, just throttled
    if (typeof Deno !== "undefined") {
      Deno.core.ops.op_log("warn", "N-param transform unavailable; downloads may be throttled");
    }
    return streams;
  }

  return streams.map(stream => {
    try {
      const url = new URL(stream.url);
      const n = url.searchParams.get("n");
      if (n) {
        const transformed = transformFn(n);
        url.searchParams.set("n", transformed);
        return { ...stream, url: url.toString() };
      }
    } catch {
      // Keep original URL on error
    }
    return stream;
  });
}
```

### Step 6: Integrate into extractViaInnerTube()

In `extractViaInnerTube()`, after `parseStreams(data)` and before returning:

```typescript
// Transform n-params for full-speed downloads
const transformedStreams = await transformStreamUrls(streams);
```

Return `transformedStreams` instead of `streams`.

### Step 7: Integrate into extractViaHTML() (youtube.ts)

Export `transformStreamUrls` from `youtube-innertube.ts`, then call it in `youtube.ts` `extractViaHTML()` before returning:

```typescript
import { extractViaInnerTube, transformStreamUrls } from "./youtube-innertube.ts";
// ... in extractViaHTML, before return:
result.streams = await transformStreamUrls(result.streams);
```

### Step 8: Bundle and test

```bash
cd extractors && npx esbuild youtube.ts --bundle --format=iife --global-name=youtube --platform=neutral --target=es2020 --outfile=dist/youtube.js
~/.cargo/bin/cargo b
# Manual test: download video, verify speed > 1 MB/s
```

## Todo List

- [x] Add module-level n-transform cache variable
- [x] Implement `getPlayerJsUrl()` -- fetch youtube.com, extract player JS URL
- [x] Implement `extractNTransformFunction()` -- regex extract from player JS
- [x] Implement `getCachedTransformFn()` -- cache layer
- [x] Implement `transformStreamUrls()` -- apply transform to all stream URLs
- [x] Integrate into `extractViaInnerTube()` in youtube-innertube.ts
- [x] Export and integrate into `extractViaHTML()` in youtube.ts
- [x] Bundle JS with esbuild
- [x] Verify Rust compilation
- [x] Manual test: confirm download speed improvement (>1 MB/s)
- [x] Test graceful degradation when n-param extraction fails

## Success Criteria

- YouTube downloads achieve full CDN speed (typically 5-50 MB/s)
- First extraction adds ~1-2s for player JS fetch; subsequent extractions use cache
- Extraction does NOT fail if n-param transform is unavailable (graceful degradation)
- JS bundle compiles, Rust compiles, server starts correctly

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| YouTube changes player JS obfuscation patterns | High | Multiple regex patterns; graceful fallback; log warnings for monitoring |
| Player JS too large for op_fetch (1.5MB) | Medium | op_fetch already handles large responses; 30s timeout should suffice |
| `new Function()` blocked in deno_core | High | deno_core V8 allows dynamic code eval by default; verify in testing |
| n-transform function uses external helper functions | Medium | May need to extract helper objects too (yt-dlp extracts entire transform chain) |
| URL class not available in deno_core | Low | Already using standard Web APIs; URL is available in V8 |

## Security Considerations

- `new Function()` executes code extracted from YouTube's player JS -- this is intentional and necessary
- The code runs inside deno_core's sandboxed V8 isolate with domain-whitelisted fetch
- Only `youtube.com` and `googlevideo.com` domains are fetchable (whitelist in runtime.rs)
- No user input reaches `new Function()` -- only YouTube's own player code

## Next Steps

- After both phases: end-to-end test with multiple YouTube videos (different resolutions, live, shorts)
- Monitor logs for n-param extraction failures after deployment
- If YouTube changes obfuscation frequently, consider more robust extraction (AST-based) in future
