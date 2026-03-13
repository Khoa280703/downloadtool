<script lang="ts">
	import { page } from '$app/stores';
	import AppIcon from '$components/AppIcon.svelte';
	import type { LayoutData } from './$types';
	import {
		adminSectionItems,
		buildAdminDashboardViewModel,
		getAdminSectionBadge
	} from '$lib/admin/dashboard-view-model';

	let { data, children }: { data: LayoutData; children: import('svelte').Snippet } = $props();

	const model = $derived(buildAdminDashboardViewModel(data.overview));

	function isActive(href: string): boolean {
		return $page.url.pathname === href;
	}

	function navIcon(sectionId: string): string {
		switch (sectionId) {
			case 'overview':
				return 'dashboard';
			case 'jobs':
				return 'work_history';
			case 'proxies':
				return 'hub';
			case 'activity':
				return 'description';
			case 'capacity':
				return 'database';
			default:
				return 'dashboard';
		}
	}
</script>

<section class="admin-page min-h-screen bg-[#f6f7f8] text-slate-900">
	<div class="flex min-h-screen">
		<aside class="hidden w-64 shrink-0 flex-col justify-between border-r border-slate-200 bg-white p-4 md:flex">
			<div class="flex flex-col gap-8">
				<div class="flex items-center gap-3 px-2">
					<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-[#137fec] text-white">
						<AppIcon name="grid_view" class="text-2xl" />
					</div>
					<div class="flex flex-col">
						<h1 class="text-lg font-bold leading-tight text-slate-900">DOWNLOADTOOL</h1>
						<p class="text-xs font-medium text-slate-500">Admin System</p>
					</div>
				</div>

				<nav class="flex flex-col gap-1">
					{#each adminSectionItems as section}
						<a
							href={section.href}
							class={`flex items-center gap-3 rounded-lg px-3 py-2.5 transition-colors ${
								isActive(section.href)
									? 'bg-[#137fec]/10 font-semibold text-[#137fec]'
									: 'text-slate-600 hover:bg-slate-100'
							}`}
						>
							<AppIcon name={navIcon(section.id)} />
							<span class="text-sm">{section.label}</span>
							{#if getAdminSectionBadge(section.id, data.overview)}
								<span class="ml-auto rounded-full bg-slate-100 px-2 py-0.5 text-[10px] font-bold text-slate-600">
									{getAdminSectionBadge(section.id, data.overview)}
								</span>
							{/if}
						</a>
					{/each}
				</nav>
			</div>

			<div class="space-y-3">
				<div class="rounded-xl border border-slate-200 bg-slate-50 p-4">
					<p class="text-[11px] font-semibold text-slate-500">SYSTEM SNAPSHOT</p>
					<div class="mt-3 space-y-2 text-sm text-slate-600">
						<div class="flex items-center justify-between gap-3">
							<span>Backlog</span>
							<span class="font-semibold text-slate-900">{model.queueBacklog}</span>
						</div>
						<div class="flex items-center justify-between gap-3">
							<span>Ready artifacts</span>
							<span class="font-semibold text-slate-900">{data.overview.readyArtifacts}</span>
						</div>
						<div class="flex items-center justify-between gap-3">
							<span>Events / 24h</span>
							<span class="font-semibold text-slate-900">{data.overview.eventsLast24h}</span>
						</div>
					</div>
				</div>

				<a
					href="/account"
					class="flex w-full items-center justify-center gap-2 rounded-lg bg-slate-100 py-2.5 text-sm font-bold text-slate-700 transition-all hover:bg-slate-200"
				>
					<AppIcon name="logout" class="text-lg" />
					<span>Back to account</span>
				</a>
			</div>
		</aside>

		<main class="min-w-0 flex-1 overflow-y-auto">
			<div class="mx-auto max-w-6xl p-4 md:p-8">
				<div class="mb-6 md:hidden">
					<div class="no-scrollbar flex gap-2 overflow-x-auto pb-1">
						{#each adminSectionItems as section}
							<a
								href={section.href}
								class={`inline-flex shrink-0 items-center gap-2 rounded-lg border px-3 py-2 text-sm font-semibold transition ${
									isActive(section.href)
										? 'border-[#137fec] bg-[#137fec]/10 text-[#137fec]'
										: 'border-slate-200 bg-white text-slate-700'
								}`}
							>
								<AppIcon name={navIcon(section.id)} class="text-base" />
								{section.label}
							</a>
						{/each}
					</div>
				</div>

				{@render children()}
			</div>
		</main>
	</div>
</section>

<style>
	:global(.admin-page) {
		font-family: 'Nunito', sans-serif;
	}

	:global(.admin-panel),
	:global(.admin-kpi-box),
	:global(.admin-stat-card) {
		border-radius: 0.75rem;
		box-shadow: 0 1px 2px rgba(15, 23, 42, 0.04);
	}

	:global(.no-scrollbar) {
		-ms-overflow-style: none;
		scrollbar-width: none;
	}

	:global(.no-scrollbar::-webkit-scrollbar) {
		display: none;
	}
</style>
