<script lang="ts">
	import type { PageData } from './$types';
	import AdminActivityTable from '$components/admin/AdminActivityTable.svelte';
	import AdminSectionHeader from '$components/admin/AdminSectionHeader.svelte';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>Admin Activity</title>
</svelte:head>

<header class="admin-panel rounded-[2rem] border border-slate-200/80 bg-white/95 px-5 py-6 shadow-[0_24px_80px_-44px_rgba(15,23,42,0.45)] md:px-7">
	<AdminSectionHeader
		eyebrow="Activity"
		title="Recent activity"
		description="Timeline event từ jobs và proxy health để debug nhanh và thấy nhịp hoạt động hệ thống."
	/>
</header>

<section class="admin-panel rounded-[2rem] border border-slate-200 bg-white">
	<div class="border-b border-slate-200 px-5 py-5 md:px-6">
		<AdminSectionHeader
			eyebrow="System activity"
			title="Latest events"
			description="Union của job events và proxy events theo thứ tự thời gian mới nhất."
		/>
	</div>
	<AdminActivityTable activity={data.activity} />
</section>
