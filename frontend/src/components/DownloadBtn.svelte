<script lang="ts">
	import { buildStreamUrl, buildMuxedStreamUrl } from '$lib/api';
	import { currentDownload, setDownloading, downloadProgress } from '$stores/download';
	import { trackDownloadStarted } from '$lib/analytics';
	import type { Stream } from '$lib/types';

	interface Props {
		stream: Stream | null;
		/** Audio-only stream to mux with video-only stream. When provided, uses /api/stream/muxed. */
		audioStream?: Stream | null;
		title: string;
		disabled?: boolean;
	}

	let { stream, audioStream = null, title, disabled = false }: Props = $props();

	let isLoading = $state(false);

	/** Trigger browser download */
	async function handleDownload(): Promise<void> {
		if (!stream) return;

		// Track download start
		trackDownloadStarted('youtube', stream.quality || 'unknown', stream.format || 'mp4');

		isLoading = true;
		setDownloading(true);
		downloadProgress.set(0);

		try {
			// Build download URL — use muxed endpoint for video-only streams
			const useMux = audioStream && !stream.hasAudio;
			const downloadUrl = useMux
				? buildMuxedStreamUrl(stream.url, audioStream!.url, title)
				: buildStreamUrl(stream.url, title, stream.format);

			// Create anchor element for download
			const anchor = document.createElement('a');
			anchor.href = downloadUrl;
			anchor.download = `${title.replace(/[^a-z0-9]/gi, '_')}.mp4`;
			anchor.style.display = 'none';

			document.body.appendChild(anchor);

			// Simulate progress (actual progress unknowable for muxed streams)
			const progressInterval = setInterval(() => {
				downloadProgress.update(p => Math.min(p + 10, 90));
			}, 200);

			// Trigger download
			anchor.click();

			// Cleanup after short delay
			setTimeout(() => {
				clearInterval(progressInterval);
				document.body.removeChild(anchor);
				downloadProgress.set(100);
				isLoading = false;
				setDownloading(false);
			}, 1000);
		} catch (err) {
			console.error('Download failed:', err);
			isLoading = false;
			setDownloading(false);
		}
	}

	/** Format file size */
	function formatSize(bytes?: number): string {
		if (!bytes) return '';
		const units = ['B', 'KB', 'MB', 'GB'];
		let size = bytes;
		let unitIndex = 0;
		while (size >= 1024 && unitIndex < units.length - 1) {
			size /= 1024;
			unitIndex++;
		}
		return `${size.toFixed(1)} ${units[unitIndex]}`;
	}
</script>

<div class="download-btn-container">
	<button
		class="download-btn"
		onclick={handleDownload}
		disabled={disabled || !stream || isLoading || $currentDownload.isDownloading}
		aria-label={isLoading ? 'Downloading...' : 'Download video'}
	>
		{#if isLoading || $currentDownload.isDownloading}
			<span class="spinner"></span>
			<span>Starting download...</span>
		{:else}
			<svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor">
				<path d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z"/>
			</svg>
			<span>Download {stream?.quality || ''}</span>
			{#if stream?.size}
				<span class="size">({formatSize(stream.size)})</span>
			{/if}
		{/if}
	</button>

	{#if $currentDownload.isDownloading}
		<div class="progress-container">
			<div class="progress-bar">
				<div class="progress-fill" style:width="{$downloadProgress}%"></div>
			</div>
			<span class="progress-text">{$downloadProgress}%</span>
		</div>
	{/if}
</div>

<style>
	.download-btn-container {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.download-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 1rem 1.5rem;
		font-size: 1rem;
		font-weight: 600;
		color: white;
		background: linear-gradient(135deg, var(--success-color, #22c55e), var(--success-hover, #16a34a));
		border: none;
		border-radius: 0.75rem;
		cursor: pointer;
		transition: all 0.2s;
		min-height: 56px;
		box-shadow: 0 4px 6px -1px rgba(34, 197, 94, 0.2);
	}

	.download-btn:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 6px 8px -1px rgba(34, 197, 94, 0.3);
	}

	.download-btn:active:not(:disabled) {
		transform: translateY(0);
	}

	.download-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
		background: var(--border-color, #e5e7eb);
		box-shadow: none;
	}

	.size {
		font-size: 0.875rem;
		font-weight: 400;
		opacity: 0.9;
	}

	.spinner {
		width: 20px;
		height: 20px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.progress-container {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.progress-bar {
		flex: 1;
		height: 6px;
		background: var(--border-color, #e5e7eb);
		border-radius: 3px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: var(--success-color, #22c55e);
		border-radius: 3px;
		transition: width 0.3s ease;
	}

	.progress-text {
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-secondary, #6b7280);
		min-width: 2.5rem;
		text-align: right;
	}
</style>
