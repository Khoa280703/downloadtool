<script lang="ts">
	import type { PageData } from './$types';
	import AdminActivityTable from '$components/admin/AdminActivityTable.svelte';
	import AppIcon from '$components/AppIcon.svelte';

	let { data }: { data: PageData } = $props();

	const auditEvents = $derived(data.activity.filter((e) => e.source === 'audit').length);
	const jobEvents = $derived(data.activity.filter((e) => e.source === 'job').length);
	const proxyEvents = $derived(data.activity.filter((e) => e.source === 'proxy').length);

	// Pagination
	const PAGE_SIZE = 20;
	let currentPage = $state(1);
	const totalPages = $derived(Math.max(1, Math.ceil(data.activity.length / PAGE_SIZE)));
	const paginatedActivity = $derived(data.activity.slice((currentPage - 1) * PAGE_SIZE, currentPage * PAGE_SIZE));

	function goPage(page: number) {
		currentPage = Math.max(1, Math.min(page, totalPages));
	}
</script>

<svelte:head>
	<title>Admin Activity</title>
</svelte:head>

<div class="mb-5">
	<h2 class="text-lg font-semibold text-gray-900">System Activity</h2>
	<p class="mt-0.5 text-[13px] text-gray-500">Audit + job + proxy events, newest first</p>
</div>

<!-- KPI Strip -->
<div class="admin-panel mb-5 overflow-hidden border border-gray-200 bg-white">
	<div class="grid grid-cols-2 divide-x divide-y divide-gray-100 sm:grid-cols-4 sm:divide-y-0">
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-gray-900">{data.activity.length}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Total Events</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-emerald-600">{auditEvents}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Audit Events</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-blue-600">{jobEvents}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Job Events</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-amber-600">{proxyEvents}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Proxy Events</p>
		</div>
	</div>
</div>

<!-- Activity Table -->
<section class="admin-panel overflow-hidden border border-gray-200 bg-white">
	<div class="flex flex-col gap-3 border-b border-gray-200 bg-gray-50 px-4 py-3 sm:flex-row sm:items-center sm:justify-between sm:px-5 sm:py-2.5">
		<div>
			<h3 class="text-[13px] font-semibold text-gray-900">Event Feed</h3>
			<p class="text-[11px] text-gray-400">Combined audit + job + proxy events, newest first.</p>
		</div>
		{#if totalPages > 1}
			<div class="flex flex-wrap items-center gap-1">
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

	<AdminActivityTable activity={paginatedActivity} />

	{#if totalPages > 1}
		<div class="flex flex-col gap-1 border-t border-gray-100 px-4 py-2 text-[11px] text-gray-400 sm:flex-row sm:items-center sm:justify-between sm:px-5">
			<p>Showing {(currentPage - 1) * PAGE_SIZE + 1}–{Math.min(currentPage * PAGE_SIZE, data.activity.length)} of {data.activity.length}</p>
			<p>Page {currentPage} of {totalPages}</p>
		</div>
	{/if}
</section>
