<script lang="ts">
	/**
	 * Compare hub page — lists all compare entries with category filter chips.
	 * SEO: title, meta, canonical, OG, JSON-LD ItemList + BreadcrumbList.
	 */
	import type { PageData } from './$types';
	import type { ContentCategory } from '$lib/seo/content/content-types';
	import { CATEGORY_CONFIG } from '$lib/seo/content/content-taxonomy';
	import { buildHubPageMeta } from '$lib/seo/content/build-page-seo';
	import { buildHubPageJsonLd } from '$lib/seo/content/build-page-schema';
	import * as m from '$lib/paraglide/messages';

	let { data }: { data: PageData } = $props();

	const meta = buildHubPageMeta(data.config);
	const jsonLd = buildHubPageJsonLd(data.config, data.entries);

	// Only show categories that have at least one entry
	const activeCategories = [...new Set(data.entries.map((e) => e.category))] as ContentCategory[];

	let selectedCategory = $state<ContentCategory | null>(null);

	const filtered = $derived(
		selectedCategory ? data.entries.filter((e) => e.category === selectedCategory) : data.entries
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

<!-- Hero -->
<section class="pt-12 pb-8 px-6 text-center">
	<nav aria-label="Breadcrumb" class="mb-4 flex justify-center gap-2 text-xs font-semibold text-plum/50">
		<a href="/" class="hover:text-primary transition-colors">{m.common_breadcrumb_snapvie()}</a>
		<span>›</span>
		<span class="text-plum/70">{m.compare_breadcrumb()}</span>
	</nav>
	<h1 class="text-3xl md:text-5xl font-bold text-plum mb-4 leading-tight" style="font-family:'Fredoka',sans-serif">
		{data.config.h1}
	</h1>
	<p class="text-lg text-plum/70 max-w-xl mx-auto font-semibold">{data.config.subtitle}</p>
</section>

<!-- Category filter chips -->
{#if activeCategories.length > 0}
	<section class="px-6 pb-6">
		<div class="max-w-4xl mx-auto flex flex-wrap justify-center gap-2">
			<button
				class="rounded-full border px-4 py-1.5 text-sm font-semibold transition-colors {selectedCategory === null ? 'bg-primary text-white border-primary' : 'bg-white border-pink-100 text-plum hover:border-primary hover:text-primary'}"
				onclick={() => (selectedCategory = null)}
			>
				{m.compare_filter_all()}
			</button>
			{#each activeCategories as cat}
				<button
					class="rounded-full border px-4 py-1.5 text-sm font-semibold transition-colors {selectedCategory === cat ? 'bg-primary text-white border-primary' : 'bg-white border-pink-100 text-plum hover:border-primary hover:text-primary'}"
					onclick={() => (selectedCategory = selectedCategory === cat ? null : cat)}
				>
					{CATEGORY_CONFIG[cat].label}
				</button>
			{/each}
		</div>
	</section>
{/if}

<!-- Compare cards grid -->
<section class="px-6 pb-16">
	<div class="max-w-4xl mx-auto">
		{#if filtered.length === 0}
			<p class="text-center text-plum/50 py-16 text-sm font-semibold">{m.compare_empty_state()}</p>
		{:else}
			<div class="grid gap-4 sm:grid-cols-2">
				{#each filtered as entry (entry.slug)}
					<a
						href="/compare/{entry.slug}"
						class="block rounded-2xl border border-pink-100 bg-white p-5 shadow-sm hover:shadow-md hover:border-primary/30 transition-all group"
					>
						<div class="flex items-center gap-2 mb-2">
							<span class="rounded-full bg-pink-50 px-3 py-0.5 text-xs font-semibold text-primary">
								{CATEGORY_CONFIG[entry.category].label}
							</span>
							<span class="text-xs text-plum/40">{entry.dateModified}</span>
						</div>
						<h2 class="text-base font-bold text-plum leading-snug mb-2 group-hover:text-primary transition-colors" style="font-family:'Fredoka',sans-serif">
							{entry.h1}
						</h2>
						<p class="text-sm text-plum/60 leading-relaxed line-clamp-2">{entry.quickAnswer}</p>
					</a>
				{/each}
			</div>
		{/if}
	</div>
</section>

<style>
	:global(.page-root.theme-dark) h1 {
		color: #ffffff;
	}
</style>
