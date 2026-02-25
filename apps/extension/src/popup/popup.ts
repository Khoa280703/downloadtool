// Extension popup — auto-detects YouTube tab, shows quality picker, triggers download

import {
  filterDownloadableFormats,
  buildMuxedPairs,
  buildMuxedUrl,
  type StreamFormat,
  type MuxedPair,
} from '../shared/stream-utils.js';

declare const __API_BASE__: string;
const API_BASE = typeof __API_BASE__ !== 'undefined' ? __API_BASE__ : 'https://your-domain.com';

const app = document.getElementById('app')!;

function render(html: string): void {
  app.innerHTML = html;
}

function renderHeader(): string {
  return `<div class="header">▶ YouTube Downloader</div>`;
}

async function init(): Promise<void> {
  render(`${renderHeader()}<p class="status">Detecting YouTube tab...</p>`);

  const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
  const tabUrl = tab?.url ?? '';

  if (!tab?.id || !(tabUrl.includes('youtube.com/watch') || tabUrl.includes('youtu.be/'))) {
    render(`${renderHeader()}<p class="status">Open a YouTube video to download.</p>`);
    return;
  }

  render(`${renderHeader()}<p class="status">Extracting streams...</p>`);

  chrome.runtime.sendMessage(
    { type: 'EXTRACT', url: tabUrl },
    (response: { error?: string; formats?: StreamFormat[]; title?: string } | null) => {
      if (chrome.runtime.lastError || !response || response.error) {
        render(`${renderHeader()}<p class="error">${response?.error ?? 'Extraction failed'}</p>`);
        return;
      }

      const downloadable = filterDownloadableFormats(response.formats ?? []);
      const pairs = buildMuxedPairs(downloadable);
      const title = response.title ?? 'video';

      if (pairs.length === 0) {
        render(`${renderHeader()}<p class="status">No downloadable streams found.</p>`);
        return;
      }

      renderStreamList(pairs, title);
    },
  );
}

function renderStreamList(pairs: MuxedPair[], title: string): void {
  const items = pairs
    .map(
      (pair) => `
      <li>
        <button data-video="${encodeURIComponent(pair.videoUrl)}"
                data-audio="${encodeURIComponent(pair.audioUrl)}"
                data-label="${encodeURIComponent(pair.label)}"
                data-vcodec="${pair.videoCodec ?? ''}"
                data-acodec="${pair.audioCodec ?? ''}">
          <span>${pair.label}</span>
          <span class="badge">${(pair.videoCodec ?? 'mp4').toUpperCase()}</span>
        </button>
      </li>`,
    )
    .join('');

  render(`${renderHeader()}<ul class="stream-list">${items}</ul>`);

  // Attach click handlers
  app.querySelectorAll<HTMLButtonElement>('.stream-list button').forEach((btn) => {
    btn.addEventListener('click', () => {
      const pair: MuxedPair = {
        label: decodeURIComponent(btn.dataset.label ?? ''),
        videoUrl: decodeURIComponent(btn.dataset.video ?? ''),
        audioUrl: decodeURIComponent(btn.dataset.audio ?? ''),
        videoCodec: btn.dataset.vcodec || undefined,
        audioCodec: btn.dataset.acodec || undefined,
      };
      const url = buildMuxedUrl(API_BASE, pair, title);
      const filename = `${title} [${pair.label}].mp4`;
      chrome.runtime.sendMessage({ type: 'DOWNLOAD', url, filename });
      window.close();
    });
  });
}

init();
