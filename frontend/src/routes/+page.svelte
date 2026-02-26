<script lang="ts">
	import { onMount } from 'svelte';
	import UrlInput from '$components/UrlInput.svelte';
	import BatchInput from '$components/BatchInput.svelte';
	import FormatPicker from '$components/FormatPicker.svelte';
	import DownloadBtn from '$components/DownloadBtn.svelte';
	import BatchProgress from '$components/BatchProgress.svelte';
	import { isValidVideoUrl } from '$lib/api';
	import { currentDownload } from '$stores/download';
	import type { ExtractResult, Stream } from '$lib/types';
	import {
		trackExtractSuccess,
		trackFormatSelected
	} from '$lib/analytics';

	const rawApiBase = (import.meta.env.VITE_API_URL || '').trim().replace(/\/+$/, '');
	const API_BASE =
		typeof window !== 'undefined' &&
		window.location.protocol === 'https:' &&
		rawApiBase.startsWith('http://')
			? `https://${rawApiBase.slice('http://'.length)}`
			: rawApiBase;
	const USERSCRIPT_INSTALL_URL =
		(API_BASE ? `${API_BASE}/userscript` : 'https://api-download.khoadangbui.online/userscript');

	let extractResult = $state<ExtractResult | null>(null);
	let isExtracting = $state(false);
	/** Audio stream to pair with video-only stream for transparent muxing */
	let selectedAudioStream = $state<Stream | null>(null);
	/** URL prefilled from share-target or clipboard detection */
	let prefilledUrl = $state('');
	/** Bookmarklet loader link generated from current origin */
	let bookmarkletHref = $state('javascript:void(0)');

	/**
	 * Handle extract completion from UrlInput
	 */
	function handleExtract(result: ExtractResult): void {
		isExtracting = false;

		// Track successful extraction
		trackExtractSuccess(result.platform, 0, result.streams.length);

		extractResult = result;
	}

	/**
	 * Handle format selection from FormatPicker (video stream + optional audio for muxing)
	 */
	function handleFormatSelect(videoStream: Stream, audioStream: Stream | null): void {
		if (extractResult) {
			currentDownload.update(s => ({ ...s, selectedStream: videoStream }));
			selectedAudioStream = audioStream;

			// Track format selection
			trackFormatSelected(
				extractResult.platform,
				videoStream.quality,
				videoStream.format,
				videoStream.hasAudio
			);
		}
	}

	function applyPrefilledUrl(candidate: string): void {
		const value = candidate.trim();
		if (!value || !isValidVideoUrl(value)) return;
		prefilledUrl = value;
	}

	onMount(() => {
		const queryUrl = new URLSearchParams(window.location.search).get('url') ?? '';
		applyPrefilledUrl(queryUrl);

		const onUrlDetected: EventListener = (event) => {
			const customEvent = event as CustomEvent<{ url?: string }>;
			applyPrefilledUrl(customEvent.detail?.url ?? '');
		};
		window.addEventListener('yturl-detected', onUrlDetected);

		const bookmarkletScriptBase = API_BASE || window.location.origin;
		bookmarkletHref =
			`javascript:(function(){var s=document.createElement("script");` +
			`s.src="${bookmarkletScriptBase}/bm.js?t="+Date.now();` +
			`document.body.appendChild(s);})()`;

		return () => {
			window.removeEventListener('yturl-detected', onUrlDetected);
		};
	});

</script>

<svelte:head>
	<title>Download YouTube Videos Free | VideoDL</title>
	<meta name="description" content="Free online YouTube video downloader. No registration required. Download videos in high quality instantly." />
	<meta name="keywords" content="video downloader, youtube downloader, free download, youtube video download" />

	<!-- Open Graph -->
	<meta property="og:title" content="Download YouTube Videos Free" />
	<meta property="og:description" content="Free online video downloader. No registration required." />
	<meta property="og:type" content="website" />
	<meta property="og:url" content="https://videodl.app" />
	<meta property="og:image" content="https://videodl.app/og-image.jpg" />

	<!-- Twitter Card -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="Download YouTube Videos Free" />
	<meta name="twitter:description" content="Free online video downloader. No registration required." />

	<!-- Structured Data -->
	<script type="application/ld+json">
		{
			"@context": "https://schema.org",
			"@type": "WebApplication",
			"name": "VideoDL",
			"description": "Free online video downloader for YouTube",
			"applicationCategory": "UtilityApplication",
			"operatingSystem": "Any",
			"offers": {
				"@type": "Offer",
				"price": "0",
				"priceCurrency": "USD"
			}
		}
	</script>
</svelte:head>

<div class="hero">
	<h1>Download YouTube Videos Free</h1>
	<p class="subtitle">
		Fast, free, no registration required. Paste a link and download instantly.
	</p>
</div>

<section class="download-section" aria-label="Video download">
	<UrlInput onExtract={handleExtract} prefilledUrl={prefilledUrl} />

	{#if isExtracting}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Extracting video information...</p>
		</div>
	{/if}

	{#if extractResult}
		<div class="result-card">
			<div class="video-info">
				{#if extractResult.thumbnail}
					<img
						src={extractResult.thumbnail}
						alt={extractResult.title}
						class="thumbnail"
						loading="lazy"
					/>
				{/if}
				<h3 class="video-title">{extractResult.title}</h3>
			</div>

			<FormatPicker
				streams={extractResult.streams}
				onSelect={handleFormatSelect}
			/>

			<DownloadBtn
				stream={$currentDownload.selectedStream}
				audioStream={selectedAudioStream}
				title={extractResult.title}
			/>
		</div>
	{/if}

	{#if $currentDownload.error}
		<div class="error-message" role="alert">
			<svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor">
				<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
			</svg>
			<span>{$currentDownload.error}</span>
		</div>
	{/if}
</section>

<section class="batch-section" aria-label="Batch download">
	<BatchInput />
	<BatchProgress />
</section>

<section class="install-section" aria-label="Omnichannel install options">
	<h2>Install Tools</h2>
	<div class="install-grid">
		<article class="install-card">
			<h3>Bookmarklet</h3>
			<p>Drag this button to your bookmarks bar for one-click downloads on YouTube pages.</p>
			<a class="install-link bookmarklet-link" href={bookmarkletHref}>
				Drag VideoDL Bookmarklet
			</a>
			<p class="install-note">If drag-and-drop is unavailable, create a bookmark and paste the link URL.</p>
		</article>

		<article class="install-card">
			<h3>UserScript</h3>
			<p>Install the userscript for Tampermonkey or Violentmonkey.</p>
			<a
				class="install-link userscript-link"
				href={USERSCRIPT_INSTALL_URL}
				target="_blank"
				rel="external noopener noreferrer"
			>
				Install UserScript
			</a>
			<p class="install-note">Requires Tampermonkey or Violentmonkey extension.</p>
		</article>
	</div>
</section>

<section class="features" aria-label="Features">
	<h2>Why Choose VideoDL?</h2>
	<div class="feature-grid">
		<div class="feature">
			<div class="feature-icon"><svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor">
				<path d="M13 3c-4.97 0-9 4.03-9 9H1l3.89 3.89.07.14L9 12H6c0-3.87 3.13-7 7-7s7 3.13 7 7-3.13 7-7 7c-1.93 0-3.68-.79-4.94-2.06l-1.42 1.42C8.27 19.99 10.51 21 13 21c4.97 0 9-4.03 9-9s-4.03-9-9-9zm-1 5v5l4.28 2.54.72-1.21-3.5-2.08V8H12z"/>
			</svg></div>
			<h3>Instant Download</h3>
			<p>No waiting, no processing delays. Your download starts immediately.</p>
		</div>
		<div class="feature">
			<div class="feature-icon"><svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor">
				<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
			</svg></div>
			<h3>Smart Muxing</h3>
			<p>Automatically combines video and audio streams for high-quality YouTube downloads.</p>
		</div>
		<div class="feature">
			<div class="feature-icon"><svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor">
				<path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm0 10.99h7c-.53 4.12-3.28 7.79-7 8.94V12H5V6.3l7-3.11v8.8z"/>
			</svg></div>
			<h3>100% Free</h3>
			<p>No registration, no hidden fees. Completely free to use.</p>
		</div>
		<div class="feature">
			<div class="feature-icon"><svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor">
				<path d="M17 1.01L7 1c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM17 19H7V5h10v14z"/>
			</svg></div>
			<h3>Mobile Friendly</h3>
			<p>Works perfectly on iPhone, Android, and all devices.</p>
		</div>
	</div>
</section>

<style>
	.hero {
		text-align: center;
		margin-bottom: 2rem;
	}

	h1 {
		font-size: clamp(1.75rem, 5vw, 2.5rem);
		font-weight: 800;
		color: var(--text-color);
		margin-bottom: 0.75rem;
		line-height: 1.2;
	}

	.subtitle {
		font-size: 1.125rem;
		color: var(--text-secondary);
		max-width: 500px;
		margin: 0 auto;
	}

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		padding: 2rem;
		color: var(--text-secondary);
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid var(--border-color);
		border-top-color: var(--primary-color);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.download-section {
		margin-bottom: 2rem;
	}

	.result-card {
		margin-top: 1.5rem;
		padding: 1.5rem;
		background: var(--card-bg);
		border-radius: 1rem;
		border: 1px solid var(--border-color);
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
	}

	.video-info {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.thumbnail {
		width: 120px;
		height: 68px;
		object-fit: cover;
		border-radius: 0.5rem;
	}

	.video-title {
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-color);
		margin: 0;
		display: -webkit-box;
		line-clamp: 2;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem;
		background: var(--error-bg);
		color: var(--error-color);
		border-radius: 0.75rem;
		margin-top: 1rem;
	}

	.batch-section {
		margin-bottom: 2rem;
	}

	.install-section {
		margin-bottom: 2rem;
		padding-top: 2rem;
		border-top: 1px solid var(--border-color);
	}

	.install-section h2 {
		text-align: center;
		font-size: 1.5rem;
		margin-bottom: 1rem;
		color: var(--text-color);
	}

	.install-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
		gap: 1rem;
	}

	.install-card {
		padding: 1.25rem;
		background: var(--card-bg);
		border-radius: 1rem;
		border: 1px solid var(--border-color);
	}

	.install-card h3 {
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-color);
		margin-bottom: 0.5rem;
	}

	.install-card p {
		font-size: 0.875rem;
		color: var(--text-secondary);
		margin: 0 0 0.75rem;
	}

	.install-link {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		padding: 0.75rem;
		border-radius: 0.75rem;
		text-decoration: none;
		font-weight: 600;
		color: #fff;
	}

	.bookmarklet-link {
		background: var(--primary-color);
	}

	.bookmarklet-link:hover {
		background: var(--primary-hover);
	}

	.userscript-link {
		background: var(--secondary-color);
	}

	.userscript-link:hover {
		background: var(--secondary-hover);
	}

	.install-note {
		margin-top: 0.75rem;
		font-size: 0.8125rem;
	}

	.features {
		padding-top: 2rem;
		border-top: 1px solid var(--border-color);
	}

	.features h2 {
		text-align: center;
		font-size: 1.5rem;
		margin-bottom: 1.5rem;
		color: var(--text-color);
	}

	.feature-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1.5rem;
	}

	.feature {
		text-align: center;
		padding: 1.5rem;
		background: var(--card-bg);
		border-radius: 1rem;
		border: 1px solid var(--border-color);
	}

	.feature-icon {
		width: 48px;
		height: 48px;
		margin: 0 auto 1rem;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--primary-alpha);
		color: var(--primary-color);
		border-radius: 12px;
	}

	.feature h3 {
		font-size: 1rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
		color: var(--text-color);
	}

	.feature p {
		font-size: 0.875rem;
		color: var(--text-secondary);
		margin: 0;
	}

	@media (max-width: 640px) {
		.thumbnail {
			width: 80px;
			height: 45px;
		}

		.feature-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
