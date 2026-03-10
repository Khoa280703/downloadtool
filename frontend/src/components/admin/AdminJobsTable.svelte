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
	<table class="admin-data-table min-w-full text-sm text-slate-700">
		<thead class="sticky top-0 z-10 bg-slate-100 text-left text-[10px] uppercase tracking-[0.18em] text-slate-500">
			<tr>
				<th class="px-4 py-3 font-bold">Job</th>
				<th class="px-4 py-3 font-bold">Status</th>
				<th class="px-4 py-3 font-bold">Owner</th>
				<th class="px-4 py-3 font-bold">Attempts</th>
				<th class="px-4 py-3 font-bold">Artifact</th>
				<th class="px-4 py-3 font-bold">Updated</th>
			</tr>
		</thead>
		<tbody class="divide-y divide-slate-200/80">
			{#each jobs as job}
				<tr class="align-top transition hover:bg-slate-50/80">
					<td class="px-4 py-3.5 pr-2">
						<p class="max-w-[24rem] truncate text-sm font-semibold text-slate-900">{job.title ?? job.id}</p>
						<p class="mt-1 font-mono text-[11px] text-slate-500">{job.id}</p>
						{#if job.lastError}
							<p class="mt-2 max-w-[28rem] text-xs text-rose-700">{job.lastError}</p>
						{/if}
					</td>
					<td class="px-4 py-3.5 pr-2">
						<AdminStatusBadge value={job.status} kind="job" />
					</td>
					<td class="px-4 py-3.5 pr-2 font-mono text-xs text-slate-600">{job.ownerLabel}</td>
					<td class="px-4 py-3.5 pr-2 text-slate-600">{job.attemptLabel}</td>
					<td class="px-4 py-3.5 pr-2">
						<p class="text-sm text-slate-700">{job.backend ?? '—'}</p>
						<p class="text-xs text-slate-500">{formatBytes(job.fileSizeBytes)}</p>
					</td>
					<td class="px-4 py-3.5 text-slate-600">{new Date(job.updatedAt).toLocaleString()}</td>
				</tr>
			{/each}
			{#if jobs.length === 0}
				<tr>
					<td colspan="6" class="px-4 py-10 text-center text-sm text-slate-500">
						Chưa có job nào trong phạm vi hiển thị.
					</td>
				</tr>
			{/if}
		</tbody>
	</table>
</div>
