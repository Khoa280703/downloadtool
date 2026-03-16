<script lang="ts">
	import type { PageData } from './$types';
	import AdminActivityTable from '$components/admin/AdminActivityTable.svelte';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>Admin Activity</title>
</svelte:head>

<div class="mb-5">
	<h2 class="text-lg font-semibold text-gray-900">System Activity</h2>
	<p class="mt-0.5 text-[13px] text-gray-500">Latest job and proxy health events.</p>
</div>

<section class="admin-panel overflow-hidden border border-gray-200 bg-white">
	<div class="border-b border-gray-200 bg-gray-50 px-5 py-3">
		<h3 class="text-[13px] font-semibold text-gray-900">Latest Events</h3>
		<p class="mt-0.5 text-[12px] text-gray-400">Combined event feed, newest first.</p>
	</div>
	<AdminActivityTable activity={data.activity} />
</section>
