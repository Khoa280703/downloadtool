<script lang="ts">
	import type { AdminActivityRow } from '$lib/admin/types';
	import AdminStatusBadge from '$components/admin/AdminStatusBadge.svelte';

	let { activity }: { activity: AdminActivityRow[] } = $props();
</script>

<div class="overflow-x-auto">
	<table class="admin-data-table min-w-full text-sm text-slate-700">
		<thead class="sticky top-0 z-10 bg-slate-50/90 text-left text-[11px] uppercase tracking-[0.18em] text-slate-500 backdrop-blur">
			<tr>
				<th class="px-4 py-3 font-bold">Scope</th>
				<th class="px-4 py-3 font-bold">Target</th>
				<th class="px-4 py-3 font-bold">Event</th>
				<th class="px-4 py-3 font-bold">Detail</th>
				<th class="px-4 py-3 font-bold">Time</th>
			</tr>
		</thead>
		<tbody class="divide-y divide-slate-200/80">
			{#each activity as item}
				<tr class="align-top transition hover:bg-slate-50/80">
					<td class="px-4 py-4 pr-2">
						<AdminStatusBadge value={item.scope} kind="scope" />
					</td>
					<td class="px-4 py-4 pr-2">
						<p class="max-w-[20rem] truncate text-sm font-semibold text-slate-900">{item.label}</p>
						<p class="mt-1 font-mono text-[11px] text-slate-500">{item.entityId}</p>
					</td>
					<td class="px-4 py-4 pr-2 text-sm text-slate-700">{item.eventType}</td>
					<td class="px-4 py-4 pr-2 text-xs text-slate-600">{item.detail ?? '—'}</td>
					<td class="px-4 py-4 text-slate-600">{new Date(item.createdAt).toLocaleString()}</td>
				</tr>
			{/each}
			{#if activity.length === 0}
				<tr>
					<td colspan="5" class="px-4 py-10 text-center text-sm text-slate-500">
						Chưa có activity nào để hiển thị.
					</td>
				</tr>
			{/if}
		</tbody>
	</table>
</div>
