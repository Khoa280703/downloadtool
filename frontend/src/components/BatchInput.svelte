<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { isValidVideoUrl, subscribeBatch } from '$lib/api';
	import BatchActiveState from '$components/BatchActiveState.svelte';
	import {
		batchQueue,
		batchProgress,
		setEventSource,
		resetBatch,
		addBatchItem,
		setBatchProgress,
		startBatch,
		completeBatch
	} from '$stores/batch';
	import {
		type QueueEntry,
		enqueueDownload,
		pickSaveDirectory,
		resetWorkerState,
		setPreferredDownloadMode,
		setPreferredQuality
	} from '$lib/playlist-download-worker';
	import {
		PLAYLIST_DOWNLOAD_MODE_OPTIONS,
		PLAYLIST_QUALITY_OPTIONS,
		type PlaylistDownloadMode,
		type PlaylistQuality,
		getStoredPlaylistDownloadMode,
		getStoredPlaylistQuality
	} from '$lib/playlist-download-stream-selection';

	type BatchPhase = 'idle' | 'fetching' | 'ready' | 'downloading';

	interface Props {
		onStart?: () => void;
		onComplete?: () => void;
	}

	let { onStart, onComplete }: Props = $props();

	let url = $state('');
	let error = $state('');
	let phase = $state<BatchPhase>('idle');
	let fsaaSupported = $state(false);
	let dirPicked = $state(false);
	let selectedDownloadMode = $state<PlaylistDownloadMode>(getStoredPlaylistDownloadMode());
	let selectedQuality = $state<PlaylistQuality>(getStoredPlaylistQuality());
	let stagedEntries = $state<QueueEntry[]>([]);
	let completionNotified = $state(false);

	const readyCount = $derived.by(() => $batchQueue.length);
	const selectedCount = $derived.by(
		() => $batchQueue.filter((item) => item.selected !== false).length
	);
	const progressPercent = $derived.by(() => {
		if ($batchProgress.total <= 0) return 0;
		return Math.min(100, Math.round(($batchProgress.received / $batchProgress.total) * 100));
	});

	$effect(() => {
		if (phase !== 'downloading' || completionNotified) return;
		if ($batchQueue.length === 0) return;

		const selectedItems = $batchQueue.filter((item) => item.selected !== false);
		const settled = selectedItems.every(
			(item) => item.status === 'completed' || item.status === 'error'
		);
		if (!settled) return;

		completionNotified = true;
		phase = 'ready';
		completeBatch();
		onComplete?.();
	});

	function isPlaylistUrl(input: string): boolean {
		if (!input.includes('youtube.com') && !input.includes('youtu.be')) return false;
		if (/[?&]list=/.test(input)) return true;
		if (/youtube\.com\/playlist/.test(input)) return true;
		return false;
	}

	function validate(input: string): boolean {
		if (!input.trim()) {
			error = 'Please paste a playlist URL.';
			return false;
		}
		if (!isValidVideoUrl(input)) {
			error = 'This does not look like a valid YouTube URL.';
			return false;
		}
		if (!isPlaylistUrl(input)) {
			error = 'Use a playlist URL containing list=...';
			return false;
		}
		error = '';
		return true;
	}

	function pushStagedEntry(entry: QueueEntry): void {
		if (stagedEntries.some((item) => item.videoId === entry.videoId)) return;
		stagedEntries = [...stagedEntries, entry];
	}

	onMount(() => {
		const picker = (window as Window & { showDirectoryPicker?: unknown }).showDirectoryPicker;
		fsaaSupported = typeof picker === 'function';
	});

	async function handleFetchPlaylist(): Promise<void> {
		if (!validate(url)) return;

		completionNotified = false;
		stagedEntries = [];
		resetWorkerState();
		resetBatch();
		startBatch();
		phase = 'fetching';
		onStart?.();

		const es = subscribeBatch(
			url,
			(data) => {
				if (data.type === 'link') {
					if (!data.videoId || !data.title) {
						console.error('Invalid batch link payload:', data);
						return;
					}

					const entry: QueueEntry = {
						videoId: data.videoId,
						title: data.title,
						thumbnail: data.thumbnail
					};

					pushStagedEntry(entry);
					addBatchItem({
						videoId: entry.videoId,
						title: entry.title,
						thumbnail: entry.thumbnail,
						status: 'pending'
					});

					if (data.index != null && data.total != null) {
						setBatchProgress(data.index, data.total);
					}
					return;
				}

				if (data.type === 'progress') {
					if (data.current != null && data.total != null) {
						setBatchProgress(data.current, data.total);
					}
					return;
				}

				if (data.type === 'done') {
					es.close();
					completeBatch();
					phase = stagedEntries.length > 0 ? 'ready' : 'idle';
					if (stagedEntries.length === 0) {
						error = 'No downloadable videos were found in this playlist.';
					}
					return;
				}

				if (data.type === 'error') {
					error = data.message || 'Could not process this playlist.';
					es.close();
					completeBatch();
					phase = stagedEntries.length > 0 ? 'ready' : 'idle';
				}
			},
			() => {
				error = 'SSE connection failed. Please retry.';
				completeBatch();
				phase = stagedEntries.length > 0 ? 'ready' : 'idle';
			}
		);

		setEventSource(es);
	}

	function persistQuality(quality: PlaylistQuality): void {
		if (typeof window === 'undefined') return;
		try {
			window.localStorage.setItem('fetchtube.playlist-quality.v1', quality);
		} catch {
			// Ignore localStorage failures.
		}
	}

	function persistDownloadMode(mode: PlaylistDownloadMode): void {
		if (typeof window === 'undefined') return;
		try {
			window.localStorage.setItem('fetchtube.playlist-download-mode.v1', mode);
		} catch {
			// Ignore localStorage failures.
		}
	}

	function handleStartDownload(): void {
		if (stagedEntries.length === 0) {
			error = 'Playlist is empty. Fetch links first.';
			return;
		}
		const selectedVideoIds = new Set(
			$batchQueue.filter((item) => item.selected !== false).map((item) => item.videoId)
		);
		const selectedEntries = stagedEntries.filter((entry) => selectedVideoIds.has(entry.videoId));
		if (selectedEntries.length === 0) {
			error = 'Please select at least one video.';
			return;
		}

		error = '';
		completionNotified = false;
		resetWorkerState();
		setPreferredDownloadMode(selectedDownloadMode);
		setPreferredQuality(selectedQuality);
		persistDownloadMode(selectedDownloadMode);
		persistQuality(selectedQuality);
		startBatch();
		phase = 'downloading';

		for (const entry of selectedEntries) {
			enqueueDownload(entry);
		}
	}

	async function handlePickDirectory(): Promise<void> {
		dirPicked = await pickSaveDirectory();
		if (!dirPicked) {
			error = 'Folder picker unavailable. Fallback download mode will be used.';
		}
	}

	function handleCancel(): void {
		resetWorkerState();
		resetBatch();
		stagedEntries = [];
		phase = 'idle';
		error = '';
		onComplete?.();
	}

	onDestroy(() => {
		resetWorkerState();
		resetBatch();
	});
</script>

<div class="batch-input">
	<div class="header">
		<h3>Paste Your YouTube Playlist URL</h3>
		<p class="subtitle">Step 1: fetch all videos. Step 2: pick type/quality. Step 3: start download.</p>
	</div>

	{#if phase === 'idle'}
		<form
			onsubmit={(event) => {
				event.preventDefault();
				void handleFetchPlaylist();
			}}
		>
			<input
				type="url"
				placeholder="https://www.youtube.com/playlist?list=..."
				bind:value={url}
				aria-label="Playlist URL"
				class="url-field"
			/>

			{#if error}
				<span class="error-text" role="alert">{error}</span>
			{/if}

			<button type="submit" class="submit-btn" disabled={!url}>Fetch Playlist Videos</button>
		</form>
	{:else if phase === 'fetching'}
		<div class="phase-card">
			<p class="phase-title">Scanning playlist links...</p>
			<p class="phase-meta">{$batchProgress.received} / {$batchProgress.total || '...'} discovered</p>
			<div class="phase-progress-track">
				<div class="phase-progress-fill" style:width={`${progressPercent}%`}></div>
			</div>

			<button type="button" class="ghost-btn" onclick={handleCancel}>Cancel</button>
		</div>
	{:else if phase === 'ready'}
		<div class="phase-card">
			<p class="phase-title">Playlist ready: {readyCount} videos</p>
			<p class="phase-meta">Choose type and quality, then start download.</p>
			<p class="phase-meta">Selected: {selectedCount} / {readyCount}</p>

			<label class="quality-label" for="playlist-download-mode">Download type</label>
			<select
				id="playlist-download-mode"
				class="mode-select"
				bind:value={selectedDownloadMode}
			>
				{#each PLAYLIST_DOWNLOAD_MODE_OPTIONS as option}
					<option value={option.value}>{option.label}</option>
				{/each}
			</select>

			{#if selectedDownloadMode === 'video'}
				<label class="quality-label" for="playlist-quality">Preferred resolution</label>
				<select
					id="playlist-quality"
					class="quality-select"
					bind:value={selectedQuality}
				>
					{#each PLAYLIST_QUALITY_OPTIONS as option}
						<option value={option.value}>{option.label}</option>
					{/each}
				</select>
			{:else}
				<p class="phase-meta">Audio only uses best available audio bitrate.</p>
			{/if}

			<div class="actions-row">
				{#if fsaaSupported}
					<button type="button" class="folder-btn" onclick={() => void handlePickDirectory()}>
						{dirPicked ? 'Save folder selected' : 'Choose save folder'}
					</button>
				{/if}
				<button type="button" class="submit-btn" onclick={handleStartDownload}>
					Start Playlist Download
				</button>
			</div>
		</div>
	{:else}
		<BatchActiveState
			{fsaaSupported}
			{dirPicked}
			onPickDirectory={handlePickDirectory}
			onCancel={handleCancel}
		/>
	{/if}

	{#if error && phase !== 'idle'}
		<div class="error-inline" role="alert">{error}</div>
	{/if}
</div>

<style>
	.batch-input {
		position: relative;
		padding: 1.5rem;
		background: linear-gradient(180deg, #ffffff 0%, #fff7fb 100%);
		border-radius: 1.5rem;
		border: 1px solid #ffd7e8;
		box-shadow: 0 16px 35px -24px rgba(255, 77, 140, 0.55);
		overflow: hidden;
	}

	.batch-input::before {
		content: '';
		position: absolute;
		inset: 0 0 auto 0;
		height: 4px;
		background: linear-gradient(90deg, #ff4d8c 0%, #ffb938 100%);
	}

	.header {
		margin-bottom: 0.85rem;
	}

	h3 {
		margin: 0;
		font-size: 1.02rem;
		font-weight: 700;
		color: #2d1b36;
	}

	.subtitle {
		margin: 0.25rem 0 0;
		font-size: 0.82rem;
		color: rgba(45, 27, 54, 0.68);
		font-weight: 600;
	}

	form {
		display: flex;
		flex-direction: column;
		gap: 0.8rem;
	}

	.url-field,
	.quality-select,
	.mode-select {
		padding: 0.875rem 1rem;
		font-size: 0.95rem;
		border: 1px solid #ffc8de;
		border-radius: 999px;
		background: #ffffff;
		color: #2d1b36;
		font-weight: 700;
		transition: border-color 0.2s ease, box-shadow 0.2s ease;
	}

	.url-field:focus,
	.quality-select:focus,
	.mode-select:focus {
		outline: none;
		border-color: #ff4d8c;
		box-shadow: 0 0 0 4px rgba(255, 77, 140, 0.14);
	}

	.quality-select,
	.mode-select {
		border-radius: 14px;
	}

	.quality-label {
		display: block;
		font-size: 0.78rem;
		font-weight: 700;
		color: rgba(45, 27, 54, 0.74);
		margin-bottom: 0.35rem;
	}

	.error-text,
	.error-inline {
		color: #dc2626;
		font-size: 0.82rem;
		font-weight: 700;
	}

	.error-inline {
		margin-top: 0.7rem;
	}

	.phase-card {
		display: flex;
		flex-direction: column;
		gap: 0.7rem;
		padding: 1rem;
		background: rgba(255, 255, 255, 0.72);
		border: 1px solid #ffd6e8;
		border-radius: 1rem;
	}

	.phase-title {
		margin: 0;
		font-size: 0.94rem;
		font-weight: 800;
		color: #2d1b36;
	}

	.phase-meta {
		margin: 0;
		font-size: 0.8rem;
		font-weight: 700;
		color: rgba(45, 27, 54, 0.68);
	}

	.phase-progress-track {
		height: 8px;
		border-radius: 999px;
		background: #ffe7f2;
		overflow: hidden;
	}

	.phase-progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #ff4d8c 0%, #ffb938 100%);
		transition: width 0.25s ease;
	}

	.actions-row {
		display: flex;
		flex-wrap: wrap;
		gap: 0.55rem;
		align-items: center;
	}

	.submit-btn,
	.ghost-btn,
	.folder-btn {
		padding: 0.82rem 1.2rem;
		font-size: 0.88rem;
		font-weight: 800;
		border: none;
		border-radius: 999px;
		cursor: pointer;
		transition: transform 0.2s ease, box-shadow 0.2s ease, filter 0.2s ease, background 0.2s ease;
	}

	.submit-btn {
		color: white;
		background: linear-gradient(90deg, #ff4d8c 0%, #ffb938 100%);
		box-shadow: 0 14px 24px -18px rgba(255, 77, 140, 0.75);
	}

	.submit-btn:hover:not(:disabled),
	.ghost-btn:hover,
	.folder-btn:hover {
		filter: brightness(1.05);
		transform: translateY(-1px);
	}

	.submit-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
		transform: none;
	}

	.ghost-btn {
		color: #2d1b36;
		background: #ffeaf4;
		border: 1px solid #ffc9df;
	}

	.folder-btn {
		color: #2d1b36;
		background: #fff2f9;
		border: 1px solid #ffcce1;
	}

	:global(.page-root.theme-dark) .batch-input {
		background: linear-gradient(180deg, rgba(30, 30, 42, 0.92) 0%, rgba(25, 25, 35, 0.92) 100%);
		border-color: rgba(255, 77, 140, 0.22);
	}

	:global(.page-root.theme-dark) h3 {
		color: #f3e8ff;
	}

	:global(.page-root.theme-dark) .subtitle,
	:global(.page-root.theme-dark) .phase-meta,
	:global(.page-root.theme-dark) .quality-label {
		color: rgba(224, 208, 245, 0.78);
	}

	:global(.page-root.theme-dark) .url-field,
	:global(.page-root.theme-dark) .mode-select,
	:global(.page-root.theme-dark) .quality-select,
	:global(.page-root.theme-dark) .phase-card,
	:global(.page-root.theme-dark) .folder-btn,
	:global(.page-root.theme-dark) .ghost-btn {
		background: rgba(255, 77, 140, 0.1);
		color: #f9e8ff;
		border-color: rgba(255, 77, 140, 0.3);
	}

	:global(.page-root.theme-dark) .phase-title {
		color: #f8ecff;
	}
</style>
