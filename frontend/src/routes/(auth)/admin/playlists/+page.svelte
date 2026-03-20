<script lang="ts">
	import type { PageData } from './$types';
	import type { AdminPlaylistJobRow } from '$lib/admin/types';
	import AppIcon from '$components/AppIcon.svelte';
	import AdminRecordDetailsModal from '$components/admin/AdminRecordDetailsModal.svelte';

	let { data }: { data: PageData } = $props();
	let selectedJob = $state<AdminPlaylistJobRow | null>(null);

	const PAGE_SIZE = 15;
	let currentPage = $state(1);
	const totalPages = $derived(Math.max(1, Math.ceil(data.playlistJobs.length / PAGE_SIZE)));
	const paginated = $derived(data.playlistJobs.slice((currentPage - 1) * PAGE_SIZE, currentPage * PAGE_SIZE));

	function goPage(page: number) {
		currentPage = Math.max(1, Math.min(page, totalPages));
	}

	function statusColor(status: string): string {
		switch (status) {
			case 'completed': return 'text-green-700 bg-green-50';
			case 'failed': return 'text-red-600 bg-red-50';
			case 'cancelled': return 'text-gray-500 bg-gray-100';
			case 'processing': return 'text-blue-600 bg-blue-50';
			case 'discovering': return 'text-amber-600 bg-amber-50';
			case 'queued': return 'text-amber-600 bg-amber-50';
			default: return 'text-gray-600 bg-gray-100';
		}
	}

	function progressPercent(completed: number, failed: number, total: number): number {
		if (total <= 0) return 0;
		return Math.round(((completed + failed) / total) * 100);
	}

	function formatTime(iso: string): string {
		try {
			return new Date(iso).toLocaleString('en-GB', { day: '2-digit', month: 'short', hour: '2-digit', minute: '2-digit' });
		} catch {
			return iso;
		}
	}
</script>

<svelte:head>
	<title>Admin Playlists</title>
</svelte:head>

<div class="mb-5">
	<h2 class="text-lg font-semibold text-gray-900">Playlist Jobs</h2>
	<p class="mt-0.5 text-[13px] text-gray-500">{data.playlistJobs.length} jobs loaded</p>
</div>

<!-- KPI Strip -->
<div class="admin-panel mb-5 overflow-hidden border border-gray-200 bg-white">
	<div class="grid grid-cols-1 divide-y divide-gray-100 sm:grid-cols-3 sm:divide-x sm:divide-y-0">
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-blue-600">{data.overview.playlistActiveJobs}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Active</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-green-700">{data.overview.playlistCompletedJobs24h}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Completed 24h</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-red-600">{data.overview.playlistFailedJobs24h}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Failed 24h</p>
		</div>
	</div>
</div>

<!-- Table -->
<div class="admin-panel overflow-hidden border border-gray-200 bg-white">
	<div class="overflow-x-auto">
		<table class="min-w-[760px] w-full text-left text-[13px]">
			<thead>
				<tr class="border-b border-gray-100 bg-gray-50 text-[10px] font-semibold uppercase tracking-wider text-gray-400">
					<th class="px-4 py-2.5">ID</th>
					<th class="px-4 py-2.5">Title / URL</th>
					<th class="px-4 py-2.5">Status</th>
					<th class="px-4 py-2.5">Progress</th>
					<th class="px-4 py-2.5">Quality</th>
					<th class="px-4 py-2.5">Owner</th>
					<th class="px-4 py-2.5">Updated</th>
				</tr>
			</thead>
			<tbody class="divide-y divide-gray-50">
				{#each paginated as job}
					<tr class="hover:bg-gray-50/60 transition-colors cursor-pointer" onclick={() => { selectedJob = job; }}>
						<td class="px-4 py-2.5 font-mono text-[11px] text-gray-500">{job.id.slice(0, 16)}</td>
						<td class="max-w-[240px] truncate px-4 py-2.5" title={job.sourceUrl}>
							{job.title ?? job.sourceUrl}
						</td>
						<td class="px-4 py-2.5">
							<span class="inline-block rounded px-1.5 py-0.5 text-[11px] font-semibold {statusColor(job.status)}">
								{job.status}
							</span>
						</td>
						<td class="px-4 py-2.5 tabular-nums">
							<span class="text-green-700">{job.completedItems}</span>
							{#if job.failedItems > 0}
								/ <span class="text-red-600">{job.failedItems}f</span>
							{/if}
							/ {job.totalItems}
							{#if job.totalItems > 0}
								<span class="ml-1 text-[11px] text-gray-400">({progressPercent(job.completedItems, job.failedItems, job.totalItems)}%)</span>
							{/if}
						</td>
						<td class="px-4 py-2.5 text-gray-500">{job.requestedQuality} · {job.requestedMode}</td>
						<td class="max-w-[120px] truncate px-4 py-2.5 text-gray-500" title={job.ownerLabel}>{job.ownerLabel}</td>
						<td class="px-4 py-2.5 text-gray-400">{formatTime(job.updatedAt)}</td>
					</tr>
				{:else}
					<tr>
						<td colspan="7" class="px-4 py-8 text-center text-gray-400">No playlist jobs found</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>

	{#if totalPages > 1}
		<div class="flex flex-col gap-2 border-t border-gray-100 px-4 py-2.5 text-[12px] text-gray-500 sm:flex-row sm:items-center sm:justify-between">
			<span>Page {currentPage} / {totalPages}</span>
			<div class="flex gap-1">
				<button onclick={() => goPage(currentPage - 1)} disabled={currentPage <= 1}
					class="rounded border border-gray-200 px-2 py-1 hover:bg-gray-50 disabled:opacity-40">
					<AppIcon name="chevron_left" class="text-sm" />
				</button>
				<button onclick={() => goPage(currentPage + 1)} disabled={currentPage >= totalPages}
					class="rounded border border-gray-200 px-2 py-1 hover:bg-gray-50 disabled:opacity-40">
					<AppIcon name="chevron_right" class="text-sm" />
				</button>
			</div>
		</div>
	{/if}
</div>

{#if selectedJob}
	<AdminRecordDetailsModal
		open={!!selectedJob}
		title={selectedJob.title ?? selectedJob.id}
		subtitle={selectedJob.sourceUrl}
		summary={[
			{ label: 'Job ID', value: selectedJob.id },
			{ label: 'Status', value: selectedJob.status },
			{ label: 'Completed', value: selectedJob.completedItems },
			{ label: 'Failed', value: selectedJob.failedItems },
			{ label: 'Total', value: selectedJob.totalItems },
			{ label: 'Quality', value: selectedJob.requestedQuality },
			{ label: 'Mode', value: selectedJob.requestedMode },
			{ label: 'Owner', value: selectedJob.ownerLabel },
			{ label: 'Created', value: selectedJob.createdAt },
			{ label: 'Updated', value: selectedJob.updatedAt }
		]}
		onClose={() => { selectedJob = null; }}
	/>
{/if}
