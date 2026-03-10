<script lang="ts">
	import type { ActionData, PageData } from './$types';
	import AdminActivityTable from '$components/admin/AdminActivityTable.svelte';
	import AdminJobsTable from '$components/admin/AdminJobsTable.svelte';
	import AdminProxyTable from '$components/admin/AdminProxyTable.svelte';
	import AdminStatCard from '$components/admin/AdminStatCard.svelte';

	let { data, form }: { data: PageData; form: ActionData } = $props();
</script>

<svelte:head>
	<title>Admin Dashboard</title>
</svelte:head>

<section class="mx-auto max-w-7xl px-4 py-10">
	<div class="rounded-[2rem] border border-white/70 bg-white/90 p-6 shadow-card">
		<div class="flex flex-wrap items-start justify-between gap-4">
			<div>
				<p class="text-xs font-bold uppercase tracking-[0.24em] text-plum/55">System admin</p>
				<h1 class="mt-2 text-4xl font-black tracking-tight text-plum">Control room</h1>
				<p class="mt-2 text-sm text-plum/70">
					Signed in as {data.user.email}. Gate mode: {data.gateMode}.
				</p>
			</div>
			<a
				href="/account"
				class="rounded-full border border-plum/20 px-5 py-3 text-sm font-bold text-plum transition hover:border-primary/40 hover:text-primary"
			>
				Back to account
			</a>
		</div>

		{#if form?.error}
			<p class="mt-5 rounded-2xl border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700">
				{form.error}
			</p>
		{/if}

		{#if form?.success}
			<p class="mt-5 rounded-2xl border border-emerald-200 bg-emerald-50 px-4 py-3 text-sm text-emerald-700">
				{form.success}
			</p>
		{/if}

		<div class="mt-8 grid gap-4 md:grid-cols-2 xl:grid-cols-5">
			<AdminStatCard label="Queued jobs" value={data.dashboard.overview.queuedJobs} tone="amber" />
			<AdminStatCard label="Leased jobs" value={data.dashboard.overview.leasedJobs} tone="pink" />
			<AdminStatCard label="Processing jobs" value={data.dashboard.overview.processingJobs} tone="pink" />
			<AdminStatCard label="Ready jobs" value={data.dashboard.overview.readyJobs} tone="emerald" />
			<AdminStatCard label="Expired jobs" value={data.dashboard.overview.expiredJobs} tone="amber" />
			<AdminStatCard label="Quarantined proxies" value={data.dashboard.overview.quarantinedProxies} tone="pink" />
			<AdminStatCard label="Events / 24h" value={data.dashboard.overview.eventsLast24h} tone="plum" />
		</div>

		<div class="mt-8 grid gap-6 xl:grid-cols-[1.2fr_0.8fr]">
			<section class="rounded-[1.75rem] border border-pink-100 bg-pink-50/50 p-5">
				<div class="flex items-center justify-between gap-3">
					<div>
						<h2 class="text-xl font-black text-plum">Recent mux jobs</h2>
						<p class="text-sm text-plum/65">Latest 20 jobs with status, attempts and artifact info.</p>
					</div>
				</div>
				<div class="mt-5">
					<AdminJobsTable jobs={data.dashboard.jobs} />
				</div>
			</section>

			<section class="rounded-[1.75rem] border border-pink-100 bg-pink-50/50 p-5">
				<h2 class="text-xl font-black text-plum">Add proxy</h2>
				<p class="mt-1 text-sm text-plum/65">
					Accepts full URL or raw <code>host:port:user:pass</code>.
				</p>
				<form method="POST" action="?/createProxy" class="mt-5 grid gap-3">
					<input
						type="text"
						name="proxyUrl"
						placeholder="socks5h://user:pass@host:port"
						class="rounded-2xl border border-pink-100 bg-white px-4 py-3 text-sm text-plum"
					/>
					<input
						type="text"
						name="displayName"
						placeholder="Display name"
						class="rounded-2xl border border-pink-100 bg-white px-4 py-3 text-sm text-plum"
					/>
					<textarea
						name="notes"
						rows="3"
						placeholder="Notes"
						class="rounded-2xl border border-pink-100 bg-white px-4 py-3 text-sm text-plum"
					></textarea>
					<button
						type="submit"
						class="rounded-2xl bg-gradient-primary px-4 py-3 text-sm font-bold uppercase tracking-wide text-white"
					>
						Save proxy
					</button>
				</form>

				<div class="mt-6 grid gap-3 sm:grid-cols-2">
					<AdminStatCard label="Active proxies" value={data.dashboard.overview.activeProxies} tone="emerald" />
					<AdminStatCard label="Disabled proxies" value={data.dashboard.overview.disabledProxies} tone="amber" />
					<AdminStatCard label="Building artifacts" value={data.dashboard.overview.buildingArtifacts} tone="pink" />
					<AdminStatCard label="Ready artifacts" value={data.dashboard.overview.readyArtifacts} tone="plum" />
				</div>
			</section>
		</div>

		<section class="mt-6 rounded-[1.75rem] border border-pink-100 bg-pink-50/50 p-5">
			<h2 class="text-xl font-black text-plum">Proxy inventory</h2>
			<p class="mt-1 text-sm text-plum/65">Update proxy status directly from the dashboard.</p>
			<div class="mt-5">
				<AdminProxyTable proxies={data.dashboard.proxies} />
			</div>
		</section>

		<section class="mt-6 rounded-[1.75rem] border border-pink-100 bg-pink-50/50 p-5">
			<h2 class="text-xl font-black text-plum">Recent activity</h2>
			<p class="mt-1 text-sm text-plum/65">Union of latest job events and proxy health events.</p>
			<div class="mt-5">
				<AdminActivityTable activity={data.dashboard.activity} />
			</div>
		</section>
	</div>
</section>
