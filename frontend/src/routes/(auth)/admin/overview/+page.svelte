<script lang="ts">
	import type { PageData } from './$types';
	import AdminMiniMetric from '$components/admin/AdminMiniMetric.svelte';
	import AdminSectionHeader from '$components/admin/AdminSectionHeader.svelte';
	import AdminStatCard from '$components/admin/AdminStatCard.svelte';
	import { adminSectionItems, buildAdminDashboardViewModel } from '$lib/admin/dashboard-view-model';

	let { data }: { data: PageData } = $props();

	const model = $derived(buildAdminDashboardViewModel(data.overview));
</script>

<svelte:head>
	<title>Admin Overview</title>
</svelte:head>

<header class="admin-panel rounded-[2rem] border border-slate-200/80 bg-white/95 px-5 py-6 shadow-[0_24px_80px_-44px_rgba(15,23,42,0.45)] md:px-7">
	<div class="flex flex-col gap-4 lg:flex-row lg:items-end lg:justify-between">
		<div>
			<p class="text-[11px] font-bold uppercase tracking-[0.22em] text-slate-500">Overview</p>
			<h2 class="mt-2 text-3xl font-black tracking-[-0.04em] text-slate-950">Tổng quan vận hành</h2>
			<p class="mt-2 max-w-3xl text-sm leading-6 text-slate-600">
				Điểm vào của admin app. Xem nhanh tình trạng hệ thống rồi đi thẳng vào workspace cần xử lý.
			</p>
		</div>
		<div class="grid gap-3 sm:grid-cols-3">
			<div class="rounded-2xl border border-slate-200 bg-slate-50 px-4 py-3">
				<p class="text-[11px] font-bold uppercase tracking-[0.18em] text-slate-500">Backlog</p>
				<p class="mt-2 text-2xl font-black tracking-[-0.03em] text-slate-950">{model.queueBacklog}</p>
			</div>
			<div class="rounded-2xl border border-slate-200 bg-slate-50 px-4 py-3">
				<p class="text-[11px] font-bold uppercase tracking-[0.18em] text-slate-500">Artifacts</p>
				<p class="mt-2 text-2xl font-black tracking-[-0.03em] text-slate-950">{model.totalArtifacts}</p>
			</div>
			<div class="rounded-2xl border border-slate-200 bg-slate-50 px-4 py-3">
				<p class="text-[11px] font-bold uppercase tracking-[0.18em] text-slate-500">Events / 24h</p>
				<p class="mt-2 text-2xl font-black tracking-[-0.03em] text-slate-950">
					{data.overview.eventsLast24h}
				</p>
			</div>
		</div>
	</div>
</header>

<section class="admin-panel rounded-[2rem] border border-slate-200 bg-white p-5 md:p-6">
	<AdminSectionHeader
		eyebrow="System overview"
		title="Operational summary"
		description="Các chỉ số chính để nắm trạng thái queue, proxy fleet và artifact pipeline."
	/>
	<div class="mt-5 grid gap-4 md:grid-cols-2 2xl:grid-cols-3">
		{#each model.topStats as stat}
			<AdminStatCard
				label={stat.label}
				value={stat.value}
				caption={stat.caption}
				tone={stat.tone}
			/>
		{/each}
	</div>
</section>

<div class="grid gap-6 2xl:grid-cols-[minmax(0,1fr)_360px]">
	<section class="admin-panel rounded-[2rem] border border-slate-200 bg-white p-5 md:p-6">
		<AdminSectionHeader
			eyebrow="Workspaces"
			title="Go to management areas"
			description="Mỗi khu quản lý là một route riêng, không còn dồn toàn bộ nội dung vào một trang."
		/>
		<div class="mt-5 grid gap-4 md:grid-cols-2">
			{#each adminSectionItems.filter((item) => item.id !== 'overview') as item}
				<a
					href={item.href}
					class="rounded-[1.5rem] border border-slate-200 bg-slate-50 p-4 transition hover:border-slate-300 hover:bg-slate-100"
				>
					<div class="flex items-center justify-between gap-3">
						<div>
							<p class="text-sm font-bold text-slate-900">{item.label}</p>
							<p class="mt-1 text-sm text-slate-600">{item.description}</p>
						</div>
						<span class="material-symbols-outlined text-slate-500">arrow_forward</span>
					</div>
				</a>
			{/each}
		</div>
	</section>

	<section class="admin-panel rounded-[2rem] border border-slate-200 bg-slate-950 p-5 text-white">
		<AdminSectionHeader
			eyebrow="System posture"
			title="Operational snapshot"
			description="Tóm tắt nhanh các vùng có thể gây áp lực lên pipeline ở thời điểm hiện tại."
		/>
		<div class="mt-5 grid gap-3">
			{#each model.snapshotStats as stat}
				<AdminMiniMetric
					label={stat.label}
					value={stat.value}
					caption={stat.caption}
					inverted={true}
				/>
			{/each}
		</div>
	</section>
</div>
