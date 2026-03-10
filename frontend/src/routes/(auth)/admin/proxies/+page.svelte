<script lang="ts">
	import type { ActionData, PageData } from './$types';
	import AdminBarChart from '$components/admin/AdminBarChart.svelte';
	import AdminProxyTable from '$components/admin/AdminProxyTable.svelte';
	import AdminSectionHeader from '$components/admin/AdminSectionHeader.svelte';
	import AdminStatCard from '$components/admin/AdminStatCard.svelte';

	let { data, form }: { data: PageData; form: ActionData } = $props();

	const proxyChartItems = $derived([
		{ label: 'Active', value: data.overview.activeProxies, tone: 'green' as const },
		{ label: 'Quarantined', value: data.overview.quarantinedProxies, tone: 'red' as const },
		{ label: 'Disabled', value: data.overview.disabledProxies, tone: 'amber' as const }
	]);
</script>

<svelte:head>
	<title>Admin Proxies</title>
</svelte:head>

{#if form?.error}
	<p class="mb-4 rounded-lg border border-rose-200 bg-rose-50 px-4 py-3 text-sm text-rose-700">
		{form.error}
	</p>
{/if}

{#if form?.success}
	<p class="mb-4 rounded-lg border border-emerald-200 bg-emerald-50 px-4 py-3 text-sm text-emerald-700">
		{form.success}
	</p>
{/if}

<section class="admin-panel rounded-xl border border-slate-200 bg-white">
	<div class="border-b border-slate-200 px-5 py-5 md:px-6">
		<AdminSectionHeader
			eyebrow="Proxies"
			title="Fleet management"
			description="Quản lý inventory proxy, trạng thái quarantine và các thay đổi thủ công của operator."
		/>
	</div>

	<div class="grid gap-4 px-5 py-5 md:grid-cols-2 xl:grid-cols-4 md:px-6">
		<AdminStatCard label="Active proxies" value={data.overview.activeProxies} caption="Sẵn sàng nhận extract traffic" tone="emerald" />
		<AdminStatCard label="Quarantined" value={data.overview.quarantinedProxies} caption="Cần kiểm tra trước khi phục hồi" tone="rose" />
		<AdminStatCard label="Disabled" value={data.overview.disabledProxies} caption="Bị loại khỏi rotation thủ công" tone="amber" />
		<AdminStatCard label="Signals / 24h" value={data.overview.eventsLast24h} caption="Tổng job + proxy events trong 24h" tone="neutral" />
	</div>
</section>

<div class="mt-6 grid gap-6 2xl:grid-cols-[minmax(0,1fr)_360px]">
	<section class="admin-panel rounded-xl border border-slate-200 bg-white">
		<div class="border-b border-slate-200 px-5 py-4 md:px-6">
			<AdminSectionHeader
				eyebrow="Inventory"
				title="Proxy registry"
				description="Bảng điều hành proxy với health metadata và thao tác cập nhật trạng thái."
			/>
		</div>
		<AdminProxyTable proxies={data.proxies} />
	</section>

	<div class="space-y-6">
		<AdminBarChart
			title="Fleet distribution"
			description="Tỷ trọng proxy đang usable so với quarantine và disabled."
			items={proxyChartItems}
		/>

		<section class="admin-panel rounded-xl border border-slate-200 bg-white p-5">
			<AdminSectionHeader
				eyebrow="Controls"
				title="Register new proxy"
				description="Nhập full URL hoặc chuỗi raw host:port:user:pass để thêm vào inventory."
			/>
			<form method="POST" action="?/createProxy" class="mt-4 grid gap-3">
				<input
					type="text"
					name="proxyUrl"
					placeholder="socks5h://user:pass@host:port"
					class="admin-field rounded-md border border-slate-300 bg-white px-3 py-2.5 text-sm text-slate-800 placeholder:text-slate-400 focus:border-slate-400 focus:ring-0"
				/>
				<input
					type="text"
					name="displayName"
					placeholder="Display name"
					class="admin-field rounded-md border border-slate-300 bg-white px-3 py-2.5 text-sm text-slate-800 placeholder:text-slate-400 focus:border-slate-400 focus:ring-0"
				/>
				<textarea
					name="notes"
					rows="4"
					placeholder="Notes"
					class="admin-field rounded-md border border-slate-300 bg-white px-3 py-2.5 text-sm text-slate-800 placeholder:text-slate-400 focus:border-slate-400 focus:ring-0"
				></textarea>
				<button
					type="submit"
					class="rounded-md bg-slate-900 px-4 py-2.5 text-sm font-bold text-white transition hover:bg-slate-800"
				>
					Save proxy
				</button>
			</form>
		</section>
	</div>
</div>
