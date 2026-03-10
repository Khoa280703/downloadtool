<script lang="ts">
	import type { AdminActivityRow } from '$lib/admin/types';

	let { activity }: { activity: AdminActivityRow[] } = $props();
</script>

<div class="overflow-x-auto">
	<table class="min-w-full text-sm">
		<thead class="text-left text-xs uppercase tracking-[0.18em] text-plum/55">
			<tr>
				<th class="pb-3">Scope</th>
				<th class="pb-3">Target</th>
				<th class="pb-3">Event</th>
				<th class="pb-3">Detail</th>
				<th class="pb-3">Time</th>
			</tr>
		</thead>
		<tbody class="divide-y divide-pink-100/80">
			{#each activity as item}
				<tr class="align-top">
					<td class="py-3 pr-4">
						<span class="inline-flex rounded-full bg-plum/10 px-3 py-1 text-xs font-bold uppercase text-plum">
							{item.scope}
						</span>
					</td>
					<td class="py-3 pr-4">
						<p class="max-w-[20rem] truncate font-semibold text-plum">{item.label}</p>
						<p class="mt-1 font-mono text-xs text-plum/55">{item.entityId}</p>
					</td>
					<td class="py-3 pr-4 text-plum/75">{item.eventType}</td>
					<td class="py-3 pr-4 text-xs text-plum/65">{item.detail ?? '—'}</td>
					<td class="py-3 text-plum/70">{new Date(item.createdAt).toLocaleString()}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
