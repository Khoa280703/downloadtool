<script lang="ts">
	type FrequentlyAskedQuestion = {
		q: string;
		a: string;
	};

	let {
		items,
		title = 'Frequently Asked Questions'
	}: {
		items: FrequentlyAskedQuestion[];
		title?: string;
	} = $props();
</script>

<section class="faq-section py-10 px-6 lg:px-20 bg-white border-t border-pink-50">
	<div class="max-w-3xl mx-auto">
		<h2 class="faq-title text-2xl font-bold text-plum mb-6 text-center">{title}</h2>
		<div class="faq-list">
			{#each items as item}
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
</section>

<style>
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

	.faq-title {
		font-family: 'Fredoka', sans-serif;
		font-weight: 700;
	}

	.faq-summary::-webkit-details-marker {
		display: none;
	}

	.faq-arrow {
		flex-shrink: 0;
		user-select: none;
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

	:global(.page-root.theme-dark) .faq-section {
		background-color: rgba(18, 18, 26, 0.88);
		border-color: rgba(255, 77, 140, 0.16);
	}

	:global(.page-root.theme-dark) .faq-title,
	:global(.page-root.theme-dark) .faq-summary {
		color: #ffffff;
	}

	:global(.page-root.theme-dark) .faq-item + .faq-item {
		border-top-color: rgba(255, 77, 140, 0.14);
	}

	:global(.page-root.theme-dark) .faq-answer {
		color: rgba(224, 208, 245, 0.74);
	}

	:global(.page-root.theme-dark) .faq-arrow {
		color: rgba(224, 208, 245, 0.42);
	}
</style>
