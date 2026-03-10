<script lang="ts">
	import type { PageData } from './$types';
	import AdminJobsTable from '$components/admin/AdminJobsTable.svelte';
	import AdminMiniMetric from '$components/admin/AdminMiniMetric.svelte';
	import AdminSectionHeader from '$components/admin/AdminSectionHeader.svelte';
	import { buildAdminDashboardViewModel } from '$lib/admin/dashboard-view-model';

	let { data }: { data: PageData } = $props();

	const model = $derived(buildAdminDashboardViewModel(data.overview));
</script>

<svelte:head>
	<title>Admin Jobs</title>
</svelte:head>

<header class="admin-panel rounded-[2rem] border border-slate-200/80 bg-white/95 px-5 py-6 shadow-[0_24px_80px_-44px_rgba(15,23,42,0.45)] md:px-7">
	<AdminSectionHeader
		eyebrow="Jobs"
		title="Mux queue"
		description="Chi tiết queue, lease, retry count và artifact backend của các job gần nhất."
	/>
</header>

<section class="admin-panel rounded-[2rem] border border-slate-200 bg-white">
	<div class="flex flex-wrap items-start justify-between gap-4 border-b border-slate-200 px-5 py-5 md:px-6">
		<AdminSectionHeader
			eyebrow="Queue detail"
			title="Recent mux jobs"
			description="20 job gần nhất với trạng thái, số lần thử và backend của artifact tương ứng."
		/>
		<div class="grid gap-3 sm:grid-cols-3">
			{#each model.queueStats as stat}
				<AdminMiniMetric label={stat.label} value={stat.value} />
			{/each}
		</div>
	</div>
	<AdminJobsTable jobs={data.jobs} />
</section>
