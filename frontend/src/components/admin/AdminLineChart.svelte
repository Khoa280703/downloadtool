<script lang="ts">
	type ChartPoint = {
		label: string;
		value: number;
	};

	let {
		title,
		description = '',
		points
	}: {
		title: string;
		description?: string;
		points: ChartPoint[];
	} = $props();

	const width = 640;
	const height = 240;
	const paddingX = 18;
	const paddingY = 18;

	const maxValue = $derived(Math.max(...points.map((point) => point.value), 1));

	function getX(index: number): number {
		if (points.length <= 1) return width / 2;
		return paddingX + (index * (width - paddingX * 2)) / (points.length - 1);
	}

	function getY(value: number): number {
		const usableHeight = height - paddingY * 2;
		return height - paddingY - (value / maxValue) * usableHeight;
	}

	const linePath = $derived.by(() => {
		if (points.length === 0) return '';
		return points
			.map((point, index) => `${index === 0 ? 'M' : 'L'} ${getX(index)} ${getY(point.value)}`)
			.join(' ');
	});

	const areaPath = $derived.by(() => {
		if (points.length === 0) return '';
		const firstX = getX(0);
		const lastX = getX(points.length - 1);
		return `${linePath} L ${lastX} ${height - paddingY} L ${firstX} ${height - paddingY} Z`;
	});
</script>

<section class="admin-panel border border-slate-200 bg-white p-6 shadow-sm">
	<div class="flex items-start justify-between gap-4">
		<div>
			<h3 class="text-lg font-bold text-slate-950">{title}</h3>
			{#if description}
				<p class="mt-1 text-sm text-slate-500">{description}</p>
			{/if}
		</div>
		<div class="rounded-lg bg-slate-100 px-3 py-1.5 text-[11px] font-bold text-slate-600">
			Live snapshot
		</div>
	</div>

	<div class="mt-6 h-[280px]">
		<svg class="h-full w-full" viewBox={`0 0 ${width} ${height}`} preserveAspectRatio="none">
			<defs>
				<linearGradient id="adminLineGradient" x1="0" y1="0" x2="0" y2="1">
					<stop offset="0%" stop-color="#137fec" stop-opacity="0.22"></stop>
					<stop offset="100%" stop-color="#137fec" stop-opacity="0"></stop>
				</linearGradient>
			</defs>

			{#each Array(4) as _, index}
				<line
					x1={paddingX}
					x2={width - paddingX}
					y1={paddingY + ((height - paddingY * 2) / 3) * index}
					y2={paddingY + ((height - paddingY * 2) / 3) * index}
					stroke="#e2e8f0"
					stroke-dasharray="4 6"
				/>
			{/each}

			<path d={areaPath} fill="url(#adminLineGradient)"></path>
			<path d={linePath} fill="none" stroke="#137fec" stroke-width="3.5" stroke-linecap="round"></path>

			{#each points as point, index}
				<circle cx={getX(index)} cy={getY(point.value)} r="4.5" fill="white" stroke="#137fec" stroke-width="2.5"></circle>
			{/each}
		</svg>
	</div>

	<div class="mt-4 grid grid-cols-6 gap-2 text-[10px] font-bold uppercase tracking-[0.22em] text-slate-400">
		{#each points as point}
			<span>{point.label}</span>
		{/each}
	</div>
</section>
