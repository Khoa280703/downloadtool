<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>Admin Capacity</title>
</svelte:head>

<div class="mb-5">
	<h2 class="text-lg font-semibold text-gray-900">Capacity</h2>
	<p class="mt-0.5 text-[13px] text-gray-500">Current worker, storage, and proxy fleet headroom.</p>
</div>

<section class="admin-panel mb-5 overflow-hidden border border-gray-200 bg-white">
	<div class="border-b border-gray-200 bg-gray-50 px-5 py-3">
		<h3 class="text-[11px] font-semibold uppercase tracking-wider text-gray-500">Capacity Summary</h3>
	</div>
	<div class="overflow-x-auto">
		<table class="w-full border-collapse text-left text-[13px]">
			<thead>
				<tr class="border-b border-gray-100 bg-gray-50/40">
					<th class="px-5 py-2.5 text-[10px] font-semibold uppercase tracking-wider text-gray-400">Signal</th>
					<th class="px-5 py-2.5 text-[10px] font-semibold uppercase tracking-wider text-gray-400">Value</th>
					<th class="px-5 py-2.5 text-[10px] font-semibold uppercase tracking-wider text-gray-400">Interpretation</th>
				</tr>
			</thead>
			<tbody class="divide-y divide-gray-50">
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Queue Backlog</td><td class="px-5 py-2.5 tabular-nums text-gray-700">{data.overview.queuedJobs + data.overview.leasedJobs}</td><td class="px-5 py-2.5 text-gray-400">Pending worker processing.</td></tr>
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Processing Load</td><td class="px-5 py-2.5 tabular-nums text-gray-700">{data.overview.processingJobs + data.overview.leasedJobs}</td><td class="px-5 py-2.5 text-gray-400">Active pipeline load.</td></tr>
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Building Artifacts</td><td class="px-5 py-2.5 tabular-nums text-gray-700">{data.overview.buildingArtifacts}</td><td class="px-5 py-2.5 text-gray-400">Muxing and uploading.</td></tr>
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Ready Artifacts</td><td class="px-5 py-2.5 tabular-nums text-gray-700">{data.overview.readyArtifacts}</td><td class="px-5 py-2.5 text-gray-400">Cached for reuse.</td></tr>
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Active Proxies</td><td class="px-5 py-2.5 tabular-nums text-gray-700">{data.overview.activeProxies}</td><td class="px-5 py-2.5 text-gray-400">Effective fleet coverage.</td></tr>
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Quarantined Proxies</td><td class="px-5 py-2.5 tabular-nums text-gray-700">{data.overview.quarantinedProxies}</td><td class="px-5 py-2.5 text-gray-400">Risk of proxy shortage if count rises.</td></tr>
			</tbody>
		</table>
	</div>
</section>

<section class="admin-panel overflow-hidden border border-gray-200 bg-white">
	<div class="border-b border-gray-200 bg-gray-50 px-5 py-3">
		<h3 class="text-[13px] font-semibold text-gray-900">Operational Notes</h3>
		<p class="mt-0.5 text-[12px] text-gray-400">Quick reference for operators.</p>
	</div>
	<div class="overflow-x-auto">
		<table class="w-full border-collapse text-left text-[13px]">
			<thead>
				<tr class="border-b border-gray-100 bg-gray-50/40">
					<th class="px-5 py-2.5 text-[10px] font-semibold uppercase tracking-wider text-gray-400">Condition</th>
					<th class="px-5 py-2.5 text-[10px] font-semibold uppercase tracking-wider text-gray-400">Action</th>
				</tr>
			</thead>
			<tbody class="divide-y divide-gray-50">
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Failed + expired rising</td><td class="px-5 py-2.5 text-gray-500">Check proxy health, upstream extract, and lease timeout.</td></tr>
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Building high but ready low</td><td class="px-5 py-2.5 text-gray-500">Review worker throughput, upload latency, storage backend.</td></tr>
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Quarantined proxies rising fast</td><td class="px-5 py-2.5 text-gray-500">Add new proxies or reduce extract load.</td></tr>
			</tbody>
		</table>
	</div>
</section>
