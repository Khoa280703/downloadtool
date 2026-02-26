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

	function enforceHttps(url: string): string {
		if (
			typeof window !== 'undefined' &&
			window.location.protocol === 'https:' &&
			url.startsWith('http://')
		) {
			return `https://${url.slice('http://'.length)}`;
		}
		return url;
	}

	function formatSize(bytes?: number): string {
		if (!bytes) return '';
		const units = ['B', 'KB', 'MB', 'GB'];
		let size = bytes;
		let unitIndex = 0;
		while (size >= 1024 && unitIndex < units.length - 1) {
			size /= 1024;
			unitIndex += 1;
		}
		return `${size.toFixed(size >= 100 || unitIndex === 0 ? 0 : 1)} ${units[unitIndex]}`;
	}

	function ctaLabel(): string {
		if (!stream) return 'Select format to continue';
		const size = formatSize(stream.size);
		if (size) return `Download Now (${size})`;
		return `Download ${stream.quality || 'Now'}`;
	}

	/** Trigger browser download */
	async function handleDownload(): Promise<void> {
		if (!stream) return;

		trackDownloadStarted('youtube', stream.quality || 'unknown', stream.format || 'mp4');

		isLoading = true;
		setDownloading(true);
		downloadProgress.set(0);

		try {
			const useMux = audioStream && !stream.hasAudio;
			const downloadUrl = useMux
				? buildMuxedStreamUrl(stream.url, audioStream!.url, title)
				: buildStreamUrl(stream.url, title, stream.format);
			const secureDownloadUrl = enforceHttps(downloadUrl);

			const anchor = document.createElement('a');
			anchor.href = secureDownloadUrl;
			anchor.download = `${title.replace(/[^a-z0-9]/gi, '_')}.mp4`;
			anchor.style.display = 'none';
			document.body.appendChild(anchor);

			const progressInterval = setInterval(() => {
				downloadProgress.update((p) => Math.min(p + 10, 90));
			}, 200);

			anchor.click();

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
</script>

<div class="cta-shell">
	<button
		type="button"
		class="download-cta"
		onclick={handleDownload}
		disabled={disabled || !stream || isLoading || $currentDownload.isDownloading}
		aria-label={isLoading ? 'Preparing download...' : 'Download selected format'}
	>
		{#if isLoading || $currentDownload.isDownloading}
			<span class="spinner"></span>
			<span>Preparing download...</span>
		{:else}
			<span class="material-symbols-outlined icon">download</span>
			<span>{ctaLabel()}</span>
		{/if}
	</button>

	{#if $currentDownload.isDownloading}
		<div class="progress-row" aria-live="polite">
			<div class="progress-track">
				<div class="progress-fill" style:width="{$downloadProgress}%"></div>
			</div>
			<span class="progress-text">{$downloadProgress}%</span>
		</div>
	{:else}
		<p class="legal-note">
			By using FetchTube, you accept our <a href="/privacy">Privacy Policy</a>.
		</p>
	{/if}
</div>

<style>
	.cta-shell {
		display: flex;
		flex-direction: column;
		gap: 0.65rem;
	}

	.download-cta {
		width: 100%;
		height: 3.5rem;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.6rem;
		border: 0;
		border-radius: 999px;
		background: linear-gradient(135deg, #ff2e93, #ec4899);
		color: #fff;
		font-size: 1rem;
		font-weight: 800;
		letter-spacing: 0.01em;
		cursor: pointer;
		box-shadow: 0 10px 22px -12px rgba(255, 46, 147, 0.85);
		transition: transform 0.2s ease, box-shadow 0.2s ease, filter 0.2s ease;
	}

	.download-cta:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 16px 28px -16px rgba(79, 70, 229, 0.75);
		filter: brightness(1.04);
	}

	.download-cta:active:not(:disabled) {
		transform: translateY(0);
	}

	.download-cta:disabled {
		cursor: not-allowed;
		opacity: 0.55;
		background: #cbd5e1;
		color: #475569;
		box-shadow: none;
	}

	.icon {
		font-size: 1.15rem;
	}

	.spinner {
		width: 1.05rem;
		height: 1.05rem;
		border-radius: 999px;
		border: 2px solid rgba(255, 255, 255, 0.32);
		border-top-color: #fff;
		animation: spin 0.75s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.progress-row {
		display: flex;
		align-items: center;
		gap: 0.65rem;
	}

	.progress-track {
		flex: 1;
		height: 0.36rem;
		border-radius: 999px;
		background: #e2e8f0;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(135deg, #4f46e5, #8b5cf6);
		border-radius: 999px;
		transition: width 0.3s ease;
	}

	.progress-text {
		font-size: 0.72rem;
		font-weight: 800;
		color: #475569;
		min-width: 2.4rem;
		text-align: right;
	}

	.legal-note {
		margin: 0;
		text-align: center;
		font-size: 0.68rem;
		font-weight: 600;
		color: #94a3b8;
	}

	.legal-note a {
		color: #64748b;
		text-decoration: underline;
		text-underline-offset: 2px;
	}

	.legal-note a:hover {
		color: #ff2e93;
	}

	@media (max-width: 560px) {
		.download-cta {
			font-size: 0.92rem;
			height: 3.25rem;
			padding-inline: 0.9rem;
		}
	}
</style>
