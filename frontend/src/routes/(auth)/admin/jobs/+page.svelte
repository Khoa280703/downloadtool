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
	const queueChart = $derived([
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

<header class="admin-panel border border-slate-200 bg-white px-5 py-5 md:px-6">
	<AdminSectionHeader
		eyebrow="Jobs"
		title="Mux queue"
		description="Chi tiết queue, lease, retry count và artifact backend của các job gần nhất."
	/>
</header>

<div class="grid gap-5 xl:grid-cols-[minmax(0,1fr)_320px]">
	<section class="admin-panel border border-slate-200 bg-white p-5">
		<AdminSectionHeader
			eyebrow="Queue profile"
			title="Current distribution"
			description="Phân bố hiện tại của queue để nhận diện backlog và failure pressure."
		/>
		<div class="mt-5">
			<AdminBarChart title="Mux job states" description="Snapshot thời điểm hiện tại." items={queueChart} />
		</div>
	</section>

	<section class="admin-panel border border-slate-200 bg-white p-5">
		<AdminSectionHeader
			eyebrow="Queue metrics"
			title="Operational counters"
			description="Các counters quan trọng cho operator theo dõi throughput và retry."
		/>
		<div class="mt-5 grid gap-3">
			{#each model.queueStats as stat}
				<AdminMiniMetric label={stat.label} value={stat.value} />
			{/each}
		</div>
		<div class="mt-4 grid gap-3">
			<AdminStatCard
				label="Artifacts ready"
				value={data.overview.readyArtifacts}
				caption="Số artifact có thể cấp file ticket"
				tone="emerald"
			/>
			<AdminStatCard
				label="Artifacts building"
				value={data.overview.buildingArtifacts}
				caption="Đang mux và upload"
				tone="sky"
			/>
		</div>
	</section>
</div>

<section class="admin-panel border border-slate-200 bg-white">
	<div class="border-b border-slate-200 px-5 py-4 md:px-6">
		<AdminSectionHeader
			eyebrow="Queue detail"
			title="Recent mux jobs"
			description="20 job gần nhất với trạng thái, số lần thử và artifact backend tương ứng."
		/>
	</div>
	<AdminJobsTable jobs={data.jobs} />
</section>
