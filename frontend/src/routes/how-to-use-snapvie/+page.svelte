<script lang="ts">
	import { onMount } from 'svelte';
	import { trackPageView } from '$lib/analytics';
	import { buildFaqSchema, buildHowToSchema } from '$lib/seo/structured-data';

	const PAGE_URL = 'https://snapvie.com/how-to-use-snapvie';
	const OG_IMAGE = 'https://snapvie.com/og-image.png';

	const howToSteps = [
		{
			name: 'Find the YouTube video you want to download',
			text: 'Open YouTube and navigate to the video, playlist, or Short you want to save. You can also find the video through search. The URL in your browser address bar is what you need.'
		},
		{
			name: 'Copy the video URL',
			text: 'Click the browser address bar and copy the full URL. It will look like youtube.com/watch?v=... for videos, youtube.com/shorts/... for Shorts, or youtube.com/playlist?list=... for playlists. You can also use the Share button on YouTube and copy the link from there.'
		},
		{
			name: 'Paste the URL into Snapvie',
			text: 'Go to snapvie.com and click into the input field at the top. Paste the URL you copied. Snapvie will automatically detect whether it is a video, Short, or playlist.'
		},
		{
			name: 'Choose your quality',
			text: 'Snapvie shows you all available quality options for the video — from 360p up to 4K or 8K HDR when available. Select the quality you want. Higher qualities take slightly longer because Snapvie needs to mux the video and audio streams server-side.'
		},
		{
			name: 'Download the file',
			text: 'Click the Download button. For direct downloads (up to 480p), the file saves to your device immediately. For higher qualities, Snapvie processes the mux job first — you will see a progress indicator. Once complete, click Save to download the finished file.'
		}
	];

	const faqs = [
		{
			q: 'Do I need an account to use Snapvie?',
			a: 'No account required. Paste a URL, select quality, and download. No sign-up, no email, no login.'
		},
		{
			q: 'Is Snapvie free?',
			a: 'Yes. Snapvie is completely free to use with no ads on the download experience. There are no hidden fees or premium tiers for basic downloads.'
		},
		{
			q: 'What video qualities does Snapvie support?',
			a: 'Snapvie supports 360p, 480p, 720p, 1080p, 1440p, 4K (2160p), and 8K HDR — depending on what the source video offers. Audio-only (MP3) downloads are also available.'
		},
		{
			q: 'Why does my high-quality download take longer than a low-quality one?',
			a: "Qualities above 480p require muxing — Snapvie downloads the video and audio streams separately and merges them server-side. This processing takes time proportional to the video's length and resolution. Low-quality downloads (360p, 480p) are direct and faster because they use YouTube's pre-combined stream."
		},
		{
			q: 'Can I download YouTube playlists with Snapvie?',
			a: 'Yes. Paste a playlist URL (containing playlist?list=) and Snapvie queues all videos automatically. You can track each video\'s progress in real time.'
		},
		{
			q: 'What formats does Snapvie output?',
			a: 'Video downloads are saved as MP4 for maximum compatibility. Audio-only downloads are saved as MP3 or AAC depending on the source stream.'
		}
	];

	const jsonLd = JSON.stringify({
		'@context': 'https://schema.org',
		'@graph': [
			buildHowToSchema('How to Use Snapvie to Download YouTube Videos', howToSteps),
			buildFaqSchema(faqs)
		]
	});

	onMount(() => {
		trackPageView('/how-to-use-snapvie', 'How to Use Snapvie — YouTube Downloader Tutorial');
	});
</script>

<svelte:head>
	<title>How to Use Snapvie — Complete Guide to Downloading YouTube Videos — Snapvie</title>
	<meta
		name="description"
		content="Step-by-step guide to using Snapvie: download YouTube videos, playlists, and Shorts at 1080p, 4K, or 8K HDR. No account needed. Free and ad-free."
	/>
	<link rel="canonical" href={PAGE_URL} />
	<meta property="og:type" content="article" />
	<meta property="og:site_name" content="Snapvie" />
	<meta property="og:title" content="How to Use Snapvie — Complete Guide to Downloading YouTube Videos" />
	<meta
		property="og:description"
		content="Step-by-step guide to downloading YouTube videos, playlists, and Shorts using Snapvie. Free, no account required."
	/>
	<meta property="og:url" content={PAGE_URL} />
	<meta property="og:image" content={OG_IMAGE} />
	<meta property="og:image:width" content="1200" />
	<meta property="og:image:height" content="630" />
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="How to Use Snapvie — Complete Guide to Downloading YouTube Videos" />
	<meta
		name="twitter:description"
		content="Learn how to download YouTube videos, playlists, and Shorts at full quality using Snapvie. Free, no sign-up."
	/>
	<meta name="twitter:image" content={OG_IMAGE} />
	{@html `<script type="application/ld+json">${jsonLd}<\/script>`}
</svelte:head>

<div class="content-page">
	<div class="breadcrumb">
		<a href="/">Snapvie</a> <span>›</span> <span>How to Use Snapvie</span>
	</div>

	<h1>How to Use Snapvie — Complete Guide to Downloading YouTube Videos</h1>

	<div class="quick-answer">
		<strong>Quick answer:</strong> Copy a YouTube URL, paste it into
		<a href="/">snapvie.com</a>, select your quality, and click Download. For videos above 480p,
		Snapvie processes a mux job server-side — then you save the finished file. No account, no
		software, no ads.
	</div>

	<section>
		<h2>Step-by-step guide</h2>
		<ol class="steps-list">
			{#each howToSteps as step, i}
				<li>
					<span class="step-num">{i + 1}</span>
					<div>
						<strong>{step.name}</strong>
						<p>{step.text}</p>
					</div>
				</li>
			{/each}
		</ol>
	</section>

	<section>
		<h2>Understanding quality options</h2>
		<p>
			Snapvie shows every quality tier available for the video. Here's what to expect at each level:
		</p>
		<div class="quality-table">
			<div class="q-row header">
				<span>Quality</span>
				<span>Resolution</span>
				<span>Download type</span>
			</div>
			<div class="q-row">
				<span>360p / 480p</span>
				<span>640×360 / 854×480</span>
				<span>Direct (fast)</span>
			</div>
			<div class="q-row">
				<span>720p / 1080p</span>
				<span>1280×720 / 1920×1080</span>
				<span>Muxed (medium)</span>
			</div>
			<div class="q-row">
				<span>4K / 8K HDR</span>
				<span>3840×2160 / 7680×4320</span>
				<span>Muxed (takes longer)</span>
			</div>
		</div>
		<p>
			"Muxed" means Snapvie fetches the video and audio streams separately and merges them — this
			is required for anything above 480p because YouTube stores higher qualities without embedded
			audio. See <a href="/why-youtube-downloads-need-muxing">why YouTube downloads need muxing</a>
			for the full explanation.
		</p>
	</section>

	<section>
		<h2>Downloading playlists</h2>
		<p>
			To download an entire playlist, paste the playlist URL (it contains
			<code>playlist?list=</code>) instead of an individual video URL. Snapvie detects it
			automatically and queues all videos in the playlist.
		</p>
		<p>
			Each video is processed individually through the mux pipeline. You can save videos as they
			complete without waiting for the full playlist to finish. See the full
			<a href="/how-to-download-youtube-playlists">playlist download guide</a> for details.
		</p>
	</section>

	<section>
		<h2>Downloading YouTube Shorts</h2>
		<p>
			Shorts work the same as regular videos. Paste the Short URL
			(<code>youtube.com/shorts/xxxx</code>) and Snapvie handles it like any other video. Shorts
			are typically capped at 1080p and vertical (9:16 aspect ratio).
		</p>
		<p>
			Audio is included automatically — Snapvie avoids the common issue of silent Short downloads
			by always fetching and merging the audio stream. See
			<a href="/download-youtube-shorts-with-audio">downloading Shorts with audio</a> for more.
		</p>
	</section>

	<section>
		<h2>Audio-only downloads</h2>
		<p>
			To download just the audio from a video, select the audio-only option in the quality picker.
			Snapvie extracts the audio stream and saves it as an audio file. This is the cleanest way to
			get audio from a YouTube video — no silent video file to deal with.
		</p>
		<p>
			Audio quality depends on what YouTube provides for the video — typically 128kbps or 256kbps
			AAC. Music videos and high-production content usually have the higher bitrate available.
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
		<p>Ready to download your first video?</p>
		<a href="/" class="cta-btn">Go to Snapvie — Free YouTube Downloader</a>
	</div>

	<div class="related-links">
		<strong>Related guides:</strong>
		<a href="/how-to-download-youtube-playlists">Download playlists</a>
		<span>·</span>
		<a href="/download-youtube-shorts-with-audio">Download Shorts with audio</a>
		<span>·</span>
		<a href="/why-youtube-downloads-need-muxing">What is muxing?</a>
		<span>·</span>
		<a href="/best-format-for-youtube-downloads-mp4-vs-webm">MP4 vs WebM</a>
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

	code {
		font-family: monospace;
		background: rgba(255, 77, 140, 0.08);
		padding: 0.1em 0.35em;
		border-radius: 0.3em;
		font-size: 0.88em;
		color: #c0256b;
	}

	a {
		color: #ff4d8c;
		font-weight: 600;
		text-decoration: none;
	}

	a:hover {
		text-decoration: underline;
	}

	.steps-list {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		flex-direction: column;
		gap: 0.85rem;
	}

	.steps-list li {
		display: flex;
		gap: 0.85rem;
		align-items: flex-start;
	}

	.step-num {
		flex-shrink: 0;
		width: 1.8rem;
		height: 1.8rem;
		border-radius: 50%;
		background: linear-gradient(135deg, #ff4d8c, #c026d3);
		color: #fff;
		font-weight: 800;
		font-size: 0.82rem;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-top: 0.1rem;
	}

	.steps-list strong {
		display: block;
		font-size: 0.95rem;
		font-weight: 700;
		color: #2d1b36;
		margin-bottom: 0.25rem;
	}

	.steps-list p {
		margin-bottom: 0;
	}

	.quality-table {
		border: 1px solid rgba(255, 77, 140, 0.18);
		border-radius: 0.65rem;
		overflow: hidden;
		margin-bottom: 1rem;
		font-size: 0.9rem;
	}

	.q-row {
		display: grid;
		grid-template-columns: 1.2fr 1.5fr 1.3fr;
	}

	.q-row span {
		padding: 0.55rem 0.75rem;
		border-right: 1px solid rgba(255, 77, 140, 0.1);
		border-bottom: 1px solid rgba(255, 77, 140, 0.1);
		color: #6d4f64;
		line-height: 1.5;
	}

	.q-row span:last-child {
		border-right: none;
	}

	.q-row.header span {
		font-weight: 700;
		color: #2d1b36;
		background: rgba(255, 77, 140, 0.06);
		font-size: 0.82rem;
		text-transform: uppercase;
		letter-spacing: 0.04em;
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

		.q-row {
			grid-template-columns: 1fr 1.3fr 1.2fr;
		}

		.q-row span {
			font-size: 0.8rem;
			padding: 0.4rem 0.5rem;
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
	:global(.app.theme-dark) .steps-list li {
		color: rgba(224, 208, 245, 0.85);
	}

	:global(.app.theme-dark) .steps-list strong,
	:global(.app.theme-dark) .faq-section h3 {
		color: #ffffff;
	}

	:global(.app.theme-dark) code {
		background: rgba(255, 77, 140, 0.15);
		color: #ff7caf;
	}

	:global(.app.theme-dark) .quality-table {
		border-color: rgba(255, 77, 140, 0.25);
	}

	:global(.app.theme-dark) .q-row span {
		color: rgba(224, 208, 245, 0.85);
		border-color: rgba(255, 77, 140, 0.15);
	}

	:global(.app.theme-dark) .q-row.header span {
		color: #ffffff;
		background: rgba(255, 77, 140, 0.12);
	}

	:global(.app.theme-dark) .cta-block {
		background: rgba(29, 26, 43, 0.8);
		border-color: rgba(255, 77, 140, 0.3);
	}

	:global(.app.theme-dark) .cta-block p {
		color: rgba(224, 208, 245, 0.9);
	}

	:global(.app.theme-dark) .breadcrumb,
	:global(.app.theme-dark) .related-links {
		color: rgba(200, 185, 220, 0.75);
	}

	:global(.app.theme-dark) .related-links strong {
		color: rgba(224, 208, 245, 0.9);
	}
</style>
