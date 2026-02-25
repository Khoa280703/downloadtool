// Bookmarklet entry point
// Bundled as IIFE → dist/bm.js, served at GET /bm.js
// Loader: javascript:(function(){var s=document.createElement('script');s.src='https://YOUR_DOMAIN/bm.js?t='+Date.now();document.body.appendChild(s);})()

import { injectButton, resetButton } from './shared/inject-button.js';
import { showQualityPicker } from './shared/quality-picker.js';
import {
  filterDownloadableFormats,
  buildMuxedPairs,
  buildMuxedUrl,
  type StreamFormat,
} from './shared/stream-utils.js';

// Detect API base from script tag src or fall back to same origin
const scriptEl = document.currentScript as HTMLScriptElement | null;
const API_BASE = scriptEl?.src
  ? new URL(scriptEl.src).origin
  : window.location.origin;

// Only run on YouTube watch pages
if (!location.href.includes('youtube.com/watch')) {
  console.info('[DownloadTool] Not a YouTube watch page, skipping.');
} else {
  injectButton(handleDownload);
}

async function handleDownload(): Promise<void> {
  const videoUrl = location.href;

  try {
    const res = await fetch(`${API_BASE}/api/extract`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ url: videoUrl }),
    });

    if (!res.ok) {
      const err = await res.json().catch(() => ({ error: 'Unknown error' }));
      alert(`[DownloadTool] Extraction failed: ${err.error ?? res.statusText}`);
      resetButton();
      return;
    }

    const data = await res.json();
    const title: string = data.metadata?.title ?? 'video';
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
      (pair) => {
        const url = buildMuxedUrl(API_BASE, pair, title);
        window.location.href = url;
      },
      resetButton,
    );
  } catch (e) {
    console.error('[DownloadTool] Error:', e);
    alert(`[DownloadTool] Error: ${e instanceof Error ? e.message : String(e)}`);
    resetButton();
  }
}
