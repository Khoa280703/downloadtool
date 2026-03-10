<script lang="ts">
	import type { PageData } from './$types';
	import AdminActivityTable from '$components/admin/AdminActivityTable.svelte';
	import AdminBarChart from '$components/admin/AdminBarChart.svelte';
	import AdminJobsTable from '$components/admin/AdminJobsTable.svelte';
	import AdminSectionHeader from '$components/admin/AdminSectionHeader.svelte';
	import AdminStatCard from '$components/admin/AdminStatCard.svelte';
	import AdminStatusBadge from '$components/admin/AdminStatusBadge.svelte';
	import { buildAdminDashboardViewModel } from '$lib/admin/dashboard-view-model';

	let { data }: { data: PageData } = $props();

	const model = $derived(buildAdminDashboardViewModel(data.overview));
	const attentionJobs = $derived(
		data.jobs.filter((job) => ['queued', 'leased', 'processing', 'failed', 'expired'].includes(job.status)).slice(0, 8)
	);
	const proxyIncidents = $derived(data.proxies.filter((proxy) => proxy.status !== 'active').slice(0, 8));

	const jobChartItems = $derived([
		{ label: 'Queued', value: data.overview.queuedJobs, tone: 'amber' as const },
		{ label: 'Leased', value: data.overview.leasedJobs, tone: 'blue' as const },
		{ label: 'Processing', value: data.overview.processingJobs, tone: 'blue' as const },
		{ label: 'Ready', value: data.overview.readyJobs, tone: 'green' as const },
		{ label: 'Failed', value: data.overview.failedJobs, tone: 'red' as const },
		{ label: 'Expired', value: data.overview.expiredJobs, tone: 'red' as const }
	]);

	const proxyChartItems = $derived([
		{ label: 'Active', value: data.overview.activeProxies, tone: 'green' as const },
		{ label: 'Quarantined', value: data.overview.quarantinedProxies, tone: 'red' as const },
		{ label: 'Disabled', value: data.overview.disabledProxies, tone: 'amber' as const }
	]);

	const artifactChartItems = $derived([
		{ label: 'Building', value: data.overview.buildingArtifacts, tone: 'blue' as const },
		{ label: 'Ready', value: data.overview.readyArtifacts, tone: 'green' as const }
	]);
</script>

<svelte:head>
	<title>Admin Overview</title>
</svelte:head>

<section class="admin-panel rounded-xl border border-slate-200 bg-white">
	<div class="border-b border-slate-200 px-5 py-5 md:px-6">
		<div class="flex flex-col gap-4 xl:flex-row xl:items-end xl:justify-between">
			<AdminSectionHeader
				eyebrow="Overview"
				title="Command center"
				description="Ảnh chụp nhanh toàn bộ control plane để thấy queue pressure, proxy risk và artifact readiness."
			/>
			<div class="grid gap-3 sm:grid-cols-3">
				<div class="rounded-lg border border-slate-300 bg-slate-50 px-4 py-3">
					<p class="text-[10px] font-bold uppercase tracking-[0.18em] text-slate-500">Backlog</p>
					<p class="mt-1 text-2xl font-black tracking-[-0.03em] text-slate-950">{model.queueBacklog}</p>
				</div>
				<div class="rounded-lg border border-slate-300 bg-slate-50 px-4 py-3">
					<p class="text-[10px] font-bold uppercase tracking-[0.18em] text-slate-500">Artifacts</p>
					<p class="mt-1 text-2xl font-black tracking-[-0.03em] text-slate-950">{model.totalArtifacts}</p>
				</div>
				<div class="rounded-lg border border-slate-300 bg-slate-50 px-4 py-3">
					<p class="text-[10px] font-bold uppercase tracking-[0.18em] text-slate-500">Signals / 24h</p>
					<p class="mt-1 text-2xl font-black tracking-[-0.03em] text-slate-950">
						{data.overview.eventsLast24h}
					</p>
				</div>
			</div>
		</div>
	</div>

	<div class="grid gap-4 px-5 py-5 md:grid-cols-2 xl:grid-cols-3 md:px-6">
		{#each model.topStats as stat}
			<AdminStatCard label={stat.label} value={stat.value} caption={stat.caption} tone={stat.tone} />
		{/each}
	</div>
</section>

<div class="mt-6 grid gap-6 xl:grid-cols-3">
	<AdminBarChart
		title="Job state distribution"
		description="Phân bố trạng thái hiện tại của mux jobs."
		items={jobChartItems}
	/>
	<AdminBarChart
		title="Proxy fleet distribution"
		description="Tỷ lệ active, quarantined và disabled trong inventory."
		items={proxyChartItems}
	/>
	<AdminBarChart
		title="Artifact cache"
		description="Số artifact đang build và đã ready để cấp ticket."
		items={artifactChartItems}
	/>
</div>

<div class="mt-6 grid gap-6 2xl:grid-cols-[minmax(0,1.4fr)_minmax(0,1fr)]">
	<section class="admin-panel rounded-xl border border-slate-200 bg-white">
		<div class="border-b border-slate-200 px-5 py-4 md:px-6">
			<AdminSectionHeader
				eyebrow="Attention"
				title="Jobs cần theo dõi"
				description="Ưu tiên những job chưa về trạng thái ready."
			/>
		</div>
		<AdminJobsTable jobs={attentionJobs.length > 0 ? attentionJobs : data.jobs.slice(0, 8)} />
	</section>

	<section class="admin-panel rounded-xl border border-slate-200 bg-white">
		<div class="border-b border-slate-200 px-5 py-4 md:px-6">
			<AdminSectionHeader
				eyebrow="Proxy issues"
				title="Proxy incidents"
				description="Danh sách proxy đang bị quarantine hoặc disabled."
			/>
		</div>
		<div class="overflow-x-auto">
			<table class="min-w-full text-sm text-slate-700">
				<thead class="bg-slate-100 text-left text-[10px] uppercase tracking-[0.18em] text-slate-500">
					<tr>
						<th class="px-4 py-3 font-bold">Proxy</th>
						<th class="px-4 py-3 font-bold">Status</th>
						<th class="px-4 py-3 font-bold">Health</th>
						<th class="px-4 py-3 font-bold">Reason</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-slate-200/80">
					{#each (proxyIncidents.length > 0 ? proxyIncidents : data.proxies.slice(0, 8)) as proxy}
						<tr class="align-top hover:bg-slate-50/80">
							<td class="px-4 py-3.5">
								<p class="text-sm font-semibold text-slate-900">{proxy.displayName || 'Unnamed proxy'}</p>
								<p class="mt-1 break-all font-mono text-[11px] text-slate-500">{proxy.maskedProxyUrl}</p>
							</td>
							<td class="px-4 py-3.5">
								<AdminStatusBadge value={proxy.status} kind="proxy" />
							</td>
							<td class="px-4 py-3.5 text-sm text-slate-700">
								{proxy.eventCount24h} events / 24h
							</td>
							<td class="px-4 py-3.5 text-xs text-slate-600">
								{proxy.lastQuarantineReason ?? proxy.lastEventType ?? 'No issue captured'}
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</section>
</div>

<section class="admin-panel mt-6 rounded-xl border border-slate-200 bg-white">
	<div class="border-b border-slate-200 px-5 py-4 md:px-6">
		<AdminSectionHeader
			eyebrow="Recent events"
			title="Latest activity"
			description="Timeline hợp nhất của jobs và proxy health để thấy nhịp vận hành gần nhất."
		/>
	</div>
	<AdminActivityTable activity={data.activity.slice(0, 12)} />
</section>
