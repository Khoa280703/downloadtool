<script lang="ts">
	import { onMount } from 'svelte';
	import { trackPageView } from '$lib/analytics';
	import { buildFaqSchema } from '$lib/seo/structured-data';

	const PAGE_URL = 'https://snapvie.com/why-youtube-downloads-show-360p-only';
	const OG_IMAGE = 'https://snapvie.com/og-image.png';

	const faqs = [
		{
			q: 'Why does my downloaded YouTube video only show 360p?',
			a: "YouTube stores high-quality video and audio in separate streams. Most simple downloaders only grab the combined stream, which is capped at 360p or 480p. To get 1080p or higher you need a tool that can merge the separate video and audio tracks — this process is called muxing."
		},
		{
			q: 'Why is 1080p not available in YouTube downloads?',
			a: "Since 2015, YouTube has served 1080p (and above) video as a video-only stream with no embedded audio. Simple downloaders that don't support muxing can't access these streams and fall back to 360p or 480p, which is the highest quality with audio included."
		},
		{
			q: 'How does Snapvie get the full quality?',
			a: 'Snapvie downloads the high-quality video stream and the separate audio stream, then merges them server-side using a muxing pipeline. The result is a single file with both video and audio at the quality you selected.'
		},
		{
			q: 'What is the highest quality I can download?',
			a: 'Snapvie supports up to 8K HDR (7680×4320) when the source video provides it. Most videos offer up to 1080p or 4K. Quality depends entirely on what the original uploader published.'
		}
	];

	const jsonLd = JSON.stringify({
		'@context': 'https://schema.org',
		'@graph': [buildFaqSchema(faqs)]
	});

	onMount(() => {
		trackPageView(
			'/why-youtube-downloads-show-360p-only',
			'Why YouTube Downloads Show 360p Only — Snapvie'
		);
	});
</script>

<svelte:head>
	<title>Why YouTube Downloads Show 360p Only (And How to Get Full Quality) — Snapvie</title>
	<meta
		name="description"
		content="Why does your YouTube download default to 360p? Learn why quality is capped at 360p for most tools and how Snapvie delivers 1080p, 4K, and 8K HDR downloads."
	/>
	<link rel="canonical" href={PAGE_URL} />
	<meta property="og:type" content="article" />
	<meta property="og:site_name" content="Snapvie" />
	<meta
		property="og:title"
		content="Why YouTube Downloads Show 360p Only (And How to Get Full Quality)"
	/>
	<meta
		property="og:description"
		content="Why does your YouTube download default to 360p? Learn why quality is capped and how to get the full resolution you expect."
	/>
	<meta property="og:url" content={PAGE_URL} />
	<meta property="og:image" content={OG_IMAGE} />
	<meta property="og:image:width" content="1200" />
	<meta property="og:image:height" content="630" />
	<meta name="twitter:card" content="summary_large_image" />
	<meta
		name="twitter:title"
		content="Why YouTube Downloads Show 360p Only (And How to Get Full Quality)"
	/>
	<meta
		name="twitter:description"
		content="Most YouTube downloaders are capped at 360p. Here's why — and how to get 1080p, 4K, and 8K."
	/>
	<meta name="twitter:image" content={OG_IMAGE} />
	{@html `<script type="application/ld+json">${jsonLd}<\/script>`}
</svelte:head>

<div class="content-page">
	<div class="breadcrumb">
		<a href="/">Snapvie</a> <span>›</span> <span>Why Downloads Show 360p Only</span>
	</div>

	<h1>Why YouTube Downloads Show 360p Only (And How to Get Full Quality)</h1>

	<div class="quick-answer">
		<strong>Quick answer:</strong> YouTube stores 1080p and higher as a video-only stream with no
		audio. Most downloaders can only grab the combined stream, which is capped at 360p or 480p. To
		get full quality you need a tool that merges the video and audio streams — called muxing.
		<a href="/">Snapvie</a> does this automatically.
	</div>

	<section>
		<h2>The real reason quality is capped at 360p</h2>
		<p>
			In 2015, YouTube changed how it delivers high-resolution video. Instead of encoding a single
			file with both video and audio, YouTube started storing them as two separate streams:
		</p>
		<ul>
			<li>A <strong>video-only stream</strong> — available at 1080p, 4K, 8K HDR</li>
			<li>An <strong>audio-only stream</strong> — available at 128kbps or 256kbps</li>
		</ul>
		<p>
			The old combined streams (video + audio in one file) are still available, but only up to 480p —
			and for many videos it's capped at 360p.
		</p>
		<p>
			Most YouTube downloader tools are built to grab a single URL from YouTube's API. They pick the
			combined stream because it's simpler, and end up giving you a 360p file even when the video was
			uploaded in 4K.
		</p>
	</section>

	<section>
		<h2>What muxing is and why it matters</h2>
		<p>
			Muxing (short for multiplexing) is the process of combining a video stream and an audio stream
			into a single playable file. This is exactly what's needed to get full quality from YouTube.
		</p>
		<p>The process works like this:</p>
		<ol>
			<li>Download the video-only stream at the quality you want (e.g. 4K)</li>
			<li>Download the audio-only stream separately</li>
			<li>Merge them into a single MP4 file using a muxer</li>
		</ol>
		<p>
			This requires actual server-side processing — it's not just a file copy. Tools that don't have
			this capability simply can't offer you anything above 480p.
		</p>
	</section>

	<section>
		<h2>Why Snapvie gets you the full quality</h2>
		<p>
			Snapvie was built specifically to handle muxing. When you paste a YouTube URL and select a
			quality above 480p, Snapvie:
		</p>
		<ul>
			<li>Fetches the high-resolution video stream and the best available audio stream</li>
			<li>Runs both through a server-side mux pipeline (written in Rust for speed)</li>
			<li>Delivers a single clean file with full audio and video</li>
		</ul>
		<p>
			The result is the actual quality the video was uploaded at — not a degraded 360p fallback. No
			third-party software to install. No command line. Just paste the URL and pick your quality.
		</p>
		<p>
			Snapvie supports up to <a href="/download-youtube-8k-hdr">8K HDR downloads</a> and handles
			<a href="/download-youtube-playlist">full playlist downloads</a> too.
		</p>
	</section>

	<section>
		<h2>How to check if your download is really full quality</h2>
		<p>
			After downloading, right-click the file, open Properties (Windows) or Get Info (Mac), and look
			at the video resolution. A true 1080p download will report 1920×1080. If you see 640×360, the
			tool gave you the low-quality combined stream.
		</p>
		<p>
			Alternatively, open the file in VLC and press Ctrl+J (Windows) or Cmd+J (Mac) to see the
			codec information including resolution and bitrate.
		</p>
	</section>

	<section class="faq-section">
		<h2>Frequently Asked Questions</h2>
		{#each faqs as { q, a }}
			<div class="faq-item">
				<h3>{q}</h3>
				<p>{a}</p>
			</div>
		{/each}
	</section>

	<div class="cta-block">
		<p>Ready to download in the quality the video was actually uploaded at?</p>
		<a href="/" class="cta-btn">Download with Snapvie — Free, No Ads</a>
	</div>

	<div class="related-links">
		<strong>Related guides:</strong>
		<a href="/why-youtube-downloads-need-muxing">Why YouTube downloads need muxing</a>
		<span>·</span>
		<a href="/best-format-for-youtube-downloads-mp4-vs-webm">MP4 vs WebM — which format to use</a>
		<span>·</span>
		<a href="/download-youtube-8k-hdr">Download 8K HDR YouTube videos</a>
	</div>
</div>

<style>
	.content-page {
		max-width: 980px;
		margin: 0 auto;
		padding: 0.25rem 0.25rem 2rem;
	}

	.breadcrumb {
		font-size: 0.8rem;
		color: #7b5f76;
		margin-bottom: 1rem;
		display: flex;
		gap: 0.35rem;
		align-items: center;
	}

	.breadcrumb a {
		color: #ff4d8c;
		text-decoration: none;
		font-weight: 600;
	}

	h1 {
		font-size: clamp(1.65rem, 4vw, 2.3rem);
		font-weight: 800;
		color: #2d1b36;
		letter-spacing: -0.02em;
		margin-bottom: 1.1rem;
		line-height: 1.25;
	}

	.quick-answer {
		background: linear-gradient(135deg, rgba(255, 77, 140, 0.07), rgba(139, 92, 246, 0.06));
		border: 1px solid rgba(255, 77, 140, 0.2);
		border-radius: 0.85rem;
		padding: 1rem 1.2rem;
		font-size: 0.96rem;
		line-height: 1.65;
		color: #4a3454;
		margin-bottom: 1.4rem;
	}

	.quick-answer a {
		color: #ff4d8c;
		font-weight: 700;
		text-decoration: none;
	}

	section {
		margin-bottom: 0.95rem;
		padding: 1rem 1.1rem;
		border-radius: 1.05rem;
		border: 1px solid rgba(255, 77, 140, 0.12);
		background: rgba(255, 255, 255, 0.72);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		box-shadow: 0 20px 28px -26px rgba(255, 77, 140, 0.45);
	}

	h2 {
		font-size: 1.14rem;
		font-weight: 800;
		color: #2d1b36;
		margin-bottom: 1rem;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid rgba(255, 77, 140, 0.14);
	}

	p {
		font-size: 0.9375rem;
		line-height: 1.7;
		color: #6d4f64;
		margin-bottom: 0.85rem;
	}

	p:last-child {
		margin-bottom: 0;
	}

	ul,
	ol {
		padding-left: 1.5rem;
		margin-bottom: 0.85rem;
	}

	li {
		font-size: 0.9375rem;
		line-height: 1.7;
		color: #6d4f64;
		margin-bottom: 0.4rem;
	}

	li strong {
		color: #2d1b36;
	}

	a {
		color: #ff4d8c;
		font-weight: 600;
		text-decoration: none;
	}

	a:hover {
		text-decoration: underline;
	}

	.faq-section h3 {
		font-size: 0.97rem;
		font-weight: 700;
		color: #2d1b36;
		margin-bottom: 0.4rem;
	}

	.faq-item {
		margin-bottom: 1rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid rgba(255, 77, 140, 0.1);
	}

	.faq-item:last-child {
		margin-bottom: 0;
		padding-bottom: 0;
		border-bottom: none;
	}

	.cta-block {
		margin: 1.4rem 0;
		padding: 1.2rem 1.4rem;
		border-radius: 1.05rem;
		background: linear-gradient(135deg, rgba(255, 77, 140, 0.1), rgba(139, 92, 246, 0.08));
		border: 1px solid rgba(255, 77, 140, 0.22);
		text-align: center;
	}

	.cta-block p {
		font-size: 1rem;
		font-weight: 600;
		color: #3d2048;
		margin-bottom: 0.85rem;
	}

	.cta-btn {
		display: inline-block;
		background: linear-gradient(135deg, #ff4d8c, #c026d3);
		color: #fff !important;
		font-weight: 700;
		font-size: 0.95rem;
		padding: 0.65rem 1.5rem;
		border-radius: 999px;
		text-decoration: none !important;
		transition: opacity 0.15s;
	}

	.cta-btn:hover {
		opacity: 0.9;
		text-decoration: none !important;
	}

	.related-links {
		font-size: 0.875rem;
		color: #7b5f76;
		display: flex;
		flex-wrap: wrap;
		gap: 0.4rem;
		align-items: center;
		margin-top: 0.5rem;
	}

	.related-links strong {
		color: #3d2048;
	}

	@media (max-width: 640px) {
		h1 {
			font-size: 1.45rem;
		}

		h2 {
			font-size: 1.05rem;
		}
	}

	:global(.app.theme-dark) h1,
	:global(.app.theme-dark) h2 {
		color: #ffffff;
	}

	:global(.app.theme-dark) .quick-answer {
		background: rgba(29, 26, 43, 0.7);
		border-color: rgba(255, 77, 140, 0.28);
		color: rgba(224, 208, 245, 0.88);
	}

	:global(.app.theme-dark) section {
		border-color: rgba(255, 77, 140, 0.2);
		background: rgba(28, 27, 40, 0.72);
		box-shadow: 0 16px 24px -22px rgba(255, 77, 140, 0.42);
	}

	:global(.app.theme-dark) p,
	:global(.app.theme-dark) li {
		color: rgba(224, 208, 245, 0.85);
	}

	:global(.app.theme-dark) li strong,
	:global(.app.theme-dark) .faq-section h3 {
		color: #ffffff;
	}

	:global(.app.theme-dark) .cta-block {
		background: rgba(29, 26, 43, 0.8);
		border-color: rgba(255, 77, 140, 0.3);
	}

	:global(.app.theme-dark) .cta-block p {
		color: rgba(224, 208, 245, 0.9);
	}

	:global(.app.theme-dark) .breadcrumb {
		color: rgba(200, 185, 220, 0.75);
	}

	:global(.app.theme-dark) .related-links {
		color: rgba(200, 185, 220, 0.75);
	}

	:global(.app.theme-dark) .related-links strong {
		color: rgba(224, 208, 245, 0.9);
	}
</style>
