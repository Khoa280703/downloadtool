<script lang="ts">
	import { trackPageView } from '$lib/analytics';
	import { onMount } from 'svelte';
	import { buildOrganizationSchema, buildBreadcrumbSchema } from '$lib/seo/structured-data';
	import { SITE_URL } from '$lib/seo/public-pages';
	import * as m from '$lib/paraglide/messages';

	const jsonLd = JSON.stringify(
		{
			'@context': 'https://schema.org',
			'@graph': [
				buildOrganizationSchema(),
				buildBreadcrumbSchema([
					{ name: 'Snapvie', url: SITE_URL },
					{ name: m.about_breadcrumb(), url: `${SITE_URL}/about` }
				])
			]
		},
		null,
		0
	);

	onMount(() => {
		trackPageView('/about', m.about_og_title());
	});
</script>

<svelte:head>
	<title>{m.about_og_title()}</title>
	<meta
		name="description"
		content={m.about_meta_description()}
	/>
	<link rel="canonical" href="https://snapvie.com/about" />
	<meta property="og:title" content={m.about_og_title()} />
	<meta
		property="og:description"
		content={m.about_meta_description()}
	/>
	<meta property="og:url" content="https://snapvie.com/about" />
	<meta property="og:type" content="website" />
	<meta name="twitter:card" content="summary_large_image" />
	{@html '<script type="application/ld+json">' + jsonLd + '<\/script>'}
</svelte:head>

<div class="legal-page">
	<h1>{m.about_title()}</h1>
	<p class="tagline">{m.about_tagline()}</p>

	<section>
		<h2>{m.about_what_is_title()}</h2>
		<p>{m.about_what_is_p1()}</p>
		<p>{m.about_what_is_p2()}</p>
	</section>

	<section>
		<h2>{m.about_supports_title()}</h2>
		<ul>
			<li><strong>{m.about_supports_4k()}</strong> — {m.about_supports_4k_desc()}</li>
			<li><strong>{m.about_supports_playlists()}</strong> — {m.about_supports_playlists_desc()}</li>
			<li><strong>{m.about_supports_shorts()}</strong> — {m.about_supports_shorts_desc()}</li>
			<li><strong>{m.about_supports_audio()}</strong> — {m.about_supports_audio_desc()}</li>
			<li><strong>{m.about_supports_no_account()}</strong> — {m.about_supports_no_account_desc()}</li>
		</ul>
	</section>

	<section>
		<h2>{m.about_how_works_title()}</h2>
		<p>{m.about_how_works_p1()}</p>
		<p>{m.about_how_works_p2()}</p>
	</section>

	<section>
		<h2>{m.about_built_quality_title()}</h2>
		<p>{m.about_built_quality_p1()}</p>
		<ul>
			<li><strong>{m.about_built_quality_no_caps()}</strong> — {m.about_built_quality_no_caps_desc()}</li>
			<li><strong>{m.about_built_quality_realtime()}</strong> — {m.about_built_quality_realtime_desc()}</li>
			<li><strong>{m.about_built_quality_no_ads()}</strong> — {m.about_built_quality_no_ads_desc()}</li>
			<li><strong>{m.about_built_quality_any_browser()}</strong> — {m.about_built_quality_any_browser_desc()}</li>
		</ul>
	</section>

	<section>
		<h2>{m.about_mission_title()}</h2>
		<p>{m.about_mission_p1()}</p>
		<p>{m.about_mission_disclaimer()}</p>
	</section>

	<section>
		<h2>{m.about_privacy_title()}</h2>
		<p>{m.about_privacy_p1()}</p>
	</section>

	<section class="internal-links">
		<h2>{m.about_learn_more_title()}</h2>
		<ul>
			<li><a href="/guides">{m.about_learn_more_guides()}</a> — {m.about_learn_more_guides_desc()}</li>
			<li>
				<a href="/compare">{m.about_learn_more_compare()}</a> — {m.about_learn_more_compare_desc()}
			</li>
			<li><a href="/contact">{m.about_learn_more_contact()}</a> — {m.about_learn_more_contact_desc()}</li>
		</ul>
	</section>

	<div class="back-link">
		<a href="/">{m.common_back_to_snapvie()}</a>
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
