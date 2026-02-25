/**
 * YouTube InnerTube API client
 *
 * Bypasses HTML scraping + signatureCipher by impersonating internal Apple/Google clients.
 * IOS client returns plain stream URLs (no decryption needed) for all resolutions up to 4K.
 */

import { Stream, ExtractionResult, ExtractionError } from "./types.ts";
import { transformStreamUrls } from "./youtube-n-transform.ts";

const INNERTUBE_ENDPOINT =
  "https://www.youtube.com/youtubei/v1/player?prettyPrint=false";

// Client definitions
// IOS: returns plain URLs for 144p–4K (adaptive, video-only + audio tracks) — PRIMARY
// ANDROID: fallback, may need API key on some regions
const CLIENTS = {
  IOS: {
    context: {
      client: {
        clientName: "IOS",
        clientVersion: "21.02.3",
        deviceModel: "iPhone16,2",
        osName: "iPhone",
        osVersion: "18.3.2.22D82",
        hl: "en",
        gl: "US",
        utcOffsetMinutes: 0,
      },
    },
    headers: {
      "Content-Type": "application/json",
      "User-Agent":
        "com.google.ios.youtube/21.02.3 (iPhone16,2; U; CPU iOS 18_3_2 like Mac OS X)",
      "X-YouTube-Client-Name": "5",
      "X-YouTube-Client-Version": "21.02.3",
      Accept: "application/json",
    } as Record<string, string>,
  },
  ANDROID: {
    context: {
      client: {
        clientName: "ANDROID",
        clientVersion: "21.06.254",
        androidSdkVersion: 34,
        osName: "Android",
        osVersion: "14",
        hl: "en",
        gl: "US",
        utcOffsetMinutes: 0,
      },
    },
    headers: {
      "Content-Type": "application/json",
      "User-Agent":
        "com.google.android.youtube/21.06.254 (Linux; U; Android 14) gzip",
      "X-YouTube-Client-Name": "3",
      "X-YouTube-Client-Version": "21.06.254",
      Accept: "application/json",
    } as Record<string, string>,
  },
} as const;

type ClientName = keyof typeof CLIENTS;

/** POST to InnerTube player endpoint with given client */
async function callInnerTube(
  videoId: string,
  clientName: ClientName,
  cookies?: string
): Promise<any> {
  const { context, headers } = CLIENTS[clientName];

  const allHeaders = { ...headers };
  if (cookies) allHeaders["Cookie"] = cookies;

  const payload = {
    context,    // must wrap as { context: { client: ... } }, not spread
    videoId,
    playbackContext: {
      contentPlaybackContext: {
        // signatureTimestamp from player JS; static value works for most videos
        signatureTimestamp: 19950,
      },
    },
  };

  const response = await fetch(INNERTUBE_ENDPOINT, {
    method: "POST",
    headers: allHeaders,
    body: JSON.stringify(payload),
  });

  if (!response.ok) {
    throw new Error(
      `InnerTube [${clientName}] returned HTTP ${response.status}`
    );
  }

  return response.json();
}

/** Extract URL from format — handles plain url and signatureCipher */
function extractUrl(format: any): string | null {
  // Plain URL: best case, ANDROID client usually provides this
  if (format.url && typeof format.url === "string") {
    return format.url;
  }

  // signatureCipher / cipher: extract the url= portion (signature not applied)
  // Streams will play but may be throttled by YouTube's n-parameter
  const cipherStr = format.signatureCipher || format.cipher;
  if (cipherStr && typeof cipherStr === "string") {
    // signatureCipher is a query string: s=SIG&sp=sig&url=ENCODED_URL
    const urlMatch = cipherStr.match(/(?:^|&)url=([^&]+)/);
    if (urlMatch) {
      return decodeURIComponent(urlMatch[1]);
    }
  }

  return null;
}

/** Map codec string to human-readable label */
function getCodecLabel(codecs: string, mediaType: string): string {
  const c = codecs.toLowerCase();
  if (c.includes("av01")) return "AV1";
  if (c.includes("vp09") || c.includes("vp9")) return "VP9";
  if (c.includes("avc1") || c.includes("h264")) return "H.264";
  if (c.includes("hev1") || c.includes("hvc1")) return "H.265";
  if (mediaType === "audio") {
    if (c.includes("mp4a")) return "AAC";
    if (c.includes("opus")) return "Opus";
  }
  // Fallback: first codec identifier uppercased
  const first = codecs.split(",")[0].trim().split(".")[0];
  return first ? first.toUpperCase() : "Unknown";
}

/** Parse a single format object into a Stream */
function parseFormat(format: any): Stream | null {
  const url = extractUrl(format);
  if (!url) return null;

  const mimeType = format.mimeType || "";
  // e.g. "video/mp4; codecs=\"avc1.64001F, mp4a.40.2\""
  const mimeMatch = mimeType.match(
    /^(video|audio)\/(\w+);\s*codecs="([^"]+)"/
  );

  const mediaType = mimeMatch?.[1] ?? "video";
  const ext = mimeMatch?.[2] ?? "mp4";
  const codecs = mimeMatch?.[3] ?? "";

  const height = format.height || 0;
  const width = format.width || 0;
  const bitrate = format.bitrate || 0;
  const fps = format.fps || 0;
  const isVideoOnly = mediaType === "video" && !codecs.includes("mp4a");
  const isAudioOnly = mediaType === "audio";
  const hasAudio = !isVideoOnly;

  let quality: string;
  if (isAudioOnly) {
    const kbps = format.averageBitrate
      ? Math.round(format.averageBitrate / 1000)
      : 0;
    quality = kbps ? `Audio ${kbps}kbps` : "Audio";
  } else if (height > 0) {
    quality = `${height}p`;
    if (fps > 30) quality += `${fps}`;
    if (isVideoOnly) quality += " (video only)";
  } else {
    quality = format.qualityLabel || format.quality || "unknown";
  }

  const codecLabel = codecs ? getCodecLabel(codecs, mediaType) : undefined;

  return {
    url, quality, format: ext, mime: mimeType,
    bitrate, codec: codecs, codecLabel,
    hasAudio, isAudioOnly,
    width, height,
  };
}

/** Check if InnerTube response has usable streaming data */
function hasStreams(data: any): boolean {
  const sd = data?.streamingData;
  return (sd?.formats?.length ?? 0) > 0 || (sd?.adaptiveFormats?.length ?? 0) > 0;
}

/** Parse all streams from an InnerTube player response, sorted by quality */
function parseStreams(data: any): Stream[] {
  const sd = data.streamingData;
  if (!sd) return [];

  const streams: Stream[] = [];
  for (const fmt of [...(sd.formats ?? []), ...(sd.adaptiveFormats ?? [])]) {
    const s = parseFormat(fmt);
    if (s) streams.push(s);
  }

  return streams.sort((a, b) => (b.height || 0) - (a.height || 0));
}

/**
 * Extract video info via InnerTube API.
 * Tries ANDROID client first (plain URLs), falls back to TV client.
 */
export async function extractViaInnerTube(
  videoId: string,
  originalUrl: string,
  cookies?: string
): Promise<ExtractionResult> {
  let data: any = null;

  // Attempt each client in order — IOS gives plain URLs for all resolutions
  for (const clientName of ["IOS", "ANDROID"] as ClientName[]) {
    try {
      const res = await callInnerTube(videoId, clientName, cookies);
      if (hasStreams(res)) {
        data = res;
        break;
      }
    } catch {
      // try next client
    }
  }

  if (!data || !hasStreams(data)) {
    throw new ExtractionError(
      "InnerTube API returned no streaming data (video may be restricted or unavailable)",
      "youtube"
    );
  }

  const rawStreams = parseStreams(data);
  const vd = data.videoDetails;

  // Apply n-parameter transform for full CDN speed (bypasses YouTube throttle)
  const streams = await transformStreamUrls(rawStreams);

  return {
    streams,
    title: vd?.title ?? "Unknown",
    thumbnail: vd?.thumbnail?.thumbnails?.pop()?.url,
    duration: vd?.lengthSeconds ? parseInt(vd.lengthSeconds, 10) : undefined,
    platform: "youtube",
  };
}
