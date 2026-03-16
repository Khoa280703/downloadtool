<script lang="ts">
	import type { PageData } from './$types';
	import AdminJobsTable from '$components/admin/AdminJobsTable.svelte';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>Admin Jobs</title>
</svelte:head>

<div class="mb-5 flex items-center justify-between">
	<div>
		<h2 class="text-lg font-semibold text-gray-900">Mux Queue</h2>
		<p class="mt-0.5 text-[13px] text-gray-500">Job status, attempts, artifact backend, and errors.</p>
	</div>
</div>

<section class="admin-panel mb-5 overflow-hidden border border-gray-200 bg-white">
	<div class="border-b border-gray-200 bg-gray-50 px-5 py-3">
		<h3 class="text-[11px] font-semibold uppercase tracking-wider text-gray-500">Queue Summary</h3>
	</div>
	<div class="overflow-x-auto">
		<table class="w-full border-collapse text-left text-[13px]">
			<thead>
				<tr class="border-b border-gray-100 bg-gray-50/40">
					<th class="px-5 py-2.5 text-[10px] font-semibold uppercase tracking-wider text-gray-400">Status</th>
					<th class="px-5 py-2.5 text-[10px] font-semibold uppercase tracking-wider text-gray-400">Count</th>
					<th class="px-5 py-2.5 text-[10px] font-semibold uppercase tracking-wider text-gray-400">Note</th>
				</tr>
			</thead>
			<tbody class="divide-y divide-gray-50">
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Queued</td><td class="px-5 py-2.5 tabular-nums text-gray-700">{data.overview.queuedJobs}</td><td class="px-5 py-2.5 text-gray-400">Waiting for worker.</td></tr>
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Leased</td><td class="px-5 py-2.5 tabular-nums text-gray-700">{data.overview.leasedJobs}</td><td class="px-5 py-2.5 text-gray-400">Leased, not yet complete.</td></tr>
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Processing</td><td class="px-5 py-2.5 tabular-nums text-gray-700">{data.overview.processingJobs}</td><td class="px-5 py-2.5 text-gray-400">Muxing and uploading.</td></tr>
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Ready</td><td class="px-5 py-2.5 tabular-nums text-gray-700">{data.overview.readyJobs}</td><td class="px-5 py-2.5 text-gray-400">Download ticket available.</td></tr>
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Failed</td><td class="px-5 py-2.5 tabular-nums text-gray-700">{data.overview.failedJobs}</td><td class="px-5 py-2.5 text-gray-400">Needs attention.</td></tr>
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Expired</td><td class="px-5 py-2.5 tabular-nums text-gray-700">{data.overview.expiredJobs}</td><td class="px-5 py-2.5 text-gray-400">Lease or lifetime expired.</td></tr>
			</tbody>
		</table>
	</div>
</section>

<section class="admin-panel overflow-hidden border border-gray-200 bg-white">
	<div class="border-b border-gray-200 bg-gray-50 px-5 py-3">
		<h3 class="text-[13px] font-semibold text-gray-900">Recent Mux Jobs</h3>
		<p class="mt-0.5 text-[12px] text-gray-400">Last 20 jobs by update time.</p>
	</div>
	<AdminJobsTable jobs={data.jobs} />
</section>
