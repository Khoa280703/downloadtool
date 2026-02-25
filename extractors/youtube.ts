/**
 * YouTube Video Extractor
 *
 * Strategy:
 *   1. InnerTube API (ANDROID → TV client) — plain URLs, no signature needed
 *   2. HTML scraping fallback — ytInitialPlayerResponse parsing
 */

import { Stream, ExtractionResult, ExtractionError } from "./types.ts";
import { extractViaInnerTube } from "./youtube-innertube.ts";
import { transformStreamUrls } from "./youtube-n-transform.ts";

/** Extract video information from a YouTube URL */
export async function extract(
  url: string,
  cookies?: string
): Promise<ExtractionResult> {
  const videoId = parseVideoId(url);
  if (!videoId) {
    throw new ExtractionError(
      "Invalid YouTube URL: could not extract video ID",
      "youtube"
    );
  }

  // Primary: InnerTube API (plain URLs, works for most videos)
  try {
    return await extractViaInnerTube(videoId, url, cookies);
  } catch (err) {
    // Fall through to HTML scraping
    console.warn(`[youtube] InnerTube failed: ${err}. Falling back to HTML scraping.`);
  }

  // Fallback: scrape ytInitialPlayerResponse from the watch page
  return extractViaHTML(videoId, url, cookies);
}

// ─── Video ID Parsing ─────────────────────────────────────────────────────────

const VIDEO_ID_PATTERNS = [
  /(?:youtube\.com\/watch\?v=|youtu\.be\/|youtube\.com\/embed\/)([a-zA-Z0-9_-]{11})/,
  /youtube\.com\/shorts\/([a-zA-Z0-9_-]{11})/,
  /youtube\.com\/live\/([a-zA-Z0-9_-]{11})/,
];

function parseVideoId(url: string): string | null {
  for (const pattern of VIDEO_ID_PATTERNS) {
    const match = url.match(pattern);
    if (match) return match[1];
  }
  return null;
}

// ─── HTML Scraping Fallback ───────────────────────────────────────────────────

const SCRAPE_HEADERS: Record<string, string> = {
  "User-Agent":
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 " +
    "(KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
  Accept:
    "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
  "Accept-Language": "en-US,en;q=0.5",
  "Accept-Encoding": "gzip, deflate, br",
  DNT: "1",
  "Sec-Fetch-Dest": "document",
  "Sec-Fetch-Mode": "navigate",
  "Sec-Fetch-Site": "none",
  "Cache-Control": "max-age=0",
};

async function extractViaHTML(
  videoId: string,
  originalUrl: string,
  cookies?: string
): Promise<ExtractionResult> {
  const headers = { ...SCRAPE_HEADERS };
  if (cookies) headers["Cookie"] = cookies;

  const response = await fetch(
    `https://www.youtube.com/watch?v=${videoId}`,
    { method: "GET", headers }
  );

  if (!response.ok) {
    throw new ExtractionError(
      `Failed to fetch YouTube page: ${response.status} ${response.statusText}`,
      "youtube"
    );
  }

  const html = await response.text();
  const playerResponse = extractPlayerResponse(html);
  if (!playerResponse) {
    throw new ExtractionError(
      "Could not extract player response from YouTube page. " +
        "The page structure may have changed or the video may be restricted.",
      "youtube"
    );
  }

  const result = parsePlayerResponse(playerResponse, originalUrl);
  // Apply n-parameter transform for full CDN speed
  result.streams = await transformStreamUrls(result.streams);
  return result;
}

/** Extract ytInitialPlayerResponse JSON from page HTML */
function extractPlayerResponse(html: string): any | null {
  // Patterns from most common to least common
  const patterns = [
    /var ytInitialPlayerResponse\s*=\s*(\{.+?\});(?:var |<\/script>)/,
    /ytInitialPlayerResponse\s*=\s*(\{.+?\});/,
    /window\["ytInitialPlayerResponse"\]\s*=\s*(\{.+?\});/,
  ];

  for (const pattern of patterns) {
    const match = html.match(pattern);
    if (match) {
      try {
        return JSON.parse(match[1]);
      } catch {
        // try next pattern
      }
    }
  }

  return null;
}

/** Parse ytInitialPlayerResponse into ExtractionResult */
function parsePlayerResponse(
  playerResponse: any,
  originalUrl: string
): ExtractionResult {
  const videoDetails = playerResponse.videoDetails;
  if (!videoDetails) {
    throw new ExtractionError(
      "No video details found in player response",
      "youtube"
    );
  }

  const streamingData = playerResponse.streamingData;
  const streams: Stream[] = [];

  for (const fmt of [
    ...(streamingData?.formats ?? []),
    ...(streamingData?.adaptiveFormats ?? []),
  ]) {
    const stream = parseHTMLFormat(fmt);
    if (stream) streams.push(stream);
  }

  if (streams.length === 0) {
    throw new ExtractionError(
      "No stream formats found. The video may be age-restricted or require authentication.",
      "youtube"
    );
  }

  streams.sort((a, b) => (b.height || 0) - (a.height || 0));

  return {
    streams,
    title: videoDetails.title || "Unknown Title",
    thumbnail: videoDetails.thumbnail?.thumbnails?.pop()?.url,
    duration: videoDetails.lengthSeconds
      ? parseInt(videoDetails.lengthSeconds, 10)
      : undefined,
    platform: "youtube",
  };
}

/** Parse a format from HTML scraping (may have signatureCipher) */
function parseHTMLFormat(format: any): Stream | null {
  // Prefer plain URL; signatureCipher will produce a URL without signature (may fail)
  let url: string | null = null;

  if (format.url && typeof format.url === "string") {
    url = format.url;
  } else {
    const cipher = format.signatureCipher || format.cipher;
    if (cipher) {
      const urlMatch = (cipher as string).match(/(?:^|&)url=([^&]+)/);
      if (urlMatch) url = decodeURIComponent(urlMatch[1]);
    }
  }

  if (!url) return null;

  const mimeType = format.mimeType || "";
  const mimeMatch = mimeType.match(/^(video|audio)\/(\w+);\s*codecs="([^"]+)"/);
  const mediaType = mimeMatch?.[1] ?? "video";
  const ext = mimeMatch?.[2] ?? "mp4";
  const codecs = mimeMatch?.[3] ?? "";
  const height = format.height || 0;

  let quality: string;
  if (mediaType === "audio") {
    quality = "Audio";
  } else if (height > 0) {
    quality = `${height}p`;
    if ((format.fps || 0) > 30) quality += `${format.fps}`;
  } else {
    quality = format.qualityLabel || format.quality || "unknown";
  }

  return {
    url,
    quality,
    format: ext,
    mime: mimeType,
    bitrate: format.bitrate || 0,
    codec: codecs,
    width: format.width || 0,
    height,
  };
}

export default { extract };
