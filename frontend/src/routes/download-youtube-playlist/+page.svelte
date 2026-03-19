<script lang="ts">
	import SeoLandingPageShell from '$components/seo-landing-page-shell.svelte';
	import { buildLandingPageJsonLd } from '$lib/seo/structured-data';
	import { LANDING_PAGES } from '$lib/seo/landing-page-config';

	const config = LANDING_PAGES.find((p) => p.slug === 'download-youtube-playlist')!;
	const SEO_ORIGIN = 'https://snapvie.com';
	const pageUrl = `${SEO_ORIGIN}/${config.slug}`;
	const ogImage = `${SEO_ORIGIN}/og-image.png`;
	const jsonLd = buildLandingPageJsonLd(config.h1, pageUrl, [
		{ name: 'Snapvie', url: SEO_ORIGIN },
		{ name: config.h1, url: pageUrl }
	], config.faqItems);
</script>

<svelte:head>
	<title>{config.title}</title>
	<meta name="description" content={config.metaDescription} />
	<link rel="canonical" href={pageUrl} />
	<meta property="og:type" content="website" />
	<meta property="og:site_name" content="Snapvie" />
	<meta property="og:title" content={config.title} />
	<meta property="og:description" content={config.metaDescription} />
	<meta property="og:url" content={pageUrl} />
	<meta property="og:image" content={ogImage} />
	<meta property="og:image:width" content="1200" />
	<meta property="og:image:height" content="630" />
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content={config.title} />
	<meta name="twitter:description" content={config.metaDescription} />
	<meta name="twitter:image" content={ogImage} />
	{@html `<script type="application/ld+json">${jsonLd}<\/script>`}
</svelte:head>

<SeoLandingPageShell {config} />
