/**
 * YouTube N-Parameter Transform
 *
 * YouTube CDN throttles downloads to ~100 KB/s when the `n` query param is not
 * transformed. This module extracts the transform function from YouTube player.js
 * and applies it to CDN URLs.
 *
 * Based on the same technique used by yt-dlp.
 */

import { Stream } from "./types.ts";

interface NTransformCache {
  playerUrl: string;
  transformFn: ((n: string) => string) | null;
}

let cache: NTransformCache | null = null;

/** Fetch YouTube homepage and extract current player.js URL */
async function getPlayerJsUrl(): Promise<string | null> {
  try {
    const resp = await fetch("https://www.youtube.com/", {
      headers: {
        "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
      },
    });
    if (!resp.ok) return null;
    const html = await resp.text();
    // e.g. /s/player/abc123/player_ias.vflset/en_US/base.js or vi_VN/base.js
    const m = html.match(
      /\/s\/player\/([a-zA-Z0-9_-]+)\/player_ias\.vflset\/[a-zA-Z_]+\/base\.js/
    );
    return m ? `https://www.youtube.com${m[0]}` : null;
  } catch {
    return null;
  }
}

/** Escape string for use in RegExp */
function escapeRe(s: string): string {
  return s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/** Parse n-transform function from player JS text */
function parseNTransformFn(js: string): ((n: string) => string) | null {
  // Step 1: Find the function name that processes the n-param
  // Patterns: .get("n"))&&(b=FUNCNAME(b)  or  .get("n"))&&(b=FUNCNAME[0](b)
  const nameMatch = js.match(
    /\.get\("n"\)\)&&\(b=([a-zA-Z0-9$]{2,}?)(?:\[(\d+)\])?\(b\)/
  );
  if (!nameMatch) return null;

  let funcName = nameMatch[1];
  const arrayIdx = nameMatch[2];

  // Resolve array access c[0] → look up: var c=[fn1,fn2,...]
  if (arrayIdx !== undefined) {
    const arrM = js.match(
      new RegExp(`var ${escapeRe(funcName)}\\s*=\\s*\\[([^\\]]+)\\]`)
    );
    if (arrM) {
      const parts = arrM[1].split(",").map((s) => s.trim());
      funcName = parts[parseInt(arrayIdx)] || funcName;
    }
  }

  // Step 2: Extract function body (try var assignment and declaration forms)
  const esc = escapeRe(funcName);
  const bodyPatterns = [
    new RegExp(`${esc}=function\\(a\\)\\{(.+?)\\};`),
    new RegExp(`function ${esc}\\(a\\)\\{(.+?)\\}`),
  ];

  let body: string | null = null;
  for (const pat of bodyPatterns) {
    const m = js.match(pat);
    if (m) { body = m[1]; break; }
  }
  if (!body) return null;

  // Step 3: Extract helper object referenced in function body
  // e.g. var b=HELPER; b.method(arr, n)  →  need to include HELPER definition
  let helperCode = "";
  const helperRef = body.match(/var\s+\w+\s*=\s*([a-zA-Z0-9$]{2,})\s*;/);
  if (helperRef) {
    const helperName = helperRef[1];
    const helperPat = new RegExp(
      `var ${escapeRe(helperName)}\\s*=\\s*\\{[\\s\\S]+?\\}\\s*;`
    );
    const hm = js.match(helperPat);
    if (hm) helperCode = hm[0];
  }

  // Step 4: Build and test the function
  try {
    // Wrap helper + body in a factory to keep scope clean
    const fn = new Function(
      `${helperCode}return function(a){${body}}`
    )() as (n: string) => string;

    // Sanity check: must return a non-empty string
    const test = fn("testvalue");
    if (typeof test === "string" && test.length > 0) return fn;
    return null;
  } catch {
    return null;
  }
}

/** Fetch player.js and extract n-transform function */
async function fetchAndParseTransformFn(
  playerUrl: string
): Promise<((n: string) => string) | null> {
  try {
    const resp = await fetch(playerUrl);
    if (!resp.ok) return null;
    const js = await resp.text();
    return parseNTransformFn(js);
  } catch {
    return null;
  }
}

/** Get cached transform function, fetching player.js if version changed */
export async function getCachedTransformFn(): Promise<
  ((n: string) => string) | null
> {
  const playerUrl = await getPlayerJsUrl();
  if (!playerUrl) return null;

  // Cache hit: same player version
  if (cache?.playerUrl === playerUrl) return cache.transformFn;

  // Cache miss: new player version or first call
  const fn = await fetchAndParseTransformFn(playerUrl);
  cache = { playerUrl, transformFn: fn };
  return fn;
}

/** Apply n-parameter transform to all stream URLs */
export async function transformStreamUrls(streams: Stream[]): Promise<Stream[]> {
  // Check if any URL actually has an n-param before fetching player.js
  const hasNParam = streams.some((s) => {
    try { return new URL(s.url).searchParams.has("n"); } catch { return false; }
  });
  // iOS InnerTube client URLs have no n-param → skip silently
  if (!hasNParam) return streams;

  const fn = await getCachedTransformFn();
  if (!fn) {
    _log("warn", "N-param transform unavailable; downloads may be throttled");
    return streams;
  }

  return streams.map((stream) => {
    try {
      const url = new URL(stream.url);
      const n = url.searchParams.get("n");
      if (n) {
        url.searchParams.set("n", fn(n));
        return { ...stream, url: url.toString() };
      }
    } catch {
      // Keep original URL on any error
    }
    return stream;
  });
}

/** Log via deno_core op when available */
function _log(level: string, msg: string): void {
  try {
    // @ts-ignore - Deno.core available in deno_core runtime
    if (typeof Deno !== "undefined" && Deno.core?.ops?.op_log) {
      // @ts-ignore
      Deno.core.ops.op_log(level, `[n-transform] ${msg}`);
    }
  } catch { /* ignore */ }
}
