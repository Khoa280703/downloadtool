<script lang="ts">
	type BadgeKind = 'job' | 'proxy' | 'scope' | 'tone';
	type Tone = 'neutral' | 'sky' | 'emerald' | 'amber' | 'rose';

	let {
		value,
		kind = 'tone',
		tone = 'neutral'
	}: {
		value: string;
		kind?: BadgeKind;
		tone?: Tone;
	} = $props();

	function resolveTone(): Tone {
		if (kind === 'job') {
			switch (value) {
				case 'ready':
					return 'emerald';
				case 'failed':
				case 'expired':
					return 'rose';
				case 'processing':
				case 'leased':
					return 'sky';
				case 'queued':
					return 'amber';
				default:
					return 'neutral';
			}
		}

		if (kind === 'proxy') {
			switch (value) {
				case 'active':
					return 'emerald';
				case 'quarantined':
					return 'rose';
				case 'disabled':
					return 'amber';
				default:
					return 'neutral';
			}
		}

		if (kind === 'scope') {
			return value === 'proxy' ? 'amber' : 'sky';
		}

		return tone;
	}

	const classes = $derived.by(() => {
		switch (resolveTone()) {
			case 'sky':
				return 'border-sky-200/80 bg-sky-100/80 text-sky-800';
			case 'emerald':
				return 'border-emerald-200/80 bg-emerald-100/80 text-emerald-800';
			case 'amber':
				return 'border-amber-200/80 bg-amber-100/80 text-amber-800';
			case 'rose':
				return 'border-rose-200/80 bg-rose-100/80 text-rose-800';
			default:
				return 'border-slate-200/90 bg-slate-100/90 text-slate-700';
		}
	});
</script>

<span
	class={`inline-flex items-center rounded-full border px-2.5 py-1 text-[11px] font-bold uppercase tracking-[0.18em] ${classes}`}
>
	{value}
</span>
