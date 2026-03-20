<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	const backlog = $derived(data.overview.queuedJobs + data.overview.leasedJobs);
	const pipelineLoad = $derived(data.overview.processingJobs + data.overview.leasedJobs);
	const totalProxies = $derived(data.overview.activeProxies + data.overview.quarantinedProxies + data.overview.disabledProxies);

	function healthTone(ok: boolean): string {
		return ok ? 'text-green-700' : 'text-red-600';
	}

	type Signal = { label: string; value: string | number; status: 'ok' | 'warn' | 'critical'; note: string };

	const signals: Signal[] = $derived([
		{
			label: 'Queue Backlog',
			value: backlog,
			status: backlog === 0 ? 'ok' : backlog <= 5 ? 'warn' : 'critical',
			note: 'Pending worker processing'
		},
		{
			label: 'Pipeline Load',
			value: pipelineLoad,
			status: pipelineLoad <= 3 ? 'ok' : pipelineLoad <= 8 ? 'warn' : 'critical',
			note: 'Active mux + upload'
		},
		{
			label: 'Building Artifacts',
			value: data.overview.buildingArtifacts,
			status: data.overview.buildingArtifacts <= 3 ? 'ok' : 'warn',
			note: 'Muxing and uploading'
		},
		{
			label: 'Ready Artifacts',
			value: data.overview.readyArtifacts,
			status: 'ok',
			note: 'Cached for reuse'
		},
		{
			label: 'Failed + Expired',
			value: data.overview.failedJobs + data.overview.expiredJobs,
			status: (data.overview.failedJobs + data.overview.expiredJobs) === 0 ? 'ok' : 'critical',
			note: 'Needs investigation'
		},
		{
			label: 'Active Proxies',
			value: `${data.overview.activeProxies}/${totalProxies}`,
			status: data.overview.activeProxies >= totalProxies * 0.5 ? 'ok' : 'critical',
			note: `${data.overview.quarantinedProxies} quarantined`
		}
	]);

	function statusDot(status: Signal['status']): string {
		switch (status) {
			case 'ok': return 'bg-green-500';
			case 'warn': return 'bg-amber-500';
			case 'critical': return 'bg-red-500';
		}
	}
</script>

<svelte:head>
	<title>Admin Capacity</title>
</svelte:head>

<div class="mb-5">
	<h2 class="text-lg font-semibold text-gray-900">Capacity</h2>
	<p class="mt-0.5 text-[13px] text-gray-500">System headroom and operational health signals.</p>
</div>

<!-- KPI Strip -->
<div class="admin-panel mb-5 overflow-hidden border border-gray-200 bg-white">
	<div class="grid grid-cols-2 divide-x divide-y divide-gray-100 md:grid-cols-6 md:divide-y-0">
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-gray-900">{backlog}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Backlog</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-gray-900">{pipelineLoad}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Pipeline</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-gray-900">{data.overview.buildingArtifacts}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Building</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-green-700">{data.overview.readyArtifacts}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Cached</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-green-700">{data.overview.activeProxies}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Proxies ↑</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-red-600">{data.overview.quarantinedProxies}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Proxies ↓</p>
		</div>
	</div>
</div>

<!-- Health Signals -->
<section class="admin-panel mb-5 overflow-hidden border border-gray-200 bg-white">
	<div class="border-b border-gray-200 bg-gray-50 px-5 py-2.5">
		<h3 class="text-[13px] font-semibold text-gray-900">Health Signals</h3>
		<p class="text-[11px] text-gray-400">Real-time system health assessment.</p>
	</div>
	<div class="divide-y divide-gray-100">
		{#each signals as signal}
			<div class="flex flex-col items-start gap-1.5 px-4 py-3 sm:flex-row sm:items-center sm:gap-3 sm:px-5 sm:py-2.5">
				<span class={`h-2 w-2 shrink-0 rounded-full ${statusDot(signal.status)}`}></span>
				<span class="text-[13px] font-medium text-gray-900 sm:w-40">{signal.label}</span>
				<span class="text-[13px] font-semibold tabular-nums text-gray-900 sm:w-20">{signal.value}</span>
				<span class="text-[12px] text-gray-400">{signal.note}</span>
			</div>
		{/each}
	</div>
</section>

<!-- Operational Notes -->
<section class="admin-panel overflow-hidden border border-gray-200 bg-white">
	<div class="border-b border-gray-200 bg-gray-50 px-5 py-2.5">
		<h3 class="text-[13px] font-semibold text-gray-900">Operational Runbook</h3>
		<p class="text-[11px] text-gray-400">Quick reference for common scenarios.</p>
	</div>
	<div class="divide-y divide-gray-100">
		<div class="flex flex-col gap-1.5 px-4 py-3 sm:flex-row sm:gap-4 sm:px-5 sm:py-2.5">
			<span class="text-[13px] font-medium text-gray-900 sm:w-56 sm:shrink-0">Failed + expired rising</span>
			<span class="text-[13px] text-gray-500">Check proxy health, upstream extract, and lease timeout.</span>
		</div>
		<div class="flex flex-col gap-1.5 px-4 py-3 sm:flex-row sm:gap-4 sm:px-5 sm:py-2.5">
			<span class="text-[13px] font-medium text-gray-900 sm:w-56 sm:shrink-0">Building high, ready low</span>
			<span class="text-[13px] text-gray-500">Review worker throughput, upload latency, storage backend.</span>
		</div>
		<div class="flex flex-col gap-1.5 px-4 py-3 sm:flex-row sm:gap-4 sm:px-5 sm:py-2.5">
			<span class="text-[13px] font-medium text-gray-900 sm:w-56 sm:shrink-0">Quarantined proxies rising</span>
			<span class="text-[13px] text-gray-500">Add new proxies or reduce extract load.</span>
		</div>
	</div>
</section>
