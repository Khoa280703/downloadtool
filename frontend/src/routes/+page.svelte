<script lang="ts">
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';
	import AuthModal from '$components/AuthModal.svelte';
	import SiteHeader from '$components/SiteHeader.svelte';
	import SiteFooter from '$components/SiteFooter.svelte';
	import DownloadBtn from '$components/DownloadBtn.svelte';
	import FormatPicker from '$components/FormatPicker.svelte';
	import BatchInput from '$components/BatchInput.svelte';
	import BatchProgress from '$components/BatchProgress.svelte';
	import { extract, extractYouTubeVideoId, isValidVideoUrl } from '$lib/api';
	import * as m from '$lib/paraglide/messages';
	import type { ExtractResult, Stream } from '$lib/types';
	import { currentDownload } from '$stores/download';

	type AuthUser = { name?: string | null; email: string; image?: string | null };

	let inputUrl = $state('');
	let extractResult = $state<ExtractResult | null>(null);
	let selectedAudioStream = $state<Stream | null>(null);
	let isExtracting = $state(false);
	let extractError = $state('');
	let isDarkMode = $state(false);
	let previewThumbnailLoadFailed = $state(false);
	let previewThumbnailId = $derived(isExtracting ? extractYouTubeVideoId(inputUrl) : null);
	let previewThumbnailUrl = $derived(
		previewThumbnailId ? `https://i.ytimg.com/vi/${previewThumbnailId}/hqdefault.jpg` : null
	);
	let authModalOpen = $state(false);
	const hasInitialBetterAuthCookie = browser && document.cookie.includes('better-auth.');
	const initialSessionRequest = hasInitialBetterAuthCookie
		? fetch('/api/auth/get-session', { credentials: 'include' })
				.then(async (resp) => (resp.ok ? ((await resp.json())?.user ?? null) : null))
				.catch(() => null)
		: null;
	/** undefined = loading skeleton, null = unauthenticated, object = authenticated */
	let authUser = $state<AuthUser | null | undefined>(hasInitialBetterAuthCookie ? undefined : null);
	let redirectTo = $state('/');
	const SEO_ORIGIN = 'https://download.khoadangbui.online';
	const SEO_LOCALES = [
		'ar',
		'bg',
		'cs',
		'da',
		'de',
		'el',
		'en',
		'es',
		'et',
		'fi',
		'fr',
		'hu',
		'id',
		'it',
		'ja',
		'ko',
		'lt',
		'lv',
		'nb',
		'nl',
		'pl',
		'pt',
		'pt-BR',
		'ro',
		'ru',
		'sk',
		'sl',
		'sv',
		'tr',
		'uk',
		'vi',
		'zh',
		'zh-TW'
	] as const;
	const HOMEPAGE_HREFLANG_LINKS = [
		{
			hreflang: 'x-default',
			href: `${SEO_ORIGIN}/`
		},
		...SEO_LOCALES.map((locale) => ({
			hreflang: locale === 'nb' ? 'no' : locale,
			href: locale === 'en' ? `${SEO_ORIGIN}/` : `${SEO_ORIGIN}/${locale}/`
		}))
	];

	function normalizeRedirectTo(value: string | null): string {
		if (!value || !value.startsWith('/') || value.startsWith('//')) return '/';
		return value;
	}

	onMount(async () => {
		const saved = localStorage.getItem('fetchtube-theme');
		if (saved === 'dark') isDarkMode = true;
		if (saved === 'light') isDarkMode = false;

		authUser = initialSessionRequest ? await initialSessionRequest : null;

		// Handle ?auth=required&redirectTo= after auth state resolves
		const params = new URLSearchParams(window.location.search);
		if (params.get('auth') === 'required' && !authUser) {
			redirectTo = normalizeRedirectTo(params.get('redirectTo'));
			authModalOpen = true;
		}
	});

	$effect(() => {
		localStorage.setItem('fetchtube-theme', isDarkMode ? 'dark' : 'light');
	});

	function handleFormatSelect(videoStream: Stream, audioStream: Stream | null): void {
		currentDownload.update((state) => ({ ...state, selectedStream: videoStream }));
		selectedAudioStream = audioStream;
	}

	async function handleFetch(event?: SubmitEvent): Promise<void> {
		event?.preventDefault();
		const url = inputUrl.trim();

		if (!url) {
			extractError = m.home_error_paste_url();
			return;
		}

		if (!isValidVideoUrl(url)) {
			extractError = m.home_error_invalid_url();
			return;
		}

		isExtracting = true;
		previewThumbnailLoadFailed = false;
		extractError = '';
		extractResult = null;
		selectedAudioStream = null;
		currentDownload.update((state) => ({ ...state, selectedStream: null, error: null }));
		requestAnimationFrame(() => {
			document
				.getElementById('download-options')
				?.scrollIntoView({ behavior: 'smooth', block: 'start' });
		});

		try {
			const result = await extract(url);
			if (!result.streams.length) {
				extractError = m.home_error_no_streams();
				return;
			}

			extractResult = result;
			queueMicrotask(() => {
				document
					.getElementById('download-options')
					?.scrollIntoView({ behavior: 'smooth', block: 'start' });
			});
		} catch (error) {
			extractError = error instanceof Error ? error.message : m.home_error_fetch_failed();
		} finally {
			isExtracting = false;
		}
	}

	function formatDuration(seconds?: number): string {
		if (!seconds || seconds <= 0) return m.common_not_available();
		const total = Math.floor(seconds);
		const h = Math.floor(total / 3600);
		const minutes = Math.floor((total % 3600) / 60);
		const s = total % 60;
		if (h > 0) return `${h}:${String(minutes).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
		return `${minutes}:${String(s).padStart(2, '0')}`;
	}

	function formatViews(views?: number): string {
		if (!views || views <= 0) return m.common_not_available();
		if (views >= 1_000_000_000) return `${(views / 1_000_000_000).toFixed(1)}B`;
		if (views >= 1_000_000) return `${(views / 1_000_000).toFixed(1)}M`;
		if (views >= 1_000) return `${(views / 1_000).toFixed(1)}K`;
		return `${views}`;
	}

	function shortDescription(text?: string, maxLength = 180): string {
		if (!text) return '';
		const normalized = text.replace(/\s+/g, ' ').trim();
		if (normalized.length <= maxLength) return normalized;
		return `${normalized.slice(0, maxLength).trimEnd()}...`;
	}

	function toggleTheme(): void {
		isDarkMode = !isDarkMode;
	}

	function openAuthModal(): void {
		authModalOpen = true;
	}

	async function closeAuthModal(): Promise<void> {
		authModalOpen = false;
		const params = new URLSearchParams(window.location.search);
		if (params.get('auth') === 'required' && !authUser) {
			await goto('/', { replaceState: true, noScroll: true, invalidateAll: false });
		}
	}

	async function handleAuthSuccess(target: string): Promise<void> {
		authModalOpen = false;
		await goto(target, { invalidateAll: true, replaceState: true });
	}
</script>

<svelte:head>
	<title>{m.home_meta_title()}</title>
	<meta name="description" content={m.home_meta_description()} />

	{#each HOMEPAGE_HREFLANG_LINKS as link}
		<link rel="alternate" hreflang={link.hreflang} href={link.href} />
	{/each}

	<link rel="preload" href="/fonts/fredoka-latin.woff2" as="font" type="font/woff2" crossorigin="anonymous"/>
	<link rel="preload" href="/fonts/nunito-normal-latin.woff2" as="font" type="font/woff2" crossorigin="anonymous"/>
	<link rel="preload" href="/fonts/material-symbols-outlined-subset.woff2" as="font" type="font/woff2" crossorigin="anonymous"/>
	<style>
		body {
			font-family: 'Nunito', sans-serif;
			background-color: #fff5f9;
			color: #2d1b36;
		}

		h1,
		h2,
		h3,
		h4,
		h5,
		h6,
		button {
			font-family: 'Fredoka', sans-serif;
		}

		.glass-header {
			background: rgba(255, 245, 249, 0.8);
			backdrop-filter: blur(12px);
			-webkit-backdrop-filter: blur(12px);
		}

		.text-gradient {
			background: linear-gradient(135deg, #ff4d8c 0%, #ffb938 100%);
			-webkit-background-clip: text;
			-webkit-text-fill-color: transparent;
		}

		.bg-gradient-primary {
			background: linear-gradient(135deg, #ff4d8c 0%, #ffb938 100%);
		}

		.dashed-border-anim {
			background-image: url("data:image/svg+xml,%3csvg width='100%25' height='100%25' xmlns='http://www.w3.org/2000/svg'%3e%3crect width='100%25' height='100%25' fill='none' rx='32' ry='32' stroke='%23FF4D8CFF' stroke-width='3' stroke-dasharray='12%2c 12' stroke-dashoffset='0' stroke-linecap='round'/%3e%3c/svg%3e");
		}

		html {
			scroll-behavior: smooth;
		}

		.bento-card {
			transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1), box-shadow 0.3s ease;
		}

		.bento-card:hover {
			transform: translateY(-8px);
			box-shadow: 0 25px 50px -12px rgba(255, 77, 140, 0.25);
		}

		.hide-scrollbar::-webkit-scrollbar {
			display: none;
		}

		.hide-scrollbar {
			-ms-overflow-style: none;
			scrollbar-width: none;
		}

		.page-root {
			background-color: #fff5f9;
			color: #2d1b36;
			transition: background-color 220ms ease, color 220ms ease;
		}

		.theme-toggle {
			background: rgba(45, 27, 54, 0.08);
			border: 1px solid rgba(45, 27, 54, 0.14);
			color: #2d1b36;
		}

		.page-root.theme-dark {
			background-color: #12121a;
			color: #e0d0f5;
		}

		.page-root.theme-dark .glass-header {
			background: rgba(18, 18, 26, 0.7);
			border-bottom: 1px solid rgba(255, 255, 255, 0.05);
		}

		.page-root.theme-dark .theme-toggle {
			background: rgba(255, 255, 255, 0.1);
			border-color: rgba(255, 255, 255, 0.12);
			color: #ffffff;
		}

		.page-root.theme-dark .text-plum,
		.page-root.theme-dark .text-text-main {
			color: #ffffff !important;
		}

		.page-root.theme-dark [class*='text-plum/70'],
		.page-root.theme-dark [class*='text-plum/80'],
		.page-root.theme-dark [class*='text-plum/60'],
		.page-root.theme-dark [class*='text-muted'] {
			color: rgba(224, 208, 245, 0.7) !important;
		}

		.page-root.theme-dark .bg-bg-page,
		.page-root.theme-dark [class*='bg-white/30'],
		.page-root.theme-dark [class*='bg-white/40'] {
			background-color: #12121a !important;
		}

		.page-root.theme-dark .bg-white,
		.page-root.theme-dark [class*='bg-white/50'],
		.page-root.theme-dark [class*='bg-white/60'],
		.page-root.theme-dark [class*='bg-white/80'],
		.page-root.theme-dark [class*='bg-pink-50'],
		.page-root.theme-dark [class*='bg-slate-50'],
		.page-root.theme-dark [class*='bg-slate-100'],
		.page-root.theme-dark [class*='bg-indigo-50'] {
			background-color: rgba(30, 30, 42, 0.6) !important;
			backdrop-filter: blur(12px);
			-webkit-backdrop-filter: blur(12px);
		}

		.page-root.theme-dark .border-white,
		.page-root.theme-dark [class*='border-pink'],
		.page-root.theme-dark [class*='border-slate'],
		.page-root.theme-dark [class*='border-indigo'],
		.page-root.theme-dark [class*='border-white/50'] {
			border-color: rgba(255, 77, 140, 0.18) !important;
		}

		.page-root.theme-dark .bg-plum {
			background-color: #1a1a24 !important;
		}

		.page-root.theme-dark .shadow-float,
		.page-root.theme-dark .shadow-card,
		.page-root.theme-dark .shadow-candy {
			box-shadow: 0 0 20px rgba(255, 77, 140, 0.35) !important;
		}

		.page-root.theme-dark .hero-orb {
			opacity: 1;
			filter: blur(30px);
		}

		.page-root.theme-dark .zig-zag-container img,
		.page-root.theme-dark [class*='rounded-full'] img {
			opacity: 0.9;
		}

		.page-root.theme-dark footer {
			background-color: #12121a !important;
			border-color: rgba(255, 255, 255, 0.08) !important;
		}

			.page-root.theme-dark .step-paste-card {
				background: linear-gradient(135deg, rgba(38, 34, 56, 0.95), rgba(29, 26, 43, 0.95)) !important;
				border-color: rgba(255, 77, 140, 0.22) !important;
				box-shadow: 0 0 20px rgba(255, 77, 140, 0.28) !important;
			}

			.page-root.theme-dark .download-result-card {
				background: rgba(22, 22, 32, 0.92) !important;
				border-color: rgba(255, 77, 140, 0.2) !important;
			}

			.page-root.theme-dark .download-result-meta {
				background: linear-gradient(180deg, rgba(35, 31, 52, 0.88), rgba(26, 23, 39, 0.95)) !important;
				border-right-color: rgba(255, 77, 140, 0.2) !important;
			}

			.page-root.theme-dark .download-result-title {
				color: #ffffff !important;
			}

			.page-root.theme-dark .download-result-chip-duration {
				background: rgba(99, 102, 241, 0.2) !important;
				color: #c7d2fe !important;
			}

			.page-root.theme-dark .download-result-chip-view {
				background: rgba(236, 72, 153, 0.2) !important;
				color: #fbcfe8 !important;
			}

			.page-root.theme-dark .download-result-chip-channel {
				background: rgba(148, 163, 184, 0.2) !important;
				color: #e2e8f0 !important;
			}

			.page-root.theme-dark .download-result-description {
				background: rgba(255, 255, 255, 0.05) !important;
				border-color: rgba(255, 255, 255, 0.1) !important;
				color: rgba(224, 208, 245, 0.88) !important;
			}

			.page-root.theme-dark .download-result-description strong {
				color: #ffffff !important;
			}

			.page-root.theme-dark .download-result-hint {
				color: rgba(224, 208, 245, 0.78) !important;
			}

			.page-root.theme-dark .download-result-actions {
				background: rgba(20, 21, 30, 0.9) !important;
			}

			.page-root.theme-dark .download-result-download {
				border-top-color: rgba(255, 77, 140, 0.2) !important;
				background: rgba(99, 102, 241, 0.12) !important;
			}

		/* Remove bright horizontal separators between major sections in dark mode */
		.page-root.theme-dark #tools {
			border-top-color: transparent !important;
			border-bottom-color: transparent !important;
		}

		.page-root.theme-dark footer {
			border-top-color: transparent !important;
		}

		.page-root.theme-dark footer a {
			color: rgba(224, 208, 245, 0.55) !important;
		}

		.page-root.theme-dark footer a:hover {
			color: #ff4d8c !important;
		}
	</style>
</svelte:head>

<div class="page-root bg-bg-page min-h-screen flex flex-col overflow-x-hidden text-plum selection:bg-primary/20" class:theme-dark={isDarkMode} class:theme-light={!isDarkMode}>
	<SiteHeader
		{authUser}
		onOpenAuthModal={openAuthModal}
		homeHref="#home"
		howItWorksHref="#how-it-works"
		toolsHref="#tools"
	/>

	<main class="flex-1 w-full">
		<section class="relative pt-12 pb-8 px-6 overflow-visible" id="home">
			<div class="hero-orb absolute top-[10%] left-[5%] w-24 h-24 rounded-full bg-accent/20 blur-xl animate-bob"></div>
			<div class="hero-orb absolute bottom-[20%] right-[10%] w-32 h-32 rounded-3xl rotate-12 bg-primary/10 blur-xl animate-bob-delayed"></div>
			<div class="hero-orb absolute top-[20%] right-[15%] w-16 h-16 rounded-full bg-secondary/30 blur-lg animate-bob"></div>
			<div class="relative z-10 w-full max-w-4xl mx-auto text-center">
				<div class="inline-flex items-center gap-2 bg-white px-3 py-1.5 rounded-full shadow-sm mb-4 animate-fade-in-up border border-pink-100">
					<span class="text-lg">✨</span>
					<span class="text-xs font-bold text-plum/80 uppercase tracking-wide">{m.home_hero_badge()}</span>
				</div>
				<h1 class="text-4xl md:text-6xl lg:text-7xl font-bold text-plum mb-4 leading-[0.95] tracking-tight">
					{m.home_hero_title_line1()} <br/>
					<span class="text-gradient inline-block hover:scale-105 transition-transform cursor-default">{m.home_hero_title_highlight()}</span>
				</h1>
				<p class="text-lg md:text-xl text-plum/70 max-w-xl mx-auto font-semibold mb-8">
					{m.home_hero_subtitle()}
				</p>

				<form class="relative w-full max-w-[700px] mx-auto group mb-12" onsubmit={handleFetch}>
					<div class="absolute -inset-1 bg-gradient-to-r from-primary to-secondary rounded-full blur opacity-25 group-hover:opacity-50 transition duration-500"></div>
					<div class="relative flex items-center bg-white rounded-full shadow-float p-2 h-[64px] transition-all duration-300 group-focus-within:ring-4 group-focus-within:ring-primary/20">
						<div class="pl-6 text-plum/30">
							<span class="material-symbols-outlined text-2xl">link</span>
						</div>
						<input id="video-url-input" class="w-full h-full bg-transparent border-none focus:ring-0 text-lg md:text-xl font-semibold placeholder:text-muted/50 text-plum px-4" placeholder={m.home_input_placeholder()} type="text" bind:value={inputUrl} disabled={isExtracting}/>
						<button class="absolute right-1.5 top-1.5 bottom-1.5 bg-gradient-primary hover:brightness-110 text-white font-bold rounded-full px-6 md:px-10 text-base md:text-lg shadow-candy transition-all hover:scale-105 active:scale-95 flex items-center gap-2 disabled:opacity-60 disabled:cursor-not-allowed disabled:hover:scale-100" type="submit" disabled={isExtracting}>
							<span>{isExtracting ? m.home_button_fetching() : m.home_button_fetch()}</span>
							<span class="material-symbols-outlined font-bold text-lg">{isExtracting ? 'progress_activity' : 'bolt'}</span>
						</button>
					</div>

					{#if extractError}
						<p class="mt-3 text-sm font-bold text-red-500">{extractError}</p>
					{/if}
					{#if isExtracting}
						<p class="mt-3 inline-flex items-center gap-2 rounded-full bg-white/90 px-4 py-2 text-sm font-bold text-primary shadow-sm">
							<span class="material-symbols-outlined animate-spin text-base">progress_activity</span>
							{m.home_analyzing_link()}
						</p>
					{/if}

					<div class="mt-6 flex flex-wrap justify-center gap-3 opacity-80 hover:opacity-100 transition-opacity">
						<div class="flex items-center gap-2 bg-white/60 px-3 py-1.5 rounded-xl border border-white/50">
							<span class="material-symbols-outlined text-green-500 text-lg">check_circle</span>
							<span class="font-bold text-xs text-plum/70">{m.home_chip_ad_free()}</span>
						</div>
						<div class="flex items-center gap-2 bg-white/60 px-3 py-1.5 rounded-xl border border-white/50">
							<span class="material-symbols-outlined text-blue-500 text-lg">verified_user</span>
							<span class="font-bold text-xs text-plum/70">{m.home_chip_safe_secure()}</span>
						</div>
						<div class="flex items-center gap-2 bg-white/60 px-3 py-1.5 rounded-xl border border-white/50">
							<span class="material-symbols-outlined text-purple-500 text-lg">rocket_launch</span>
							<span class="font-bold text-xs text-plum/70">{m.home_chip_super_fast()}</span>
						</div>
					</div>
				</form>
			</div>
		</section>

		{#if isExtracting}
			<section class="py-8 px-6 lg:px-20" id="download-options">
				<div class="max-w-7xl mx-auto">
					<div class="bg-white rounded-[2rem] shadow-card border border-indigo-50 overflow-hidden flex flex-col lg:flex-row animate-pulse">
						<div class="w-full lg:w-[42%] p-6 md:p-8 flex flex-col gap-5 bg-gradient-to-b from-indigo-50/50 to-white lg:border-r border-indigo-50">
							<div class="relative w-full aspect-video rounded-3xl overflow-hidden bg-slate-200">
								{#if previewThumbnailUrl && !previewThumbnailLoadFailed}
									<img
										class="absolute inset-0 w-full h-full object-cover"
										src={previewThumbnailUrl}
										alt={m.home_thumbnail_preview_alt()}
										decoding="async"
										onerror={() => (previewThumbnailLoadFailed = true)}
									/>
								{:else}
									<div class="absolute inset-0 grid place-items-center text-slate-400">
										<span class="material-symbols-outlined text-6xl">movie</span>
									</div>
								{/if}
								<div class="absolute inset-0 bg-gradient-to-t from-black/40 to-transparent"></div>
								<div class="absolute bottom-4 right-4 bg-black/60 backdrop-blur-md px-3 py-1.5 rounded-full text-xs font-bold text-white border border-white/20 flex items-center gap-1">
									<span class="material-symbols-outlined animate-spin text-sm">progress_activity</span>
									{m.home_fetching()}
								</div>
							</div>
							<div class="h-7 w-11/12 rounded-xl bg-slate-200"></div>
							<div class="flex gap-2">
								<div class="h-8 w-24 rounded-full bg-slate-200"></div>
								<div class="h-8 w-24 rounded-full bg-slate-200"></div>
								<div class="h-8 w-28 rounded-full bg-slate-200"></div>
							</div>
							<div class="h-20 rounded-2xl bg-slate-100"></div>
						</div>
						<div class="flex-1 flex flex-col bg-white">
							<div class="p-6 md:p-8 flex flex-col gap-3">
								<div class="h-11 rounded-2xl bg-slate-200"></div>
								<div class="h-20 rounded-2xl bg-slate-100"></div>
								<div class="h-20 rounded-2xl bg-slate-100"></div>
								<div class="h-20 rounded-2xl bg-slate-100"></div>
							</div>
							<div class="p-6 md:p-8 border-t border-indigo-50 bg-indigo-50/20">
								<div class="h-14 rounded-full bg-slate-200"></div>
							</div>
						</div>
					</div>
				</div>
			</section>
			{:else if extractResult}
				<section class="py-8 px-6 lg:px-20" id="download-options">
					<div class="max-w-7xl mx-auto">
						<div class="download-result-card bg-white rounded-[2rem] shadow-card border border-indigo-50 overflow-hidden flex flex-col lg:flex-row">
							<div class="download-result-meta w-full lg:w-[42%] p-6 md:p-8 flex flex-col gap-5 bg-gradient-to-b from-indigo-50/50 to-white lg:border-r border-indigo-50">
								<div class="relative w-full aspect-video rounded-3xl overflow-hidden shadow-lg border-4 border-white bg-slate-100">
									{#if extractResult.thumbnail}
										<img class="absolute inset-0 w-full h-full object-cover" src={extractResult.thumbnail} alt={extractResult.title}/>
								{:else}
									<div class="absolute inset-0 grid place-items-center text-slate-400">
										<span class="material-symbols-outlined text-6xl">movie</span>
									</div>
								{/if}
									<div class="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent"></div>
									<div class="absolute bottom-4 right-4 bg-black/60 backdrop-blur-md px-3 py-1.5 rounded-full text-xs font-bold text-white border border-white/20">{m.home_video_available()}</div>
								</div>
								<h3 class="download-result-title text-2xl md:text-3xl font-bold text-slate-900 leading-tight">{extractResult.title}</h3>
								<div class="flex flex-wrap items-center gap-3">
									<div class="download-result-chip-duration flex items-center gap-2 bg-indigo-50 px-3 py-1.5 rounded-full text-indigo-600 text-sm font-bold">
										<span class="material-symbols-outlined text-[18px]">schedule</span>
										{formatDuration(extractResult.duration)}
									</div>
									{#if extractResult.viewCount}
										<div class="download-result-chip-view flex items-center gap-2 bg-pink-50 px-3 py-1.5 rounded-full text-pink-600 text-sm font-bold">
											<span class="material-symbols-outlined text-[18px]">visibility</span>
											{formatViews(extractResult.viewCount)}
										</div>
									{/if}
									{#if extractResult.channel}
										<div class="download-result-chip-channel flex items-center gap-2 bg-slate-100 px-3 py-1.5 rounded-full text-slate-600 text-sm font-bold">
											<span class="material-symbols-outlined text-[18px]">person</span>
											{extractResult.channel}
										</div>
									{/if}
								</div>
								{#if extractResult.description}
									<div class="download-result-description p-4 bg-slate-50 rounded-2xl border border-slate-100 text-sm text-slate-600 leading-relaxed">
										<span class="font-bold text-slate-800">{m.home_description_label()}</span>
										{shortDescription(extractResult.description)}
									</div>
								{/if}
								<p class="download-result-hint text-slate-500 font-semibold">{m.home_choose_format_hint()}</p>
							</div>
							<div class="download-result-actions flex-1 flex flex-col bg-white">
								<div class="p-6 md:p-8">
									<FormatPicker streams={extractResult.streams} onSelect={handleFormatSelect}/>
								</div>
								<div class="download-result-download p-6 md:p-8 border-t border-indigo-50 bg-indigo-50/20">
									<DownloadBtn stream={$currentDownload.selectedStream} audioStream={selectedAudioStream} title={extractResult.title}/>
								</div>
							</div>
					</div>
				</div>
			</section>
		{/if}

		<section class="py-10 px-6 lg:px-20 relative">
			<div class="max-w-5xl mx-auto relative">
				<div class="absolute -top-10 left-[8%] h-36 w-36 rounded-full bg-primary/20 blur-3xl"></div>
				<div class="absolute -bottom-10 right-[10%] h-40 w-40 rounded-full bg-secondary/25 blur-3xl"></div>

				<div class="relative rounded-[2.25rem] border border-white/70 bg-white/65 backdrop-blur-xl shadow-float p-6 md:p-8 overflow-hidden">
					<div class="absolute inset-0 bg-[radial-gradient(circle_at_15%_20%,rgba(255,77,140,0.14),transparent_40%),radial-gradient(circle_at_85%_80%,rgba(255,185,56,0.16),transparent_42%)]"></div>
					<div class="relative">
						<div class="text-center mx-auto max-w-3xl">
							<span class="inline-flex items-center rounded-full bg-white/80 px-4 py-1 text-xs font-bold uppercase tracking-wider text-primary shadow-sm">
								{m.home_playlist_badge()}
							</span>
							<h2 class="mt-3 text-2xl md:text-4xl font-bold text-plum leading-tight">
								{m.home_playlist_title()}
							</h2>
							<p class="mt-2 text-sm md:text-base text-plum/70 font-semibold">
								{m.home_playlist_subtitle()}
							</p>
						</div>

						<div class="mt-6 grid grid-cols-1 lg:grid-cols-[1.05fr,0.95fr] gap-4">
							<BatchInput/>
							<BatchProgress/>
						</div>
					</div>
				</div>
			</div>
		</section>

		<section class="py-8 px-6 lg:px-20 relative z-20" id="how-it-works">
			<div class="max-w-6xl mx-auto">
				<div class="grid grid-cols-1 md:grid-cols-3 gap-6 relative">
					<div class="relative group">
						<div class="bg-white p-6 rounded-[2rem] shadow-sm border border-pink-50 hover:shadow-float transition-all duration-300 h-full flex flex-col items-center text-center relative overflow-hidden">
							<div class="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-primary to-transparent opacity-50"></div>
							<div class="w-16 h-16 rounded-full bg-primary/10 flex items-center justify-center mb-4 group-hover:scale-110 transition-transform duration-300">
								<span class="material-symbols-outlined text-3xl text-primary">search</span>
							</div>
							<h3 class="text-xl font-bold text-plum mb-2">{m.home_step1_title()}</h3>
							<p class="text-plum/60 text-sm font-medium leading-snug">{m.home_step1_desc()}</p>
						</div>
						<div class="md:hidden flex justify-center py-2 text-plum/20">
							<span class="material-symbols-outlined">arrow_downward</span>
						</div>
					</div>
					<div class="relative group md:-translate-y-4">
						<div class="step-paste-card bg-gradient-to-br from-white to-pink-50 p-6 rounded-[2rem] shadow-candy border border-pink-100 hover:-translate-y-1 transition-all duration-300 h-full flex flex-col items-center text-center relative overflow-hidden z-10">
							<div class="absolute -right-10 -top-10 w-24 h-24 bg-secondary/10 rounded-full blur-xl"></div>
							<div class="w-16 h-16 rounded-full bg-secondary text-white shadow-lg flex items-center justify-center mb-4 animate-bob">
								<span class="material-symbols-outlined text-3xl">content_paste_go</span>
							</div>
							<h3 class="text-xl font-bold text-plum mb-2">{m.home_step2_title()}</h3>
							<p class="text-plum/60 text-sm font-medium leading-snug">{m.home_step2_desc()}</p>
						</div>
						<div class="md:hidden flex justify-center py-2 text-plum/20">
							<span class="material-symbols-outlined">arrow_downward</span>
						</div>
					</div>
					<div class="relative group">
						<div class="bg-white p-6 rounded-[2rem] shadow-sm border border-pink-50 hover:shadow-float transition-all duration-300 h-full flex flex-col items-center text-center relative overflow-hidden">
							<div class="absolute top-0 right-0 w-full h-1 bg-gradient-to-l from-accent to-transparent opacity-50"></div>
							<div class="w-16 h-16 rounded-full bg-accent/10 flex items-center justify-center mb-4 group-hover:scale-110 transition-transform duration-300">
								<span class="material-symbols-outlined text-3xl text-accent">download_for_offline</span>
							</div>
							<h3 class="text-xl font-bold text-plum mb-2">{m.home_step3_title()}</h3>
							<p class="text-plum/60 text-sm font-medium leading-snug">{m.home_step3_desc()}</p>
						</div>
					</div>
				</div>
			</div>
		</section>

		<section class="py-10 px-6 lg:px-20 bg-white/40 border-y border-white/60" id="tools">
			<div class="max-w-7xl mx-auto">
				<div class="mb-8">
					<div class="flex flex-col lg:flex-row lg:items-end lg:justify-between gap-4">
						<div class="text-left">
							<span class="text-xs font-bold text-primary uppercase tracking-wider mb-1 block">{m.home_tools_badge()}</span>
							<h2 class="text-2xl md:text-3xl lg:text-4xl font-bold text-plum leading-tight">
								{m.home_tools_title()}
								<span class="inline-block animate-bounce text-2xl" style="animation-duration: 3s;">😴</span>
							</h2>
						</div>
						<p class="text-sm md:text-base text-plum/70 lg:max-w-[460px] lg:text-right leading-relaxed font-medium">
							{m.home_tools_subtitle()}
						</p>
					</div>
				</div>
				<div class="grid grid-cols-1 lg:grid-cols-3 gap-5 items-stretch">
					<div class="bento-card group relative flex flex-col bg-white rounded-2xl p-5 border border-pink-50 shadow-sm overflow-hidden min-h-[300px]">
						<div class="absolute top-4 right-4 bg-secondary text-white text-[10px] font-bold px-2 py-0.5 rounded-full shadow-sm z-10 tracking-widest uppercase">{m.home_tool_recommended_badge()}</div>
						<div class="flex-1 flex flex-col items-start z-10 mt-2">
							<div class="size-14 mb-3 bg-blue-50 rounded-xl flex items-center justify-center text-blue-500">
								<span class="material-symbols-outlined text-3xl">extension</span>
							</div>
							<h3 class="text-lg font-bold text-plum mb-1">{m.home_tool_extension_title()}</h3>
							<p class="text-plum/60 font-medium mb-4 text-sm leading-relaxed">{m.home_tool_extension_desc()}</p>
						</div>
						<div class="mt-auto z-10 w-full">
							<button class="w-full h-10 bg-primary hover:bg-primary/90 text-white font-bold rounded-xl shadow-candy flex items-center justify-center gap-2 transition-all hover:scale-[1.02] tracking-wide uppercase text-[11px]">
								<span class="material-symbols-outlined text-base">add_to_queue</span>
								{m.home_tool_extension_cta()}
							</button>
						</div>
					</div>
					<div class="bento-card group relative flex flex-col bg-white rounded-2xl p-1 border border-pink-100 shadow-sm min-h-[300px]">
						<div class="h-full flex flex-col p-4 bg-pink-50/30 rounded-xl backdrop-blur-sm">
							<div class="flex-1 flex flex-col items-start">
								<div class="size-14 mb-3 bg-secondary/10 rounded-xl flex items-center justify-center text-secondary rotate-3 group-hover:rotate-12 transition-transform">
									<span class="material-symbols-outlined text-3xl">bookmarks</span>
								</div>
								<h3 class="text-lg font-bold text-plum mb-1">{m.home_tool_bookmarklet_title()}</h3>
								<p class="text-plum/60 font-medium mb-4 text-sm leading-relaxed">{m.home_tool_bookmarklet_desc()}</p>
								<div class="w-full py-3 px-2 bg-white rounded-lg border border-pink-200 border-dashed mb-2 flex justify-center items-center relative overflow-hidden group-hover:border-primary/30 transition-colors">
									<a class="cursor-grab active:cursor-grabbing inline-flex items-center gap-1.5 bg-gradient-to-r from-primary to-secondary text-white font-bold py-1.5 px-4 rounded-full shadow-md hover:shadow-lg transform hover:scale-105 active:scale-95 transition-all select-none z-10 text-xs" href="https://download.khoadangbui.online">
										<span class="material-symbols-outlined text-base">touch_app</span>
										{m.home_tool_bookmarklet_cta()}
									</a>
								</div>
							</div>
						</div>
					</div>
					<div class="bento-card group relative flex flex-col bg-plum rounded-2xl p-5 shadow-xl min-h-[300px] text-white overflow-hidden">
						<div class="absolute top-0 right-0 w-32 h-32 bg-accent/20 rounded-full blur-2xl -translate-y-1/2 translate-x-1/2"></div>
						<div class="flex-1 flex flex-col items-start z-10 relative">
							<div class="size-14 mb-3 bg-white/10 rounded-xl flex items-center justify-center text-accent border border-white/10">
								<span class="material-symbols-outlined text-3xl">terminal</span>
							</div>
							<h3 class="text-lg font-bold mb-1">{m.home_tool_script_title()}</h3>
							<p class="text-white/60 font-medium mb-4 text-sm leading-relaxed">{m.home_tool_script_desc()}</p>
							<div class="w-full bg-black/40 rounded-lg p-3 font-mono text-[10px] text-accent mb-2 border border-white/5 shadow-inner">
								<p class="opacity-60">// install.js</p>
								<p class="truncate"><span class="text-secondary">const</span> <span class="text-white">q</span> = <span class="text-primary">'4k'</span>;</p>
							</div>
						</div>
						<div class="mt-auto z-10 relative w-full">
							<button class="w-full h-10 bg-accent hover:bg-accent/90 text-white font-bold rounded-xl shadow-lg hover:shadow-accent/50 flex items-center justify-center gap-2 transition-all hover:scale-[1.02] uppercase text-[11px] tracking-wide">
								<span class="material-symbols-outlined text-base">download</span>
								{m.home_tool_script_cta()}
							</button>
						</div>
					</div>
				</div>
			</div>
		</section>

		<section class="py-12 px-6 lg:px-20 relative overflow-hidden">
			<div class="absolute top-0 left-0 w-full h-full overflow-hidden pointer-events-none">
				<div class="absolute top-20 left-[10%] w-64 h-64 bg-primary/5 rounded-full blur-3xl"></div>
				<div class="absolute bottom-10 right-[10%] w-64 h-64 bg-secondary/5 rounded-full blur-3xl"></div>
			</div>
			<div class="max-w-7xl mx-auto relative z-10">
				<div class="flex flex-col md:flex-row items-center gap-8 mb-10">
					<div class="flex-1 text-center md:text-left">
						<span class="inline-block py-1 px-3 rounded-full bg-green-100 text-green-700 font-bold text-xs uppercase mb-3 tracking-wider">{m.home_testimonials_badge()}</span>
						<h2 class="text-3xl md:text-4xl font-bold text-plum mb-3">{m.home_testimonials_title()}</h2>
						<p class="text-base text-plum/60 font-semibold max-w-md mx-auto md:mx-0">{m.home_testimonials_subtitle()}</p>
						<div class="hidden md:flex mt-6 -space-x-3">
							<div class="w-10 h-10 rounded-full border-2 border-white bg-gray-200 flex items-center justify-center overflow-hidden" title="User 1"><img alt="User 1" class="w-full h-full object-cover" loading="lazy" decoding="async" src="https://lh3.googleusercontent.com/aida-public/AB6AXuCr1s4gikAC15kJYl5qNBbsjOo7NvGwP-H5lCWIDqZKVOYU2h9j-E2_UZdlEnXou4LrtMM3Ff1HTyxqbnOiC9bvRpWEqowt71waDpEDOaw_zAIm6_-p4CwWDFdu2Vaf9JhVeubsZZxQ7z-0qXs69DyDWHKhhgJUQF_VsGt5GlM6pQEkGzjhF3F1hanb3N9naflxMGIE2BeHCcALu-TezlyNc54bWQTZbhdmC9ShcyTB-9jwh9b2qN9ix6CI63pHKHVAoW27bq2zkRc"/></div>
							<div class="w-10 h-10 rounded-full border-2 border-white bg-gray-200 flex items-center justify-center overflow-hidden" title="User 2"><img alt="User 2" class="w-full h-full object-cover" loading="lazy" decoding="async" src="https://lh3.googleusercontent.com/aida-public/AB6AXuB5xhTXOwEgpTaKXMTknZrIIzsx0AeA6BgE4MHNg7Mvuw2dyghH4DETumEU1BROmzX9S-GOeS3nO3MA96z7DuSbMVKMRLdptn0I6qoKXvwa-CdRMyOxI9Or3Sv4NTv7muyoitFAQBbN0zSbZzsXq09uM4W6k5mvTJr8TcolR6DLduFQK7qkVcwEJpHxpA9o8HYaYGG6ftflLPWobNeWxiHjNGfW9O2tomENUVaB0lF-MD6MDO6GM4OqlAM2IL8yP2wcvwk4kzkG9uM"/></div>
							<div class="w-10 h-10 rounded-full border-2 border-white bg-gray-200 flex items-center justify-center overflow-hidden" title="User 3"><img alt="User 3" class="w-full h-full object-cover" loading="lazy" decoding="async" src="https://lh3.googleusercontent.com/aida-public/AB6AXuA9wwmTlRqKsiXoYfLY2NAB9wPMkICqm-3zO7GvQPsEW4UgWCMp3uOisNRZukLdsGuYCj7gFNwNviQ-mzLlGQ4ewFpnKTJThORs18zDOOcN9LOKjjJJuVm9eNo3IyTNVOKClYaWzzuo_ObdhWgNZk7rAIFZ44BtDZQn5wkzRUilIzxRUUfItE6WvU0x_RJ429m-oSsKXUnAysF6Ngo6T0rSVYNZ4yr8ksti54vV6xGmVzHxIBMvC7Mr-jkT532iWU2Pe-1CFggyRLw"/></div>
							<div class="w-10 h-10 rounded-full border-2 border-white bg-plum text-white text-xs font-bold flex items-center justify-center">+9k</div>
						</div>
					</div>
					<div class="flex-1 w-full grid grid-cols-1 sm:grid-cols-2 gap-4">
						<div class="bg-white p-5 rounded-2xl shadow-sm border border-pink-50 hover:shadow-float hover:-translate-y-1 transition-all duration-300">
							<div class="flex items-center gap-3 mb-3">
								<div class="w-8 h-8 rounded-full bg-purple-100 overflow-hidden"><img alt="Sarah" class="w-full h-full" loading="lazy" decoding="async" src="https://lh3.googleusercontent.com/aida-public/AB6AXuDVFoqrmvbgSZf05UFx8xRHswErghnzjNOy7leL0xLeiMewdeCaInDrQHxNnvbjuxXsgrIWckkpM59VJug5wCYqRiVKPWIq1BfzsG4UP6Y8fOVTtQsRTXx_m1AD0IAcF2695QhAeGb2iMnEHrzeX38QZ9kwCa0ZUXXOWOwWGszBJTmF9uH2NPQxegbEAxrZoho7UsmyZQRVnVtJ_czTdESPKBrtI3geH5dWcj8GLJH8TQ8MQdUjj7pm0ciiCABSj1vZIieoe5fIzrk"/></div>
								<div>
									<h4 class="font-bold text-plum text-sm">Sarah J.</h4>
									<div class="flex text-yellow-400 text-[10px]">★★★★★</div>
								</div>
							</div>
							<p class="text-plum/70 font-medium italic text-xs leading-relaxed">{m.home_testimonial_sarah_quote()}</p>
						</div>
						<div class="bg-white p-5 rounded-2xl shadow-sm border border-pink-50 hover:shadow-float hover:-translate-y-1 transition-all duration-300 sm:translate-y-4">
							<div class="flex items-center gap-3 mb-3">
								<div class="w-8 h-8 rounded-full bg-blue-100 overflow-hidden"><img alt="Mike" class="w-full h-full" loading="lazy" decoding="async" src="https://lh3.googleusercontent.com/aida-public/AB6AXuC5aEM0rZBPm42dz3OfS3JXRJzMD-riFsBxazw_V1sJmmRgLPSc35MBXlnCumxZYs_dmbgtm9g9P0_RU6A082cBKMEh9GIU2NTlkKeyj634aKYLSvflJq858W8NrUK840VaZHd_NE-GHp28czJWzQNC8h7sYVqI15ogdkMHQi9kPN_wzXAUAH-U9XMa051_n40aRPs19d_Z4QhX1l2e0rs8_bLM4FpGjjopXp0gDCiuk-HdbwNJ4C3bmuW58R2CXRieiwoH86RL-ZU"/></div>
								<div>
									<h4 class="font-bold text-plum text-sm">Chef Mike</h4>
									<div class="flex text-yellow-400 text-[10px]">★★★★★</div>
								</div>
							</div>
							<p class="text-plum/70 font-medium italic text-xs leading-relaxed">{m.home_testimonial_mike_quote()}</p>
						</div>
						<div class="col-span-1 sm:col-span-2 mt-2 bg-gradient-to-r from-primary/10 to-secondary/10 p-4 rounded-2xl border border-white/50 flex items-center justify-between">
							<div class="flex-1">
								<h4 class="font-bold text-plum text-sm mb-1">{m.home_join_party_title()}</h4>
								<p class="text-plum/60 text-xs">{m.home_join_party_subtitle()}</p>
							</div>
							<button class="bg-plum text-white font-bold text-xs px-5 py-2.5 rounded-full shadow-lg hover:bg-plum/90 hover:scale-105 active:scale-95 transition-all duration-300 flex items-center gap-2">
								<span>{m.home_join_party_cta()}</span>
								<span class="material-symbols-outlined text-sm">arrow_forward</span>
							</button>
							<div class="w-12 h-12 ml-4 animate-wiggle"><span class="text-4xl">🎉</span></div>
						</div>
					</div>
				</div>
			</div>
		</section>
		</main>

		<AuthModal
			open={authModalOpen}
			redirectTo={redirectTo}
			onClose={closeAuthModal}
			onSuccess={handleAuthSuccess}
		/>

	<SiteFooter />

	<button
		type="button"
		class="theme-toggle fixed bottom-5 right-5 z-[70] flex h-12 min-w-[120px] items-center justify-center gap-2 rounded-full px-4 text-sm font-bold shadow-xl hover:scale-105 active:scale-95 transition-all duration-300 backdrop-blur-md"
		onclick={toggleTheme}
		aria-label={isDarkMode ? m.common_theme_switch_to_light() : m.common_theme_switch_to_dark()}
	>
		<span class="material-symbols-outlined text-[18px]">{isDarkMode ? 'light_mode' : 'dark_mode'}</span>
		<span>{isDarkMode ? m.common_theme_light_mode() : m.common_theme_dark_mode()}</span>
	</button>
</div>
