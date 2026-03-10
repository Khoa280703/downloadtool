<script lang="ts">
	import { page } from '$app/stores';
	import type { LayoutData } from './$types';
	import AdminMiniMetric from '$components/admin/AdminMiniMetric.svelte';
	import {
		adminSectionItems,
		buildAdminDashboardViewModel,
		getAdminSectionBadge
	} from '$lib/admin/dashboard-view-model';

	let { data, children }: { data: LayoutData; children: import('svelte').Snippet } = $props();

	const model = $derived(buildAdminDashboardViewModel(data.overview));
	const activeSection = $derived(
		adminSectionItems.find((section) => $page.url.pathname === section.href) ?? adminSectionItems[0]
	);

	function formatGateMode(mode: string): string {
		return mode.replace(/[_-]/g, ' ');
	}

	function isActive(href: string): boolean {
		return $page.url.pathname === href;
	}
</script>

<section class="admin-page min-h-screen bg-[#f3f4f6]">
	<div class="grid min-h-screen xl:grid-cols-[252px_minmax(0,1fr)]">
		<aside class="admin-sidebar border-b border-slate-800 bg-slate-950 text-slate-100 xl:border-b-0 xl:border-r">
			<div class="flex h-full flex-col px-5 py-5">
				<div class="border-b border-slate-800 pb-5">
					<p class="text-[10px] font-bold uppercase tracking-[0.24em] text-slate-500">Admin console</p>
					<h1 class="mt-2 text-xl font-bold tracking-[-0.02em] text-white">Mux control plane</h1>
					<p class="mt-2 text-sm leading-6 text-slate-400">
						Quản lý queue, proxy fleet, storage artifact và tín hiệu vận hành.
					</p>
				</div>

				<div class="mt-5 border-b border-slate-800 pb-5">
					<p class="text-[10px] font-bold uppercase tracking-[0.2em] text-slate-500">Operator</p>
					<p class="mt-2 truncate text-sm font-semibold text-white">{data.user.email}</p>
					<div class="mt-3 flex flex-wrap gap-2 text-[11px]">
						<span class="rounded-md border border-slate-700 bg-slate-900 px-2.5 py-1 text-slate-300">
							{formatGateMode(data.gateMode)}
						</span>
						<span class="rounded-md border border-slate-700 bg-slate-900 px-2.5 py-1 text-slate-300">
							{data.overview.activeProxies} active proxies
						</span>
					</div>
				</div>

				<nav class="mt-5 space-y-5">
					{#each ['Monitor', 'Operations'] as group}
						<div>
							<p class="mb-2 text-[10px] font-bold uppercase tracking-[0.22em] text-slate-500">{group}</p>
							<div class="space-y-1">
								{#each adminSectionItems.filter((section) => section.group === group) as section}
									<a
										href={section.href}
										class={`flex items-start gap-3 rounded-md border px-3 py-2.5 transition ${
											isActive(section.href)
												? 'border-slate-600 bg-slate-800 text-white'
												: 'border-transparent text-slate-300 hover:border-slate-800 hover:bg-slate-900'
										}`}
									>
										<span class="material-symbols-outlined mt-0.5 text-[18px]">{section.icon}</span>
										<span class="min-w-0 flex-1">
											<span class="flex items-center justify-between gap-2">
												<span class="block text-sm font-semibold">{section.label}</span>
												{#if getAdminSectionBadge(section.id, data.overview)}
													<span
														class={`rounded-sm px-1.5 py-0.5 text-[10px] font-bold ${
															isActive(section.href)
																? 'bg-slate-700 text-slate-100'
																: 'bg-slate-800 text-slate-300'
														}`}
													>
														{getAdminSectionBadge(section.id, data.overview)}
													</span>
												{/if}
											</span>
											<span class="mt-1 block text-xs leading-5 text-slate-500">
												{section.description}
											</span>
										</span>
									</a>
								{/each}
							</div>
						</div>
					{/each}
				</nav>

				<div class="mt-5 grid gap-2 xl:mt-auto">
					{#each model.headerStats as stat}
						<AdminMiniMetric
							label={stat.label}
							value={stat.value}
							caption={stat.caption}
							inverted={true}
						/>
					{/each}
				</div>

				<a
					href="/account"
					class="mt-5 inline-flex items-center justify-center rounded-md border border-slate-700 px-3 py-2 text-sm font-semibold text-slate-200 transition hover:border-slate-500 hover:bg-slate-900"
				>
					Back to account
				</a>
			</div>
		</aside>

		<div class="min-w-0">
			<header class="border-b border-slate-200 bg-white px-4 py-4 md:px-6">
				<div class="flex flex-col gap-4 xl:flex-row xl:items-center xl:justify-between">
					<div>
						<p class="text-[10px] font-bold uppercase tracking-[0.2em] text-slate-500">Workspace</p>
						<h2 class="mt-1 text-2xl font-bold tracking-[-0.03em] text-slate-950">
							{activeSection.label}
						</h2>
						<p class="mt-1 text-sm text-slate-600">{activeSection.description}</p>
					</div>
					<div class="grid gap-2 sm:grid-cols-3 xl:min-w-[420px]">
						<AdminMiniMetric label="Backlog" value={model.queueBacklog} caption="queued + leased" />
						<AdminMiniMetric
							label="Active load"
							value={model.activeJobs}
							caption="processing + leased"
						/>
						<AdminMiniMetric
							label="Events / 24h"
							value={data.overview.eventsLast24h}
							caption="job + proxy signals"
						/>
					</div>
				</div>
			</header>

			<div class="space-y-5 px-4 py-5 md:px-6 md:py-6">
				{@render children()}
			</div>
		</div>
	</div>
</section>

<style>
	:global(.admin-panel),
	:global(.admin-kpi-box),
	:global(.admin-stat-card) {
		border-radius: 0.75rem;
		box-shadow: none;
	}

	:global(.no-scrollbar) {
		-ms-overflow-style: none;
		scrollbar-width: none;
	}

	:global(.no-scrollbar::-webkit-scrollbar) {
		display: none;
	}
</style>
