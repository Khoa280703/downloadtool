<script lang="ts">
	import type { AdminActivityRow, AdminJobStatus, ProxyStatus } from '$lib/admin/types';

	type Tone = 'neutral' | 'sky' | 'emerald' | 'amber' | 'rose';
	type BadgeProps =
		| { kind: 'job'; value: AdminJobStatus; tone?: never }
		| { kind: 'proxy'; value: ProxyStatus; tone?: never }
		| { kind: 'scope'; value: AdminActivityRow['scope']; tone?: never }
		| { kind?: 'tone'; value: string; tone?: Tone };

	let {
		value,
		kind = 'tone',
		tone = 'neutral'
	}: BadgeProps = $props();

	function resolveJobTone(status: AdminJobStatus): Tone {
		switch (status) {
			case 'queued':
				return 'amber';
			case 'leased':
			case 'processing':
				return 'sky';
			case 'ready':
				return 'emerald';
			case 'failed':
			case 'expired':
				return 'rose';
		}
	}

	function resolveProxyTone(status: ProxyStatus): Tone {
		switch (status) {
			case 'active':
				return 'emerald';
			case 'quarantined':
				return 'rose';
			case 'disabled':
				return 'amber';
		}
	}

	function resolveScopeTone(scope: AdminActivityRow['scope']): Tone {
		switch (scope) {
			case 'job':
				return 'sky';
			case 'proxy':
				return 'amber';
		}
	}

	function resolveTone(): Tone {
		if (kind === 'job') {
			return resolveJobTone(value as AdminJobStatus);
		}

		if (kind === 'proxy') {
			return resolveProxyTone(value as ProxyStatus);
		}

		if (kind === 'scope') {
			return resolveScopeTone(value as AdminActivityRow['scope']);
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
	class={`inline-flex items-center rounded-md border px-2.5 py-1 text-[10px] font-bold uppercase tracking-[0.18em] ${classes}`}
>
	{value}
</span>
