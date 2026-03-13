<script lang="ts">
	import type { PageData } from './$types';
	import AppIcon from '$components/AppIcon.svelte';
	import AdminActivityTable from '$components/admin/AdminActivityTable.svelte';
	import AdminJobsTable from '$components/admin/AdminJobsTable.svelte';
	import AdminProxyTable from '$components/admin/AdminProxyTable.svelte';

	let { data }: { data: PageData } = $props();

	const attentionJobs = $derived(
		data.jobs.filter((job) => ['failed', 'expired', 'queued', 'leased'].includes(job.status)).slice(0, 8)
	);
	const proxyRows = $derived(
		(data.proxies.filter((proxy) => proxy.status !== 'active').slice(0, 6).length > 0
			? data.proxies.filter((proxy) => proxy.status !== 'active').slice(0, 6)
			: data.proxies.slice(0, 6))
	);
	const recentActivity = $derived(data.activity.slice(0, 8));
</script>

<svelte:head>
	<title>Admin Overview</title>
</svelte:head>

<div class="mb-8 flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
	<div>
		<h2 class="text-3xl font-black tracking-tight text-slate-900">System Overview</h2>
		<p class="mt-1 text-sm text-slate-500">
			Theo dõi queue, proxy fleet, artifact cache và activity gần nhất của hệ thống.
		</p>
	</div>
	<a
		href="/admin/jobs"
		class="inline-flex items-center gap-2 rounded-xl bg-[#137fec] px-6 py-3 font-bold text-white shadow-lg shadow-[#137fec]/25 transition-all hover:opacity-90"
	>
		<AppIcon name="open_in_new" />
		<span>Open Queue</span>
	</a>
</div>

<section class="admin-panel mb-6 overflow-hidden border border-slate-200 bg-white">
	<div class="border-b border-slate-200 bg-slate-50 px-6 py-4">
		<h3 class="text-xs font-bold uppercase tracking-wider text-slate-500">System Summary</h3>
	</div>
	<div class="overflow-x-auto">
		<table class="w-full border-collapse text-left">
			<thead>
				<tr class="border-b border-slate-200 bg-slate-50/60">
					<th class="px-6 py-4 text-xs font-bold uppercase tracking-wider text-slate-500">Metric</th>
					<th class="px-6 py-4 text-xs font-bold uppercase tracking-wider text-slate-500">Value</th>
					<th class="px-6 py-4 text-xs font-bold uppercase tracking-wider text-slate-500">Detail</th>
				</tr>
			</thead>
			<tbody class="divide-y divide-slate-100 text-sm">
				<tr>
					<td class="px-6 py-4 font-semibold text-slate-900">Queued Jobs</td>
					<td class="px-6 py-4 text-slate-700">{data.overview.queuedJobs}</td>
					<td class="px-6 py-4 text-slate-500">Chưa được worker nhận lease.</td>
				</tr>
				<tr>
					<td class="px-6 py-4 font-semibold text-slate-900">Processing Jobs</td>
					<td class="px-6 py-4 text-slate-700">{data.overview.processingJobs}</td>
					<td class="px-6 py-4 text-slate-500">Đang mux và upload artifact.</td>
				</tr>
				<tr>
					<td class="px-6 py-4 font-semibold text-slate-900">Ready Jobs</td>
					<td class="px-6 py-4 text-slate-700">{data.overview.readyJobs}</td>
					<td class="px-6 py-4 text-slate-500">Có thể cấp ticket tải xuống.</td>
				</tr>
				<tr>
					<td class="px-6 py-4 font-semibold text-slate-900">Failed / Expired</td>
					<td class="px-6 py-4 text-slate-700">{data.overview.failedJobs + data.overview.expiredJobs}</td>
					<td class="px-6 py-4 text-slate-500">Job cần attention hoặc retry logic.</td>
				</tr>
				<tr>
					<td class="px-6 py-4 font-semibold text-slate-900">Ready Artifacts</td>
					<td class="px-6 py-4 text-slate-700">{data.overview.readyArtifacts}</td>
					<td class="px-6 py-4 text-slate-500">Dung lượng cache file sẵn sàng để reuse.</td>
				</tr>
				<tr>
					<td class="px-6 py-4 font-semibold text-slate-900">Proxy Fleet</td>
					<td class="px-6 py-4 text-slate-700">
						{data.overview.activeProxies + data.overview.quarantinedProxies + data.overview.disabledProxies}
					</td>
					<td class="px-6 py-4 text-slate-500">
						{data.overview.activeProxies} active, {data.overview.quarantinedProxies} quarantined, {data.overview.disabledProxies} disabled.
					</td>
				</tr>
				<tr>
					<td class="px-6 py-4 font-semibold text-slate-900">Events / 24h</td>
					<td class="px-6 py-4 text-slate-700">{data.overview.eventsLast24h}</td>
					<td class="px-6 py-4 text-slate-500">Tổng job events và proxy health events.</td>
				</tr>
			</tbody>
		</table>
	</div>
</section>

<section class="admin-panel mb-6 overflow-hidden border border-slate-200 bg-white">
	<div class="flex items-center justify-between border-b border-slate-200 bg-slate-50 px-6 py-4">
		<div>
			<h3 class="text-sm font-bold text-slate-900">Jobs Requiring Attention</h3>
			<p class="mt-1 text-sm text-slate-500">Ưu tiên xử lý các job lỗi, expired, queued hoặc còn treo lease.</p>
		</div>
		<a href="/admin/jobs" class="text-sm font-semibold text-[#137fec] hover:underline">View all</a>
	</div>
	<AdminJobsTable jobs={attentionJobs} />
</section>

<div class="grid grid-cols-1 gap-6 xl:grid-cols-2">
	<section class="admin-panel overflow-hidden border border-slate-200 bg-white">
		<div class="flex items-center justify-between border-b border-slate-200 bg-slate-50 px-6 py-4">
			<div>
				<h3 class="text-sm font-bold text-slate-900">Proxy Fleet</h3>
				<p class="mt-1 text-sm text-slate-500">Danh sách proxy cần kiểm tra hoặc nhóm gần nhất.</p>
			</div>
			<a href="/admin/proxies" class="text-sm font-semibold text-[#137fec] hover:underline">Manage</a>
		</div>
		<AdminProxyTable proxies={proxyRows} />
	</section>

	<section class="admin-panel overflow-hidden border border-slate-200 bg-white">
		<div class="flex items-center justify-between border-b border-slate-200 bg-slate-50 px-6 py-4">
			<div>
				<h3 class="text-sm font-bold text-slate-900">Recent Activity</h3>
				<p class="mt-1 text-sm text-slate-500">Các event mới nhất từ jobs và proxies.</p>
			</div>
			<a href="/admin/activity" class="text-sm font-semibold text-[#137fec] hover:underline">View all</a>
		</div>
		<AdminActivityTable activity={recentActivity} />
	</section>
</div>
