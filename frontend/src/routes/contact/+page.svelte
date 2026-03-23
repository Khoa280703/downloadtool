<script lang="ts">
	import { trackPageView } from '$lib/analytics';
	import { onMount } from 'svelte';
	import { buildOrganizationSchema, buildBreadcrumbSchema } from '$lib/seo/structured-data';
	import { SITE_URL } from '$lib/seo/public-pages';

	const jsonLd = JSON.stringify(
		{
			'@context': 'https://schema.org',
			'@graph': [
				buildOrganizationSchema(),
				buildBreadcrumbSchema([
					{ name: 'Snapvie', url: SITE_URL },
					{ name: 'Contact', url: `${SITE_URL}/contact` }
				])
			]
		},
		null,
		0
	);

	onMount(() => {
		trackPageView('/contact', 'Contact Snapvie');
	});
</script>

<svelte:head>
	<title>Contact Snapvie — Get Help or Report Issues</title>
	<meta
		name="description"
		content="Contact Snapvie for support, to report a bug, submit a DMCA takedown, or send general feedback."
	/>
	<link rel="canonical" href="https://snapvie.com/contact" />
	<meta property="og:title" content="Contact Snapvie — Get Help or Report Issues" />
	<meta
		property="og:description"
		content="Contact Snapvie for support, bug reports, DMCA takedown requests, or general feedback."
	/>
	<meta property="og:url" content="https://snapvie.com/contact" />
	<meta property="og:type" content="website" />
	<meta name="twitter:card" content="summary_large_image" />
	{@html '<script type="application/ld+json">' + jsonLd + '<\/script>'}
</svelte:head>

<div class="legal-page">
	<h1>Contact Us</h1>
	<p class="tagline">We read every message.</p>

	<section>
		<h2>General Support</h2>
		<p>
			Having trouble downloading a video? Something not working as expected? Reach out and we'll do
			our best to help.
		</p>
		<p>
			Email us at: <a href="mailto:support@snapvie.com">support@snapvie.com</a>
		</p>
		<p>
			Please include the video URL and a brief description of what happened. We typically respond
			within 1–2 business days.
		</p>
	</section>

	<section>
		<h2>Bug Reports</h2>
		<p>
			Found a bug or broken feature? We want to know. When reporting, please include:
		</p>
		<ul>
			<li>The video URL you were trying to download</li>
			<li>What happened (error message, blank result, etc.)</li>
			<li>Your browser and operating system</li>
		</ul>
		<p>
			Send bug reports to: <a href="mailto:support@snapvie.com">support@snapvie.com</a> with subject
			line <strong>"Bug Report"</strong>.
		</p>
	</section>

	<section>
		<h2>DMCA &amp; Copyright</h2>
		<p>
			If you are a copyright holder and believe content facilitated by Snapvie infringes your rights,
			please see our <a href="/dmca">DMCA &amp; Copyright Policy</a> for the proper takedown procedure.
		</p>
		<p>
			DMCA notices must be submitted via email to: <a href="mailto:dmca@snapvie.com"
				>dmca@snapvie.com</a
			>
		</p>
	</section>

	<section>
		<h2>Other Inquiries</h2>
		<p>
			For anything else — partnerships, press, or general feedback — email us at <a
				href="mailto:hello@snapvie.com">hello@snapvie.com</a
			>.
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

	:global(.app.theme-dark) .legal-page a {
		color: #ff7caf;
	}
</style>
