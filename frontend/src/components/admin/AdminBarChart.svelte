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
				return 'bg-blue-600';
			case 'green':
				return 'bg-emerald-600';
			case 'amber':
				return 'bg-amber-600';
			case 'red':
				return 'bg-rose-600';
			default:
				return 'bg-slate-500';
		}
	}
</script>

<section class="admin-panel rounded-lg border border-slate-200 bg-white p-4">
	<div class="border-b border-slate-200 pb-3">
		<h3 class="text-sm font-bold text-slate-900">{title}</h3>
		{#if description}
			<p class="mt-1 text-[12px] leading-5 text-slate-500">{description}</p>
		{/if}
	</div>

	<div class="mt-4 space-y-3">
		{#each items as item}
			<div class="space-y-1.5">
				<div class="flex items-center justify-between gap-3 text-xs">
					<span class="font-medium text-slate-600">{item.label}</span>
					<span class="font-bold text-slate-900">{item.value}</span>
				</div>
				<div class="h-2 w-full overflow-hidden rounded-sm bg-slate-100">
					<div
						class={`h-full rounded-sm ${toneClass(item.tone)}`}
						style={`width:${Math.max((item.value / maxValue) * 100, item.value > 0 ? 8 : 0)}%`}
					></div>
				</div>
			</div>
		{/each}
	</div>
</section>
