<script lang="ts">
	import type { PageData } from './$types';
	import AdminBarChart from '$components/admin/AdminBarChart.svelte';
	import AdminSectionHeader from '$components/admin/AdminSectionHeader.svelte';
	import AdminStatCard from '$components/admin/AdminStatCard.svelte';
	import { buildAdminDashboardViewModel } from '$lib/admin/dashboard-view-model';

	let { data }: { data: PageData } = $props();

	const model = $derived(buildAdminDashboardViewModel(data.overview));
	const queueItems = $derived([
		{ label: 'Queued', value: data.overview.queuedJobs, tone: 'amber' as const },
		{ label: 'Leased', value: data.overview.leasedJobs, tone: 'blue' as const },
		{ label: 'Processing', value: data.overview.processingJobs, tone: 'blue' as const }
	]);
	const artifactItems = $derived([
		{ label: 'Building', value: data.overview.buildingArtifacts, tone: 'blue' as const },
		{ label: 'Ready', value: data.overview.readyArtifacts, tone: 'green' as const }
	]);
	const proxyItems = $derived([
		{ label: 'Active', value: data.overview.activeProxies, tone: 'green' as const },
		{ label: 'Quarantined', value: data.overview.quarantinedProxies, tone: 'red' as const },
		{ label: 'Disabled', value: data.overview.disabledProxies, tone: 'amber' as const }
	]);
</script>

<svelte:head>
	<title>Admin Capacity</title>
</svelte:head>

<section class="admin-panel rounded-xl border border-slate-200 bg-white">
	<div class="border-b border-slate-200 px-5 py-5 md:px-6">
		<AdminSectionHeader
			eyebrow="Capacity"
			title="Pipeline headroom"
			description="Đánh giá nhanh các vùng đang ăn capacity: queue, artifact cache và usable proxy pool."
		/>
	</div>

	<div class="grid gap-4 px-5 py-5 md:grid-cols-2 xl:grid-cols-4 md:px-6">
		<AdminStatCard label="Backlog" value={model.queueBacklog} caption="Queued + leased" tone={model.queueBacklog > 0 ? 'amber' : 'neutral'} />
		<AdminStatCard label="Worker load" value={model.activeJobs} caption="Leased + processing" tone={model.activeJobs > 0 ? 'sky' : 'neutral'} />
		<AdminStatCard label="Ready artifacts" value={data.overview.readyArtifacts} caption="Có thể cấp download ticket ngay" tone="emerald" />
		<AdminStatCard label="Usable proxies" value={data.overview.activeProxies} caption="Pool còn phục vụ extract được" tone="emerald" />
	</div>
</section>

<div class="mt-6 grid gap-6 xl:grid-cols-3">
	<AdminBarChart
		title="Queue pressure"
		description="Khối lượng đang chờ worker hoặc đang bị lease."
		items={queueItems}
	/>
	<AdminBarChart
		title="Artifact cache"
		description="Tương quan giữa artifact đang build và artifact đã ready."
		items={artifactItems}
	/>
	<AdminBarChart
		title="Proxy availability"
		description="Tỷ lệ usable pool so với quarantine và disabled."
		items={proxyItems}
	/>
</div>
