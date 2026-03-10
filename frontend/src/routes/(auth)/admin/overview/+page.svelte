<script lang="ts">
	import type { PageData } from './$types';
	import AdminActivityTable from '$components/admin/AdminActivityTable.svelte';
	import AdminBarChart from '$components/admin/AdminBarChart.svelte';
	import AdminJobsTable from '$components/admin/AdminJobsTable.svelte';
	import AdminMiniMetric from '$components/admin/AdminMiniMetric.svelte';
	import AdminProxyTable from '$components/admin/AdminProxyTable.svelte';
	import AdminSectionHeader from '$components/admin/AdminSectionHeader.svelte';
	import AdminStatCard from '$components/admin/AdminStatCard.svelte';
	import { buildAdminDashboardViewModel } from '$lib/admin/dashboard-view-model';

	let { data }: { data: PageData } = $props();

	const model = $derived(buildAdminDashboardViewModel(data.overview));
	const queueChart = $derived([
		{ label: 'Queued', value: data.overview.queuedJobs, tone: 'amber' as const },
		{ label: 'Leased', value: data.overview.leasedJobs, tone: 'blue' as const },
		{ label: 'Processing', value: data.overview.processingJobs, tone: 'blue' as const },
		{ label: 'Ready', value: data.overview.readyJobs, tone: 'green' as const },
		{ label: 'Failed', value: data.overview.failedJobs, tone: 'red' as const },
		{ label: 'Expired', value: data.overview.expiredJobs, tone: 'red' as const }
	]);
	const proxyChart = $derived([
		{ label: 'Active', value: data.overview.activeProxies, tone: 'green' as const },
		{ label: 'Quarantined', value: data.overview.quarantinedProxies, tone: 'red' as const },
		{ label: 'Disabled', value: data.overview.disabledProxies, tone: 'amber' as const }
	]);
	const attentionJobs = $derived(
		data.jobs.filter((job) => ['failed', 'expired', 'queued', 'leased'].includes(job.status)).slice(0, 8)
	);
	const proxyAlerts = $derived(data.proxies.filter((proxy) => proxy.status !== 'active').slice(0, 8));
	const recentActivity = $derived(data.activity.slice(0, 10));
</script>

<svelte:head>
	<title>Admin Overview</title>
</svelte:head>

<header class="admin-panel border border-slate-200 bg-white px-5 py-5 md:px-6">
	<div class="flex flex-col gap-4 xl:flex-row xl:items-end xl:justify-between">
		<div>
			<p class="text-[10px] font-bold uppercase tracking-[0.22em] text-slate-500">Overview</p>
			<h2 class="mt-1 text-2xl font-bold tracking-[-0.03em] text-slate-950">Operational dashboard</h2>
			<p class="mt-2 max-w-3xl text-sm leading-6 text-slate-600">
				Bảng điều hành chính cho queue mux, proxy fleet, artifact storage và activity stream.
			</p>
		</div>
		<div class="grid gap-2 sm:grid-cols-3 xl:min-w-[460px]">
			<AdminMiniMetric label="Backlog" value={model.queueBacklog} caption="queued + leased" />
			<AdminMiniMetric label="Artifacts" value={model.totalArtifacts} caption="building + ready" />
			<AdminMiniMetric label="Events / 24h" value={data.overview.eventsLast24h} caption="jobs + proxies" />
		</div>
	</div>
</header>

<section class="admin-panel border border-slate-200 bg-white p-5 md:p-6">
	<AdminSectionHeader
		eyebrow="System overview"
		title="Primary signals"
		description="Nhóm KPI cốt lõi để nhìn thấy backlog, failure, cache artifact và độ phủ proxy."
	/>
	<div class="mt-5 grid gap-4 md:grid-cols-2 2xl:grid-cols-3">
		{#each model.topStats as stat}
			<AdminStatCard
				label={stat.label}
				value={stat.value}
				caption={stat.caption}
				tone={stat.tone}
			/>
		{/each}
	</div>
</section>

<div class="grid gap-5 xl:grid-cols-2">
	<AdminBarChart
		title="Queue distribution"
		description="Phân bố trạng thái hiện tại của mux jobs."
		items={queueChart}
	/>
	<AdminBarChart
		title="Proxy fleet distribution"
		description="Phân bố inventory theo trạng thái phục vụ."
		items={proxyChart}
	/>
</div>

<div class="grid gap-5 xl:grid-cols-2">
	<section class="admin-panel border border-slate-200 bg-white">
		<div class="border-b border-slate-200 px-5 py-4 md:px-6">
			<AdminSectionHeader
				eyebrow="Jobs"
				title="Needs attention"
				description="Ưu tiên các job đang lỗi, hết hạn, chưa được worker lấy hoặc còn treo lease."
			/>
		</div>
		<AdminJobsTable jobs={attentionJobs} />
	</section>

	<section class="admin-panel border border-slate-200 bg-white">
		<div class="border-b border-slate-200 px-5 py-4 md:px-6">
			<AdminSectionHeader
				eyebrow="Proxies"
				title="Proxy issues"
				description="Danh sách proxy không ở trạng thái active để operator xử lý nhanh."
			/>
		</div>
		<AdminProxyTable proxies={proxyAlerts} />
	</section>
</div>

<div class="grid gap-5 2xl:grid-cols-[minmax(0,1.25fr)_minmax(0,0.75fr)]">
	<section class="admin-panel border border-slate-200 bg-white">
		<div class="border-b border-slate-200 px-5 py-4 md:px-6">
			<AdminSectionHeader
				eyebrow="Activity"
				title="Recent activity stream"
				description="Activity feed hợp nhất từ mux jobs và proxy health events."
			/>
		</div>
		<AdminActivityTable activity={recentActivity} />
	</section>

	<section class="admin-panel border border-slate-200 bg-white p-5">
		<AdminSectionHeader
			eyebrow="Snapshot"
			title="Operational notes"
			description="Các vùng áp lực chính theo góc nhìn vận hành hiện tại."
		/>
		<div class="mt-5 grid gap-3">
			{#each model.snapshotStats as stat}
				<AdminMiniMetric label={stat.label} value={stat.value} caption={stat.caption} />
			{/each}
		</div>
		<div class="mt-5 border-t border-slate-200 pt-5">
			<p class="text-[10px] font-bold uppercase tracking-[0.2em] text-slate-500">Operator guidance</p>
			<ul class="mt-3 space-y-3 text-sm text-slate-600">
				<li>Nếu failed + expired tăng liên tục, kiểm tra proxy health và upstream extract trước.</li>
				<li>Nếu building artifacts cao nhưng ready tăng chậm, tập trung vào worker throughput hoặc storage.</li>
				<li>Nếu quarantined proxies chiếm tỷ trọng lớn, inventory hiện tại đang hụt độ ổn định.</li>
			</ul>
		</div>
	</section>
</div>
