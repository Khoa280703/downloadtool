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

<svelte:head>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet" />
</svelte:head>

<section class="admin-page min-h-screen bg-gray-50 text-gray-900">
	<div class="flex min-h-screen">
		<aside class="hidden w-56 shrink-0 flex-col justify-between border-r border-gray-200 bg-white md:flex">
			<div class="flex flex-col">
				<div class="flex items-center gap-2.5 border-b border-gray-200 px-4 py-4">
					<div class="flex h-7 w-7 items-center justify-center rounded bg-gray-900 text-white">
						<AppIcon name="terminal" class="text-sm" />
					</div>
					<div class="flex flex-col">
						<span class="text-sm font-semibold leading-tight text-gray-900">DownloadTool</span>
						<span class="text-[10px] font-medium text-gray-400">Admin</span>
					</div>
				</div>

				<nav class="flex flex-col gap-0.5 p-2">
					{#each adminSectionItems as section}
						<a
							href={section.href}
							class={`flex items-center gap-2.5 rounded-md px-3 py-2 text-[13px] transition-colors ${
								isActive(section.href)
									? 'bg-gray-100 font-semibold text-gray-900'
									: 'text-gray-600 hover:bg-gray-50 hover:text-gray-900'
							}`}
						>
							<AppIcon name={navIcon(section.id)} class="text-base text-gray-400" />
							<span>{section.label}</span>
							{#if getAdminSectionBadge(section.id, data.overview)}
								<span class="ml-auto rounded bg-gray-100 px-1.5 py-0.5 text-[10px] font-semibold tabular-nums text-gray-500">
									{getAdminSectionBadge(section.id, data.overview)}
								</span>
							{/if}
						</a>
					{/each}
				</nav>
			</div>

			<div class="space-y-2 border-t border-gray-200 p-3">
				<div class="rounded-md border border-gray-200 bg-gray-50 p-3">
					<p class="text-[10px] font-semibold uppercase tracking-wider text-gray-400">Snapshot</p>
					<div class="mt-2 space-y-1.5 text-[12px] text-gray-600">
						<div class="flex items-center justify-between">
							<span>Backlog</span>
							<span class="font-semibold tabular-nums text-gray-900">{model.queueBacklog}</span>
						</div>
						<div class="flex items-center justify-between">
							<span>Artifacts</span>
							<span class="font-semibold tabular-nums text-gray-900">{data.overview.readyArtifacts}</span>
						</div>
					</div>
				</div>

				<a
					href="/account"
					class="flex w-full items-center justify-center gap-1.5 rounded-md border border-gray-200 py-2 text-[12px] font-medium text-gray-600 transition hover:bg-gray-50"
				>
					<AppIcon name="arrow_back" class="text-sm" />
					<span>Back to account</span>
				</a>
			</div>
		</aside>

		<main class="min-w-0 flex-1 overflow-y-auto">
			<div class="mx-auto max-w-6xl p-4 md:p-6">
				<div class="mb-4 md:hidden">
					<div class="no-scrollbar flex gap-1.5 overflow-x-auto pb-1">
						{#each adminSectionItems as section}
							<a
								href={section.href}
								class={`inline-flex shrink-0 items-center gap-1.5 rounded-md border px-3 py-1.5 text-[12px] font-medium transition ${
									isActive(section.href)
										? 'border-gray-900 bg-gray-900 text-white'
										: 'border-gray-200 bg-white text-gray-600'
								}`}
							>
								<AppIcon name={navIcon(section.id)} class="text-sm" />
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
		font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
	}

	:global(.admin-panel) {
		border-radius: 0.5rem;
	}

	:global(.no-scrollbar) {
		-ms-overflow-style: none;
		scrollbar-width: none;
	}

	:global(.no-scrollbar::-webkit-scrollbar) {
		display: none;
	}
</style>
