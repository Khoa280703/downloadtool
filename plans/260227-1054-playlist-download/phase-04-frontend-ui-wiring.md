---
title: "Phase 4 - Frontend UI Wiring"
status: pending
---

# Phase 4: Frontend UI Components & Store

## Overview
Tạo/cập nhật Svelte components và store để kết nối SSE → IndexedDB → download worker.

## Files
- **Create:** `frontend/src/components/BatchActiveState.svelte`
- **Create/Update:** `frontend/src/components/BatchInput.svelte`
- **Update:** `frontend/src/stores/batch.ts`
- **Update:** `frontend/src/lib/types.ts` (BatchMessage)

## BatchActiveState.svelte

Real-time pool status khi đang download.

```svelte
<!-- Props -->
received: number   // items processed
total: number      // total items
onCancel: () => void

<!-- Shows -->
- Progress bar (received/total)
- Active/pending/ready counts (getStatus())
- Cancel button
```

## BatchInput.svelte

Main orchestrator component.

**Phases:** `idle | fetching | ready | downloading`

**Key flows:**
1. `handleFetchPlaylist()` → SSE via `subscribeBatch()` → add entries to IDB + store
2. `handleStartDownload()` → load from IDB → `enqueueDownload()` each entry
3. `handleCancel()` → `cancelAll()`, phase = 'ready'
4. `handleClearQueue()` → `resetWorkerState()` + `clearAllEntries()` + `resetBatch()`

**onMount:**
- Detect FSAA support (`showDirectoryPicker`)
- `restorePendingQueue()` — resume pending downloads từ IDB sau reload
- `setInterval(400ms)` — poll progress, detect completion
- Listen to `playlist-fetch-request` custom event (từ URL input)
- **Cleanup:** `resetBatch()` on unmount (close SSE EventSource — tránh zombie connections)

**SSE message handling:**
- `link` → `addBatchItem()` + `upsertEntry()` + `setBatchProgress()`
- `progress` → `setBatchProgress()`
- `done` → phase = 'ready', `completeBatch()`
- `error` → show error, phase = idle/ready

## batch.ts Store

```ts
export const batchQueue = writable<BatchItem[]>([])
export const batchProgress = writable<BatchProgress>({ received: 0, total: 0 })
export const batchActive = writable(false)

export function addBatchItem(item: BatchItem): void
export function updateBatchItemByVideoId(videoId, status, error?): void
export function setBatchQueue(items: BatchItem[]): void
export function setBatchProgress(received, total): void
export function startBatch(): void
export function completeBatch(): void  // closes EventSource
export function resetBatch(): void     // closes EventSource, resets all state
export function setEventSource(es: EventSource): void
```

## types.ts — BatchMessage
```ts
export interface BatchMessage {
  type: 'link' | 'done' | 'progress' | 'error';
  videoId?: string;
  url?: string;
  title?: string;
  thumbnail?: string;
  index?: number;
  current?: number;
  total?: number;
  message?: string;
}
```

## Key Design
- `resetBatch()` gọi trong cleanup của `onMount` → đóng SSE EventSource khi navigate away
- `stateVersion` counter — tránh race conditions khi fetch mới trong khi fetch cũ chưa xong
- `downloadRunId` counter — detect stale download loops
- `persistChain` — serialized IDB writes (không race)
- Quality selector chỉ hiện khi phase = 'ready'
- FSAA folder picker chỉ hiện khi browser hỗ trợ `showDirectoryPicker`

## Connection Pool Awareness
- 1 SSE + 1 download + 1 extract = 3 connections (để lại 3 slots cho page loads)
- MAX_CONCURRENT=1, READY_QUEUE_MAX=1 là intentional dev config
