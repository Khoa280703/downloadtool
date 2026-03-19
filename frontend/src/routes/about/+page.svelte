<script lang="ts">
	import { trackPageView } from '$lib/analytics';
	import { onMount } from 'svelte';

	onMount(() => {
		trackPageView('/about', 'About Snapvie — Free 4K YouTube Video Downloader');
	});
</script>

<svelte:head>
	<title>About Snapvie — Free 4K YouTube Video Downloader</title>
	<meta
		name="description"
		content="Learn about Snapvie — a free, ad-free YouTube video downloader supporting 4K, 8K HDR, playlists, Shorts, and audio-only downloads."
	/>
	<link rel="canonical" href="https://snapvie.com/about" />
</svelte:head>

<div class="legal-page">
	<h1>About Snapvie</h1>
	<p class="tagline">Free, fast, and genuinely ad-free.</p>

	<section>
		<h2>What is Snapvie?</h2>
		<p>
			Snapvie is a free YouTube video downloader that lets you save videos in the highest available
			quality — including 4K, 8K HDR, and HDR10+. No account required, no ads, no watermarks.
		</p>
		<p>
			We built Snapvie because most downloader tools are cluttered with ads, require sign-ups, or cap
			quality at 1080p. Snapvie strips all of that away and focuses on one thing: getting you the
			best possible file from the video you want.
		</p>
	</section>

	<section>
		<h2>What Snapvie Supports</h2>
		<ul>
			<li><strong>4K &amp; 8K HDR</strong> — download in full original quality, no compression</li>
			<li><strong>Playlists</strong> — batch download entire YouTube playlists at once</li>
			<li><strong>YouTube Shorts</strong> — works just like regular videos</li>
			<li><strong>Audio-only</strong> — extract audio as a clean file without video</li>
			<li><strong>No account required</strong> — paste a URL and download, that's it</li>
		</ul>
	</section>

	<section>
		<h2>Our Mission</h2>
		<p>
			To provide safe, ad-free, high-quality video downloads for personal and educational use. We
			believe accessing content you're entitled to download shouldn't require installing sketchy
			software or sitting through five ad popups.
		</p>
		<p>
			Snapvie is provided as a tool for lawful personal use only. We ask all users to respect
			copyright and YouTube's Terms of Service.
		</p>
	</section>

	<section>
		<h2>Privacy &amp; No Tracking</h2>
		<p>
			We don't sell your data. We don't track what videos you download. Video URLs you submit are
			processed to fetch stream info and then discarded. See our <a href="/privacy">Privacy Policy</a
			> for full details.
		</p>
	</section>

	<div class="back-link">
		<a href="/">← Back to Snapvie</a>
	</div>
</div>

<style>
	.legal-page {
		max-width: 980px;
		margin: 0 auto;
		padding: 0.25rem 0.25rem 1.6rem;
	}

	.legal-page h1 {
		font-size: clamp(1.95rem, 4vw, 2.6rem);
		font-weight: 800;
		color: #2d1b36;
		letter-spacing: -0.02em;
		margin-bottom: 0.5rem;
	}

	.tagline {
		display: inline-flex;
		align-items: center;
		border-radius: 999px;
		border: 1px solid rgba(255, 77, 140, 0.2);
		background: rgba(255, 255, 255, 0.8);
		padding: 0.35rem 0.8rem;
		font-size: 0.83rem;
		font-weight: 700;
		color: #7b3f61;
		margin-bottom: 1.4rem;
	}

	.legal-page section {
		margin-bottom: 0.95rem;
		padding: 1rem 1.1rem;
		border-radius: 1.05rem;
		border: 1px solid rgba(255, 77, 140, 0.12);
		background: rgba(255, 255, 255, 0.72);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		box-shadow: 0 20px 28px -26px rgba(255, 77, 140, 0.45);
	}

	.legal-page h2 {
		font-size: 1.14rem;
		font-weight: 800;
		color: #2d1b36;
		margin-bottom: 1rem;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid rgba(255, 77, 140, 0.14);
	}

	.legal-page p {
		font-size: 0.9375rem;
		line-height: 1.7;
		color: #6d4f64;
		margin-bottom: 1rem;
	}

	.legal-page ul {
		list-style: disc;
		padding-left: 1.5rem;
		margin-bottom: 1rem;
	}

	.legal-page li {
		font-size: 0.9375rem;
		line-height: 1.7;
		color: #6d4f64;
		margin-bottom: 0.5rem;
	}

	.legal-page li strong {
		color: #2d1b36;
	}

	.legal-page a {
		color: #ff4d8c;
		font-weight: 700;
		text-decoration: none;
	}

	.legal-page a:hover {
		text-decoration: underline;
	}

	.back-link {
		margin-top: 1.5rem;
		font-size: 0.9rem;
		font-weight: 700;
	}

	@media (max-width: 640px) {
		.legal-page {
			padding: 0.15rem 0 1rem;
		}

		.legal-page h1 {
			font-size: 1.5rem;
		}

		.legal-page h2 {
			font-size: 1.125rem;
		}
	}

	:global(.app.theme-dark) .legal-page h1 {
		color: #ffffff;
	}

	:global(.app.theme-dark) .tagline {
		border-color: rgba(255, 77, 140, 0.3);
		background: rgba(29, 26, 43, 0.86);
		color: rgba(255, 218, 233, 0.92);
	}

	:global(.app.theme-dark) .legal-page section {
		border-color: rgba(255, 77, 140, 0.2);
		background: rgba(28, 27, 40, 0.72);
		box-shadow: 0 16px 24px -22px rgba(255, 77, 140, 0.42);
	}

	:global(.app.theme-dark) .legal-page h2 {
		color: #ffffff;
		border-bottom-color: rgba(255, 77, 140, 0.22);
	}

	:global(.app.theme-dark) .legal-page p,
	:global(.app.theme-dark) .legal-page li {
		color: rgba(224, 208, 245, 0.85);
	}

	:global(.app.theme-dark) .legal-page li strong {
		color: #ffffff;
	}

	:global(.app.theme-dark) .legal-page a {
		color: #ff7caf;
	}
</style>
