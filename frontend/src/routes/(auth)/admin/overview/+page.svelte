<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	const totalProxies = $derived(
		data.overview.activeProxies + data.overview.quarantinedProxies + data.overview.disabledProxies
	);

	const avgProxyHealth = $derived(
		data.proxies.length
			? Math.round(data.proxies.reduce((sum, p) => sum + p.healthScore, 0) / data.proxies.length)
			: 0
	);

	const totalRelevant = $derived(data.proxies.reduce((s, p) => s + p.proxyRelevantAttempts24h, 0));
	const totalSuccess = $derived(data.proxies.reduce((s, p) => s + p.extractSuccesses24h, 0));
	const successRate = $derived(totalRelevant > 0 ? Math.round((totalSuccess / totalRelevant) * 100) : 0);

	function healthColor(score: number): string {
		if (score >= 80) return 'text-green-700';
		if (score >= 55) return 'text-amber-600';
		return 'text-red-600';
	}
</script>

<svelte:head>
	<title>Admin Overview</title>
</svelte:head>

<div class="mb-5">
	<h2 class="text-lg font-semibold text-gray-900">System Overview</h2>
	<p class="mt-0.5 text-[13px] text-gray-500">Real-time system health at a glance.</p>
</div>

<!-- Job Pipeline KPI -->
<div class="admin-panel mb-5 overflow-hidden border border-gray-200 bg-white">
	<div class="border-b border-gray-100 bg-gray-50 px-5 py-2">
		<p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Job Pipeline</p>
	</div>
	<div class="grid grid-cols-2 divide-x divide-y divide-gray-100 md:grid-cols-6 md:divide-y-0">
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-amber-600">{data.overview.queuedJobs}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Queued</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-blue-600">{data.overview.leasedJobs + data.overview.processingJobs}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Processing</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-green-700">{data.overview.readyJobs}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Ready</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-red-600">{data.overview.failedJobs}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Failed</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-gray-400">{data.overview.expiredJobs}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Expired</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-gray-900">{data.overview.eventsLast24h}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Events 24h</p>
		</div>
	</div>
</div>

<!-- Artifact + Proxy KPI -->
<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
	<div class="admin-panel overflow-hidden border border-gray-200 bg-white">
		<div class="border-b border-gray-100 bg-gray-50 px-5 py-2">
			<p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Artifacts</p>
		</div>
		<div class="grid grid-cols-1 divide-y divide-gray-100 sm:grid-cols-3 sm:divide-x sm:divide-y-0">
			<div class="px-4 py-3 text-center">
				<p class="text-2xl font-semibold tabular-nums text-blue-600">{data.overview.buildingArtifacts}</p>
				<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Building</p>
			</div>
			<div class="px-4 py-3 text-center">
				<p class="text-2xl font-semibold tabular-nums text-green-700">{data.overview.readyArtifacts}</p>
				<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Cached</p>
			</div>
			<div class="px-4 py-3 text-center">
				<p class="text-2xl font-semibold tabular-nums text-gray-900">{data.overview.readyJobs}</p>
				<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Downloadable</p>
			</div>
		</div>
	</div>

	<div class="admin-panel overflow-hidden border border-gray-200 bg-white">
		<div class="border-b border-gray-100 bg-gray-50 px-5 py-2">
			<p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Proxy Fleet</p>
		</div>
		<div class="grid grid-cols-1 divide-y divide-gray-100 sm:grid-cols-3 sm:divide-x sm:divide-y-0">
			<div class="px-4 py-3 text-center">
				<p class="text-2xl font-semibold tabular-nums text-green-700">{data.overview.activeProxies}<span class="text-sm text-gray-400">/{totalProxies}</span></p>
				<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Active</p>
			</div>
			<div class="px-4 py-3 text-center">
				<p class={`text-2xl font-semibold tabular-nums ${healthColor(avgProxyHealth)}`}>{avgProxyHealth}</p>
				<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Avg Health</p>
			</div>
			<div class="px-4 py-3 text-center">
				<p class="text-2xl font-semibold tabular-nums text-gray-900">{successRate}%</p>
				<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Success 24h</p>
			</div>
		</div>
	</div>
</div>

<!-- Quick links -->
<div class="mt-5 grid grid-cols-1 gap-3 sm:grid-cols-2 md:grid-cols-4">
	<a href="/admin/jobs" class="admin-panel flex flex-col items-center rounded-lg border border-gray-200 bg-white px-4 py-4 text-center transition hover:bg-gray-50">
		<span class="text-lg font-semibold tabular-nums text-gray-900">{data.overview.queuedJobs + data.overview.processingJobs + data.overview.leasedJobs}</span>
		<span class="mt-0.5 text-[12px] font-medium text-gray-600">Mux Queue →</span>
	</a>
	<a href="/admin/proxies" class="admin-panel flex flex-col items-center rounded-lg border border-gray-200 bg-white px-4 py-4 text-center transition hover:bg-gray-50">
		<span class="text-lg font-semibold tabular-nums text-gray-900">{data.overview.activeProxies}</span>
		<span class="mt-0.5 text-[12px] font-medium text-gray-600">Proxies →</span>
	</a>
	<a href="/admin/activity" class="admin-panel flex flex-col items-center rounded-lg border border-gray-200 bg-white px-4 py-4 text-center transition hover:bg-gray-50">
		<span class="text-lg font-semibold tabular-nums text-gray-900">{data.overview.eventsLast24h}</span>
		<span class="mt-0.5 text-[12px] font-medium text-gray-600">Activity →</span>
	</a>
	<a href="/admin/capacity" class="admin-panel flex flex-col items-center rounded-lg border border-gray-200 bg-white px-4 py-4 text-center transition hover:bg-gray-50">
		<span class="text-lg font-semibold tabular-nums text-gray-900">{data.overview.failedJobs + data.overview.expiredJobs}</span>
		<span class="mt-0.5 text-[12px] font-medium text-gray-600">Capacity →</span>
	</a>
</div>
