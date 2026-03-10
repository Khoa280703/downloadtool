<script lang="ts">
	import type { PageData } from './$types';
	import AdminActivityTable from '$components/admin/AdminActivityTable.svelte';
	import AdminMiniMetric from '$components/admin/AdminMiniMetric.svelte';
	import AdminSectionHeader from '$components/admin/AdminSectionHeader.svelte';
	import { buildAdminDashboardViewModel } from '$lib/admin/dashboard-view-model';

	let { data }: { data: PageData } = $props();

	const model = $derived(buildAdminDashboardViewModel(data.overview));
</script>

<svelte:head>
	<title>Admin Activity</title>
</svelte:head>

<header class="admin-panel border border-slate-200 bg-white px-5 py-5 md:px-6">
	<div class="flex flex-col gap-4 xl:flex-row xl:items-end xl:justify-between">
		<AdminSectionHeader
			eyebrow="Activity"
			title="Recent activity"
			description="Timeline event từ jobs và proxy health để debug nhanh và thấy nhịp hoạt động hệ thống."
		/>
		<div class="grid gap-2 sm:grid-cols-3 xl:min-w-[420px]">
			{#each model.snapshotStats as stat}
				<AdminMiniMetric label={stat.label} value={stat.value} />
			{/each}
		</div>
	</div>
</header>

<section class="admin-panel border border-slate-200 bg-white">
	<div class="border-b border-slate-200 px-5 py-4 md:px-6">
		<AdminSectionHeader
			eyebrow="System activity"
			title="Latest events"
			description="Union của job events và proxy events theo thứ tự thời gian mới nhất."
		/>
	</div>
	<AdminActivityTable activity={data.activity} />
</section>
