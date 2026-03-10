<script lang="ts">
	import type { PageData } from './$types';
	import AdminActivityTable from '$components/admin/AdminActivityTable.svelte';
	import AdminDonutChart from '$components/admin/AdminDonutChart.svelte';
	import AdminLineChart from '$components/admin/AdminLineChart.svelte';
	import AdminOverviewMetricCard from '$components/admin/AdminOverviewMetricCard.svelte';

	let { data }: { data: PageData } = $props();

	const backlog = $derived(data.overview.queuedJobs + data.overview.leasedJobs);
	const activeLoad = $derived(data.overview.processingJobs + data.overview.leasedJobs);
	const proxyAlerts = $derived(data.overview.quarantinedProxies + data.overview.disabledProxies);
	const pipelinePoints = $derived([
		{ label: 'Queued', value: data.overview.queuedJobs },
		{ label: 'Leased', value: data.overview.leasedJobs },
		{ label: 'Process', value: data.overview.processingJobs },
		{ label: 'Ready', value: data.overview.readyJobs },
		{ label: 'Build', value: data.overview.buildingArtifacts },
		{ label: 'Cache', value: data.overview.readyArtifacts }
	]);
	const fleetSegments = $derived([
		{ label: 'Active proxies', value: data.overview.activeProxies, color: '#137fec' },
		{ label: 'Quarantined', value: data.overview.quarantinedProxies, color: '#8b5cf6' },
		{ label: 'Disabled', value: data.overview.disabledProxies, color: '#f59e0b' }
	]);
	const recentActivity = $derived(data.activity.slice(0, 8));
	const attentionRows = $derived(
		data.jobs.filter((job) => ['failed', 'expired', 'queued', 'leased'].includes(job.status)).slice(0, 5)
	);
</script>

<svelte:head>
	<title>Admin Overview</title>
</svelte:head>

<div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
	<div>
		<h2 class="text-[1.8rem] font-bold tracking-[-0.03em] text-slate-950">Dashboard Overview</h2>
		<p class="text-sm text-slate-500">Welcome back. Đây là snapshot vận hành mới nhất của hệ thống.</p>
	</div>
	<div class="flex items-center gap-3">
		<a
			href="/admin/activity"
			class="inline-flex items-center gap-2 rounded-lg border border-slate-200 bg-white px-4 py-2 text-sm font-semibold text-slate-700 transition hover:bg-slate-50"
		>
			<span class="material-symbols-outlined text-lg">timeline</span>
			View Activity
		</a>
		<a
			href="/admin/jobs"
			class="inline-flex items-center gap-2 rounded-lg bg-[#137fec] px-4 py-2 text-sm font-semibold text-white shadow-[0_10px_28px_-18px_rgba(19,127,236,0.8)] transition hover:opacity-95"
		>
			<span class="material-symbols-outlined text-lg">add</span>
			Open Queue
		</a>
	</div>
</div>

<div class="grid grid-cols-1 gap-6 sm:grid-cols-2 xl:grid-cols-4">
	<AdminOverviewMetricCard
		label="Queue Backlog"
		value={backlog}
		icon="work_history"
		badge={`${data.overview.processingJobs} processing`}
		badgeTone="primary"
		description="Queued và leased jobs đang chờ pipeline giải quyết."
	/>
	<AdminOverviewMetricCard
		label="Ready Artifacts"
		value={data.overview.readyArtifacts}
		icon="inventory_2"
		badge={`${data.overview.buildingArtifacts} building`}
		badgeTone="emerald"
		description="File đã sẵn sàng để cấp vé tải hoặc dedupe reuse."
	/>
	<AdminOverviewMetricCard
		label="Active Load"
		value={activeLoad}
		icon="rocket_launch"
		badge={`${data.overview.readyJobs} ready jobs`}
		badgeTone="violet"
		description="Tổng processing và leased, phản ánh áp lực worker hiện tại."
	/>
	<AdminOverviewMetricCard
		label="Proxy Alerts"
		value={proxyAlerts}
		icon="shield"
		badge={`${data.overview.activeProxies} healthy`}
		badgeTone={proxyAlerts > 0 ? 'amber' : 'emerald'}
		description="Proxy bị quarantine hoặc disabled cần operator kiểm tra."
	/>
</div>

<div class="grid grid-cols-1 gap-8 xl:grid-cols-3">
	<div class="xl:col-span-2">
		<AdminLineChart
			title="Pipeline Flow"
			description="Diễn biến phân bố tải dọc theo các stage chính của hệ thống."
			points={pipelinePoints}
		/>
	</div>
	<div>
		<AdminDonutChart
			title="Proxy Distribution"
			description="Tỷ trọng fleet hiện tại theo tình trạng vận hành."
			totalLabel="TOTAL"
			segments={fleetSegments}
		/>
	</div>
</div>

<div class="grid grid-cols-1 gap-8 xl:grid-cols-[minmax(0,1.4fr)_minmax(320px,0.8fr)]">
	<section class="admin-panel border border-slate-200 bg-white shadow-sm">
		<div class="flex items-center justify-between border-b border-slate-200 px-6 py-5">
			<div>
				<h3 class="text-lg font-bold text-slate-950">Recent Activities</h3>
				<p class="mt-1 text-sm text-slate-500">Activity stream mới nhất từ jobs và proxy health.</p>
			</div>
			<a href="/admin/activity" class="text-sm font-semibold text-[#137fec] hover:underline">View all</a>
		</div>
		<AdminActivityTable activity={recentActivity} />
	</section>

	<section class="admin-panel border border-slate-200 bg-white p-6 shadow-sm">
		<div class="flex items-center justify-between">
			<div>
				<h3 class="text-lg font-bold text-slate-950">Attention Queue</h3>
				<p class="mt-1 text-sm text-slate-500">Các job nên được xử lý trước ở thời điểm này.</p>
			</div>
			<a href="/admin/jobs" class="text-sm font-semibold text-[#137fec] hover:underline">Open jobs</a>
		</div>

		<div class="mt-5 space-y-3">
			{#if attentionRows.length > 0}
				{#each attentionRows as job}
					<a
						href="/admin/jobs"
						class="block rounded-xl border border-slate-200 bg-slate-50 px-4 py-3 transition hover:border-slate-300 hover:bg-slate-100"
					>
						<div class="flex items-start justify-between gap-3">
							<div class="min-w-0">
								<p class="truncate text-sm font-semibold text-slate-900">{job.title ?? job.id}</p>
								<p class="mt-1 text-xs text-slate-500">{job.id}</p>
							</div>
							<span class="rounded-full bg-white px-2 py-1 text-[11px] font-bold uppercase text-slate-600">
								{job.status}
							</span>
						</div>
						<p class="mt-2 text-[13px] text-slate-500">{job.attemptLabel} • {job.backend ?? 'no backend'}</p>
					</a>
				{/each}
			{:else}
				<div class="rounded-xl border border-slate-200 bg-slate-50 px-4 py-5 text-sm text-slate-500">
					Không có job nào đang cần operator can thiệp.
				</div>
			{/if}
		</div>
	</section>
</div>
