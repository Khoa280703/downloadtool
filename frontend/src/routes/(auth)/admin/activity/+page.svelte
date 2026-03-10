<script lang="ts">
	import type { PageData } from './$types';
	import AdminActivityTable from '$components/admin/AdminActivityTable.svelte';
	import AdminSectionHeader from '$components/admin/AdminSectionHeader.svelte';
	import AdminStatCard from '$components/admin/AdminStatCard.svelte';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>Admin Activity</title>
</svelte:head>

<section class="admin-panel rounded-xl border border-slate-200 bg-white">
	<div class="border-b border-slate-200 px-5 py-5 md:px-6">
		<AdminSectionHeader
			eyebrow="Activity"
			title="Operational event stream"
			description="Timeline hợp nhất từ jobs và proxy health để debug nhanh các biến động mới nhất."
		/>
	</div>

	<div class="grid gap-4 px-5 py-5 md:grid-cols-3 md:px-6">
		<AdminStatCard label="Signals / 24h" value={data.overview.eventsLast24h} caption="Tổng event trong 24 giờ gần nhất" tone="neutral" />
		<AdminStatCard label="Queue backlog" value={data.overview.queuedJobs + data.overview.leasedJobs} caption="Khối lượng job đang chờ hoặc đã lease" tone={data.overview.queuedJobs + data.overview.leasedJobs > 0 ? 'amber' : 'neutral'} />
		<AdminStatCard label="Proxy risk" value={data.overview.quarantinedProxies + data.overview.disabledProxies} caption="Proxy không nằm trong usable pool" tone={data.overview.quarantinedProxies + data.overview.disabledProxies > 0 ? 'rose' : 'neutral'} />
	</div>
</section>

<section class="admin-panel mt-6 rounded-xl border border-slate-200 bg-white">
	<div class="border-b border-slate-200 px-5 py-4 md:px-6">
		<AdminSectionHeader
			eyebrow="Timeline"
			title="Latest events"
			description="Danh sách sự kiện mới nhất để truy ngược lỗi, quarantine và các thay đổi trạng thái."
		/>
	</div>
	<AdminActivityTable activity={data.activity} />
</section>
