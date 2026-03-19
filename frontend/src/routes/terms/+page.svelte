<script lang="ts">
	import { trackPageView } from '$lib/analytics';
	import { onMount } from 'svelte';

	onMount(() => {
		trackPageView('/terms', 'Terms of Service — Snapvie');
	});
</script>

<svelte:head>
	<title>Terms of Service — Snapvie</title>
	<meta
		name="description"
		content="Snapvie Terms of Service: acceptable use, copyright compliance, and user responsibilities for our free YouTube video downloader."
	/>
	<link rel="canonical" href="https://snapvie.com/terms" />
</svelte:head>

<div class="legal-page">
	<h1>Terms of Service</h1>
	<p class="tagline">Last updated: March 19, 2026</p>

	<section>
		<h2>1. Acceptance of Terms</h2>
		<p>
			By using Snapvie ("the Service"), you agree to these Terms of Service. If you do not agree,
			please do not use the Service. These terms apply to all users of snapvie.com.
		</p>
	</section>

	<section>
		<h2>2. Acceptable Use</h2>
		<p>Snapvie is provided for personal and educational use only. You may use the Service to:</p>
		<ul>
			<li>Download videos for offline personal viewing</li>
			<li>Save content for educational or research purposes</li>
			<li>Extract audio for personal listening</li>
		</ul>
		<p>You may <strong>not</strong> use Snapvie to:</p>
		<ul>
			<li>Commercially redistribute, sell, or monetize downloaded content</li>
			<li>Upload downloaded content to other platforms without rights to do so</li>
			<li>Scrape or automate requests to our service in ways that harm availability</li>
			<li>Circumvent technical measures in a way that violates applicable law</li>
		</ul>
	</section>

	<section>
		<h2>3. Copyright Compliance</h2>
		<p>
			You are solely responsible for ensuring that your use of downloaded content complies with
			applicable copyright law and the Terms of Service of the source platform. Snapvie does not
			host, store, or control any video content — it facilitates access to publicly available
			streams.
		</p>
		<p>
			Downloading copyrighted material without authorization from the rights holder may violate
			applicable law. Use the Service only for content you have rights to download.
		</p>
	</section>

	<section>
		<h2>4. Service Provided As-Is</h2>
		<p>
			Snapvie is provided "as-is" without warranties of any kind, express or implied. We do not
			guarantee that the Service will be uninterrupted, error-free, or that any specific video will
			be downloadable at any given time.
		</p>
		<p>
			We reserve the right to modify, suspend, or discontinue any part of the Service at any time
			without notice.
		</p>
	</section>

	<section>
		<h2>5. User Responsibility</h2>
		<p>
			You use the Service at your own risk. Snapvie and its operators are not liable for any damages
			arising from your use or inability to use the Service, or from any content you download using
			it.
		</p>
	</section>

	<section>
		<h2>6. Changes to These Terms</h2>
		<p>
			We may update these Terms from time to time. Changes will be posted on this page with an
			updated date. Continued use of the Service after changes constitutes acceptance of the revised
			Terms.
		</p>
	</section>

	<section>
		<h2>7. Contact</h2>
		<p>
			Questions about these Terms? Email us at <a href="mailto:hello@snapvie.com"
				>hello@snapvie.com</a
			>
			or see our <a href="/contact">contact page</a>.
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
