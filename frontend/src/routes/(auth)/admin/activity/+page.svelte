<script lang="ts">
	import type { PageData } from './$types';
	import AdminActivityTable from '$components/admin/AdminActivityTable.svelte';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>Admin Activity</title>
</svelte:head>

<div class="mb-8">
	<h2 class="text-3xl font-black tracking-tight text-slate-900">System Activity</h2>
	<p class="mt-1 text-sm text-slate-500">Timeline mới nhất của mux jobs và proxy health events.</p>
</div>

<section class="admin-panel overflow-hidden border border-slate-200 bg-white">
	<div class="border-b border-slate-200 bg-slate-50 px-6 py-4">
		<h3 class="text-sm font-bold text-slate-900">Latest Events</h3>
		<p class="mt-1 text-sm text-slate-500">Union event feed, sắp xếp theo thời gian tạo mới nhất.</p>
	</div>
	<AdminActivityTable activity={data.activity} />
</section>
