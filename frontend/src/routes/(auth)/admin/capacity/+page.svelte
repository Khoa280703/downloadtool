<script lang="ts">
	import type { PageData } from './$types';
	import AdminBarChart from '$components/admin/AdminBarChart.svelte';
	import AdminMiniMetric from '$components/admin/AdminMiniMetric.svelte';
	import AdminSectionHeader from '$components/admin/AdminSectionHeader.svelte';
	import AdminStatCard from '$components/admin/AdminStatCard.svelte';
	import { buildAdminDashboardViewModel } from '$lib/admin/dashboard-view-model';

	let { data }: { data: PageData } = $props();

	const model = $derived(buildAdminDashboardViewModel(data.overview));
	const pipelineChart = $derived([
		{ label: 'Backlog', value: model.queueBacklog, tone: 'amber' as const },
		{ label: 'Active workers', value: model.activeJobs, tone: 'blue' as const },
		{ label: 'Ready artifacts', value: data.overview.readyArtifacts, tone: 'green' as const },
		{ label: 'Quarantined proxies', value: data.overview.quarantinedProxies, tone: 'red' as const }
	]);
	const storageChart = $derived([
		{ label: 'Building artifacts', value: data.overview.buildingArtifacts, tone: 'blue' as const },
		{ label: 'Ready artifacts', value: data.overview.readyArtifacts, tone: 'green' as const }
	]);
</script>

<svelte:head>
	<title>Admin Capacity</title>
</svelte:head>

<header class="admin-panel border border-slate-200 bg-white px-5 py-5 md:px-6">
	<AdminSectionHeader
		eyebrow="Capacity"
		title="Pipeline headroom"
		description="Đánh giá sức chứa hiện tại của worker, artifact cache và proxy fleet."
	/>
</header>

<div class="grid gap-5 xl:grid-cols-2">
	<AdminBarChart
		title="Pipeline pressure"
		description="Đo headroom hiện tại giữa queue, worker load và proxy risk."
		items={pipelineChart}
	/>
	<AdminBarChart
		title="Artifact storage"
		description="So sánh artifact đang build với artifact đã sẵn sàng."
		items={storageChart}
	/>
</div>

<div class="grid gap-6 2xl:grid-cols-[minmax(0,1fr)_420px]">
	<section class="admin-panel border border-slate-200 bg-white p-5 md:p-6">
		<AdminSectionHeader
			eyebrow="Capacity"
			title="Key capacity signals"
			description="Các chỉ số chính phản ánh tình trạng headroom của pipeline."
		/>
		<div class="mt-5 grid gap-4 md:grid-cols-2">
			<AdminStatCard
				label="Building artifacts"
				value={data.overview.buildingArtifacts}
				caption="Đang mux và upload lên storage"
				tone="sky"
			/>
			<AdminStatCard
				label="Ready artifacts"
				value={data.overview.readyArtifacts}
				caption="File đã có sẵn để cấp ticket"
				tone="emerald"
			/>
			<AdminStatCard
				label="Queue backlog"
				value={model.queueBacklog}
				caption="Tổng queued và leased"
				tone={model.queueBacklog > 0 ? 'amber' : 'neutral'}
			/>
			<AdminStatCard
				label="Worker load"
				value={model.activeJobs}
				caption="Tổng leased và processing"
				tone={model.activeJobs > 0 ? 'sky' : 'neutral'}
			/>
		</div>
	</section>

	<section class="admin-panel border border-slate-200 bg-white p-5">
		<AdminSectionHeader
			eyebrow="System posture"
			title="Operational snapshot"
			description="Tóm tắt các vùng áp lực nhất theo thời điểm."
		/>
		<div class="mt-5 grid gap-3">
			{#each model.snapshotStats as stat}
				<AdminMiniMetric label={stat.label} value={stat.value} caption={stat.caption} />
			{/each}
		</div>
	</section>
</div>
