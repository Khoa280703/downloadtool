<script lang="ts">
	import type { AdminJobRow } from '$lib/admin/types';
	import AdminStatusBadge from '$components/admin/AdminStatusBadge.svelte';

	let { jobs }: { jobs: AdminJobRow[] } = $props();

	function formatBytes(value: number | null): string {
		if (!value) return '—';
		if (value < 1024 * 1024) return `${Math.round(value / 1024)} KB`;
		if (value < 1024 * 1024 * 1024) return `${(value / 1024 / 1024).toFixed(1)} MB`;
		return `${(value / 1024 / 1024 / 1024).toFixed(2)} GB`;
	}
</script>

<div class="overflow-x-auto">
	<table class="min-w-full text-[13px] text-gray-700">
		<thead class="sticky top-0 z-10 border-b border-gray-200 bg-gray-50 text-left text-[10px] uppercase tracking-wider text-gray-500">
			<tr>
				<th class="px-4 py-2.5 font-semibold">Job</th>
				<th class="px-4 py-2.5 font-semibold">Status</th>
				<th class="px-4 py-2.5 font-semibold">Owner</th>
				<th class="px-4 py-2.5 font-semibold">Attempts</th>
				<th class="px-4 py-2.5 font-semibold">Artifact</th>
				<th class="px-4 py-2.5 font-semibold">Updated</th>
			</tr>
		</thead>
		<tbody class="divide-y divide-gray-100">
			{#each jobs as job}
				<tr class="align-top hover:bg-gray-50/50">
					<td class="px-4 py-3">
						<p class="max-w-[22rem] truncate font-medium text-gray-900">{job.title ?? job.id}</p>
						<p class="mt-0.5 font-mono text-[10px] text-gray-400">{job.id}</p>
						{#if job.lastError}
							<p class="mt-1.5 max-w-[26rem] text-[11px] text-red-600">{job.lastError}</p>
						{/if}
					</td>
					<td class="px-4 py-3">
						<AdminStatusBadge value={job.status} kind="job" />
					</td>
					<td class="px-4 py-3 font-mono text-[11px] text-gray-500">{job.ownerLabel}</td>
					<td class="px-4 py-3 tabular-nums text-gray-600">{job.attemptLabel}</td>
					<td class="px-4 py-3">
						<p class="text-gray-700">{job.backend ?? '—'}</p>
						<p class="text-[11px] text-gray-400">{formatBytes(job.fileSizeBytes)}</p>
					</td>
					<td class="px-4 py-3 tabular-nums text-gray-500">{new Date(job.updatedAt).toLocaleString()}</td>
				</tr>
			{/each}
			{#if jobs.length === 0}
				<tr>
					<td colspan="6" class="px-4 py-8 text-center text-sm text-gray-400">
						No jobs to display.
					</td>
				</tr>
			{/if}
		</tbody>
	</table>
</div>
