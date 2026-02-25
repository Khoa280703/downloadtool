// Background service worker
// Handles API calls (bypasses CORS) and triggers chrome.downloads

// Injected at build time via vite define
declare const __API_BASE__: string;

const API_BASE = typeof __API_BASE__ !== 'undefined' ? __API_BASE__ : 'https://your-domain.com';

chrome.runtime.onMessage.addListener((msg, _sender, sendResponse) => {
  if (msg.type === 'EXTRACT') {
    handleExtract(msg.url).then(sendResponse).catch((e: Error) => sendResponse({ error: e.message }));
    return true; // Keep channel open for async response
  }

  if (msg.type === 'DOWNLOAD') {
    handleDownload(msg.url, msg.filename);
    sendResponse({ ok: true });
  }
});

async function handleExtract(videoUrl: string): Promise<{ formats?: unknown[]; title?: string; error?: string }> {
  const res = await fetch(`${API_BASE}/api/extract`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ url: videoUrl }),
  });

  if (!res.ok) {
    const err = await res.json().catch(() => ({ error: res.statusText }));
    return { error: err.error ?? `HTTP ${res.status}` };
  }

  const data = await res.json();
  return {
    formats: data.metadata?.formats ?? [],
    title: data.metadata?.title ?? 'video',
  };
}

function handleDownload(url: string, filename: string): void {
  chrome.downloads.download({
    url,
    filename: filename.endsWith('.mp4') ? filename : `${filename}.mp4`,
    saveAs: false,
  });
}
