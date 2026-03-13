<script lang="ts">
	import type { PageData } from './$types';
	import AppIcon from '$components/AppIcon.svelte';
	import AdminJobsTable from '$components/admin/AdminJobsTable.svelte';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>Admin Jobs</title>
</svelte:head>

<div class="mb-8 flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
	<div>
		<h2 class="text-3xl font-black tracking-tight text-slate-900">Mux Queue</h2>
		<p class="mt-1 text-sm text-slate-500">Theo dõi trạng thái, attempts, artifact backend và lỗi của các job gần nhất.</p>
	</div>
	<div class="inline-flex items-center gap-2 rounded-xl bg-[#137fec] px-5 py-3 text-sm font-bold text-white shadow-lg shadow-[#137fec]/25">
		<AppIcon name="table_rows" />
		<span>Recent jobs</span>
	</div>
</div>

<section class="admin-panel mb-6 overflow-hidden border border-slate-200 bg-white">
	<div class="border-b border-slate-200 bg-slate-50 px-6 py-4">
		<h3 class="text-xs font-bold uppercase tracking-wider text-slate-500">Queue Summary</h3>
	</div>
	<div class="overflow-x-auto">
		<table class="w-full border-collapse text-left">
			<thead>
				<tr class="border-b border-slate-200 bg-slate-50/60">
					<th class="px-6 py-4 text-xs font-bold uppercase tracking-wider text-slate-500">Status</th>
					<th class="px-6 py-4 text-xs font-bold uppercase tracking-wider text-slate-500">Count</th>
					<th class="px-6 py-4 text-xs font-bold uppercase tracking-wider text-slate-500">Meaning</th>
				</tr>
			</thead>
			<tbody class="divide-y divide-slate-100 text-sm">
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Queued</td><td class="px-6 py-4">{data.overview.queuedJobs}</td><td class="px-6 py-4 text-slate-500">Chờ worker nhận.</td></tr>
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Leased</td><td class="px-6 py-4">{data.overview.leasedJobs}</td><td class="px-6 py-4 text-slate-500">Đã có worker lease nhưng chưa hoàn tất.</td></tr>
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Processing</td><td class="px-6 py-4">{data.overview.processingJobs}</td><td class="px-6 py-4 text-slate-500">Đang mux/upload.</td></tr>
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Ready</td><td class="px-6 py-4">{data.overview.readyJobs}</td><td class="px-6 py-4 text-slate-500">Sẵn sàng cấp file ticket.</td></tr>
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Failed</td><td class="px-6 py-4">{data.overview.failedJobs}</td><td class="px-6 py-4 text-slate-500">Thất bại cần attention.</td></tr>
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Expired</td><td class="px-6 py-4">{data.overview.expiredJobs}</td><td class="px-6 py-4 text-slate-500">Lease hoặc lifetime đã hết.</td></tr>
			</tbody>
		</table>
	</div>
</section>

<section class="admin-panel overflow-hidden border border-slate-200 bg-white">
	<div class="border-b border-slate-200 bg-slate-50 px-6 py-4">
		<h3 class="text-sm font-bold text-slate-900">Recent Mux Jobs</h3>
		<p class="mt-1 text-sm text-slate-500">20 job mới nhất theo thời gian cập nhật.</p>
	</div>
	<AdminJobsTable jobs={data.jobs} />
</section>
