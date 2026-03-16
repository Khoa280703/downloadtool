<script lang="ts">
	import type { AdminActivityRow } from '$lib/admin/types';
	import AdminStatusBadge from '$components/admin/AdminStatusBadge.svelte';

	let { activity }: { activity: AdminActivityRow[] } = $props();
</script>

<div class="overflow-x-auto">
	<table class="min-w-full text-[13px] text-gray-700">
		<thead class="sticky top-0 z-10 border-b border-gray-200 bg-gray-50 text-left text-[10px] uppercase tracking-wider text-gray-500">
			<tr>
				<th class="px-4 py-2.5 font-semibold">Scope</th>
				<th class="px-4 py-2.5 font-semibold">Target</th>
				<th class="px-4 py-2.5 font-semibold">Event</th>
				<th class="px-4 py-2.5 font-semibold">Detail</th>
				<th class="px-4 py-2.5 font-semibold">Time</th>
			</tr>
		</thead>
		<tbody class="divide-y divide-gray-100">
			{#each activity as item}
				<tr class="align-top hover:bg-gray-50/50">
					<td class="px-4 py-3">
						<AdminStatusBadge value={item.scope} kind="scope" />
					</td>
					<td class="px-4 py-3">
						<p class="max-w-[18rem] truncate font-medium text-gray-900">{item.label}</p>
						<p class="mt-0.5 font-mono text-[10px] text-gray-400">{item.entityId}</p>
					</td>
					<td class="px-4 py-3 text-gray-600">{item.eventType}</td>
					<td class="px-4 py-3 text-[12px] text-gray-500">{item.detail ?? '—'}</td>
					<td class="px-4 py-3 tabular-nums text-gray-500">{new Date(item.createdAt).toLocaleString()}</td>
				</tr>
			{/each}
			{#if activity.length === 0}
				<tr>
					<td colspan="5" class="px-4 py-8 text-center text-sm text-gray-400">
						No activity to display.
					</td>
				</tr>
			{/if}
		</tbody>
	</table>
</div>
