<script lang="ts">
	import type { PageData } from './$types';
	import AdminBarChart from '$components/admin/AdminBarChart.svelte';
	import AdminJobsTable from '$components/admin/AdminJobsTable.svelte';
	import AdminMiniMetric from '$components/admin/AdminMiniMetric.svelte';
	import AdminSectionHeader from '$components/admin/AdminSectionHeader.svelte';
	import AdminStatCard from '$components/admin/AdminStatCard.svelte';
	import { buildAdminDashboardViewModel } from '$lib/admin/dashboard-view-model';

	let { data }: { data: PageData } = $props();

	const model = $derived(buildAdminDashboardViewModel(data.overview));
	const jobChartItems = $derived([
		{ label: 'Queued', value: data.overview.queuedJobs, tone: 'amber' as const },
		{ label: 'Leased', value: data.overview.leasedJobs, tone: 'blue' as const },
		{ label: 'Processing', value: data.overview.processingJobs, tone: 'blue' as const },
		{ label: 'Ready', value: data.overview.readyJobs, tone: 'green' as const },
		{ label: 'Failed', value: data.overview.failedJobs, tone: 'red' as const },
		{ label: 'Expired', value: data.overview.expiredJobs, tone: 'red' as const }
	]);
</script>

<svelte:head>
	<title>Admin Jobs</title>
</svelte:head>

<section class="admin-panel rounded-xl border border-slate-200 bg-white">
	<div class="border-b border-slate-200 px-5 py-5 md:px-6">
		<div class="flex flex-col gap-4 xl:flex-row xl:items-end xl:justify-between">
			<AdminSectionHeader
				eyebrow="Jobs"
				title="Mux queue operations"
				description="Theo dõi trạng thái queue, retry pressure và nhịp hoàn thành của worker plane."
			/>
			<div class="grid gap-2 sm:grid-cols-3">
				{#each model.queueStats as stat}
					<AdminMiniMetric label={stat.label} value={stat.value} />
				{/each}
			</div>
		</div>
	</div>

	<div class="grid gap-4 px-5 py-5 md:grid-cols-2 xl:grid-cols-4 md:px-6">
		<AdminStatCard label="Queue backlog" value={model.queueBacklog} caption="Queued + leased" tone={model.queueBacklog > 0 ? 'amber' : 'neutral'} />
		<AdminStatCard label="Active load" value={model.activeJobs} caption="Processing + leased" tone={model.activeJobs > 0 ? 'sky' : 'neutral'} />
		<AdminStatCard label="Ready jobs" value={data.overview.readyJobs} caption="Đã hoàn tất trong queue" tone="emerald" />
		<AdminStatCard label="Failure surface" value={data.overview.failedJobs + data.overview.expiredJobs} caption="Failed + expired" tone={data.overview.failedJobs + data.overview.expiredJobs > 0 ? 'rose' : 'neutral'} />
	</div>
</section>

<div class="mt-6 grid gap-6 xl:grid-cols-[360px_minmax(0,1fr)]">
	<AdminBarChart
		title="Job state distribution"
		description="Phân bố hiện tại của toàn bộ pipeline jobs."
		items={jobChartItems}
	/>

	<section class="admin-panel rounded-xl border border-slate-200 bg-white">
		<div class="border-b border-slate-200 px-5 py-4 md:px-6">
			<AdminSectionHeader
				eyebrow="Queue detail"
				title="Recent mux jobs"
				description="20 job gần nhất với trạng thái, owner, attempt count và backend artifact."
			/>
		</div>
		<AdminJobsTable jobs={data.jobs} />
	</section>
</div>
