/**
 * YouTube playlist extractor via InnerTube browse API.
 *
 * Scope: playlist URLs with ?list=... only (channel/@handle deferred).
 */

import { ExtractionError } from "./types.ts";

export interface BatchVideoInfo {
  videoId: string;
  title: string;
  thumbnail?: string;
  index: number;
}

const BROWSE_ENDPOINT =
  "https://www.youtube.com/youtubei/v1/browse?prettyPrint=false";
const MAX_PAGES = 50;
const WEB_CONTEXT = {
  client: {
    clientName: "WEB",
    clientVersion: "2.20240101.00.00",
    hl: "en",
    gl: "US",
  },
};
const WEB_HEADERS: Record<string, string> = {
  "Content-Type": "application/json",
  "User-Agent":
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 " +
    "(KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
  Accept: "application/json",
};

export async function extractPlaylist(
  url: string,
  cookies?: string
): Promise<BatchVideoInfo[]> {
  const playlistId = parsePlaylistId(url);
  if (!playlistId) {
    throw new ExtractionError(
      "Invalid playlist URL: could not find list= parameter",
      "youtube"
    );
  }

  const results: BatchVideoInfo[] = [];
  const seenIds = new Set<string>();
  let continuation: string | null = null;
  const browseId = `VL${playlistId}`;

  for (let page = 0; page < MAX_PAGES; page++) {
    const data = await fetchBrowsePage(browseId, continuation, cookies);
    const items = extractContinuationItems(data);
    if (items.length === 0) break;

    for (const item of items) {
      const renderer = item?.playlistVideoRenderer;
      const videoId = renderer?.videoId;
      if (!videoId || seenIds.has(videoId)) continue;

      const title = extractText(renderer?.title) || "Untitled video";
      const thumbnail = normalizeThumbnailUrl(
        renderer?.thumbnail?.thumbnails?.[renderer?.thumbnail?.thumbnails?.length - 1]?.url
      );

      seenIds.add(videoId);
      results.push({
        videoId,
        title,
        thumbnail,
        index: results.length + 1,
      });
    }

    continuation = extractContinuationToken(items) || null;
    if (!continuation) break;
  }

  return results;
}

function parsePlaylistId(url: string): string | null {
  try {
    const parsed = new URL(url);
    const list = parsed.searchParams.get("list");
    if (list) return list;
  } catch {
    // Fallback regex below.
  }

  const match = url.match(/[?&]list=([a-zA-Z0-9_-]+)/);
  return match?.[1] ?? null;
}

async function fetchBrowsePage(
  browseId: string,
  continuation: string | null,
  cookies?: string
): Promise<any> {
  const headers = { ...WEB_HEADERS };
  if (cookies) headers.Cookie = cookies;

  const payload: Record<string, unknown> = { context: WEB_CONTEXT };
  if (continuation) payload.continuation = continuation;
  else payload.browseId = browseId;

  const response = await fetch(BROWSE_ENDPOINT, {
    method: "POST",
    headers,
    body: JSON.stringify(payload),
  });

  if (!response.ok) {
    throw new ExtractionError(
      `InnerTube browse failed: ${response.status} ${response.statusText}`,
      "youtube"
    );
  }

  return response.json();
}

function extractContinuationItems(data: any): any[] {
  const primary =
    data?.contents?.twoColumnBrowseResultsRenderer?.tabs?.[0]?.tabRenderer
      ?.content?.sectionListRenderer?.contents?.[0]?.itemSectionRenderer
      ?.contents?.[0]?.playlistVideoListRenderer?.contents;

  if (Array.isArray(primary) && primary.length > 0) {
    return primary;
  }

  const fromActions = data?.onResponseReceivedActions;
  if (Array.isArray(fromActions)) {
    for (const action of fromActions) {
      const items = action?.appendContinuationItemsAction?.continuationItems;
      if (Array.isArray(items) && items.length > 0) {
        return items;
      }
    }
  }

  const fromEndpoints = data?.onResponseReceivedEndpoints;
  if (Array.isArray(fromEndpoints)) {
    for (const endpoint of fromEndpoints) {
      const items = endpoint?.appendContinuationItemsAction?.continuationItems;
      if (Array.isArray(items) && items.length > 0) {
        return items;
      }
    }
  }

  return [];
}

function extractContinuationToken(items: any[]): string | null {
  for (const item of items) {
    const token =
      item?.continuationItemRenderer?.continuationEndpoint?.continuationCommand
        ?.token ||
      item?.continuationItemRenderer?.continuationEndpoint
        ?.playlistPanelContinuation?.continuation ||
      item?.continuationItemRenderer?.continuationEndpoint?.commandMetadata
        ?.webCommandMetadata?.url?.match(/[?&]ctoken=([^&]+)/)?.[1];

    if (typeof token === "string" && token.length > 0) return token;
  }
  return null;
}

function extractText(textNode: any): string | null {
  if (typeof textNode?.simpleText === "string" && textNode.simpleText.length > 0) {
    return textNode.simpleText;
  }
  if (Array.isArray(textNode?.runs) && textNode.runs.length > 0) {
    const text = textNode.runs
      .map((run: any) => (typeof run?.text === "string" ? run.text : ""))
      .join("")
      .trim();
    if (text) return text;
  }
  return null;
}

function normalizeThumbnailUrl(url?: string): string | undefined {
  if (!url) return undefined;
  if (url.startsWith("//")) return `https:${url}`;
  return url;
}

