<script lang="ts">
	import type { PageData } from './$types';
	import AdminJobsTable from '$components/admin/AdminJobsTable.svelte';
	import AppIcon from '$components/AppIcon.svelte';

	let { data }: { data: PageData } = $props();

	// Pagination
	const PAGE_SIZE = 15;
	let currentPage = $state(1);
	const totalPages = $derived(Math.max(1, Math.ceil(data.jobs.length / PAGE_SIZE)));
	const paginatedJobs = $derived(data.jobs.slice((currentPage - 1) * PAGE_SIZE, currentPage * PAGE_SIZE));

	function goPage(page: number) {
		currentPage = Math.max(1, Math.min(page, totalPages));
	}
</script>

<svelte:head>
	<title>Admin Jobs</title>
</svelte:head>

<div class="mb-5">
	<h2 class="text-lg font-semibold text-gray-900">Mux Queue</h2>
	<p class="mt-0.5 text-[13px] text-gray-500">{data.jobs.length} jobs loaded · Queue status and errors</p>
</div>

<!-- KPI Strip -->
<div class="admin-panel mb-5 overflow-hidden border border-gray-200 bg-white">
	<div class="grid grid-cols-3 divide-x divide-gray-100 md:grid-cols-6">
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-amber-600">{data.overview.queuedJobs}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Queued</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-blue-600">{data.overview.leasedJobs}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Leased</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-blue-600">{data.overview.processingJobs}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Processing</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-green-700">{data.overview.readyJobs}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Ready</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-red-600">{data.overview.failedJobs}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Failed</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-gray-400">{data.overview.expiredJobs}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Expired</p>
		</div>
	</div>
</div>

<!-- Jobs Table -->
<section class="admin-panel overflow-hidden border border-gray-200 bg-white">
	<div class="flex items-center justify-between border-b border-gray-200 bg-gray-50 px-5 py-2.5">
		<div>
			<h3 class="text-[13px] font-semibold text-gray-900">Recent Mux Jobs</h3>
			<p class="text-[11px] text-gray-400">Sorted by update time, newest first.</p>
		</div>
		{#if totalPages > 1}
			<div class="flex items-center gap-1">
				<button type="button" disabled={currentPage <= 1} onclick={() => goPage(currentPage - 1)}
					class="inline-flex h-7 w-7 items-center justify-center rounded border border-gray-200 text-gray-500 transition hover:bg-gray-100 disabled:opacity-30">
					<AppIcon name="chevron_left" class="text-sm" />
				</button>
				{#each Array.from({ length: totalPages }, (_, i) => i + 1) as page}
					<button type="button" onclick={() => goPage(page)}
						class={`inline-flex h-7 min-w-7 items-center justify-center rounded px-1.5 text-[12px] font-medium tabular-nums transition ${
							page === currentPage ? 'bg-gray-900 text-white' : 'border border-gray-200 text-gray-600 hover:bg-gray-100'
						}`}>
						{page}
					</button>
				{/each}
				<button type="button" disabled={currentPage >= totalPages} onclick={() => goPage(currentPage + 1)}
					class="inline-flex h-7 w-7 items-center justify-center rounded border border-gray-200 text-gray-500 transition hover:bg-gray-100 disabled:opacity-30">
					<AppIcon name="chevron_right" class="text-sm" />
				</button>
			</div>
		{/if}
	</div>

	<AdminJobsTable jobs={paginatedJobs} />

	{#if totalPages > 1}
		<div class="flex items-center justify-between border-t border-gray-100 px-5 py-2">
			<p class="text-[11px] text-gray-400">Showing {(currentPage - 1) * PAGE_SIZE + 1}–{Math.min(currentPage * PAGE_SIZE, data.jobs.length)} of {data.jobs.length}</p>
			<p class="text-[11px] text-gray-400">Page {currentPage} of {totalPages}</p>
		</div>
	{/if}
</section>
