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
	class={`admin-kpi-box rounded-md border px-3 py-2.5 ${inverted ? 'border-white/10 bg-white/5 text-white' : 'border-slate-300 bg-slate-50 text-slate-950'}`}
>
	<p
		class={`text-[10px] font-bold uppercase tracking-[0.18em] ${inverted ? 'text-slate-400' : 'text-slate-500'}`}
	>
		{label}
	</p>
	<p class={`mt-1 text-lg font-black tracking-[-0.03em] ${inverted ? 'text-white' : 'text-slate-950'}`}>
		{value}
	</p>
	{#if caption}
		<p class={`mt-1 text-[12px] leading-5 ${inverted ? 'text-slate-300' : 'text-slate-500'}`}>{caption}</p>
	{/if}
</div>
