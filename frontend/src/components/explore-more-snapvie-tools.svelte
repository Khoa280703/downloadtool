<script lang="ts">
	import AppIcon from '$components/AppIcon.svelte';

	type ExploreLink = {
		href: string;
		label: string;
	};

	let {
		links,
		showHomeLink = false,
		title = 'Explore More Snapvie Tools'
	}: {
		links: ExploreLink[];
		showHomeLink?: boolean;
		title?: string;
	} = $props();
</script>

<section class="explore-tools-section py-8 px-6 lg:px-20 bg-slate-50 border-t border-pink-50">
	<div class="max-w-3xl mx-auto text-center">
		<h2 class="explore-tools-title text-lg font-bold text-plum mb-5">{title}</h2>
		<div class="flex flex-wrap justify-center gap-3">
			{#if showHomeLink}
				<a
					href="/"
					class="explore-tools-link inline-flex items-center gap-2 rounded-full border border-pink-100 bg-white px-5 py-2 text-sm font-bold text-plum hover:text-primary hover:border-primary transition-colors"
				>
					<AppIcon name="home" class="text-base" />
					Back to Snapvie
				</a>
			{/if}
			{#each links as link}
				<a
					href={link.href}
					class="explore-tools-link inline-flex items-center gap-2 rounded-full border border-pink-100 bg-white px-5 py-2 text-sm font-bold text-plum hover:text-primary hover:border-primary transition-colors"
				>
					{link.label}
				</a>
			{/each}
		</div>
	</div>
</section>

<style>
	:global(.page-root.theme-dark) .explore-tools-section {
		background-color: rgba(18, 18, 26, 0.88);
		border-color: rgba(255, 77, 140, 0.16);
	}

	:global(.page-root.theme-dark) .explore-tools-title {
		color: #ffffff;
	}

	:global(.page-root.theme-dark) .explore-tools-link {
		background-color: rgba(255, 255, 255, 0.04);
		border-color: rgba(255, 77, 140, 0.16);
		color: #ffffff;
	}

	:global(.page-root.theme-dark) .explore-tools-link:hover {
		border-color: rgba(255, 77, 140, 0.42);
		color: #ff8fb7;
	}
</style>
