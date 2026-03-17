<script lang="ts">
	import type { AdminActivityRow } from '$lib/admin/types';
	import AdminStatusBadge from '$components/admin/AdminStatusBadge.svelte';
	import AdminRecordDetailsModal from '$components/admin/AdminRecordDetailsModal.svelte';

	let { activity }: { activity: AdminActivityRow[] } = $props();
	let selectedActivity = $state<AdminActivityRow | null>(null);

	function outcomeTone(outcome: string | null): 'neutral' | 'sky' | 'emerald' | 'amber' | 'rose' {
		switch (outcome) {
			case 'success':
				return 'emerald';
			case 'denied':
			case 'rate_limited':
				return 'amber';
			case 'error':
			case 'failure':
			case 'not_found':
				return 'rose';
			default:
				return 'neutral';
		}
	}

	function buildSummary(item: AdminActivityRow) {
		return [
			{ label: 'Source', value: item.source },
			{ label: 'Scope', value: item.scope },
			{ label: 'Event', value: item.eventType },
			{ label: 'Actor', value: item.actorLabel ?? '—' },
			{ label: 'Client IP', value: item.clientIp ?? '—' },
			{ label: 'Target', value: item.label },
			{ label: 'Entity ID', value: item.entityId ?? '—' },
			{ label: 'Route', value: item.routePath ?? '—' },
			{ label: 'Method', value: item.method ?? '—' },
			{ label: 'Outcome', value: item.outcome ?? '—' },
			{ label: 'Status Code', value: item.statusCode ?? '—' },
			{ label: 'Detail', value: item.detail ?? '—' },
			{ label: 'Created At', value: new Date(item.createdAt).toLocaleString() }
		];
	}
</script>

<div class="overflow-x-auto">
	<table class="min-w-full text-[13px] text-gray-700">
		<thead class="sticky top-0 z-10 border-b border-gray-200 bg-gray-50 text-left text-[10px] uppercase tracking-wider text-gray-500">
			<tr>
				<th class="px-4 py-2.5 font-semibold">Scope</th>
				<th class="px-4 py-2.5 font-semibold">Actor</th>
				<th class="px-4 py-2.5 font-semibold">Target</th>
				<th class="px-4 py-2.5 font-semibold">Event</th>
				<th class="px-4 py-2.5 font-semibold">Route</th>
				<th class="px-4 py-2.5 font-semibold">Result</th>
				<th class="px-4 py-2.5 font-semibold">Detail</th>
				<th class="px-4 py-2.5 font-semibold">Time</th>
				<th class="px-4 py-2.5 font-semibold">Inspect</th>
			</tr>
		</thead>
		<tbody class="divide-y divide-gray-100">
			{#each activity as item}
				<tr class="align-top hover:bg-gray-50/50">
					<td class="px-4 py-3">
						<AdminStatusBadge value={item.scope} kind="scope" />
					</td>
					<td class="px-4 py-3">
						<p class="max-w-[14rem] truncate font-medium text-gray-900">{item.actorLabel ?? '—'}</p>
						<p class="mt-0.5 font-mono text-[10px] text-gray-400">{item.clientIp ?? '—'}</p>
					</td>
					<td class="px-4 py-3">
						<p class="max-w-[18rem] truncate font-medium text-gray-900">{item.label}</p>
						<p class="mt-0.5 font-mono text-[10px] text-gray-400">{item.entityId ?? '—'}</p>
					</td>
					<td class="px-4 py-3 text-gray-600">
						<p>{item.eventType}</p>
						<p class="mt-0.5 text-[10px] uppercase tracking-wide text-gray-400">{item.source}</p>
					</td>
					<td class="px-4 py-3 text-[12px] text-gray-500">
						<p class="max-w-[16rem] truncate font-mono text-[11px] text-gray-600">{item.routePath ?? '—'}</p>
						<p class="mt-0.5 text-[10px] uppercase tracking-wide text-gray-400">{item.method ?? '—'}</p>
					</td>
					<td class="px-4 py-3">
						<div class="flex flex-col items-start gap-1">
							<AdminStatusBadge
								kind="tone"
								value={item.outcome ?? 'info'}
								tone={outcomeTone(item.outcome)}
							/>
							<p class="font-mono text-[10px] text-gray-400">{item.statusCode ?? '—'}</p>
						</div>
					</td>
					<td class="px-4 py-3 text-[12px] text-gray-500">{item.detail ?? '—'}</td>
					<td class="px-4 py-3 tabular-nums text-gray-500">{new Date(item.createdAt).toLocaleString()}</td>
					<td class="px-4 py-3">
						<button
							type="button"
							class="inline-flex items-center border border-gray-200 px-2.5 py-1 text-[11px] font-medium text-gray-700 transition hover:bg-gray-100"
							onclick={() => (selectedActivity = item)}
						>
							View
						</button>
					</td>
				</tr>
			{/each}
			{#if activity.length === 0}
				<tr>
					<td colspan="9" class="px-4 py-8 text-center text-sm text-gray-400">
						No activity to display.
					</td>
				</tr>
			{/if}
		</tbody>
	</table>
</div>

<AdminRecordDetailsModal
	open={selectedActivity !== null}
	title={selectedActivity?.label ?? selectedActivity?.eventType ?? 'Activity details'}
	subtitle={selectedActivity ? `${selectedActivity.eventType} · ${selectedActivity.source}` : null}
	summary={selectedActivity ? buildSummary(selectedActivity) : []}
	payload={selectedActivity?.detailJson ?? null}
	onClose={() => (selectedActivity = null)}
/>
