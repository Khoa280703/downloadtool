<script lang="ts">
	import AppIcon from '$components/AppIcon.svelte';
	import * as m from '$lib/paraglide/messages';

	type FrequentlyAskedQuestion = {
		q: string;
		a: string;
	};

	type ResourceLink = {
		href: string;
		label: string;
	};

	type ResourceGroup = {
		label: string;
		links: ResourceLink[];
		ctaHref?: string;
		ctaLabel?: string;
	};

	let {
		faqItems,
		faqTitle = m.knowledge_faq_default_title(),
		resourceTitle = m.knowledge_resource_default_title(),
		resourceLinks = [],
		resourceGroups = [],
		showHomeLink = false,
		homeLabel = m.common_back_to_snapvie()
	}: {
		faqItems: FrequentlyAskedQuestion[];
		faqTitle?: string;
		resourceTitle?: string;
		resourceLinks?: ResourceLink[];
		resourceGroups?: ResourceGroup[];
		showHomeLink?: boolean;
		homeLabel?: string;
	} = $props();
</script>

<section class="knowledge-section py-10 px-6 lg:px-20 bg-white border-t border-pink-50">
	<div class="knowledge-grid max-w-6xl mx-auto">
		<div class="knowledge-card">
			<h2 class="knowledge-title">{faqTitle}</h2>
			<div class="faq-list">
				{#each faqItems as item}
					<details class="faq-item group">
						<summary class="faq-summary">
							{item.q}
							<span class="faq-arrow">▼</span>
						</summary>
						<p class="faq-answer">{item.a}</p>
					</details>
				{/each}
			</div>
		</div>

		<div class="knowledge-card">
			<h2 class="knowledge-title">{resourceTitle}</h2>
			{#if resourceGroups.length > 0}
				<div class="resource-groups">
					{#each resourceGroups as group}
						<div class="resource-group">
							<p class="resource-group-label">{group.label}</p>
							<ul class="resource-list">
								{#each group.links as link}
									<li>
										<a href={link.href} class="resource-link">{link.label}</a>
									</li>
								{/each}
							</ul>
							{#if group.ctaHref && group.ctaLabel}
								<a href={group.ctaHref} class="resource-cta">{group.ctaLabel}</a>
							{/if}
						</div>
					{/each}
				</div>
			{:else}
				<div class="resource-links">
					{#if showHomeLink}
						<a href="/" class="resource-pill">
							<AppIcon name="home" class="text-base" />
							{homeLabel}
						</a>
					{/if}
					{#each resourceLinks as link}
						<a href={link.href} class="resource-pill">{link.label}</a>
					{/each}
				</div>
			{/if}
		</div>
	</div>
</section>

<style>
	.knowledge-grid {
		display: grid;
		gap: 1.5rem;
	}

	.knowledge-card {
		border: 1px solid rgba(251, 207, 232, 0.7);
		border-radius: 1.5rem;
		background: rgba(255, 255, 255, 0.9);
		padding: 1.5rem;
	}

	.knowledge-title {
		margin: 0 0 1rem;
		font-family: 'Fredoka', sans-serif;
		font-size: 1.125rem;
		font-weight: 700;
		color: #2d1b36;
	}

	.faq-list {
		display: grid;
	}

	.faq-item {
		padding: 0.75rem 0;
	}

	.faq-item + .faq-item {
		border-top: 1px solid rgba(251, 207, 232, 0.7);
	}

	.faq-summary {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.5rem;
		cursor: pointer;
		list-style: none;
		font-family: 'Fredoka', sans-serif;
		font-size: 0.875rem;
		font-weight: 500;
		color: #2d1b36;
	}

	.faq-summary::-webkit-details-marker {
		display: none;
	}

	.faq-arrow {
		flex-shrink: 0;
		font-size: 0.75rem;
		color: rgba(45, 27, 54, 0.4);
		transition: transform 180ms ease;
	}

	.group[open] .faq-arrow {
		transform: rotate(180deg);
	}

	.faq-answer {
		margin-top: 0.5rem;
		font-size: 0.875rem;
		line-height: 1.625;
		color: rgba(45, 27, 54, 0.7);
	}

	.resource-groups {
		display: grid;
		gap: 1rem;
	}

	.resource-group-label {
		margin: 0 0 0.6rem;
		font-size: 0.7rem;
		font-weight: 800;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		color: rgba(45, 27, 54, 0.55);
	}

	.resource-list {
		display: grid;
		gap: 0.5rem;
		padding: 0;
		margin: 0;
		list-style: none;
	}

	.resource-link,
	.resource-cta {
		font-size: 0.875rem;
		font-weight: 600;
		color: rgba(45, 27, 54, 0.84);
		text-decoration: none;
		transition: color 160ms ease;
	}

	.resource-link:hover,
	.resource-cta:hover {
		color: #ff4d8c;
	}

	.resource-cta {
		display: inline-block;
		margin-top: 0.75rem;
		font-size: 0.75rem;
		font-weight: 800;
		color: #ff4d8c;
	}

	.resource-links {
		display: flex;
		flex-wrap: wrap;
		gap: 0.75rem;
	}

	.resource-pill {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		border-radius: 999px;
		border: 1px solid rgba(251, 207, 232, 0.9);
		background: #fff;
		padding: 0.6rem 1rem;
		font-family: 'Fredoka', sans-serif;
		font-size: 0.875rem;
		font-weight: 600;
		color: #2d1b36;
		text-decoration: none;
		transition: border-color 160ms ease, color 160ms ease;
	}

	.resource-pill:hover {
		border-color: #ff4d8c;
		color: #ff4d8c;
	}

	@media (min-width: 1024px) {
		.knowledge-grid {
			grid-template-columns: minmax(0, 1.15fr) minmax(0, 0.85fr);
			align-items: start;
		}
	}

	@media (min-width: 640px) {
		.resource-groups {
			grid-template-columns: repeat(2, minmax(0, 1fr));
			align-items: start;
		}
	}

	:global(.page-root.theme-dark) .knowledge-section {
		background-color: rgba(18, 18, 26, 0.88);
		border-color: rgba(255, 77, 140, 0.16);
	}

	:global(.page-root.theme-dark) .knowledge-card {
		background: rgba(255, 255, 255, 0.04);
		border-color: rgba(255, 77, 140, 0.16);
	}

	:global(.page-root.theme-dark) .knowledge-title,
	:global(.page-root.theme-dark) .faq-summary,
	:global(.page-root.theme-dark) .resource-link,
	:global(.page-root.theme-dark) .resource-pill {
		color: #ffffff;
	}

	:global(.page-root.theme-dark) .faq-answer {
		color: rgba(224, 208, 245, 0.74);
	}

	:global(.page-root.theme-dark) .faq-item + .faq-item {
		border-top-color: rgba(255, 77, 140, 0.14);
	}

	:global(.page-root.theme-dark) .faq-arrow,
	:global(.page-root.theme-dark) .resource-group-label {
		color: rgba(224, 208, 245, 0.42);
	}

	:global(.page-root.theme-dark) .resource-pill {
		background-color: rgba(255, 255, 255, 0.04);
		border-color: rgba(255, 77, 140, 0.16);
	}

	:global(.page-root.theme-dark) .resource-link:hover,
	:global(.page-root.theme-dark) .resource-cta:hover,
	:global(.page-root.theme-dark) .resource-pill:hover {
		color: #ff8fb7;
		border-color: rgba(255, 77, 140, 0.42);
	}
	</style>
