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
		if (score >= 80) return 'text-emerald-700';
		if (score >= 55) return 'text-amber-700';
		return 'text-rose-700';
	}
</script>

<div class="overflow-x-auto">
	<table class="admin-data-table min-w-full text-sm text-slate-700">
		<thead class="sticky top-0 z-10 bg-slate-100 text-left text-[10px] uppercase tracking-[0.18em] text-slate-500">
			<tr>
				<th class="px-4 py-3 font-bold">Proxy</th>
				<th class="px-4 py-3 font-bold">Status</th>
				<th class="px-4 py-3 font-bold">Score</th>
				<th class="px-4 py-3 font-bold">Extract 24h</th>
				<th class="px-4 py-3 font-bold">Success</th>
				<th class="px-4 py-3 font-bold">Full format</th>
				<th class="px-4 py-3 font-bold">360-only</th>
				<th class="px-4 py-3 font-bold">Timeout</th>
				<th class="px-4 py-3 font-bold">P95</th>
				<th class="px-4 py-3 font-bold">Last quarantine</th>
				<th class="px-4 py-3 font-bold">Action</th>
			</tr>
		</thead>
		<tbody class="divide-y divide-slate-200/80">
			{#each proxies as proxy (proxy.id)}
				<tr class="align-top transition hover:bg-slate-50/80">
					<td class="px-4 py-3.5 pr-2">
						<p class="text-sm font-semibold text-slate-900">{proxy.displayName || 'Unnamed proxy'}</p>
						<p class="mt-1 break-all font-mono text-[11px] text-slate-500">{proxy.maskedProxyUrl}</p>
						{#if proxy.notes}
							<p class="mt-2 text-xs text-slate-600">{proxy.notes}</p>
						{/if}
					</td>
					<td class="px-4 py-3.5 pr-2">
						<AdminStatusBadge value={proxy.status} kind="proxy" />
						<p class="mt-2 text-xs uppercase tracking-[0.16em] text-slate-500">{proxy.source}</p>
						{#if proxy.autoDisabledReason}
							<p class="mt-2 max-w-[14rem] text-xs text-rose-700">{proxy.autoDisabledReason}</p>
						{/if}
					</td>
					<td class="px-4 py-3.5 pr-2">
						<p class={`text-base font-bold ${scoreTone(proxy.healthScore)}`}>{proxy.healthScore}</p>
						<p class="text-xs text-slate-500">{proxy.lastExtractOutcome ?? proxy.lastEventType ?? 'No signals yet'}</p>
					</td>
					<td class="px-4 py-3.5 pr-2">
						<p class="text-sm font-semibold text-slate-900">{proxy.extractAttempts24h}</p>
						<p class="text-xs text-slate-500">{proxy.proxyRelevantAttempts24h} relevant</p>
					</td>
					<td class="px-4 py-3.5 pr-2">
						<p class="text-sm font-semibold text-slate-900">
							{formatPercent(proxy.extractSuccesses24h, proxy.proxyRelevantAttempts24h)}
						</p>
						<p class="text-xs text-slate-500">{proxy.extractSuccesses24h}/{proxy.proxyRelevantAttempts24h}</p>
					</td>
					<td class="px-4 py-3.5 pr-2">
						<p class="text-sm font-semibold text-slate-900">
							{formatPercent(proxy.fullFormatHits24h, proxy.proxyRelevantAttempts24h)}
						</p>
						<p class="text-xs text-slate-500">{proxy.fullFormatHits24h} hits</p>
					</td>
					<td class="px-4 py-3.5 pr-2">
						<p class="text-sm font-semibold text-slate-900">
							{formatPercent(proxy.combined360OnlyHits24h, proxy.proxyRelevantAttempts24h)}
						</p>
						<p class="text-xs text-slate-500">{proxy.combined360OnlyHits24h} hits</p>
					</td>
					<td class="px-4 py-3.5 pr-2">
						<p class="text-sm font-semibold text-slate-900">
							{formatPercent(proxy.timeoutHits24h, proxy.proxyRelevantAttempts24h)}
						</p>
						<p class="text-xs text-slate-500">{proxy.timeoutHits24h} hits</p>
					</td>
					<td class="px-4 py-3.5 pr-2">
						<p class="text-sm font-semibold text-slate-900">{formatLatency(proxy.p95ExtractLatencyMs)}</p>
						<p class="text-xs text-slate-500">p95 latency</p>
					</td>
					<td class="px-4 py-3.5 pr-2">
						<p class="text-sm text-slate-700">
							{proxy.lastQuarantinedAt ? new Date(proxy.lastQuarantinedAt).toLocaleString() : '—'}
						</p>
						<p class="mt-1 max-w-[16rem] text-xs text-rose-700">{proxy.lastQuarantineReason ?? ''}</p>
						{#if proxy.autoDisabledAt}
							<p class="mt-2 text-xs text-slate-500">Auto-disabled {new Date(proxy.autoDisabledAt).toLocaleString()}</p>
						{/if}
					</td>
					<td class="px-4 py-3.5">
						<form method="POST" action="?/updateProxyStatus" class="grid gap-2 lg:max-w-[13rem]">
							<input type="hidden" name="proxyId" value={proxy.id} />
							<select
								name="status"
								class="admin-field rounded-md border border-slate-300 bg-white px-3 py-2 text-sm text-slate-700 focus:border-slate-400 focus:ring-0"
							>
								{#each nextStatuses.filter((status) => !(proxy.status === 'quarantined' && status === 'active')) as status}
									<option value={status} selected={status === proxy.status}>{status}</option>
								{/each}
							</select>
							<input
								type="text"
								name="reason"
								class="admin-field rounded-md border border-slate-300 bg-white px-3 py-2 text-sm text-slate-700 placeholder:text-slate-400 focus:border-slate-400 focus:ring-0"
								placeholder="Reason"
							/>
							<button
								type="submit"
								class="rounded-md bg-slate-900 px-3 py-2 text-sm font-bold text-white transition hover:bg-slate-800"
							>
								Update
							</button>
						</form>
					</td>
				</tr>
			{/each}
			{#if proxies.length === 0}
				<tr>
					<td colspan="11" class="px-4 py-10 text-center text-sm text-slate-500">
						Chưa có proxy nào trong inventory.
					</td>
				</tr>
			{/if}
		</tbody>
	</table>
</div>
