<script lang="ts">
	let {
		label,
		value,
		caption = '',
		inverted = false
	}: {
		label: string;
		value: string | number;
		caption?: string;
		inverted?: boolean;
	} = $props();
</script>

<div
	class={`admin-kpi-box rounded-2xl border p-4 ${inverted ? 'border-white/10 bg-white/5 text-white' : 'border-slate-200 bg-slate-50 text-slate-950'}`}
>
	<p
		class={`text-[11px] font-bold uppercase tracking-[0.18em] ${inverted ? 'text-slate-400' : 'text-slate-500'}`}
	>
		{label}
	</p>
	<p class={`mt-2 text-2xl font-black tracking-[-0.03em] ${inverted ? 'text-white' : 'text-slate-950'}`}>
		{value}
	</p>
	{#if caption}
		<p class={`mt-1 text-sm ${inverted ? 'text-slate-300' : 'text-slate-500'}`}>{caption}</p>
	{/if}
</div>
