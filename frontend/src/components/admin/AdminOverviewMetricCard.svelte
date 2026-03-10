<script lang="ts">
	type Tone = 'primary' | 'violet' | 'emerald' | 'amber' | 'rose';

	let {
		label,
		value,
		icon,
		badge,
		badgeTone = 'emerald',
		description
	}: {
		label: string;
		value: string | number;
		icon: string;
		badge: string;
		badgeTone?: Tone;
		description: string;
	} = $props();

	function toneClasses(tone: Tone): { icon: string; badge: string } {
		switch (tone) {
			case 'violet':
				return {
					icon: 'bg-violet-100 text-violet-600',
					badge: 'bg-violet-50 text-violet-700'
				};
			case 'emerald':
				return {
					icon: 'bg-emerald-100 text-emerald-600',
					badge: 'bg-emerald-50 text-emerald-700'
				};
			case 'amber':
				return {
					icon: 'bg-amber-100 text-amber-600',
					badge: 'bg-amber-50 text-amber-700'
				};
			case 'rose':
				return {
					icon: 'bg-rose-100 text-rose-600',
					badge: 'bg-rose-50 text-rose-700'
				};
			default:
				return {
					icon: 'bg-blue-100 text-blue-600',
					badge: 'bg-blue-50 text-blue-700'
				};
		}
	}

	const classes = $derived(toneClasses(badgeTone));
</script>

<article class="admin-panel border border-slate-200 bg-white p-5 shadow-sm">
	<div class="flex items-start justify-between gap-4">
		<div class={`flex h-11 w-11 items-center justify-center rounded-xl ${classes.icon}`}>
			<span class="material-symbols-outlined text-[22px]">{icon}</span>
		</div>
		<div class={`inline-flex items-center gap-1 rounded-full px-2.5 py-1 text-[11px] font-bold ${classes.badge}`}>
			<span class="material-symbols-outlined text-[14px]">trending_up</span>
			{badge}
		</div>
	</div>
	<p class="mt-4 text-sm font-medium text-slate-500">{label}</p>
	<p class="mt-1 text-[1.9rem] font-bold tracking-[-0.03em] text-slate-950">{value}</p>
	<p class="mt-1 text-[13px] leading-5 text-slate-500">{description}</p>
</article>
