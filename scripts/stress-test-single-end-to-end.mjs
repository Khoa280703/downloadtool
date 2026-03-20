import fs from 'node:fs/promises';

const API = process.env.API_BASE || 'http://127.0.0.1:3068';
const URL_FILE = process.env.URL_FILE || '4k_youtube_videos.md';
const TOTAL = Number(process.env.TOTAL || 32);
const MAX_POLL_MS = Number(process.env.MAX_POLL_MS || 30 * 60 * 1000);
const POLL_MS = Number(process.env.POLL_MS || 500);
const EXTRACT_RETRY_ATTEMPTS = Number(process.env.EXTRACT_RETRY_ATTEMPTS || 3);
const EXTRACT_RETRY_BASE_MS = Number(process.env.EXTRACT_RETRY_BASE_MS || 900);
const EXTRACT_RETRY_MAX_MS = Number(process.env.EXTRACT_RETRY_MAX_MS || 2500);
const DOWNLOAD_RETRY_ATTEMPTS = Number(process.env.DOWNLOAD_RETRY_ATTEMPTS || 2);
const SESSION_ID = process.env.SESSION_ID || `loadtest-single-${Date.now()}`;
const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

function jitterBackoff(attempt) {
  const base = Math.min(EXTRACT_RETRY_BASE_MS * (2 ** (attempt - 1)), EXTRACT_RETRY_MAX_MS);
  return Math.min(EXTRACT_RETRY_MAX_MS, base + Math.floor(Math.random() * Math.min(400, base * 0.3)));
}

function resolutionScore(label = '') {
  const lower = String(label).toLowerCase();
  if (lower.includes('8k')) return 4320;
  if (lower.includes('4k')) return 2160;
  if (lower.includes('2k')) return 1440;
  const match = lower.match(/(\d{3,4})p/);
  return match ? Number.parseInt(match[1], 10) : 0;
}
function isSingleCombined360pFallback(formats) {
  return formats.length === 1
    && !formats[0].is_audio_only
    && formats[0].has_audio
    && String(formats[0].ext).toLowerCase() === 'mp4'
    && resolutionScore(formats[0].quality) === 360;
}

function selectBestStreams(formats) {
  const audio = [...formats]
    .filter((item) => item.is_audio_only)
    .sort((a, b) => {
      if (String(a.ext).toLowerCase() !== String(b.ext).toLowerCase()) {
        return String(a.ext).toLowerCase() === 'mp4' ? -1 : 1;
      }
      return (b.bitrate || 0) - (a.bitrate || 0);
    })[0] || null;

  const video = [...formats]
    .filter((item) => !item.is_audio_only)
    .filter((item) => !(!item.has_audio && String(item.ext).toLowerCase() === 'webm'))
    .sort((a, b) => {
      const resolutionDiff = resolutionScore(b.quality) - resolutionScore(a.quality);
      if (resolutionDiff !== 0) return resolutionDiff;
      if (!!a.has_audio !== !!b.has_audio) return a.has_audio ? -1 : 1;
      if (String(a.ext).toLowerCase() !== String(b.ext).toLowerCase()) {
        return String(a.ext).toLowerCase() === 'mp4' ? -1 : 1;
      }
      return (b.bitrate || 0) - (a.bitrate || 0);
    })[0] || null;
  return { video, audio };
}
function buildStreamUrl(streamUrl, title, ext, options = {}) {
  const params = new URLSearchParams({ url: streamUrl, title, format: ext || 'mp4' });
  if (options.sourceUrl) params.set('source_url', options.sourceUrl);
  if (options.formatId) params.set('format_id', options.formatId);
  if (options.patchInitMetadata) params.set('patch_init_metadata', 'true');
  return `${API}/api/stream?${params.toString()}`;
}

function sessionIdForIndex(index) {
  return `${SESSION_ID}-item-${index + 1}`;
}

async function postJson(path, payload, headers = {}) {
  const response = await fetch(`${API}${path}`, {
    method: 'POST',
    headers: { 'content-type': 'application/json', ...headers },
    body: JSON.stringify(payload),
  });
  const text = await response.text();
  const data = text ? JSON.parse(text) : null;
  if (!response.ok) throw new Error(`HTTP ${response.status} ${path} :: ${text}`);
  return data;
}
async function fetchExtractWithRetry(url) {
  let lastResult = null;
  for (let attempt = 1; attempt <= EXTRACT_RETRY_ATTEMPTS; attempt += 1) {
    const bypassCache = attempt > 1;
    const raw = await postJson('/api/extract', { url, bypass_cache: bypassCache });
    const metadata = raw?.metadata;
    const formats = metadata?.formats || [];
    const degraded = isSingleCombined360pFallback(formats);
    lastResult = { raw, attempt, degraded, bypassCache };
    if (!degraded || attempt >= EXTRACT_RETRY_ATTEMPTS) return lastResult;
    await sleep(jitterBackoff(attempt));
  }
  return lastResult;
}
async function waitForReady(jobId, sessionId) {
  const startedAt = Date.now();
  let pollCount = 0;
  while (Date.now() - startedAt < MAX_POLL_MS) {
    pollCount += 1;
    const response = await fetch(`${API}/api/jobs/${jobId}`, {
      headers: { 'x-download-session-id': sessionId },
    });
    const text = await response.text();
    const data = text ? JSON.parse(text) : {};
    if (!response.ok) throw new Error(`Status poll failed ${response.status} for ${jobId}: ${text}`);
    if (data.status === 'ready') return { elapsedMs: Date.now() - startedAt, pollCount };
    if (data.status === 'failed' || data.status === 'expired') {
      throw new Error(`Job ${jobId} ${data.status}: ${data.error || 'unknown error'}`);
    }
    await sleep(POLL_MS);
  }
  throw new Error(`Job ${jobId} timed out after ${MAX_POLL_MS}ms`);
}
async function fetchAndDrain(url, headers = {}) {
  const startedAt = Date.now();
  const response = await fetch(url, { headers, redirect: 'follow' });
  if (!response.ok) throw new Error(`Download failed ${response.status} ${url}`);
  const reader = response.body?.getReader();
  if (!reader) throw new Error(`No response body for ${url}`);
  let bytes = 0;
  while (true) {
    const { done, value } = await reader.read();
    if (done) break;
    bytes += value.byteLength;
  }
  return { bytes, elapsedMs: Date.now() - startedAt, finalUrl: response.url };
}

async function fetchAndDrainWithRetry(url, headers = {}) {
  let lastError = null;
  for (let attempt = 1; attempt <= DOWNLOAD_RETRY_ATTEMPTS; attempt += 1) {
    try {
      return await fetchAndDrain(url, headers);
    } catch (error) {
      lastError = error;
      if (attempt >= DOWNLOAD_RETRY_ATTEMPTS) throw error;
      await sleep(Math.min(1500 * attempt, 4000));
    }
  }
  throw lastError;
}

async function fetchJobFileTicket(jobId, sessionId) {
  const response = await fetch(`${API}/api/jobs/${jobId}/file-ticket`, {
    headers: { 'x-download-session-id': sessionId },
  });
  const text = await response.text();
  const data = text ? JSON.parse(text) : {};
  if (!response.ok) {
    throw new Error(`File ticket failed ${response.status} for ${jobId}: ${text}`);
  }
  if (!data.download_url) {
    throw new Error(`Missing download_url for ${jobId}`);
  }
  return data.download_url;
}

function buildCommonResult(index, url, title, extractMs, extract, video, audio) {
  return {
    index: index + 1,
    url,
    title,
    extractMs,
    extractAttempts: extract.attempt,
    degradedFallbackPersisted: extract.degraded,
    selectedVideo: { formatId: video.format_id, quality: video.quality, ext: video.ext, hasAudio: video.has_audio },
    selectedAudio: audio ? { formatId: audio.format_id, quality: audio.quality, ext: audio.ext } : null,
  };
}

async function runOne(index, url) {
  const startedAt = Date.now();
  const itemSessionId = sessionIdForIndex(index);
  const extractStarted = Date.now();
  const extract = await fetchExtractWithRetry(url);
  const extractMs = Date.now() - extractStarted;
  const metadata = extract.raw?.metadata;
  const formats = metadata?.formats || [];
  const { video, audio } = selectBestStreams(formats);
  if (!video) throw new Error('No compatible video stream found');
  const title = metadata?.title || `video-${index + 1}`;
  const common = buildCommonResult(index, url, title, extractMs, extract, video, audio);
  if (!video.has_audio && audio) {
    const created = await postJson('/api/jobs', {
      video_url: video.url,
      audio_url: audio.url,
      source_url: metadata?.original_url || url,
      video_format_id: video.format_id,
      audio_format_id: audio.format_id,
      title,
    }, { 'x-download-session-id': itemSessionId });
    const jobId = created.job_id;
    const ready = await waitForReady(jobId, itemSessionId);
    const downloadUrl = await fetchJobFileTicket(jobId, itemSessionId);
    const download = await fetchAndDrainWithRetry(downloadUrl);
    await fetch(`${API}/api/jobs/${jobId}/release`, {
      method: 'POST',
      headers: { 'x-download-session-id': itemSessionId },
    }).catch(() => {});
    return { ...common, mode: 'mux', jobId, readyMs: ready.elapsedMs, readyPolls: ready.pollCount, downloadMs: download.elapsedMs, bytes: download.bytes, totalElapsedMs: Date.now() - startedAt };
  }
  const streamUrl = buildStreamUrl(video.url, title, video.ext || 'mp4', {
    sourceUrl: metadata?.original_url || url,
    formatId: video.format_id,
    patchInitMetadata: !video.has_audio && String(video.ext || 'mp4').toLowerCase() === 'mp4',
  });
  const download = await fetchAndDrainWithRetry(streamUrl);
  return { ...common, mode: 'direct-combined', jobId: null, readyMs: 0, readyPolls: 0, downloadMs: download.elapsedMs, bytes: download.bytes, totalElapsedMs: Date.now() - startedAt };
}
const markdown = await fs.readFile(URL_FILE, 'utf8');
const sourceUrls = [...markdown.matchAll(/https:\/\/www\.youtube\.com\/watch\?v=[^\s|)]+/g)].map((match) => match[0]);
if (sourceUrls.length === 0) throw new Error(`No YouTube URLs found in ${URL_FILE}`);
const urls = Array.from({ length: TOTAL }, (_, index) => sourceUrls[index % sourceUrls.length]);

const startedAt = Date.now();
const results = await Promise.allSettled(urls.map((url, index) => runOne(index, url)));
const payload = {
  sessionId: SESSION_ID,
  startedAt,
  endedAt: Date.now(),
  elapsedMs: Date.now() - startedAt,
  success: results.filter((item) => item.status === 'fulfilled').length,
  failed: results.filter((item) => item.status === 'rejected').length,
  results: results.map((item, index) => item.status === 'fulfilled'
    ? { status: 'fulfilled', value: item.value }
    : { status: 'rejected', url: urls[index], reason: String(item.reason?.stack || item.reason) }),
};

console.log(JSON.stringify(payload, null, 2));
if (payload.failed > 0) process.exitCode = 1;
