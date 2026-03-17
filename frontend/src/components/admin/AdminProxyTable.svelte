<script lang="ts">
	import type { AdminProxyRow, ProxyStatus } from '$lib/admin/types';
	import AdminStatusBadge from '$components/admin/AdminStatusBadge.svelte';
	import AppIcon from '$components/AppIcon.svelte';

	let { proxies }: { proxies: AdminProxyRow[] } = $props();

	const nextStatuses: ProxyStatus[] = ['active', 'disabled', 'quarantined'];
	let expandedId: string | null = $state(null);

	function toggleExpand(id: string) {
		expandedId = expandedId === id ? null : id;
	}

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
				<th class="w-8 px-2 py-2.5"></th>
				<th class="px-4 py-2.5 font-semibold">Proxy ID</th>
				<th class="px-4 py-2.5 font-semibold">Status</th>
				<th class="px-4 py-2.5 font-semibold">Score</th>
				<th class="px-4 py-2.5 font-semibold">Success</th>
				<th class="px-4 py-2.5 font-semibold">Access 24h</th>
				<th class="px-4 py-2.5 font-semibold">Notes</th>
				<th class="px-4 py-2.5 font-semibold">P95</th>
				<th class="px-4 py-2.5 font-semibold">Action</th>
			</tr>
		</thead>
		<tbody class="divide-y divide-gray-100">
			{#each proxies as proxy (proxy.id)}
				<!-- Main compact row -->
				<tr class={`hover:bg-gray-50/50 ${expandedId === proxy.id ? 'bg-gray-50' : ''}`}>
					<td class="px-2 py-2.5 text-center">
						<button
							type="button"
							onclick={() => toggleExpand(proxy.id)}
							class="inline-flex h-6 w-6 items-center justify-center rounded text-gray-400 transition hover:bg-gray-100 hover:text-gray-600"
						>
							<AppIcon name={expandedId === proxy.id ? 'expand_less' : 'expand_more'} class="text-base" />
						</button>
					</td>
					<td class="px-4 py-2.5">
						<p class="font-mono text-[11px] font-semibold text-gray-900">{proxy.id}</p>
						<p class="mt-0.5 break-all font-mono text-[10px] text-gray-400">{proxy.maskedProxyUrl}</p>
						{#if proxy.displayName}
							<p class="mt-1 text-[11px] text-gray-500">{proxy.displayName}</p>
						{/if}
					</td>
					<td class="px-4 py-2.5">
						<AdminStatusBadge value={proxy.status} kind="proxy" />
					</td>
					<td class="px-4 py-2.5">
						<span class={`font-semibold tabular-nums ${scoreTone(proxy.healthScore)}`}>{proxy.healthScore}</span>
					</td>
					<td class="px-4 py-2.5">
						<span class="font-medium tabular-nums text-gray-900">
							{formatPercent(proxy.extractSuccesses24h, proxy.proxyRelevantAttempts24h)}
						</span>
						<span class="ml-1 text-[11px] text-gray-400">{proxy.extractSuccesses24h}/{proxy.proxyRelevantAttempts24h}</span>
					</td>
					<td class="px-4 py-2.5">
						<span class="font-medium tabular-nums text-gray-900">{proxy.downloadAccesses24h}</span>
						<span class="ml-1 text-[11px] text-gray-400">/{proxy.downloadAccessCount}</span>
					</td>
					<td class="px-4 py-2.5">
						<button
							type="button"
							onclick={() => toggleExpand(proxy.id)}
							class="rounded border border-gray-200 px-2 py-1 text-[11px] font-medium text-gray-600 transition hover:bg-gray-100 hover:text-gray-900"
						>
							{expandedId === proxy.id ? 'Hide' : proxy.notes ? 'View' : 'Notes'}
						</button>
					</td>
					<td class="px-4 py-2.5 tabular-nums text-gray-700">
						{formatLatency(proxy.p95ExtractLatencyMs)}
					</td>
					<td class="px-4 py-2.5">
						<form method="POST" action="?/updateProxyStatus" class="flex items-center gap-1.5">
							<input type="hidden" name="proxyId" value={proxy.id} />
							<select
								name="status"
								class="rounded border border-gray-200 bg-white px-2 py-1 text-[11px] text-gray-700 focus:border-gray-400 focus:outline-none focus:ring-0"
							>
								{#each nextStatuses.filter((s) => !(proxy.status === 'quarantined' && s === 'active')) as status}
									<option value={status} selected={status === proxy.status}>{status}</option>
								{/each}
							</select>
							<button
								type="submit"
								class="rounded bg-gray-900 px-2.5 py-1 text-[11px] font-medium text-white transition hover:bg-gray-800"
							>
								Save
							</button>
						</form>
					</td>
				</tr>

				<!-- Expandable detail row -->
				{#if expandedId === proxy.id}
					<tr class="bg-gray-50/50">
						<td colspan="9" class="px-6 py-4">
							<div class="grid grid-cols-2 gap-x-8 gap-y-2 text-[12px] md:grid-cols-4">
								<div>
									<p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Extract 24h</p>
									<p class="mt-0.5 font-semibold tabular-nums text-gray-900">{proxy.extractAttempts24h}</p>
									<p class="text-gray-400">{proxy.proxyRelevantAttempts24h} relevant</p>
								</div>
								<div>
									<p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Full Format</p>
									<p class="mt-0.5 font-semibold tabular-nums text-gray-900">
										{formatPercent(proxy.fullFormatHits24h, proxy.proxyRelevantAttempts24h)}
									</p>
									<p class="text-gray-400">{proxy.fullFormatHits24h} hits</p>
								</div>
								<div>
									<p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">360-Only</p>
									<p class="mt-0.5 font-semibold tabular-nums text-gray-900">
										{formatPercent(proxy.combined360OnlyHits24h, proxy.proxyRelevantAttempts24h)}
									</p>
									<p class="text-gray-400">{proxy.combined360OnlyHits24h} hits</p>
								</div>
								<div>
									<p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Timeout</p>
									<p class="mt-0.5 font-semibold tabular-nums text-gray-900">
										{formatPercent(proxy.timeoutHits24h, proxy.proxyRelevantAttempts24h)}
									</p>
									<p class="text-gray-400">{proxy.timeoutHits24h} hits</p>
								</div>
								<div>
									<p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Download Access</p>
									<p class="mt-0.5 font-semibold tabular-nums text-gray-900">{proxy.downloadAccesses24h}</p>
									<p class="text-gray-400">{proxy.downloadAccessCount} total</p>
								</div>
								<div>
									<p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Mux / Direct 24h</p>
									<p class="mt-0.5 font-semibold tabular-nums text-gray-900">{proxy.muxJobAccesses24h}/{proxy.directStreamAccesses24h}</p>
									<p class="text-gray-400">mux / direct</p>
								</div>
								<div>
									<p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Source</p>
									<p class="mt-0.5 text-gray-700">{proxy.source}</p>
								</div>
								<div>
									<p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Last Outcome</p>
									<p class="mt-0.5 text-gray-700">{proxy.lastExtractOutcome ?? proxy.lastEventType ?? '—'}</p>
								</div>
								<div>
									<p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Last Download Access</p>
									<p class="mt-0.5 text-gray-700">
										{proxy.lastDownloadAccessAt ? new Date(proxy.lastDownloadAccessAt).toLocaleString() : '—'}
									</p>
								</div>
								<div>
									<p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Last Quarantine</p>
									<p class="mt-0.5 text-gray-700">
										{proxy.lastQuarantinedAt ? new Date(proxy.lastQuarantinedAt).toLocaleString() : '—'}
									</p>
									{#if proxy.lastQuarantineReason}
										<p class="text-red-600">{proxy.lastQuarantineReason}</p>
									{/if}
								</div>
								<div>
									<p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Auto-Disabled</p>
									{#if proxy.autoDisabledAt}
										<p class="mt-0.5 text-gray-700">{new Date(proxy.autoDisabledAt).toLocaleString()}</p>
										<p class="text-red-600">{proxy.autoDisabledReason ?? ''}</p>
									{:else}
										<p class="mt-0.5 text-gray-400">—</p>
									{/if}
								</div>
							</div>
							<form method="POST" action="?/updateProxyNotes" class="mt-3 border-t border-gray-200 pt-3">
								<input type="hidden" name="proxyId" value={proxy.id} />
								<label for={`proxy-notes-${proxy.id}`} class="block text-[10px] font-medium uppercase tracking-wider text-gray-400">Notes</label>
								<div class="mt-2 flex flex-col gap-2 md:flex-row md:items-start">
									<textarea
										id={`proxy-notes-${proxy.id}`}
										name="notes"
										rows="3"
										class="min-h-[84px] flex-1 rounded-md border border-gray-200 bg-white px-3 py-2 text-[12px] text-gray-700 placeholder:text-gray-300 focus:border-gray-400 focus:outline-none focus:ring-0"
										placeholder="Add operator notes for this proxy"
									>{proxy.notes ?? ''}</textarea>
									<button
										type="submit"
										class="rounded-md bg-gray-900 px-3 py-2 text-[12px] font-medium text-white transition hover:bg-gray-800"
									>
										Save Notes
									</button>
								</div>
							</form>
						</td>
					</tr>
				{/if}
			{/each}
			{#if proxies.length === 0}
				<tr>
					<td colspan="9" class="px-4 py-8 text-center text-sm text-gray-400">
						No proxies in inventory.
					</td>
				</tr>
			{/if}
		</tbody>
	</table>
</div>
