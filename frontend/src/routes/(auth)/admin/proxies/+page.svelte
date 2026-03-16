<script lang="ts">
	import type { ActionData, PageData } from './$types';
	import AppIcon from '$components/AppIcon.svelte';
	import AdminProxyTable from '$components/admin/AdminProxyTable.svelte';

	let { data, form }: { data: PageData; form: ActionData } = $props();

	const riskyProxies = $derived(data.proxies.filter((proxy) => proxy.healthScore < 55).length);
	const strongProxies = $derived(data.proxies.filter((proxy) => proxy.healthScore >= 80).length);
	const avgHealthScore = $derived(
		data.proxies.length
			? Math.round(data.proxies.reduce((sum, proxy) => sum + proxy.healthScore, 0) / data.proxies.length)
			: 0
	);
</script>

<svelte:head>
	<title>Admin Proxies</title>
</svelte:head>

<div class="mb-8 flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
	<div>
		<h2 class="text-3xl font-black tracking-tight text-slate-900">Proxy Management</h2>
		<p class="mt-1 text-sm text-slate-500">Quản lý inventory proxy, trạng thái health và ghi chú vận hành.</p>
	</div>
	<a
		href="#add-proxy"
		class="inline-flex items-center gap-2 rounded-xl bg-[#137fec] px-6 py-3 font-bold text-white shadow-lg shadow-[#137fec]/25 transition-all hover:opacity-90"
	>
		<AppIcon name="add" />
		<span>Add Proxy</span>
	</a>
</div>

{#if form?.error}
	<p class="mb-6 rounded-lg border border-rose-200 bg-rose-50 px-4 py-3 text-sm text-rose-700">{form.error}</p>
{/if}

{#if form?.success}
	<p class="mb-6 rounded-lg border border-emerald-200 bg-emerald-50 px-4 py-3 text-sm text-emerald-700">{form.success}</p>
{/if}

<section class="admin-panel mb-6 overflow-hidden border border-slate-200 bg-white">
	<div class="border-b border-slate-200 bg-slate-50 px-6 py-4">
		<h3 class="text-xs font-bold uppercase tracking-wider text-slate-500">Fleet Summary</h3>
	</div>
	<div class="overflow-x-auto">
		<table class="w-full border-collapse text-left">
			<thead>
				<tr class="border-b border-slate-200 bg-slate-50/60">
					<th class="px-6 py-4 text-xs font-bold uppercase tracking-wider text-slate-500">State</th>
					<th class="px-6 py-4 text-xs font-bold uppercase tracking-wider text-slate-500">Count</th>
					<th class="px-6 py-4 text-xs font-bold uppercase tracking-wider text-slate-500">Detail</th>
				</tr>
			</thead>
			<tbody class="divide-y divide-slate-100 text-sm">
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Active</td><td class="px-6 py-4">{data.overview.activeProxies}</td><td class="px-6 py-4 text-slate-500">Có thể phục vụ extract/download.</td></tr>
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Quarantined</td><td class="px-6 py-4">{data.overview.quarantinedProxies}</td><td class="px-6 py-4 text-slate-500">Tạm loại do gặp issue upstream.</td></tr>
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Disabled</td><td class="px-6 py-4">{data.overview.disabledProxies}</td><td class="px-6 py-4 text-slate-500">Tắt thủ công, không đưa vào vòng xoay.</td></tr>
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Avg. health score</td><td class="px-6 py-4">{avgHealthScore}</td><td class="px-6 py-4 text-slate-500">{strongProxies} proxy mạnh, {riskyProxies} proxy rủi ro.</td></tr>
			</tbody>
		</table>
	</div>
</section>

<section class="admin-panel mb-6 overflow-hidden border border-slate-200 bg-white">
	<div class="border-b border-slate-200 bg-slate-50 px-6 py-4">
		<h3 class="text-sm font-bold text-slate-900">Proxy Inventory</h3>
		<p class="mt-1 text-sm text-slate-500">Cập nhật status và reason trực tiếp trên từng dòng.</p>
	</div>
	<AdminProxyTable proxies={data.proxies} />
</section>

<section id="add-proxy" class="admin-panel overflow-hidden border border-slate-200 bg-white">
	<div class="border-b border-slate-200 bg-slate-50 px-6 py-4">
		<h3 class="text-sm font-bold text-slate-900">Add Proxy</h3>
		<p class="mt-1 text-sm text-slate-500">Nhập URL đầy đủ hoặc chuỗi raw host:port:user:pass.</p>
	</div>
	<form method="POST" action="?/createProxy" class="grid gap-4 p-6">
		<input
			type="text"
			name="proxyUrl"
			placeholder="socks5h://user:pass@host:port"
			class="admin-field rounded-lg border border-slate-200 bg-slate-50 px-4 py-3 text-sm text-slate-800 placeholder:text-slate-400 focus:border-slate-400 focus:ring-0"
		/>
		<div class="grid gap-4 md:grid-cols-2">
			<input
				type="text"
				name="displayName"
				placeholder="Display name"
				class="admin-field rounded-lg border border-slate-200 bg-slate-50 px-4 py-3 text-sm text-slate-800 placeholder:text-slate-400 focus:border-slate-400 focus:ring-0"
			/>
			<input
				type="text"
				name="notes"
				placeholder="Notes"
				class="admin-field rounded-lg border border-slate-200 bg-slate-50 px-4 py-3 text-sm text-slate-800 placeholder:text-slate-400 focus:border-slate-400 focus:ring-0"
			/>
		</div>
		<div>
			<button
				type="submit"
				class="inline-flex items-center gap-2 rounded-xl bg-[#137fec] px-6 py-3 font-bold text-white shadow-lg shadow-[#137fec]/25 transition-all hover:opacity-90"
			>
				<AppIcon name="save" />
				<span>Save Proxy</span>
			</button>
		</div>
	</form>
</section>
