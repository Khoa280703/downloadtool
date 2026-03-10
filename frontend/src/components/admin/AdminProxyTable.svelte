<script lang="ts">
	import type { AdminProxyRow, ProxyStatus } from '$lib/admin/types';

	let { proxies }: { proxies: AdminProxyRow[] } = $props();

	const nextStatuses: ProxyStatus[] = ['active', 'disabled', 'quarantined'];
</script>

<div class="overflow-x-auto">
	<table class="min-w-full text-sm">
		<thead class="text-left text-xs uppercase tracking-[0.18em] text-plum/55">
			<tr>
				<th class="pb-3">Proxy</th>
				<th class="pb-3">Status</th>
				<th class="pb-3">Health</th>
				<th class="pb-3">Last quarantine</th>
				<th class="pb-3">Action</th>
			</tr>
		</thead>
		<tbody class="divide-y divide-pink-100/80">
			{#each proxies as proxy (proxy.id)}
				<tr class="align-top">
					<td class="py-3 pr-4">
						<p class="font-semibold text-plum">{proxy.displayName || 'Unnamed proxy'}</p>
						<p class="mt-1 break-all font-mono text-xs text-plum/55">{proxy.maskedProxyUrl}</p>
						{#if proxy.notes}
							<p class="mt-2 text-xs text-plum/70">{proxy.notes}</p>
						{/if}
					</td>
					<td class="py-3 pr-4">
						<span class="inline-flex rounded-full bg-pink-100 px-3 py-1 text-xs font-bold uppercase tracking-wide text-pink-700">
							{proxy.status}
						</span>
						<p class="mt-2 text-xs text-plum/55">{proxy.source}</p>
					</td>
					<td class="py-3 pr-4">
						<p class="text-plum/75">{proxy.eventCount24h} events / 24h</p>
						<p class="text-xs text-plum/55">{proxy.lastEventType ?? 'No events yet'}</p>
					</td>
					<td class="py-3 pr-4">
						<p class="text-plum/75">{proxy.lastQuarantinedAt ? new Date(proxy.lastQuarantinedAt).toLocaleString() : '—'}</p>
						<p class="mt-1 max-w-[16rem] text-xs text-red-700">{proxy.lastQuarantineReason ?? ''}</p>
					</td>
					<td class="py-3">
						<form method="POST" action="?/updateProxyStatus" class="flex flex-col gap-2">
							<input type="hidden" name="proxyId" value={proxy.id} />
							<select
								name="status"
								class="rounded-2xl border border-pink-100 bg-white px-3 py-2 text-sm text-plum"
							>
								{#each nextStatuses.filter((status) => !(proxy.status === 'quarantined' && status === 'active')) as status}
									<option value={status} selected={status === proxy.status}>{status}</option>
								{/each}
							</select>
							<input
								type="text"
								name="reason"
								class="rounded-2xl border border-pink-100 bg-white px-3 py-2 text-sm text-plum"
								placeholder="Reason"
							/>
							<button
								type="submit"
								class="rounded-2xl bg-plum px-3 py-2 text-sm font-bold text-white transition hover:bg-plum/90"
							>
								Update
							</button>
						</form>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
