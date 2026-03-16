<script lang="ts">
	type ChartItem = {
		label: string;
		value: number;
		tone?: 'neutral' | 'blue' | 'green' | 'amber' | 'red';
	};

	let {
		title,
		description = '',
		items
	}: {
		title: string;
		description?: string;
		items: ChartItem[];
	} = $props();

	const maxValue = $derived(Math.max(...items.map((item) => item.value), 1));

	function toneClass(tone: ChartItem['tone']): string {
		switch (tone) {
			case 'blue':
				return 'bg-blue-500';
			case 'green':
				return 'bg-green-500';
			case 'amber':
				return 'bg-amber-500';
			case 'red':
				return 'bg-red-500';
			default:
				return 'bg-gray-400';
		}
	}
</script>

<section class="admin-panel rounded-lg border border-gray-200 bg-white p-4">
	<div class="border-b border-gray-100 pb-3">
		<h3 class="text-sm font-semibold text-gray-900">{title}</h3>
		{#if description}
			<p class="mt-0.5 text-[11px] leading-4 text-gray-500">{description}</p>
		{/if}
	</div>

	<div class="mt-3 space-y-2.5">
		{#each items as item}
			<div class="space-y-1">
				<div class="flex items-center justify-between gap-3 text-[12px]">
					<span class="text-gray-600">{item.label}</span>
					<span class="font-semibold tabular-nums text-gray-900">{item.value}</span>
				</div>
				<div class="h-1.5 w-full overflow-hidden rounded-sm bg-gray-100">
					<div
						class={`h-full rounded-sm ${toneClass(item.tone)}`}
						style={`width:${Math.max((item.value / maxValue) * 100, item.value > 0 ? 6 : 0)}%`}
					></div>
				</div>
			</div>
		{/each}
	</div>
</section>
