<script lang="ts">
	import type { ActionData, PageData } from './$types';
	import AdminProxyTable from '$components/admin/AdminProxyTable.svelte';
	import AdminSectionHeader from '$components/admin/AdminSectionHeader.svelte';
	import AdminStatCard from '$components/admin/AdminStatCard.svelte';

	let { data, form }: { data: PageData; form: ActionData } = $props();
</script>

<svelte:head>
	<title>Admin Proxies</title>
</svelte:head>

<header class="admin-panel rounded-[2rem] border border-slate-200/80 bg-white/95 px-5 py-6 shadow-[0_24px_80px_-44px_rgba(15,23,42,0.45)] md:px-7">
	<AdminSectionHeader
		eyebrow="Proxies"
		title="Fleet management"
		description="Quản lý inventory proxy, quarantine state và thêm proxy mới vào hệ thống."
	/>
</header>

{#if form?.error}
	<p class="rounded-2xl border border-rose-200 bg-rose-50 px-4 py-3 text-sm text-rose-700">
		{form.error}
	</p>
{/if}

{#if form?.success}
	<p class="rounded-2xl border border-emerald-200 bg-emerald-50 px-4 py-3 text-sm text-emerald-700">
		{form.success}
	</p>
{/if}

<div class="grid gap-6 2xl:grid-cols-[minmax(0,1fr)_380px]">
	<section class="admin-panel rounded-[2rem] border border-slate-200 bg-white">
		<div class="border-b border-slate-200 px-5 py-5 md:px-6">
			<AdminSectionHeader
				eyebrow="Proxy fleet"
				title="Inventory"
				description="Theo dõi trạng thái, quarantine reason, event gần nhất và update trực tiếp."
			/>
		</div>
		<AdminProxyTable proxies={data.proxies} />
	</section>

	<div class="space-y-6">
		<section class="admin-panel rounded-[2rem] border border-slate-200 bg-white p-5">
			<AdminSectionHeader
				eyebrow="Proxy controls"
				title="Add proxy"
				description="Nhận full URL hoặc định dạng raw host:port:user:pass."
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

		<section class="admin-panel rounded-[2rem] border border-slate-200 bg-white p-5">
			<AdminSectionHeader
				eyebrow="Fleet health"
				title="Proxy capacity"
				description="Sức chứa hiện tại của inventory proxy."
			/>
			<div class="mt-5 grid gap-3">
				<AdminStatCard
					label="Active proxies"
					value={data.overview.activeProxies}
					caption="Proxy có thể phục vụ request"
					tone="emerald"
				/>
				<AdminStatCard
					label="Disabled proxies"
					value={data.overview.disabledProxies}
					caption="Bị tắt thủ công"
					tone="amber"
				/>
				<AdminStatCard
					label="Quarantined proxies"
					value={data.overview.quarantinedProxies}
					caption="Cần kiểm tra trước khi tái sử dụng"
					tone="rose"
				/>
			</div>
		</section>
	</div>
</div>
