<script lang="ts">
	import { page } from '$app/stores';
	import type { LayoutData } from './$types';
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
	const notificationCount = $derived(Math.min(data.overview.eventsLast24h, 9));
	const artifactUsage = $derived(
		model.totalArtifacts > 0 ? Math.round((data.overview.readyArtifacts / model.totalArtifacts) * 100) : 0
	);
	const profileInitials = $derived.by(() => {
		const [first, second] = data.user.email.split(/[@._-]/).filter(Boolean);
		return `${first?.[0] ?? 'A'}${second?.[0] ?? ''}`.toUpperCase();
	});

	function searchPlaceholder(): string {
		switch (activeSection.id) {
			case 'jobs':
				return 'Search queue, artifacts or job ids...';
			case 'proxies':
				return 'Search proxies, quarantine reasons...';
			case 'activity':
				return 'Search events, incidents or entities...';
			case 'capacity':
				return 'Search throughput and storage signals...';
			default:
				return 'Search analytics or system signals...';
		}
	}

	function quickAction(): { label: string; href: string; icon: string } {
		switch (activeSection.id) {
			case 'jobs':
				return { label: 'Open Activity', href: '/admin/activity', icon: 'monitoring' };
			case 'proxies':
				return { label: 'Open Jobs', href: '/admin/jobs', icon: 'work_history' };
			case 'activity':
				return { label: 'Open Capacity', href: '/admin/capacity', icon: 'speed' };
			case 'capacity':
				return { label: 'Open Jobs', href: '/admin/jobs', icon: 'work_history' };
			default:
				return { label: 'Open Jobs', href: '/admin/jobs', icon: 'add' };
		}
	}

	function isActive(href: string): boolean {
		return $page.url.pathname === href;
	}
</script>

<section class="admin-page min-h-screen bg-[#f6f7f8] text-slate-900">
	<div class="flex min-h-screen overflow-hidden">
		<aside class="hidden w-64 shrink-0 flex-col border-r border-slate-200 bg-white md:flex">
			<div class="flex items-center gap-3 px-6 py-6">
				<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-[#137fec] text-white">
					<span class="material-symbols-outlined">dashboard_customize</span>
				</div>
				<div>
					<p class="text-sm font-bold uppercase tracking-[0.22em] text-slate-500">Downloadtool</p>
					<p class="text-xs font-semibold text-[#137fec]">Admin Systems</p>
				</div>
			</div>

			<nav class="flex-1 space-y-1 px-4 py-4">
				{#each adminSectionItems as section}
					<a
						href={section.href}
						class={`flex items-center gap-3 rounded-lg px-3 py-2.5 transition ${
							isActive(section.href)
								? 'bg-[#137fec] text-white'
								: 'text-slate-600 hover:bg-slate-100'
						}`}
					>
						<span class="material-symbols-outlined text-[21px]">{section.icon}</span>
						<span class="text-sm font-medium">{section.label}</span>
						{#if getAdminSectionBadge(section.id, data.overview)}
							<span
								class={`ml-auto rounded-full px-2 py-0.5 text-[10px] font-bold ${
									isActive(section.href) ? 'bg-white/20 text-white' : 'bg-slate-200 text-slate-600'
								}`}
							>
								{getAdminSectionBadge(section.id, data.overview)}
							</span>
						{/if}
					</a>
				{/each}
			</nav>

			<div class="mt-auto p-4">
				<div class="rounded-xl border border-slate-200 bg-slate-100 p-4">
					<p class="text-[11px] font-semibold text-slate-500">ARTIFACT READY CACHE</p>
					<div class="mt-3 h-1.5 overflow-hidden rounded-full bg-slate-200">
						<div class="h-full rounded-full bg-[#137fec]" style={`width:${artifactUsage}%`}></div>
					</div>
					<p class="mt-2 text-[11px] text-slate-500">
						{data.overview.readyArtifacts} ready / {model.totalArtifacts} total artifacts
					</p>
				</div>
				<a
					href="/account"
					class="mt-3 inline-flex w-full items-center justify-center rounded-lg border border-slate-200 bg-white px-3 py-2 text-sm font-semibold text-slate-700 transition hover:bg-slate-50"
				>
					Back to account
				</a>
			</div>
		</aside>

		<div class="flex min-w-0 flex-1 flex-col overflow-hidden">
			<header class="flex h-16 items-center justify-between border-b border-slate-200 bg-white px-4 md:px-8">
				<div class="flex max-w-md flex-1 items-center">
					<div class="relative w-full">
						<span class="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-slate-400 text-xl">
							search
						</span>
						<input
							type="text"
							placeholder={searchPlaceholder()}
							class="w-full rounded-lg border-none bg-slate-100 py-2 pl-10 pr-4 text-sm text-slate-700 placeholder:text-slate-400 focus:ring-2 focus:ring-[#137fec]/25"
						/>
					</div>
				</div>

				<div class="ml-4 flex items-center gap-3 md:gap-4">
					<button
						type="button"
						class="hidden items-center gap-2 rounded-lg border border-slate-200 bg-white px-4 py-2 text-sm font-semibold text-slate-700 transition hover:bg-slate-50 sm:flex"
					>
						<span class="material-symbols-outlined text-lg">calendar_today</span>
						Last 24h
					</button>
					<a
						href={quickAction().href}
						class="hidden items-center gap-2 rounded-lg bg-[#137fec] px-4 py-2 text-sm font-semibold text-white shadow-[0_10px_28px_-18px_rgba(19,127,236,0.8)] transition hover:opacity-95 sm:flex"
					>
						<span class="material-symbols-outlined text-lg">{quickAction().icon}</span>
						{quickAction().label}
					</a>
					<button
						type="button"
						class="relative rounded-lg p-2 text-slate-500 transition hover:bg-slate-100"
					>
						<span class="material-symbols-outlined">notifications</span>
						{#if notificationCount > 0}
							<span class="absolute right-2.5 top-2.5 h-2 w-2 rounded-full border-2 border-white bg-rose-500"></span>
						{/if}
					</button>
					<div class="hidden h-8 w-px bg-slate-200 sm:block"></div>
					<div class="flex items-center gap-3">
						<div class="hidden text-right sm:block">
							<p class="text-sm font-semibold text-slate-900">{data.user.email}</p>
							<p class="text-xs text-slate-500">System Operator</p>
						</div>
						<div class="flex h-10 w-10 items-center justify-center rounded-full border-2 border-[#137fec]/15 bg-[#137fec]/10 text-sm font-bold text-[#137fec]">
							{profileInitials}
						</div>
					</div>
				</div>
			</header>

			<main class="flex-1 overflow-y-auto px-4 py-6 md:px-8 md:py-8">
				<div class="space-y-8">
					<div class="md:hidden">
						<div class="no-scrollbar flex gap-2 overflow-x-auto pb-1">
							{#each adminSectionItems as section}
								<a
									href={section.href}
									class={`inline-flex shrink-0 items-center gap-2 rounded-lg border px-3 py-2 text-sm font-semibold transition ${
										isActive(section.href)
											? 'border-[#137fec] bg-[#137fec] text-white'
											: 'border-slate-200 bg-white text-slate-700'
									}`}
								>
									<span class="material-symbols-outlined text-base">{section.icon}</span>
									{section.label}
								</a>
							{/each}
						</div>
					</div>

					{@render children()}
				</div>
			</main>
		</div>
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
