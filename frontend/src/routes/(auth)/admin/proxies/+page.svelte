<script lang="ts">
	import type { ActionData, PageData } from './$types';
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

<div class="mb-5 flex items-center justify-between">
	<div>
		<h2 class="text-lg font-semibold text-gray-900">Proxy Management</h2>
		<p class="mt-0.5 text-[13px] text-gray-500">Inventory, health status, and operational notes.</p>
	</div>
	<a
		href="#add-proxy"
		class="inline-flex items-center gap-1.5 rounded-md bg-gray-900 px-3.5 py-2 text-[13px] font-medium text-white transition hover:bg-gray-800"
	>
		+ Add Proxy
	</a>
</div>

{#if form?.error}
	<p class="mb-4 rounded-md border border-red-200 bg-red-50 px-4 py-2.5 text-[13px] text-red-700">{form.error}</p>
{/if}

{#if form?.success}
	<p class="mb-4 rounded-md border border-green-200 bg-green-50 px-4 py-2.5 text-[13px] text-green-700">{form.success}</p>
{/if}

<section class="admin-panel mb-5 overflow-hidden border border-gray-200 bg-white">
	<div class="border-b border-gray-200 bg-gray-50 px-5 py-3">
		<h3 class="text-[11px] font-semibold uppercase tracking-wider text-gray-500">Fleet Summary</h3>
	</div>
	<div class="overflow-x-auto">
		<table class="w-full border-collapse text-left text-[13px]">
			<thead>
				<tr class="border-b border-gray-100 bg-gray-50/40">
					<th class="px-5 py-2.5 text-[10px] font-semibold uppercase tracking-wider text-gray-400">State</th>
					<th class="px-5 py-2.5 text-[10px] font-semibold uppercase tracking-wider text-gray-400">Count</th>
					<th class="px-5 py-2.5 text-[10px] font-semibold uppercase tracking-wider text-gray-400">Note</th>
				</tr>
			</thead>
			<tbody class="divide-y divide-gray-50">
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Active</td><td class="px-5 py-2.5 tabular-nums text-gray-700">{data.overview.activeProxies}</td><td class="px-5 py-2.5 text-gray-400">Available for extract/download.</td></tr>
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Quarantined</td><td class="px-5 py-2.5 tabular-nums text-gray-700">{data.overview.quarantinedProxies}</td><td class="px-5 py-2.5 text-gray-400">Temporarily removed due to upstream issues.</td></tr>
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Disabled</td><td class="px-5 py-2.5 tabular-nums text-gray-700">{data.overview.disabledProxies}</td><td class="px-5 py-2.5 text-gray-400">Manually disabled, not in rotation.</td></tr>
				<tr><td class="px-5 py-2.5 font-medium text-gray-900">Avg. health score</td><td class="px-5 py-2.5 tabular-nums text-gray-700">{avgHealthScore}</td><td class="px-5 py-2.5 text-gray-400">{strongProxies} strong · {riskyProxies} at risk</td></tr>
			</tbody>
		</table>
	</div>
</section>

<section class="admin-panel mb-5 overflow-hidden border border-gray-200 bg-white">
	<div class="border-b border-gray-200 bg-gray-50 px-5 py-3">
		<h3 class="text-[13px] font-semibold text-gray-900">Proxy Inventory</h3>
		<p class="mt-0.5 text-[12px] text-gray-400">Update status and reason inline per proxy.</p>
	</div>
	<AdminProxyTable proxies={data.proxies} />
</section>

<section id="add-proxy" class="admin-panel overflow-hidden border border-gray-200 bg-white">
	<div class="border-b border-gray-200 bg-gray-50 px-5 py-3">
		<h3 class="text-[13px] font-semibold text-gray-900">Add Proxy</h3>
		<p class="mt-0.5 text-[12px] text-gray-400">Full URL or raw host:port:user:pass format.</p>
	</div>
	<form method="POST" action="?/createProxy" class="grid gap-3 p-5">
		<input
			type="text"
			name="proxyUrl"
			placeholder="socks5h://user:pass@host:port"
			class="rounded-md border border-gray-200 bg-white px-3 py-2 text-[13px] text-gray-800 placeholder:text-gray-300 focus:border-gray-400 focus:outline-none focus:ring-0"
		/>
		<div class="grid gap-3 md:grid-cols-2">
			<input
				type="text"
				name="displayName"
				placeholder="Display name"
				class="rounded-md border border-gray-200 bg-white px-3 py-2 text-[13px] text-gray-800 placeholder:text-gray-300 focus:border-gray-400 focus:outline-none focus:ring-0"
			/>
			<input
				type="text"
				name="notes"
				placeholder="Notes"
				class="rounded-md border border-gray-200 bg-white px-3 py-2 text-[13px] text-gray-800 placeholder:text-gray-300 focus:border-gray-400 focus:outline-none focus:ring-0"
			/>
		</div>
		<div>
			<button
				type="submit"
				class="rounded-md bg-gray-900 px-4 py-2 text-[13px] font-medium text-white transition hover:bg-gray-800"
			>
				Save Proxy
			</button>
		</div>
	</form>
</section>
