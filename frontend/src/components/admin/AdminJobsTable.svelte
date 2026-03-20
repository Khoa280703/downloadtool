<script lang="ts">
	import type { AdminJobRow } from '$lib/admin/types';
	import AdminStatusBadge from '$components/admin/AdminStatusBadge.svelte';
	import AdminRecordDetailsModal from '$components/admin/AdminRecordDetailsModal.svelte';

	let { jobs }: { jobs: AdminJobRow[] } = $props();
	let selectedJob = $state<AdminJobRow | null>(null);

	function formatBytes(value: number | null): string {
		if (!value) return '—';
		if (value < 1024 * 1024) return `${Math.round(value / 1024)} KB`;
		if (value < 1024 * 1024 * 1024) return `${(value / 1024 / 1024).toFixed(1)} MB`;
		return `${(value / 1024 / 1024 / 1024).toFixed(2)} GB`;
	}

	function buildSummary(job: AdminJobRow) {
		return [
			{ label: 'Job ID', value: job.id },
			{ label: 'Title', value: job.title ?? '—' },
			{ label: 'Status', value: job.status },
			{ label: 'Owner', value: job.ownerLabel },
			{ label: 'Attempts', value: job.attemptLabel },
			{ label: 'Artifact Backend', value: job.backend ?? '—' },
			{ label: 'File Size', value: formatBytes(job.fileSizeBytes) },
			{ label: 'Updated At', value: new Date(job.updatedAt).toLocaleString() },
			{ label: 'Last Error', value: job.lastError ?? '—' },
			{ label: 'Source URL', value: job.sourceUrl ?? '—' }
		];
	}
</script>

<div class="overflow-x-auto">
	<table class="min-w-[760px] text-[13px] text-gray-700 md:min-w-full">
		<thead class="sticky top-0 z-10 border-b border-gray-200 bg-gray-50 text-left text-[10px] uppercase tracking-wider text-gray-500">
			<tr>
				<th class="px-4 py-2.5 font-semibold">Job</th>
				<th class="px-4 py-2.5 font-semibold">Status</th>
				<th class="px-4 py-2.5 font-semibold">Owner</th>
				<th class="px-4 py-2.5 font-semibold">Attempts</th>
				<th class="px-4 py-2.5 font-semibold">Artifact</th>
				<th class="px-4 py-2.5 font-semibold">Updated</th>
				<th class="px-4 py-2.5 font-semibold">Inspect</th>
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
					<td class="px-4 py-3">
						<button
							type="button"
							class="inline-flex items-center border border-gray-200 px-2.5 py-1 text-[11px] font-medium text-gray-700 transition hover:bg-gray-100"
							onclick={() => (selectedJob = job)}
						>
							View
						</button>
					</td>
				</tr>
			{/each}
			{#if jobs.length === 0}
				<tr>
					<td colspan="7" class="px-4 py-8 text-center text-sm text-gray-400">
						No jobs to display.
					</td>
				</tr>
			{/if}
		</tbody>
	</table>
</div>

<AdminRecordDetailsModal
	open={selectedJob !== null}
	title={selectedJob?.title ?? selectedJob?.id ?? 'Job details'}
	subtitle={selectedJob ? `Job ${selectedJob.id}` : null}
	summary={selectedJob ? buildSummary(selectedJob) : []}
	payload={selectedJob?.detailJson ?? null}
	onClose={() => (selectedJob = null)}
/>
