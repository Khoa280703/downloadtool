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

<section class="admin-page min-h-screen bg-slate-100 text-slate-900">
	<div class="xl:grid xl:min-h-screen xl:grid-cols-[248px_minmax(0,1fr)]">
		<aside class="hidden border-r border-slate-800 bg-slate-950 text-slate-100 xl:flex xl:flex-col">
			<div class="border-b border-slate-800 px-5 py-5">
				<p class="text-[10px] font-bold uppercase tracking-[0.24em] text-slate-400">Downloadtool</p>
				<h1 class="mt-2 text-xl font-black tracking-[-0.03em] text-white">Admin Console</h1>
				<p class="mt-2 text-[13px] leading-5 text-slate-400">
					Điều phối jobs, proxy fleet và artifact storage.
				</p>
			</div>

			<nav class="flex-1 px-3 py-4">
				{#each ['Monitor', 'Operations'] as group}
					<div class="mb-6">
						<p class="mb-2 px-2 text-[10px] font-bold uppercase tracking-[0.2em] text-slate-500">
							{group}
						</p>
						<div class="space-y-1">
							{#each adminSectionItems.filter((section) => section.group === group) as section}
								<a
									href={section.href}
									class={`flex items-start gap-3 rounded-lg border px-3 py-3 transition ${
										isActive(section.href)
											? 'border-slate-700 bg-slate-900 text-white'
											: 'border-transparent text-slate-300 hover:border-slate-800 hover:bg-slate-900/70 hover:text-white'
									}`}
								>
									<span class="material-symbols-outlined mt-0.5 text-[18px]">{section.icon}</span>
									<span class="min-w-0 flex-1">
										<span class="flex items-center justify-between gap-2">
											<span class="text-sm font-semibold">{section.label}</span>
											{#if getAdminSectionBadge(section.id, data.overview)}
												<span
													class={`rounded-md px-2 py-0.5 text-[10px] font-bold ${
														isActive(section.href)
															? 'bg-white/10 text-white'
															: 'bg-slate-800 text-slate-200'
													}`}
												>
													{getAdminSectionBadge(section.id, data.overview)}
												</span>
											{/if}
										</span>
										<span class={`mt-1 block text-xs leading-5 ${isActive(section.href) ? 'text-slate-400' : 'text-slate-500'}`}>
											{section.description}
										</span>
									</span>
								</a>
							{/each}
						</div>
					</div>
				{/each}
			</nav>

			<div class="border-t border-slate-800 px-4 py-4">
				<div class="grid gap-2">
					{#each model.headerStats as stat}
						<AdminMiniMetric
							label={stat.label}
							value={stat.value}
							caption={stat.caption}
							inverted={true}
						/>
					{/each}
				</div>
				<div class="mt-4 rounded-lg border border-slate-800 bg-slate-900 px-3 py-3">
					<p class="text-[10px] font-bold uppercase tracking-[0.18em] text-slate-500">Operator</p>
					<p class="mt-2 text-sm font-semibold text-white">{data.user.email}</p>
					<p class="mt-1 text-xs uppercase tracking-[0.18em] text-slate-400">
						Mode: {formatGateMode(data.gateMode)}
					</p>
				</div>
				<a
					href="/account"
					class="mt-3 inline-flex w-full items-center justify-center rounded-md border border-slate-700 px-3 py-2 text-sm font-semibold text-slate-200 transition hover:border-slate-600 hover:bg-slate-900"
				>
					Back to account
				</a>
			</div>
		</aside>

		<div class="min-w-0">
			<header class="sticky top-0 z-20 border-b border-slate-200 bg-white/95 backdrop-blur">
				<div class="px-4 py-4 md:px-6">
					<div class="flex flex-col gap-4 xl:flex-row xl:items-center xl:justify-between">
						<div>
							<p class="text-[10px] font-bold uppercase tracking-[0.24em] text-slate-500">Control plane</p>
							<h2 class="mt-1 text-2xl font-black tracking-[-0.03em] text-slate-950">
								{activeSection.label}
							</h2>
							<p class="mt-1 text-[13px] leading-5 text-slate-600">{activeSection.description}</p>
						</div>

						<div class="grid gap-2 sm:grid-cols-3 xl:min-w-[420px]">
							{#each model.headerStats as stat}
								<AdminMiniMetric label={stat.label} value={stat.value} caption={stat.caption} />
							{/each}
						</div>
					</div>

					<div class="no-scrollbar mt-4 flex gap-2 overflow-x-auto pb-1 xl:hidden">
						{#each adminSectionItems as section}
							<a
								href={section.href}
								class={`inline-flex shrink-0 items-center gap-2 rounded-md border px-3 py-2 text-sm font-semibold transition ${
									isActive(section.href)
										? 'border-slate-900 bg-slate-900 text-white'
										: 'border-slate-300 bg-white text-slate-700'
								}`}
							>
								<span class="material-symbols-outlined text-base">{section.icon}</span>
								{section.label}
							</a>
						{/each}
					</div>
				</div>
			</header>

			<div class="px-4 py-4 md:px-6 md:py-6">
				{@render children()}
			</div>
		</div>
	</div>
</section>

<style>
	:global(.admin-page) {
		font-family:
			ui-sans-serif,
			system-ui,
			-apple-system,
			BlinkMacSystemFont,
			'Segoe UI',
			sans-serif;
	}

	:global(.no-scrollbar) {
		-ms-overflow-style: none;
		scrollbar-width: none;
	}

	:global(.no-scrollbar::-webkit-scrollbar) {
		display: none;
	}
</style>
