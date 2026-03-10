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

<article
	class={`admin-stat-card rounded-[1.75rem] border p-5 shadow-[0_18px_45px_-32px_rgba(15,23,42,0.45)] ${toneClass.surface}`}
>
	<div class={`h-1.5 w-12 rounded-full ${toneClass.accent}`}></div>
	<p class={`mt-4 text-[11px] font-bold uppercase tracking-[0.22em] ${toneClass.label}`}>{label}</p>
	<p class={`mt-3 text-3xl font-black tracking-[-0.03em] ${toneClass.value}`}>{value}</p>
	{#if caption}
		<p class={`mt-2 text-sm ${toneClass.caption}`}>{caption}</p>
	{/if}
</article>
