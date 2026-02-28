---
title: "Phase 3 - Frontend IndexedDB + FSAA + Concurrency Pool"
status: pending
---

# Phase 3: Frontend Library Files

## Overview
Tạo 3 lib files cho queue persistence, file saving, và stream selection.

## Files
- **Create:** `frontend/src/lib/playlist-queue-db.ts` (~166 lines)
- **Create:** `frontend/src/lib/playlist-download-file-saver.ts` (~115 lines)
- **Create:** `frontend/src/lib/playlist-download-stream-selection.ts` (~146 lines)
- **Create:** `frontend/src/lib/playlist-download-worker.ts` (~236 lines)

## playlist-queue-db.ts

IndexedDB persistence với memory fallback (Safari private mode).

```ts
export interface QueueEntry {
  videoId: string;  // required, not optional
  title: string;
  thumbnail?: string;
  status: 'pending' | 'downloading' | 'completed' | 'error';
  error?: string;
  addedAt: number;
}

export function openDb(): Promise<IDBDatabase>
export function upsertEntry(entry: QueueEntry): Promise<void>
export function getEntry(videoId: string): Promise<QueueEntry | null>
export function getAllEntries(): Promise<QueueEntry[]>
export function getPendingEntries(): Promise<QueueEntry[]>  // normalizes 'downloading' → 'pending'
export function clearAllEntries(): Promise<void>
```

## playlist-download-file-saver.ts

FSAA (File System Access API) với anchor fallback.

```ts
export function hasSelectedSaveDirectory(): boolean
export async function pickSaveDirectory(): Promise<boolean>
export function isAbortError(error: unknown): boolean
export interface SaveDownloadOptions {
  requireFsaa?: boolean;
  allowAnchorFallback?: boolean;
}
export async function saveDownload(url, filename, signal, options): Promise<void>
```

**Fix OOM:** khi `response.body` null → close writable, fall through to anchor (không dùng `response.blob()`).

## playlist-download-stream-selection.ts

Reuse existing stream selection logic.

```ts
export const PLAYLIST_QUALITY_OPTIONS = [
  { value: 'best', label: 'Best available' },
  { value: '2160', label: '2160p (4K)' },
  { value: '1440', label: '1440p (2K)' },
  { value: '1080', label: '1080p (Full HD)' },
  { value: '720', label: '720p (HD)' },
  { value: '480', label: '480p' },
  { value: '360', label: '360p' }
] as const;
export type PlaylistQuality = ...
export function toWatchUrl(videoId: string): string
export function safeFilename(title: string, extension?: string): string
export function pickBestStreams(streams, quality, options): { video: Stream; audio: Stream | null }
```

## playlist-download-worker.ts

Concurrency pool với prefetch buffer.

```ts
const MAX_CONCURRENT = 1;
const READY_QUEUE_MAX = 1; // keep low in dev (Vite proxy HTTP/1.1 pool limit)

export function setPreferredQuality(quality: PlaylistQuality): void
export function setStrictFsaaMode(enabled: boolean): void
export function resetWorkerState(): void
export async function enqueueDownload(entry: QueueEntry): Promise<void>
export function cancelAll(): void
export function getStatus(): { active, pending, ready, max }
export { hasSelectedSaveDirectory, pickSaveDirectory }
```

**beforeunload:** abort all active fetch connections on page unload.

## Key Design
- **`pickBestStreams` — "Target Ceiling" fallback:** `quality` là ceiling, không phải exact match.
  1. Filter: chỉ giữ MP4 streams (H.264/AV1), bỏ WebM/VP9 (không mux client-side)
  2. Sort by height descending
  3. Tìm stream có `height >= target` → lấy thấp nhất thỏa mãn (exact or nearest above)
  4. Fallback: nếu không có → lấy `height` cao nhất còn lại (best available)
  5. `quality = 'best'` → bỏ qua filter ceiling, lấy cao nhất
  - **Không bao giờ throw** khi không tìm được exact resolution — luôn trả về best available
- `getPendingEntries()` normalizes `downloading` → `pending` (resume safety sau reload)
- `videoId: string` required (không optional)
- MAX_CONCURRENT=1, READY_QUEUE_MAX=1 tránh exhaust Vite proxy connection pool (6 limit)
- `AbortController` per active download, stored in Map for cancel support
- `workerEpoch` counter — stale async callbacks bị ignore sau reset
- **Jitter:** trước mỗi `prepareReadyEntry` (extract), sleep `2000 + Math.random() * 3000` ms (2-5s) để tránh rate-limit InnerTube
- **Circuit Breaker:** trong `fillReadyQueue`, nếu extract throw HTTP 429/403 → set `circuitOpenUntil = Date.now() + 300_000` (5 phút) → push videoId lại `pendingIds` → return; worker tự resume sau khi hết cooldown
- `getStatus()` expose thêm `circuitOpen: boolean` và `cooldownMs: number` để UI hiển thị đếm ngược
