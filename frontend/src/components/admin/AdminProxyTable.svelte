<script lang="ts">
	import type { AdminProxyRow, ProxyStatus } from '$lib/admin/types';
	import AdminStatusBadge from '$components/admin/AdminStatusBadge.svelte';

	let { proxies }: { proxies: AdminProxyRow[] } = $props();

	const nextStatuses: ProxyStatus[] = ['active', 'disabled', 'quarantined'];

	function formatPercent(numerator: number, denominator: number): string {
		if (denominator <= 0) return '—';
		return `${Math.round((numerator / denominator) * 100)}%`;
	}

	function formatLatency(value: number | null): string {
		if (!value || Number.isNaN(value)) return '—';
		return `${Math.round(value)} ms`;
	}

	function scoreTone(score: number): string {
		if (score >= 80) return 'text-green-700';
		if (score >= 55) return 'text-amber-600';
		return 'text-red-600';
	}
</script>

<div class="overflow-x-auto">
	<table class="min-w-full text-[13px] text-gray-700">
		<thead class="sticky top-0 z-10 border-b border-gray-200 bg-gray-50 text-left text-[10px] uppercase tracking-wider text-gray-500">
			<tr>
				<th class="px-4 py-2.5 font-semibold">Proxy</th>
				<th class="px-4 py-2.5 font-semibold">Status</th>
				<th class="px-4 py-2.5 font-semibold">Score</th>
				<th class="px-4 py-2.5 font-semibold">Extract 24h</th>
				<th class="px-4 py-2.5 font-semibold">Success</th>
				<th class="px-4 py-2.5 font-semibold">Full format</th>
				<th class="px-4 py-2.5 font-semibold">360-only</th>
				<th class="px-4 py-2.5 font-semibold">Timeout</th>
				<th class="px-4 py-2.5 font-semibold">P95</th>
				<th class="px-4 py-2.5 font-semibold">Last quarantine</th>
				<th class="px-4 py-2.5 font-semibold">Action</th>
			</tr>
		</thead>
		<tbody class="divide-y divide-gray-100">
			{#each proxies as proxy (proxy.id)}
				<tr class="align-top hover:bg-gray-50/50">
					<td class="px-4 py-3">
						<p class="font-medium text-gray-900">{proxy.displayName || 'Unnamed proxy'}</p>
						<p class="mt-0.5 break-all font-mono text-[10px] text-gray-400">{proxy.maskedProxyUrl}</p>
						{#if proxy.notes}
							<p class="mt-1 text-[11px] text-gray-500">{proxy.notes}</p>
						{/if}
					</td>
					<td class="px-4 py-3">
						<AdminStatusBadge value={proxy.status} kind="proxy" />
						<p class="mt-1.5 text-[10px] uppercase tracking-wider text-gray-400">{proxy.source}</p>
						{#if proxy.autoDisabledReason}
							<p class="mt-1 max-w-[12rem] text-[11px] text-red-600">{proxy.autoDisabledReason}</p>
						{/if}
					</td>
					<td class="px-4 py-3">
						<p class={`text-sm font-semibold tabular-nums ${scoreTone(proxy.healthScore)}`}>{proxy.healthScore}</p>
						<p class="text-[11px] text-gray-400">{proxy.lastExtractOutcome ?? proxy.lastEventType ?? '—'}</p>
					</td>
					<td class="px-4 py-3">
						<p class="font-medium tabular-nums text-gray-900">{proxy.extractAttempts24h}</p>
						<p class="text-[11px] text-gray-400">{proxy.proxyRelevantAttempts24h} relevant</p>
					</td>
					<td class="px-4 py-3">
						<p class="font-medium tabular-nums text-gray-900">
							{formatPercent(proxy.extractSuccesses24h, proxy.proxyRelevantAttempts24h)}
						</p>
						<p class="text-[11px] text-gray-400">{proxy.extractSuccesses24h}/{proxy.proxyRelevantAttempts24h}</p>
					</td>
					<td class="px-4 py-3">
						<p class="font-medium tabular-nums text-gray-900">
							{formatPercent(proxy.fullFormatHits24h, proxy.proxyRelevantAttempts24h)}
						</p>
						<p class="text-[11px] text-gray-400">{proxy.fullFormatHits24h} hits</p>
					</td>
					<td class="px-4 py-3">
						<p class="font-medium tabular-nums text-gray-900">
							{formatPercent(proxy.combined360OnlyHits24h, proxy.proxyRelevantAttempts24h)}
						</p>
						<p class="text-[11px] text-gray-400">{proxy.combined360OnlyHits24h} hits</p>
					</td>
					<td class="px-4 py-3">
						<p class="font-medium tabular-nums text-gray-900">
							{formatPercent(proxy.timeoutHits24h, proxy.proxyRelevantAttempts24h)}
						</p>
						<p class="text-[11px] text-gray-400">{proxy.timeoutHits24h} hits</p>
					</td>
					<td class="px-4 py-3">
						<p class="font-medium tabular-nums text-gray-900">{formatLatency(proxy.p95ExtractLatencyMs)}</p>
						<p class="text-[11px] text-gray-400">p95</p>
					</td>
					<td class="px-4 py-3">
						<p class="tabular-nums text-gray-600">
							{proxy.lastQuarantinedAt ? new Date(proxy.lastQuarantinedAt).toLocaleString() : '—'}
						</p>
						<p class="mt-0.5 max-w-[14rem] text-[11px] text-red-600">{proxy.lastQuarantineReason ?? ''}</p>
						{#if proxy.autoDisabledAt}
							<p class="mt-1 text-[11px] text-gray-400">Disabled {new Date(proxy.autoDisabledAt).toLocaleString()}</p>
						{/if}
					</td>
					<td class="px-4 py-3">
						<form method="POST" action="?/updateProxyStatus" class="grid gap-1.5 lg:max-w-[12rem]">
							<input type="hidden" name="proxyId" value={proxy.id} />
							<select
								name="status"
								class="rounded border border-gray-200 bg-white px-2.5 py-1.5 text-[12px] text-gray-700 focus:border-gray-400 focus:outline-none focus:ring-0"
							>
								{#each nextStatuses.filter((status) => !(proxy.status === 'quarantined' && status === 'active')) as status}
									<option value={status} selected={status === proxy.status}>{status}</option>
								{/each}
							</select>
							<input
								type="text"
								name="reason"
								class="rounded border border-gray-200 bg-white px-2.5 py-1.5 text-[12px] text-gray-700 placeholder:text-gray-300 focus:border-gray-400 focus:outline-none focus:ring-0"
								placeholder="Reason"
							/>
							<button
								type="submit"
								class="rounded bg-gray-900 px-2.5 py-1.5 text-[12px] font-medium text-white transition hover:bg-gray-800"
							>
								Update
							</button>
						</form>
					</td>
				</tr>
			{/each}
			{#if proxies.length === 0}
				<tr>
					<td colspan="11" class="px-4 py-8 text-center text-sm text-gray-400">
						No proxies in inventory.
					</td>
				</tr>
			{/if}
		</tbody>
	</table>
</div>
