<script lang="ts">
	import type { ActionData, PageData } from './$types';
	import AdminProxyTable from '$components/admin/AdminProxyTable.svelte';
	import AppIcon from '$components/AppIcon.svelte';

	let { data, form }: { data: PageData; form: ActionData } = $props();

	// Fleet metrics
	const total = $derived(data.overview.activeProxies + data.overview.quarantinedProxies + data.overview.disabledProxies);
	const riskyProxies = $derived(data.proxies.filter((p) => p.healthScore < 55).length);
	const strongProxies = $derived(data.proxies.filter((p) => p.healthScore >= 80).length);
	const avgHealth = $derived(
		data.proxies.length ? Math.round(data.proxies.reduce((s, p) => s + p.healthScore, 0) / data.proxies.length) : 0
	);
	const totalExtract = $derived(data.proxies.reduce((s, p) => s + p.extractAttempts24h, 0));
	const totalSuccess = $derived(data.proxies.reduce((s, p) => s + p.extractSuccesses24h, 0));
	const totalRelevant = $derived(data.proxies.reduce((s, p) => s + p.proxyRelevantAttempts24h, 0));
	const successRate = $derived(totalRelevant > 0 ? Math.round((totalSuccess / totalRelevant) * 100) : 0);

	// Pagination
	const PAGE_SIZE = 10;
	let currentPage = $state(1);
	const totalPages = $derived(Math.max(1, Math.ceil(data.proxies.length / PAGE_SIZE)));
	const paginatedProxies = $derived(data.proxies.slice((currentPage - 1) * PAGE_SIZE, currentPage * PAGE_SIZE));

	function goPage(page: number) {
		currentPage = Math.max(1, Math.min(page, totalPages));
	}

	function healthColor(score: number): string {
		if (score >= 80) return 'text-green-700';
		if (score >= 55) return 'text-amber-600';
		return 'text-red-600';
	}

	// Modal
	let showAddModal = $state(false);

	// Auto-open modal on form success/error so user sees feedback
	$effect(() => {
		if (form?.error || form?.success) {
			showAddModal = true;
		}
	});
</script>

<svelte:head>
	<title>Admin Proxies</title>
</svelte:head>

<!-- Page header -->
<div class="mb-5 flex items-center justify-between">
	<div>
		<h2 class="text-lg font-semibold text-gray-900">Proxy Management</h2>
		<p class="mt-0.5 text-[13px] text-gray-500">{total} proxies in fleet · {data.proxies.length} visible</p>
	</div>
	<button
		type="button"
		onclick={() => (showAddModal = true)}
		class="inline-flex items-center gap-1.5 rounded-md bg-gray-900 px-3.5 py-2 text-[13px] font-medium text-white transition hover:bg-gray-800"
	>
		+ Add Proxy
	</button>
</div>

{#if form?.error}
	<p class="mb-4 rounded-md border border-red-200 bg-red-50 px-4 py-2.5 text-[13px] text-red-700">{form.error}</p>
{/if}
{#if form?.success}
	<p class="mb-4 rounded-md border border-green-200 bg-green-50 px-4 py-2.5 text-[13px] text-green-700">{form.success}</p>
{/if}

<!-- Fleet Summary KPI strip -->
<div class="admin-panel mb-5 overflow-hidden border border-gray-200 bg-white">
	<div class="grid grid-cols-3 divide-x divide-gray-100 md:grid-cols-6">
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-green-700">{data.overview.activeProxies}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Active</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-red-600">{data.overview.quarantinedProxies}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Quarantined</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-gray-400">{data.overview.disabledProxies}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Disabled</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class={`text-2xl font-semibold tabular-nums ${healthColor(avgHealth)}`}>{avgHealth}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Avg Health</p>
			<p class="text-[10px] text-gray-400">{strongProxies}↑ {riskyProxies}↓</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-gray-900">{successRate}%</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Success 24h</p>
			<p class="text-[10px] tabular-nums text-gray-400">{totalSuccess}/{totalRelevant}</p>
		</div>
		<div class="px-4 py-3 text-center">
			<p class="text-2xl font-semibold tabular-nums text-gray-900">{totalExtract}</p>
			<p class="mt-0.5 text-[10px] font-medium uppercase tracking-wider text-gray-400">Extracts 24h</p>
			<p class="text-[10px] tabular-nums text-gray-400">{totalRelevant} relevant</p>
		</div>
	</div>
</div>

<!-- Proxy Inventory -->
<section class="admin-panel overflow-hidden border border-gray-200 bg-white">
	<div class="flex items-center justify-between border-b border-gray-200 bg-gray-50 px-5 py-2.5">
		<div>
			<h3 class="text-[13px] font-semibold text-gray-900">Proxy Inventory</h3>
			<p class="text-[11px] text-gray-400">Expand row for extract stats. Update status inline.</p>
		</div>
		<!-- Pagination controls -->
		{#if totalPages > 1}
			<div class="flex items-center gap-1">
				<button
					type="button"
					disabled={currentPage <= 1}
					onclick={() => goPage(currentPage - 1)}
					class="inline-flex h-7 w-7 items-center justify-center rounded border border-gray-200 text-gray-500 transition hover:bg-gray-100 disabled:opacity-30 disabled:hover:bg-transparent"
				>
					<AppIcon name="chevron_left" class="text-sm" />
				</button>
				{#each Array.from({ length: totalPages }, (_, i) => i + 1) as page}
					<button
						type="button"
						onclick={() => goPage(page)}
						class={`inline-flex h-7 min-w-7 items-center justify-center rounded px-1.5 text-[12px] font-medium tabular-nums transition ${
							page === currentPage
								? 'bg-gray-900 text-white'
								: 'border border-gray-200 text-gray-600 hover:bg-gray-100'
						}`}
					>
						{page}
					</button>
				{/each}
				<button
					type="button"
					disabled={currentPage >= totalPages}
					onclick={() => goPage(currentPage + 1)}
					class="inline-flex h-7 w-7 items-center justify-center rounded border border-gray-200 text-gray-500 transition hover:bg-gray-100 disabled:opacity-30 disabled:hover:bg-transparent"
				>
					<AppIcon name="chevron_right" class="text-sm" />
				</button>
			</div>
		{/if}
	</div>

	<AdminProxyTable proxies={paginatedProxies} />

	<!-- Bottom pagination info -->
	{#if totalPages > 1}
		<div class="flex items-center justify-between border-t border-gray-100 px-5 py-2">
			<p class="text-[11px] text-gray-400">
				Showing {(currentPage - 1) * PAGE_SIZE + 1}–{Math.min(currentPage * PAGE_SIZE, data.proxies.length)} of {data.proxies.length}
			</p>
			<p class="text-[11px] text-gray-400">Page {currentPage} of {totalPages}</p>
		</div>
	{/if}
</section>

<!-- Add Proxy Modal -->
{#if showAddModal}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/30"
		onclick={(e) => { if (e.target === e.currentTarget) showAddModal = false; }}
		onkeydown={(e) => { if (e.key === 'Escape') showAddModal = false; }}
		role="dialog"
		tabindex="-1"
	>
		<div class="w-full max-w-lg rounded-lg border border-gray-200 bg-white shadow-xl">
			<div class="flex items-center justify-between border-b border-gray-200 px-5 py-3">
				<div>
					<h3 class="text-[14px] font-semibold text-gray-900">Add Proxy</h3>
					<p class="text-[12px] text-gray-400">Full URL or raw host:port:user:pass format.</p>
				</div>
				<button
					type="button"
					onclick={() => (showAddModal = false)}
					class="inline-flex h-7 w-7 items-center justify-center rounded text-gray-400 transition hover:bg-gray-100 hover:text-gray-600"
				>
					<AppIcon name="close" class="text-lg" />
				</button>
			</div>
			<form method="POST" action="?/createProxy" class="grid gap-3 p-5">
				<div>
					<label for="proxyUrl" class="mb-1 block text-[11px] font-medium uppercase tracking-wider text-gray-500">Proxy URL</label>
					<input
						id="proxyUrl"
						type="text"
						name="proxyUrl"
						placeholder="socks5h://user:pass@host:port"
						class="w-full rounded-md border border-gray-200 bg-white px-3 py-2 text-[13px] text-gray-800 placeholder:text-gray-300 focus:border-gray-400 focus:outline-none focus:ring-0"
					/>
				</div>
				<div class="grid gap-3 md:grid-cols-2">
					<div>
						<label for="displayName" class="mb-1 block text-[11px] font-medium uppercase tracking-wider text-gray-500">Display Name</label>
						<input
							id="displayName"
							type="text"
							name="displayName"
							placeholder="e.g. US-West-01"
							class="w-full rounded-md border border-gray-200 bg-white px-3 py-2 text-[13px] text-gray-800 placeholder:text-gray-300 focus:border-gray-400 focus:outline-none focus:ring-0"
						/>
					</div>
					<div>
						<label for="notes" class="mb-1 block text-[11px] font-medium uppercase tracking-wider text-gray-500">Notes</label>
						<input
							id="notes"
							type="text"
							name="notes"
							placeholder="Optional notes"
							class="w-full rounded-md border border-gray-200 bg-white px-3 py-2 text-[13px] text-gray-800 placeholder:text-gray-300 focus:border-gray-400 focus:outline-none focus:ring-0"
						/>
					</div>
				</div>
				<div class="flex items-center justify-end gap-2 pt-1">
					<button
						type="button"
						onclick={() => (showAddModal = false)}
						class="rounded-md border border-gray-200 px-3.5 py-2 text-[13px] font-medium text-gray-600 transition hover:bg-gray-50"
					>
						Cancel
					</button>
					<button
						type="submit"
						class="rounded-md bg-gray-900 px-4 py-2 text-[13px] font-medium text-white transition hover:bg-gray-800"
					>
						Save Proxy
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

