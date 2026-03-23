<script lang="ts">
	/**
	 * Compare detail page — renders a single compare ContentEntry with:
	 * quick answer box, content sections, FAQ, related entries, CTA.
	 */
	import type { PageData } from './$types';
	import { buildContentPageMeta } from '$lib/seo/content/build-page-seo';
	import { buildContentPageJsonLd } from '$lib/seo/content/build-page-schema';
	import { getRelatedContent } from '$lib/seo/content/related-content';
	import { trackSeoPageView } from '$lib/analytics/seo-page-events';
	import FrequentlyAskedQuestionsSection from '$components/frequently-asked-questions-section.svelte';
	import ExploreMoreSnapvieTools from '$components/explore-more-snapvie-tools.svelte';
	import { onMount } from 'svelte';

	let { data }: { data: PageData } = $props();

	const meta = buildContentPageMeta(data.entry);
	const jsonLd = buildContentPageJsonLd(data.entry);
	const related = getRelatedContent(data.entry.slug, 3);

	onMount(() =>
		trackSeoPageView({
			page_group: 'compare',
			page_slug: data.entry.slug,
			cluster: data.entry.category,
			locale: data.entry.locale
		})
	);
</script>

<svelte:head>
	<title>{meta.title}</title>
	<meta name="description" content={meta.description} />
	<link rel="canonical" href={meta.canonical} />
	<meta property="og:title" content={meta.og.title} />
	<meta property="og:description" content={meta.og.description} />
	<meta property="og:url" content={meta.og.url} />
	<meta property="og:type" content={meta.og.type} />
	<meta property="og:image" content={meta.og.image} />
	<meta name="twitter:card" content={meta.twitter.card} />
	<meta name="twitter:title" content={meta.twitter.title} />
	<meta name="twitter:description" content={meta.twitter.description} />
	<!-- eslint-disable-next-line svelte/no-at-html-tags -->
	{@html `<script type="application/ld+json">${jsonLd}<\/script>`}
</svelte:head>

<article class="max-w-3xl mx-auto px-6 pt-10 pb-16">
	<!-- Breadcrumb -->
	<nav aria-label="Breadcrumb" class="mb-6 flex gap-2 text-xs font-semibold text-plum/50">
		<a href="/" class="hover:text-primary transition-colors">Snapvie</a>
		<span>›</span>
		<a href="/compare" class="hover:text-primary transition-colors">Compare</a>
		<span>›</span>
		<span class="text-plum/70 line-clamp-1">{data.entry.h1}</span>
	</nav>

	<!-- H1 -->
	<h1 class="text-3xl md:text-4xl font-bold text-plum leading-tight mb-3" style="font-family:'Fredoka',sans-serif">
		{data.entry.h1}
	</h1>

	{#if data.entry.subtitle}
		<p class="text-lg text-plum/70 font-semibold mb-6">{data.entry.subtitle}</p>
	{/if}

	<!-- Dates -->
	<p class="text-xs text-plum/40 mb-8">
		Published {data.entry.datePublished}
		{#if data.entry.dateModified !== data.entry.datePublished}
			· Updated {data.entry.dateModified}
		{/if}
	</p>

	<!-- Quick answer box -->
	<div class="quick-answer rounded-2xl border border-primary/20 bg-pink-50 p-5 mb-10">
		<p class="text-xs font-bold text-primary uppercase tracking-wider mb-2">Quick Summary</p>
		<p class="text-sm font-semibold text-plum leading-relaxed">{data.entry.quickAnswer}</p>
	</div>

	<!-- Content sections -->
	{#each data.entry.sections as section (section.heading)}
		<section class="content-section mb-8">
			<h2 class="text-xl font-bold text-plum mb-3" style="font-family:'Fredoka',sans-serif">
				{section.heading}
			</h2>
			<div class="prose-content text-sm text-plum/80 leading-relaxed">
				<!-- eslint-disable-next-line svelte/no-at-html-tags -->
				{@html section.content}
			</div>
		</section>
	{/each}

	<!-- Related comparisons -->
	{#if related.length > 0}
		<section class="mt-10 pt-8 border-t border-pink-100">
			<h2 class="text-lg font-bold text-plum mb-4" style="font-family:'Fredoka',sans-serif">Related Comparisons</h2>
			<div class="grid gap-3 sm:grid-cols-2">
				{#each related as rel (rel.slug)}
					<a
						href="/compare/{rel.slug}"
						class="block rounded-xl border border-pink-100 bg-white p-4 text-sm font-semibold text-plum hover:text-primary hover:border-primary/30 transition-colors leading-snug"
					>
						{rel.h1}
					</a>
				{/each}
			</div>
		</section>
	{/if}

	<!-- CTA to money page -->
	{#if data.entry.relatedMoneyPage}
		<section class="mt-10 rounded-2xl bg-gradient-to-r from-primary/10 to-secondary/10 border border-primary/20 p-6 text-center">
			<p class="text-sm font-bold text-plum mb-3">See for yourself — try Snapvie free.</p>
			<a
				href="/{data.entry.relatedMoneyPage}"
				class="inline-block rounded-full bg-gradient-to-r from-primary to-secondary px-6 py-2.5 text-sm font-bold text-white shadow-sm hover:brightness-110 transition-all"
				style="font-family:'Fredoka',sans-serif"
			>
				Try Snapvie Free
			</a>
		</section>
	{/if}
</article>

<!-- FAQ accordion -->
{#if data.entry.faqItems && data.entry.faqItems.length > 0}
	<FrequentlyAskedQuestionsSection items={data.entry.faqItems} />
{/if}

<!-- Cross-links -->
<ExploreMoreSnapvieTools
	showHomeLink={true}
	title="More from Snapvie"
	links={related.map((r) => ({ href: `/compare/${r.slug}`, label: r.h1 }))}
/>

<style>
	.prose-content :global(ol) {
		list-style: decimal;
		padding-left: 1.25rem;
		display: grid;
		gap: 0.5rem;
	}
	.prose-content :global(ul) {
		list-style: disc;
		padding-left: 1.25rem;
		display: grid;
		gap: 0.25rem;
	}
	.prose-content :global(strong) { font-weight: 700; }
	.prose-content :global(table) {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.85em;
	}
	.prose-content :global(th),
	.prose-content :global(td) {
		border: 1px solid rgba(255, 77, 140, 0.15);
		padding: 0.4rem 0.6rem;
		text-align: left;
	}
	.prose-content :global(th) { font-weight: 700; background: rgba(255, 77, 140, 0.05); }
	.prose-content :global(code) {
		background: rgba(255, 77, 140, 0.08);
		border-radius: 4px;
		padding: 0.1em 0.35em;
		font-size: 0.85em;
	}
	.prose-content :global(p + p) { margin-top: 0.75rem; }

	:global(.page-root.theme-dark) .quick-answer {
		background-color: rgba(255, 77, 140, 0.08);
		border-color: rgba(255, 77, 140, 0.2);
	}
</style>
