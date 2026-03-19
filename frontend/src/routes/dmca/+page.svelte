<script lang="ts">
	import { trackPageView } from '$lib/analytics';
	import { onMount } from 'svelte';

	onMount(() => {
		trackPageView('/dmca', 'DMCA & Copyright — Snapvie');
	});
</script>

<svelte:head>
	<title>DMCA &amp; Copyright Policy — Snapvie</title>
	<meta
		name="description"
		content="Snapvie DMCA takedown procedure, counter-notification process, and copyright contact information."
	/>
	<link rel="canonical" href="https://snapvie.com/dmca" />
</svelte:head>

<div class="legal-page">
	<h1>DMCA &amp; Copyright</h1>
	<p class="tagline">Last updated: March 19, 2026</p>

	<section>
		<h2>Overview</h2>
		<p>
			Snapvie respects intellectual property rights and complies with the Digital Millennium
			Copyright Act (DMCA). We do not host or store video content — we provide a tool that accesses
			publicly available streams from third-party platforms.
		</p>
		<p>
			If you believe content accessible via Snapvie infringes your copyright, you may submit a
			takedown notice using the procedure below.
		</p>
	</section>

	<section>
		<h2>DMCA Takedown Notice</h2>
		<p>
			To submit a valid DMCA notice, email <a href="mailto:dmca@snapvie.com">dmca@snapvie.com</a> with
			the following information:
		</p>
		<ul>
			<li>
				<strong>Identification of the copyrighted work</strong> — describe the work you claim has been
				infringed
			</li>
			<li>
				<strong>Identification of the infringing material</strong> — the specific URL(s) being used with
				Snapvie
			</li>
			<li>
				<strong>Your contact information</strong> — name, address, phone number, and email address
			</li>
			<li>
				<strong>Good faith statement</strong> — a statement that you believe in good faith the use is
				not authorized by the copyright owner
			</li>
			<li>
				<strong>Accuracy statement</strong> — a statement that the information in the notice is
				accurate, and under penalty of perjury, that you are authorized to act on behalf of the
				copyright owner
			</li>
			<li>
				<strong>Signature</strong> — your physical or electronic signature
			</li>
		</ul>
		<p>
			We will review valid notices and take appropriate action, which may include restricting access
			to the identified content.
		</p>
	</section>

	<section>
		<h2>Counter-Notification Procedure</h2>
		<p>
			If you believe your content was wrongly restricted, you may submit a counter-notification to
			<a href="mailto:dmca@snapvie.com">dmca@snapvie.com</a> including:
		</p>
		<ul>
			<li>Identification of the material that was restricted and its prior location</li>
			<li>
				A statement under penalty of perjury that you have a good faith belief the content was
				removed by mistake or misidentification
			</li>
			<li>Your name, address, phone number, and consent to jurisdiction</li>
			<li>Your physical or electronic signature</li>
		</ul>
		<p>
			Upon receiving a valid counter-notification, we will forward it to the original complainant.
			If no legal action is filed within 10 business days, the restriction may be lifted at our
			discretion.
		</p>
	</section>

	<section>
		<h2>Repeat Infringers</h2>
		<p>
			Snapvie reserves the right to terminate access to users who are repeat infringers of
			third-party intellectual property rights, in appropriate circumstances.
		</p>
	</section>

	<section>
		<h2>Contact</h2>
		<p>
			DMCA agent: <a href="mailto:dmca@snapvie.com">dmca@snapvie.com</a>
		</p>
		<p>
			For general inquiries, see our <a href="/contact">contact page</a>.
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
