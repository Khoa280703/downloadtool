<script lang="ts">
	import type { ActionData, PageData } from './$types';
	import AdminActivityTable from '$components/admin/AdminActivityTable.svelte';
	import AdminJobsTable from '$components/admin/AdminJobsTable.svelte';
	import AdminMiniMetric from '$components/admin/AdminMiniMetric.svelte';
	import AdminProxyTable from '$components/admin/AdminProxyTable.svelte';
	import AdminSectionHeader from '$components/admin/AdminSectionHeader.svelte';
	import AdminStatCard from '$components/admin/AdminStatCard.svelte';

	type StatTone = 'neutral' | 'sky' | 'emerald' | 'amber' | 'rose';
	type DashboardStat = {
		label: string;
		value: number;
		caption: string;
		tone: StatTone;
	};

	let { data, form }: { data: PageData; form: ActionData } = $props();

	const overview = $derived(data.dashboard.overview);
	const queueBacklog = $derived(overview.queuedJobs + overview.leasedJobs);
	const activeJobs = $derived(overview.processingJobs + overview.leasedJobs);
	const totalArtifacts = $derived(overview.buildingArtifacts + overview.readyArtifacts);
	const totalProxies = $derived(
		overview.activeProxies + overview.disabledProxies + overview.quarantinedProxies
	);
	const topStats = $derived.by<DashboardStat[]>(() => [
		{
			label: 'Queue backlog',
			value: queueBacklog,
			caption: `${overview.processingJobs} đang xử lý`,
			tone: queueBacklog > 0 ? 'amber' : 'neutral'
		},
		{
			label: 'Active workers',
			value: activeJobs,
			caption: `${overview.readyJobs} job hoàn tất`,
			tone: activeJobs > 0 ? 'sky' : 'neutral'
		},
		{
			label: 'Failed / expired',
			value: overview.failedJobs + overview.expiredJobs,
			caption: `${overview.failedJobs} failed, ${overview.expiredJobs} expired`,
			tone: overview.failedJobs + overview.expiredJobs > 0 ? 'rose' : 'neutral'
		},
		{
			label: 'Artifacts',
			value: totalArtifacts,
			caption: `${overview.readyArtifacts} file sẵn sàng`,
			tone: overview.readyArtifacts > 0 ? 'emerald' : 'neutral'
		},
		{
			label: 'Proxy fleet',
			value: totalProxies,
			caption: `${overview.quarantinedProxies} quarantined`,
			tone: overview.quarantinedProxies > 0 ? 'rose' : 'sky'
		},
		{
			label: 'Signals / 24h',
			value: overview.eventsLast24h,
			caption: 'job events + proxy events',
			tone: 'neutral'
		}
	]);
	const headerStats = $derived([
		{ label: 'Queued', value: overview.queuedJobs, caption: 'Chờ worker nhận lease' },
		{ label: 'Processing', value: overview.processingJobs, caption: 'Đang mux và upload' },
		{ label: 'Ready', value: overview.readyJobs, caption: 'Đã có ticket để tải' }
	]);
	const queueStats = $derived([
		{ label: 'Leased', value: overview.leasedJobs },
		{ label: 'Failed', value: overview.failedJobs },
		{ label: 'Expired', value: overview.expiredJobs }
	]);
	const snapshotStats = $derived([
		{
			label: 'Queue pressure',
			value: queueBacklog > 0 ? `${queueBacklog} job cần hoàn thành` : 'Ổn định',
			caption: `${overview.processingJobs} processing, ${overview.leasedJobs} leased.`
		},
		{
			label: 'Proxy coverage',
			value: `${overview.activeProxies} active / ${totalProxies}`,
			caption: `${overview.quarantinedProxies} quarantined, ${overview.disabledProxies} disabled.`
		},
		{
			label: 'Artifact cache',
			value: `${overview.readyArtifacts} ready / ${totalArtifacts}`,
			caption: `${overview.buildingArtifacts} artifact đang build.`
		}
	]);

	function formatGateMode(mode: string): string {
		return mode.replace(/[_-]/g, ' ');
	}
</script>

<svelte:head>
	<title>Admin Dashboard</title>
</svelte:head>

<section class="admin-page mx-auto max-w-[1600px] px-4 py-8 md:px-6 md:py-10">
	<div class="admin-shell relative overflow-hidden rounded-[2rem] border border-slate-200/80 bg-white shadow-[0_30px_80px_-48px_rgba(15,23,42,0.55)]">
		<div class="pointer-events-none absolute inset-x-0 top-0 h-56 bg-[radial-gradient(circle_at_top_left,_rgba(59,130,246,0.14),_transparent_34%),radial-gradient(circle_at_top_right,_rgba(16,185,129,0.12),_transparent_28%),linear-gradient(180deg,_rgba(248,250,252,0.95),_rgba(255,255,255,0))]"></div>

		<div class="relative border-b border-slate-200/80 px-5 py-6 md:px-8 md:py-8">
			<div class="flex flex-col gap-6 xl:flex-row xl:items-start xl:justify-between">
				<div class="max-w-3xl">
					<div class="flex flex-wrap items-center gap-2">
						<span class="rounded-full border border-slate-300 bg-white/80 px-3 py-1 text-[11px] font-bold uppercase tracking-[0.24em] text-slate-600">
							Operations
						</span>
						<span class="rounded-full border border-sky-200 bg-sky-50 px-3 py-1 text-[11px] font-bold uppercase tracking-[0.22em] text-sky-800">
							{formatGateMode(data.gateMode)}
						</span>
					</div>
					<h1 class="mt-4 text-3xl font-black tracking-[-0.04em] text-slate-950 md:text-5xl">
						Admin control plane
					</h1>
					<p class="mt-3 max-w-2xl text-sm leading-6 text-slate-600 md:text-base">
						Theo dõi queue mux, fleet proxy và artifact pipeline trong cùng một màn hình
						vận hành. Giữ thông tin cô đọng, ưu tiên scan nhanh và xử lý sự cố.
					</p>
					<div class="mt-5 flex flex-wrap items-center gap-3 text-sm text-slate-500">
						<span class="rounded-full bg-slate-100 px-3 py-1.5 font-medium text-slate-700">
							Signed in: {data.user.email}
						</span>
						<span class="rounded-full bg-slate-100 px-3 py-1.5 font-medium text-slate-700">
							{overview.activeProxies} active proxies
						</span>
						<span class="rounded-full bg-slate-100 px-3 py-1.5 font-medium text-slate-700">
							{overview.readyArtifacts} ready artifacts
						</span>
					</div>
				</div>

				<div class="grid gap-3 sm:grid-cols-3 xl:w-[28rem]">
					{#each headerStats as stat}
						<AdminMiniMetric label={stat.label} value={stat.value} caption={stat.caption} />
					{/each}
				</div>
			</div>

			{#if form?.error}
				<p class="mt-5 rounded-2xl border border-rose-200 bg-rose-50 px-4 py-3 text-sm text-rose-700">
					{form.error}
				</p>
			{/if}

			{#if form?.success}
				<p class="mt-5 rounded-2xl border border-emerald-200 bg-emerald-50 px-4 py-3 text-sm text-emerald-700">
					{form.success}
				</p>
			{/if}

			<div class="mt-6 flex flex-wrap items-center gap-3">
				<a
					href="/account"
					class="rounded-full border border-slate-300 bg-white px-5 py-3 text-sm font-bold text-slate-700 transition hover:border-slate-400 hover:text-slate-950"
				>
					Back to account
				</a>
			</div>
		</div>

		<div class="relative px-5 py-5 md:px-8 md:py-8">
			<div class="grid gap-4 md:grid-cols-2 2xl:grid-cols-6">
				{#each topStats as stat}
					<AdminStatCard
						label={stat.label}
						value={stat.value}
						caption={stat.caption}
						tone={stat.tone}
					/>
				{/each}
			</div>

			<div class="mt-6 grid gap-6 2xl:grid-cols-[minmax(0,1fr)_360px]">
				<div class="space-y-6">
					<section class="admin-panel rounded-[1.75rem] border border-slate-200 bg-white">
						<div class="flex flex-wrap items-start justify-between gap-4 border-b border-slate-200 px-5 py-5 md:px-6">
							<AdminSectionHeader
								eyebrow="Queue detail"
								title="Recent mux jobs"
								description="20 job gần nhất với trạng thái, số lần thử và artifact backend."
							/>
							<div class="grid gap-3 sm:grid-cols-3">
								{#each queueStats as stat}
									<AdminMiniMetric label={stat.label} value={stat.value} />
								{/each}
							</div>
						</div>
						<div class="px-0 py-0">
							<AdminJobsTable jobs={data.dashboard.jobs} />
						</div>
					</section>

					<section class="admin-panel rounded-[1.75rem] border border-slate-200 bg-white">
						<div class="border-b border-slate-200 px-5 py-5 md:px-6">
							<AdminSectionHeader
								eyebrow="Proxy fleet"
								title="Proxy inventory"
								description="Cập nhật trạng thái proxy trực tiếp từ dashboard, theo dõi quarantine và event gần nhất."
							/>
						</div>
						<AdminProxyTable proxies={data.dashboard.proxies} />
					</section>

					<section class="admin-panel rounded-[1.75rem] border border-slate-200 bg-white">
						<div class="border-b border-slate-200 px-5 py-5 md:px-6">
							<AdminSectionHeader
								eyebrow="System activity"
								title="Recent activity"
								description="Hợp nhất event từ job pipeline và proxy health để soi timeline sự cố."
							/>
						</div>
						<AdminActivityTable activity={data.dashboard.activity} />
					</section>
				</div>

				<aside class="space-y-6">
					<section class="admin-panel rounded-[1.75rem] border border-slate-200 bg-slate-950 p-5 text-white">
						<AdminSectionHeader
							eyebrow="System posture"
							title="Operational snapshot"
							description="Tóm tắt nhanh các vùng có thể gây áp lực lên pipeline ở thời điểm hiện tại."
						/>
						<div class="mt-5 grid gap-3">
							{#each snapshotStats as stat}
								<AdminMiniMetric
									label={stat.label}
									value={stat.value}
									caption={stat.caption}
									inverted={true}
								/>
							{/each}
						</div>
					</section>

					<section class="admin-panel rounded-[1.75rem] border border-slate-200 bg-white p-5">
						<AdminSectionHeader
							eyebrow="Proxy controls"
							title="Add proxy"
							description="Nhận full URL hoặc định dạng raw host:port:user:pass. Credentials chỉ được hiển thị dưới dạng masked khi render lại."
						/>
						<form method="POST" action="?/createProxy" class="mt-5 grid gap-3">
							<input
								type="text"
								name="proxyUrl"
								placeholder="socks5h://user:pass@host:port"
								class="admin-field rounded-2xl border border-slate-200 bg-white px-4 py-3 text-sm text-slate-800 placeholder:text-slate-400 focus:border-slate-400 focus:ring-0"
							/>
							<input
								type="text"
								name="displayName"
								placeholder="Display name"
								class="admin-field rounded-2xl border border-slate-200 bg-white px-4 py-3 text-sm text-slate-800 placeholder:text-slate-400 focus:border-slate-400 focus:ring-0"
							/>
							<textarea
								name="notes"
								rows="4"
								placeholder="Notes"
								class="admin-field rounded-2xl border border-slate-200 bg-white px-4 py-3 text-sm text-slate-800 placeholder:text-slate-400 focus:border-slate-400 focus:ring-0"
							></textarea>
							<button
								type="submit"
								class="rounded-2xl bg-slate-950 px-4 py-3 text-sm font-bold uppercase tracking-[0.18em] text-white transition hover:bg-slate-800"
							>
								Save proxy
							</button>
						</form>
					</section>

					<section class="admin-panel rounded-[1.75rem] border border-slate-200 bg-white p-5">
						<AdminSectionHeader
							eyebrow="Support metrics"
							title="Capacity"
							description="Sức chứa hiện tại của fleet và artifact pipeline."
						/>
						<div class="mt-5 grid gap-3 sm:grid-cols-2 xl:grid-cols-1">
							<AdminStatCard
								label="Active proxies"
								value={overview.activeProxies}
								caption="Proxy có thể phục vụ request"
								tone="emerald"
							/>
							<AdminStatCard
								label="Disabled proxies"
								value={overview.disabledProxies}
								caption="Bị tắt thủ công"
								tone="amber"
							/>
							<AdminStatCard
								label="Building artifacts"
								value={overview.buildingArtifacts}
								caption="Đang mux và upload lên storage"
								tone="sky"
							/>
							<AdminStatCard
								label="Quarantined proxies"
								value={overview.quarantinedProxies}
								caption="Cần kiểm tra trước khi tái sử dụng"
								tone="rose"
							/>
						</div>
					</section>
				</aside>
			</div>
		</div>
	</div>
</section>

<style>
	:global(.app.theme-dark) .admin-shell {
		border-color: rgba(148, 163, 184, 0.16);
		background:
			radial-gradient(circle at top left, rgba(59, 130, 246, 0.12), transparent 28%),
			radial-gradient(circle at top right, rgba(16, 185, 129, 0.1), transparent 24%),
			rgba(15, 23, 42, 0.92);
		box-shadow: 0 32px 80px -44px rgba(15, 23, 42, 0.85);
	}

	:global(.app.theme-dark .admin-page .admin-panel),
	:global(.app.theme-dark .admin-page .admin-kpi-box),
	:global(.app.theme-dark .admin-page .admin-stat-card) {
		border-color: rgba(148, 163, 184, 0.14) !important;
	}

	:global(.app.theme-dark .admin-page .admin-panel),
	:global(.app.theme-dark .admin-page .admin-kpi-box),
	:global(.app.theme-dark .admin-page .admin-stat-card),
	:global(.app.theme-dark .admin-page .admin-field),
	:global(.app.theme-dark .admin-page .admin-data-table thead) {
		background: rgba(15, 23, 42, 0.72) !important;
	}

	:global(.app.theme-dark .admin-page .text-slate-950),
	:global(.app.theme-dark .admin-page .text-slate-900),
	:global(.app.theme-dark .admin-page .text-slate-800) {
		color: rgba(248, 250, 252, 0.98) !important;
	}

	:global(.app.theme-dark .admin-page .text-slate-700),
	:global(.app.theme-dark .admin-page .text-slate-600),
	:global(.app.theme-dark .admin-page .text-slate-500) {
		color: rgba(203, 213, 225, 0.78) !important;
	}

	:global(.app.theme-dark .admin-page .admin-field) {
		background: rgba(15, 23, 42, 0.76) !important;
		color: rgba(248, 250, 252, 0.98) !important;
	}

	:global(.app.theme-dark .admin-page .admin-data-table tbody tr:hover) {
		background: rgba(51, 65, 85, 0.42) !important;
	}
</style>
