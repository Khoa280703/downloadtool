// UserScript entry point for Tampermonkey / Violentmonkey / Greasemonkey
// Reuses shared modules from Phase 1 (inject-button, quality-picker, stream-utils)
// Adds GM_xmlhttpRequest (bypasses CORS) + SPA navigation observer

import { injectButton, resetButton } from './shared/inject-button.js';
import { showQualityPicker } from './shared/quality-picker.js';
import {
  filterDownloadableFormats,
  buildMuxedPairs,
  buildMuxedUrl,
  type StreamFormat,
} from './shared/stream-utils.js';

// GM_xmlhttpRequest is declared via @grant in the UserScript header (managed by vite-plugin-monkey)
declare function GM_xmlhttpRequest(details: {
  method: string;
  url: string;
  headers?: Record<string, string>;
  data?: string;
  onload: (response: { status: number; responseText: string }) => void;
  onerror: (error: unknown) => void;
}): void;

const API_BASE = 'https://yourdomain.com';

/** Promisified GM_xmlhttpRequest — bypasses CORS restrictions in userscript context */
function gmFetch(url: string, options: { method: string; body?: string }): Promise<{ ok: boolean; json: () => Promise<unknown> }> {
  return new Promise((resolve, reject) => {
    GM_xmlhttpRequest({
      method: options.method,
      url,
      headers: { 'Content-Type': 'application/json' },
      data: options.body,
      onload: (res) => {
        resolve({
          ok: res.status >= 200 && res.status < 300,
          json: () => Promise.resolve(JSON.parse(res.responseText)),
        });
      },
      onerror: reject,
    });
  });
}

async function handleDownload(): Promise<void> {
  const videoUrl = location.href;

  try {
    const res = await gmFetch(`${API_BASE}/api/extract`, {
      method: 'POST',
      body: JSON.stringify({ url: videoUrl }),
    });

    if (!res.ok) {
      const err = await res.json().catch(() => ({ error: 'Unknown error' })) as { error?: string };
      alert(`[DownloadTool] Extraction failed: ${err.error}`);
      resetButton();
      return;
    }

    const data = await res.json() as { metadata?: { title?: string; formats?: StreamFormat[] } };
    const title = data.metadata?.title ?? 'video';
    const allFormats: StreamFormat[] = data.metadata?.formats ?? [];

    const downloadable = filterDownloadableFormats(allFormats);
    const pairs = buildMuxedPairs(downloadable);

    if (pairs.length === 0) {
      alert('[DownloadTool] No downloadable formats found.');
      resetButton();
      return;
    }

    showQualityPicker(
      pairs,
      (pair) => { window.location.href = buildMuxedUrl(API_BASE, pair, title); },
      resetButton,
    );
  } catch (e) {
    console.error('[DownloadTool]', e);
    alert(`[DownloadTool] Error: ${e instanceof Error ? e.message : String(e)}`);
    resetButton();
  }
}

/** Observe YouTube SPA navigation via title change; debounced re-injection */
function observeNavigation(): void {
  let lastTitle = document.title;
  let debounce: ReturnType<typeof setTimeout>;

  new MutationObserver(() => {
    if (document.title !== lastTitle) {
      lastTitle = document.title;
      clearTimeout(debounce);
      debounce = setTimeout(() => {
        if (location.href.includes('youtube.com/watch')) {
          injectButton(handleDownload);
        }
      }, 500);
    }
  }).observe(document.querySelector('title') ?? document.head, {
    subtree: true,
    characterData: true,
    childList: true,
  });
}

// Initial inject
if (location.href.includes('youtube.com/watch')) {
  injectButton(handleDownload);
}

// Keep button alive through SPA navigation
observeNavigation();
