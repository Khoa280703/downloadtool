<script lang="ts">
	type Segment = {
		label: string;
		value: number;
		color: string;
	};

	let {
		title,
		description = '',
		totalLabel = 'TOTAL',
		segments
	}: {
		title: string;
		description?: string;
		totalLabel?: string;
		segments: Segment[];
	} = $props();

	const total = $derived(segments.reduce((sum, segment) => sum + segment.value, 0));
	const radius = 16;
	const circumference = 2 * Math.PI * radius;

	function dashArray(value: number): string {
		if (total === 0) return `0 ${circumference}`;
		return `${(value / total) * circumference} ${circumference}`;
	}

	function dashOffset(before: number): number {
		if (total === 0) return 0;
		return circumference - (before / total) * circumference;
	}
</script>

<section class="admin-panel border border-slate-200 bg-white p-6 shadow-sm">
	<h3 class="text-lg font-bold text-slate-950">{title}</h3>
	{#if description}
		<p class="mt-1 text-sm text-slate-500">{description}</p>
	{/if}

	<div class="relative mx-auto mt-6 h-52 w-52">
		<svg class="h-full w-full -rotate-90" viewBox="0 0 36 36">
			<circle cx="18" cy="18" r={radius} fill="none" stroke="#e5e7eb" stroke-width="4"></circle>
			{#each segments as segment, index}
				{@const before = segments.slice(0, index).reduce((sum, item) => sum + item.value, 0)}
				<circle
					cx="18"
					cy="18"
					r={radius}
					fill="none"
					stroke={segment.color}
					stroke-width="4"
					stroke-linecap="round"
					stroke-dasharray={dashArray(segment.value)}
					stroke-dashoffset={dashOffset(before)}
				></circle>
			{/each}
		</svg>
		<div class="absolute inset-0 flex flex-col items-center justify-center">
			<span class="text-3xl font-bold tracking-[-0.03em] text-slate-950">{total}</span>
			<span class="text-[10px] font-bold uppercase tracking-[0.22em] text-slate-500">
				{totalLabel}
			</span>
		</div>
	</div>

	<div class="mt-6 space-y-3">
		{#each segments as segment}
			<div class="flex items-center justify-between gap-3">
				<div class="flex items-center gap-2">
					<div class="h-2.5 w-2.5 rounded-full" style={`background:${segment.color}`}></div>
					<span class="text-sm font-medium text-slate-600">{segment.label}</span>
				</div>
				<span class="text-sm font-bold text-slate-950">{segment.value}</span>
			</div>
		{/each}
	</div>
</section>
