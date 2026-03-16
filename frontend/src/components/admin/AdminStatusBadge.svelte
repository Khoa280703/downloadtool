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
				return 'neutral';
			case 'proxy':
				return 'amber';
		}
	}

	function resolveTone(): Tone {
		if (kind === 'job') return resolveJobTone(value as AdminJobStatus);
		if (kind === 'proxy') return resolveProxyTone(value as ProxyStatus);
		if (kind === 'scope') return resolveScopeTone(value as AdminActivityRow['scope']);
		return tone;
	}

	const classes = $derived.by(() => {
		switch (resolveTone()) {
			case 'sky':
				return 'border-blue-200 bg-blue-50 text-blue-700';
			case 'emerald':
				return 'border-green-200 bg-green-50 text-green-700';
			case 'amber':
				return 'border-amber-200 bg-amber-50 text-amber-700';
			case 'rose':
				return 'border-red-200 bg-red-50 text-red-700';
			default:
				return 'border-gray-200 bg-gray-50 text-gray-600';
		}
	});
</script>

<span
	class={`inline-flex items-center rounded border px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide ${classes}`}
>
	{value}
</span>
