<script lang="ts">
	import type { PageData } from './$types';
	import AdminStatCard from '$components/admin/AdminStatCard.svelte';
	import AdminBarChart from '$components/admin/AdminBarChart.svelte';

	let { data }: { data: PageData } = $props();

	const totalProxies = $derived(
		data.overview.activeProxies + data.overview.quarantinedProxies + data.overview.disabledProxies
	);

	const avgProxyHealth = $derived(
		data.proxies.length
			? Math.round(data.proxies.reduce((sum, p) => sum + p.healthScore, 0) / data.proxies.length)
			: 0
	);

	const recentFailedJobs = $derived(
		data.jobs.filter((j) => j.status === 'failed' || j.status === 'expired').length
	);

	const proxySuccessRate = $derived(() => {
		const total = data.proxies.reduce((s, p) => s + p.proxyRelevantAttempts24h, 0);
		const success = data.proxies.reduce((s, p) => s + p.extractSuccesses24h, 0);
		return total > 0 ? Math.round((success / total) * 100) : 0;
	});
</script>

<svelte:head>
	<title>Admin Overview</title>
</svelte:head>

<div class="mb-6">
	<h2 class="text-lg font-semibold text-gray-900">System Overview</h2>
	<p class="mt-0.5 text-[13px] text-gray-500">Real-time system health and key metrics.</p>
</div>

<!-- KPI Cards -->
<div class="mb-5 grid grid-cols-2 gap-3 md:grid-cols-3 xl:grid-cols-6">
	<AdminStatCard label="Queued" value={data.overview.queuedJobs} caption="Waiting for worker" />
	<AdminStatCard label="Processing" value={data.overview.processingJobs + data.overview.leasedJobs} caption="Active pipeline" />
	<AdminStatCard label="Ready" value={data.overview.readyJobs} caption="Download available" />
	<AdminStatCard label="Failed" value={data.overview.failedJobs + data.overview.expiredJobs} caption="Needs attention" />
	<AdminStatCard label="Artifacts" value={data.overview.readyArtifacts} caption="Cached for reuse" />
	<AdminStatCard label="Proxies" value="{data.overview.activeProxies}/{totalProxies}" caption="Active / total" />
</div>

<!-- Charts -->
<div class="mb-5 grid grid-cols-1 gap-4 lg:grid-cols-2">
	<AdminBarChart
		title="Job Status Distribution"
		description="Current job counts by status"
		items={[
			{ label: 'Queued', value: data.overview.queuedJobs, tone: 'amber' },
			{ label: 'Leased', value: data.overview.leasedJobs, tone: 'blue' },
			{ label: 'Processing', value: data.overview.processingJobs, tone: 'blue' },
			{ label: 'Ready', value: data.overview.readyJobs, tone: 'green' },
			{ label: 'Failed', value: data.overview.failedJobs, tone: 'red' },
			{ label: 'Expired', value: data.overview.expiredJobs, tone: 'red' }
		]}
	/>

	<AdminBarChart
		title="Proxy Fleet Health"
		description="Fleet status and extract metrics"
		items={[
			{ label: 'Active', value: data.overview.activeProxies, tone: 'green' },
			{ label: 'Quarantined', value: data.overview.quarantinedProxies, tone: 'red' },
			{ label: 'Disabled', value: data.overview.disabledProxies, tone: 'amber' },
			{ label: 'Avg. Health Score', value: avgProxyHealth, tone: 'blue' },
			{ label: 'Extract Success %', value: proxySuccessRate(), tone: 'green' }
		]}
	/>
</div>

<div class="mb-5 grid grid-cols-1 gap-4 lg:grid-cols-2">
	<AdminBarChart
		title="Artifact Pipeline"
		description="Build and cache status"
		items={[
			{ label: 'Building', value: data.overview.buildingArtifacts, tone: 'blue' },
			{ label: 'Ready (cached)', value: data.overview.readyArtifacts, tone: 'green' },
			{ label: 'Events / 24h', value: data.overview.eventsLast24h, tone: 'neutral' }
		]}
	/>

	<section class="admin-panel rounded-lg border border-gray-200 bg-white p-4">
		<div class="border-b border-gray-100 pb-3">
			<h3 class="text-sm font-semibold text-gray-900">Quick Links</h3>
			<p class="mt-0.5 text-[11px] text-gray-500">Jump to detailed views</p>
		</div>
		<div class="mt-3 grid grid-cols-2 gap-2">
			<a href="/admin/jobs" class="rounded-md border border-gray-200 px-3 py-2.5 text-center text-[13px] font-medium text-gray-700 transition hover:bg-gray-50">
				Mux Queue
				<span class="mt-0.5 block text-[11px] tabular-nums text-gray-400">{data.overview.queuedJobs + data.overview.processingJobs + data.overview.leasedJobs} active</span>
			</a>
			<a href="/admin/proxies" class="rounded-md border border-gray-200 px-3 py-2.5 text-center text-[13px] font-medium text-gray-700 transition hover:bg-gray-50">
				Proxy Fleet
				<span class="mt-0.5 block text-[11px] tabular-nums text-gray-400">{data.overview.activeProxies} active</span>
			</a>
			<a href="/admin/activity" class="rounded-md border border-gray-200 px-3 py-2.5 text-center text-[13px] font-medium text-gray-700 transition hover:bg-gray-50">
				Activity Log
				<span class="mt-0.5 block text-[11px] tabular-nums text-gray-400">{data.overview.eventsLast24h} events/24h</span>
			</a>
			<a href="/admin/capacity" class="rounded-md border border-gray-200 px-3 py-2.5 text-center text-[13px] font-medium text-gray-700 transition hover:bg-gray-50">
				Capacity
				<span class="mt-0.5 block text-[11px] tabular-nums text-gray-400">{recentFailedJobs} failed recent</span>
			</a>
		</div>
	</section>
</div>
