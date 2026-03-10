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

	function formatGateMode(mode: string): string {
		return mode.replace(/[_-]/g, ' ');
	}

	function isActive(href: string): boolean {
		return $page.url.pathname === href;
	}
</script>

<section class="admin-page min-h-screen bg-[linear-gradient(180deg,#f8fafc_0%,#eef2ff_100%)] px-4 py-6 md:px-6 md:py-8">
	<div class="mx-auto grid max-w-[1700px] gap-6 xl:grid-cols-[280px_minmax(0,1fr)]">
		<aside class="admin-sidebar rounded-[2rem] border border-slate-200/80 bg-white/95 p-5 shadow-[0_24px_80px_-44px_rgba(15,23,42,0.45)] xl:sticky xl:top-6 xl:h-[calc(100vh-3rem)] xl:overflow-hidden">
			<div class="flex h-full flex-col">
				<div>
					<div class="inline-flex rounded-full border border-slate-300 bg-slate-50 px-3 py-1 text-[11px] font-bold uppercase tracking-[0.24em] text-slate-600">
						Admin shell
					</div>
					<h1 class="mt-4 text-3xl font-black tracking-[-0.04em] text-slate-950">Control plane</h1>
					<p class="mt-3 text-sm leading-6 text-slate-600">
						Khu quản trị riêng cho mux jobs, proxy fleet và artifact pipeline.
					</p>
				</div>

				<div class="mt-6 rounded-[1.5rem] border border-slate-200 bg-slate-50 p-4">
					<p class="text-[11px] font-bold uppercase tracking-[0.18em] text-slate-500">Signed in</p>
					<p class="mt-2 text-sm font-semibold text-slate-900">{data.user.email}</p>
					<div class="mt-3 flex flex-wrap gap-2">
						<span class="rounded-full bg-white px-3 py-1 text-xs font-medium text-slate-700">
							{formatGateMode(data.gateMode)}
						</span>
						<span class="rounded-full bg-white px-3 py-1 text-xs font-medium text-slate-700">
							{data.overview.activeProxies} proxies active
						</span>
					</div>
				</div>

				<nav class="mt-6 hidden xl:block">
					<div class="space-y-5">
						{#each ['Monitor', 'Operations'] as group}
							<div>
								<p class="mb-2 px-1 text-[11px] font-bold uppercase tracking-[0.2em] text-slate-500">
									{group}
								</p>
								<div class="space-y-2">
									{#each adminSectionItems.filter((section) => section.group === group) as section}
										<a
											href={section.href}
											class={`flex items-start gap-3 rounded-[1.35rem] border px-4 py-3 text-left transition ${
												isActive(section.href)
													? 'border-slate-900 bg-slate-900 text-white shadow-[0_16px_36px_-24px_rgba(15,23,42,0.8)]'
													: 'border-slate-200 bg-white text-slate-700 hover:border-slate-300 hover:bg-slate-50'
											}`}
										>
											<span class="material-symbols-outlined mt-0.5 text-xl">{section.icon}</span>
											<span class="min-w-0 flex-1">
												<span class="flex items-center justify-between gap-2">
													<span class="block text-sm font-bold">{section.label}</span>
													{#if getAdminSectionBadge(section.id, data.overview)}
														<span
															class={`rounded-full px-2 py-0.5 text-[11px] font-bold ${
																isActive(section.href)
																	? 'bg-white/15 text-white'
																	: 'bg-slate-100 text-slate-700'
															}`}
														>
															{getAdminSectionBadge(section.id, data.overview)}
														</span>
													{/if}
												</span>
												<span
													class={`mt-1 block text-xs leading-5 ${
														isActive(section.href) ? 'text-slate-300' : 'text-slate-500'
													}`}
												>
													{section.description}
												</span>
											</span>
										</a>
									{/each}
								</div>
							</div>
						{/each}
					</div>
				</nav>

				<div class="mt-6 xl:hidden">
					<div class="no-scrollbar flex gap-2 overflow-x-auto pb-1">
						{#each adminSectionItems as section}
							<a
								href={section.href}
								class={`inline-flex shrink-0 items-center gap-2 rounded-full border px-4 py-2 text-sm font-bold transition ${
									isActive(section.href)
										? 'border-slate-900 bg-slate-900 text-white'
										: 'border-slate-200 bg-white text-slate-700'
								}`}
							>
								<span class="material-symbols-outlined text-base">{section.icon}</span>
								{section.label}
							</a>
						{/each}
					</div>
				</div>

				<div class="mt-6 grid gap-3 sm:grid-cols-3 xl:mt-auto xl:grid-cols-1">
					{#each model.headerStats as stat}
						<AdminMiniMetric label={stat.label} value={stat.value} caption={stat.caption} />
					{/each}
				</div>

				<div class="mt-6 flex flex-wrap gap-3">
					<a
						href="/account"
						class="rounded-full border border-slate-300 bg-white px-4 py-2.5 text-sm font-bold text-slate-700 transition hover:border-slate-400 hover:text-slate-950"
					>
						Back to account
					</a>
				</div>
			</div>
		</aside>

		<div class="space-y-6">
			{@render children()}
		</div>
	</div>
</section>

<style>
	:global(.app.theme-dark) .admin-page {
		background: linear-gradient(180deg, rgba(2, 6, 23, 0.96) 0%, rgba(15, 23, 42, 0.98) 100%);
	}

	:global(.app.theme-dark) .admin-sidebar,
	:global(.app.theme-dark) .admin-panel,
	:global(.app.theme-dark) .admin-kpi-box,
	:global(.app.theme-dark) .admin-stat-card {
		border-color: rgba(148, 163, 184, 0.14) !important;
		background: rgba(15, 23, 42, 0.74) !important;
	}

	:global(.app.theme-dark) .admin-page .text-slate-950,
	:global(.app.theme-dark) .admin-page .text-slate-900,
	:global(.app.theme-dark) .admin-page .text-slate-800 {
		color: rgba(248, 250, 252, 0.98) !important;
	}

	:global(.app.theme-dark) .admin-page .text-slate-700,
	:global(.app.theme-dark) .admin-page .text-slate-600,
	:global(.app.theme-dark) .admin-page .text-slate-500 {
		color: rgba(203, 213, 225, 0.78) !important;
	}

	:global(.app.theme-dark .admin-page .bg-slate-50),
	:global(.app.theme-dark .admin-page .bg-white) {
		background: rgba(15, 23, 42, 0.74) !important;
	}

	:global(.app.theme-dark .admin-page .admin-field),
	:global(.app.theme-dark .admin-page .admin-data-table thead) {
		background: rgba(15, 23, 42, 0.82) !important;
		color: rgba(248, 250, 252, 0.98) !important;
	}

	:global(.app.theme-dark .admin-page .admin-data-table tbody tr:hover) {
		background: rgba(51, 65, 85, 0.42) !important;
	}

	:global(.no-scrollbar) {
		-ms-overflow-style: none;
		scrollbar-width: none;
	}

	:global(.no-scrollbar::-webkit-scrollbar) {
		display: none;
	}
</style>
