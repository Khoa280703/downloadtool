<script lang="ts">
	let {
		label,
		value,
		caption = '',
		tone = 'neutral'
	}: {
		label: string;
		value: string | number;
		caption?: string;
		tone?: 'neutral' | 'sky' | 'emerald' | 'amber' | 'rose';
	} = $props();

	const toneClass = $derived.by(() => {
		switch (tone) {
			case 'sky':
				return {
					accent: 'bg-sky-500',
					value: 'text-slate-950',
					label: 'text-slate-500',
					caption: 'text-slate-500',
					surface: 'border-slate-200 bg-white/95'
				};
			case 'emerald':
				return {
					accent: 'bg-emerald-500',
					value: 'text-slate-950',
					label: 'text-slate-500',
					caption: 'text-slate-500',
					surface: 'border-slate-200 bg-white/95'
				};
			case 'amber':
				return {
					accent: 'bg-amber-500',
					value: 'text-slate-950',
					label: 'text-slate-500',
					caption: 'text-slate-500',
					surface: 'border-slate-200 bg-white/95'
				};
			case 'rose':
				return {
					accent: 'bg-rose-500',
					value: 'text-slate-950',
					label: 'text-slate-500',
					caption: 'text-slate-500',
					surface: 'border-slate-200 bg-white/95'
				};
			default:
				return {
					accent: 'bg-slate-900',
					value: 'text-slate-950',
					label: 'text-slate-500',
					caption: 'text-slate-500',
					surface: 'border-slate-200 bg-white/95'
				};
		}
	});
</script>

<article class={`admin-stat-card rounded-xl border p-4 ${toneClass.surface}`}>
	<div class={`h-0.5 w-8 ${toneClass.accent}`}></div>
	<p class={`mt-3 text-[10px] font-bold uppercase tracking-[0.2em] ${toneClass.label}`}>{label}</p>
	<p class={`mt-2 text-[1.9rem] font-black tracking-[-0.03em] ${toneClass.value}`}>{value}</p>
	{#if caption}
		<p class={`mt-1 text-[12px] leading-5 ${toneClass.caption}`}>{caption}</p>
	{/if}
</article>
