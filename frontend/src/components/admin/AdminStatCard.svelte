<script lang="ts">
	let {
		label,
		value,
		tone = 'plum'
	}: {
		label: string;
		value: string | number;
		tone?: 'plum' | 'pink' | 'emerald' | 'amber';
	} = $props();

	const toneClass = $derived.by(() => {
		switch (tone) {
			case 'pink':
				return 'from-pink-500/15 to-pink-100 text-pink-700';
			case 'emerald':
				return 'from-emerald-500/15 to-emerald-100 text-emerald-700';
			case 'amber':
				return 'from-amber-500/15 to-amber-100 text-amber-700';
			default:
				return 'from-plum/15 to-plum/5 text-plum';
		}
	});
</script>

<article class={`rounded-3xl border border-white/70 bg-gradient-to-br p-5 shadow-card ${toneClass}`}>
	<p class="text-xs font-bold uppercase tracking-[0.2em] text-current/70">{label}</p>
	<p class="mt-3 text-3xl font-black tracking-tight text-current">{value}</p>
</article>
