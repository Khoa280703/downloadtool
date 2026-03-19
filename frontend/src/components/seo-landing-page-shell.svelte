<script lang="ts">
	/**
	 * Shared shell for EN long-tail SEO landing pages.
	 * Provides: header, downloader input, USP cards, FAQ accordion, cross-links, footer.
	 * Keeps each page file under 200 lines by centralising all shared logic here.
	 */
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';
	import AppIcon from '$components/AppIcon.svelte';
	import SiteHeader from '$components/SiteHeader.svelte';
	import FormatPicker from '$components/FormatPicker.svelte';
	import DownloadBtn from '$components/DownloadBtn.svelte';
	import { extract, extractYouTubeVideoId, isValidVideoUrl } from '$lib/api';
	import { currentDownload } from '$stores/download';
	import type { ExtractResult, Stream } from '$lib/types';
	import type { LandingPageConfig } from '$lib/seo/landing-page-config';

	let { config }: { config: LandingPageConfig } = $props();

	let inputUrl = $state('');
	let isExtracting = $state(false);
	let extractError = $state('');
	let extractResult = $state<ExtractResult | null>(null);
	let selectedAudioStream = $state<Stream | null>(null);
	let isDarkMode = $state(false);

	function syncTheme(): void {
		if (!browser) return;
		isDarkMode = window.localStorage.getItem('snapvie-theme') === 'dark';
	}

	function toggleTheme(): void {
		isDarkMode = !isDarkMode;
		if (!browser) return;
		window.localStorage.setItem('snapvie-theme', isDarkMode ? 'dark' : 'light');
		window.dispatchEvent(new CustomEvent('snapvie-theme-change', { detail: { isDarkMode } }));
	}

	function handleFormatSelect(videoStream: Stream, audioStream: Stream | null): void {
		currentDownload.update((s) => ({ ...s, selectedStream: videoStream }));
		selectedAudioStream = audioStream;
	}

	async function handleSubmit(event: SubmitEvent): Promise<void> {
		event.preventDefault();
		const url = inputUrl.trim();

		if (!url) {
			extractError = 'Please paste a YouTube URL first.';
			return;
		}
		if (!isValidVideoUrl(url)) {
			extractError = 'That does not look like a valid YouTube URL. Please check and try again.';
			return;
		}

		isExtracting = true;
		extractError = '';
		extractResult = null;
		selectedAudioStream = null;
		currentDownload.update((s) => ({ ...s, selectedStream: null, error: null }));

		try {
			const result = await extract(url);
			if (!result.streams.length) {
				extractError = 'No downloadable streams found for this video.';
				return;
			}
			extractResult = result;
			requestAnimationFrame(() => {
				document.getElementById('lp-result')?.scrollIntoView({ behavior: 'smooth', block: 'start' });
			});
		} catch (err) {
			extractError = err instanceof Error ? err.message : 'Something went wrong. Please try again.';
		} finally {
			isExtracting = false;
		}
	}

	onMount(() => {
		syncTheme();
		const themeHandler = (e: Event) => {
			const ce = e as CustomEvent<{ isDarkMode?: boolean }>;
			isDarkMode = ce.detail?.isDarkMode ?? (window.localStorage.getItem('snapvie-theme') === 'dark');
		};
		const storageHandler = (e: StorageEvent) => {
			if (e.key === 'snapvie-theme') isDarkMode = e.newValue === 'dark';
		};
		window.addEventListener('snapvie-theme-change', themeHandler as EventListener);
		window.addEventListener('storage', storageHandler);
		return () => {
			window.removeEventListener('snapvie-theme-change', themeHandler as EventListener);
			window.removeEventListener('storage', storageHandler);
		};
	});
</script>

<div
	class="page-root bg-bg-page min-h-screen flex flex-col overflow-x-hidden text-plum"
	class:theme-dark={isDarkMode}
	class:theme-light={!isDarkMode}
>
	<SiteHeader
		authUser={null}
		onOpenAuthModal={() => {}}
		homeHref="/"
		howItWorksHref="/#how-it-works"
		toolsHref="/#download-options"
	/>

	<main class="flex-1 w-full">
		<!-- Hero + Downloader -->
		<section class="relative pt-12 pb-8 px-6 overflow-visible" id="lp-hero">
			<div class="hero-orb absolute top-[10%] left-[5%] w-24 h-24 rounded-full bg-accent/20 blur-xl animate-bob"></div>
			<div class="hero-orb absolute bottom-[20%] right-[10%] w-32 h-32 rounded-3xl rotate-12 bg-primary/10 blur-xl animate-bob-delayed"></div>
			<div class="relative z-10 w-full max-w-4xl mx-auto text-center">
				<!-- Breadcrumb -->
				<nav aria-label="Breadcrumb" class="mb-4 flex justify-center gap-2 text-xs font-semibold text-plum/50">
					<a href="/" class="hover:text-primary transition-colors">Snapvie</a>
					<span>›</span>
					<span class="text-plum/70">{config.h1}</span>
				</nav>

				<h1 class="text-3xl md:text-5xl lg:text-6xl font-bold text-plum mb-4 leading-tight tracking-tight">
					{config.h1}
				</h1>
				<p class="text-lg md:text-xl text-plum/70 max-w-xl mx-auto font-semibold mb-8">
					{config.heroSubtitle}
				</p>

				<!-- URL Input -->
				<form
					id="lp-download-form"
					class="relative mx-auto mb-6 w-full max-w-[700px] group"
					onsubmit={handleSubmit}
				>
					<div class="absolute -inset-1 rounded-full bg-gradient-to-r from-primary to-secondary blur opacity-25 transition duration-500 group-hover:opacity-50"></div>
					<div class="relative flex h-[64px] items-center rounded-full bg-white p-2 shadow-float transition-all duration-300 group-focus-within:ring-4 group-focus-within:ring-primary/20">
						<div class="pl-6 text-plum/30">
							<AppIcon name="link" class="text-2xl" />
						</div>
						<input
							class="h-full w-full border-none bg-transparent px-4 text-lg font-semibold text-plum placeholder:text-muted/50 focus:ring-0 md:text-xl"
							placeholder="Paste YouTube URL here..."
							type="text"
							bind:value={inputUrl}
							disabled={isExtracting}
							aria-label="YouTube video URL"
						/>
						<button
							class="absolute right-1.5 top-1.5 bottom-1.5 flex items-center justify-center rounded-full bg-gradient-primary px-3 text-sm font-bold text-white shadow-candy transition-all hover:scale-105 hover:brightness-110 active:scale-95 disabled:cursor-not-allowed disabled:opacity-60 disabled:hover:scale-100 md:gap-2 md:px-10 md:text-lg"
							type="submit"
							disabled={isExtracting}
						>
							<span class="hidden md:inline">{isExtracting ? 'Fetching...' : 'Get Download'}</span>
							<AppIcon
								name={isExtracting ? 'progress_activity' : 'bolt'}
								class={`text-base font-bold md:text-lg ${isExtracting ? 'animate-spin' : ''}`}
							/>
						</button>
					</div>

					{#if extractError}
						<p class="mt-3 text-sm font-bold text-red-500">{extractError}</p>
					{/if}
					{#if isExtracting}
						<p class="mt-3 inline-flex items-center gap-2 rounded-full bg-white/90 px-4 py-2 text-sm font-bold text-primary shadow-sm">
							<AppIcon name="progress_activity" class="animate-spin text-base" />
							Analyzing link...
						</p>
					{/if}
				</form>

				<!-- Trust chips -->
				<div class="flex flex-wrap justify-center gap-3 opacity-80">
					<div class="flex items-center gap-2 rounded-xl border border-white/50 bg-white/60 px-3 py-1.5">
						<AppIcon name="check_circle" class="text-lg text-green-500" />
						<span class="text-xs font-bold text-plum/70">No Ads</span>
					</div>
					<div class="flex items-center gap-2 rounded-xl border border-white/50 bg-white/60 px-3 py-1.5">
						<AppIcon name="verified_user" class="text-lg text-blue-500" />
						<span class="text-xs font-bold text-plum/70">Safe &amp; Secure</span>
					</div>
					<div class="flex items-center gap-2 rounded-xl border border-white/50 bg-white/60 px-3 py-1.5">
						<AppIcon name="rocket_launch" class="text-lg text-purple-500" />
						<span class="text-xs font-bold text-plum/70">Super Fast</span>
					</div>
				</div>
			</div>
		</section>

		<!-- Download Result -->
		{#if extractResult}
			<section class="py-5 px-6 lg:px-20" id="lp-result">
				<div class="max-w-7xl mx-auto">
					<div class="bg-white rounded-[2rem] shadow-card border border-indigo-50 overflow-hidden flex flex-col lg:flex-row">
						<div class="w-full lg:w-[42%] p-6 md:p-7 flex flex-col gap-4 bg-gradient-to-b from-indigo-50/50 to-white lg:border-r border-indigo-50">
							<div class="relative w-full aspect-video rounded-3xl overflow-hidden shadow-lg border-4 border-white bg-slate-100">
								{#if extractResult.thumbnail}
									<img class="absolute inset-0 w-full h-full object-cover" src={extractResult.thumbnail} alt={extractResult.title} />
								{:else}
									<div class="absolute inset-0 grid place-items-center text-slate-400">
										<AppIcon name="movie" class="text-6xl" />
									</div>
								{/if}
								<div class="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent"></div>
								<div class="absolute bottom-4 right-4 bg-black/60 backdrop-blur-md px-3 py-1.5 rounded-full text-xs font-bold text-white border border-white/20">Ready</div>
							</div>
							<h3 class="text-2xl md:text-3xl font-bold text-slate-900 leading-tight">{extractResult.title}</h3>
							<p class="text-slate-500 font-semibold">Select a format below, then download.</p>
						</div>
						<div class="flex-1 flex flex-col bg-white">
							<div class="p-5 md:p-6 pb-4">
								<FormatPicker streams={extractResult.streams} onSelect={handleFormatSelect} />
							</div>
							<div class="p-5 pt-4 md:p-6 md:pt-5 border-t border-indigo-50 bg-indigo-50/20">
								<DownloadBtn
									stream={$currentDownload.selectedStream}
									audioStream={selectedAudioStream}
									sourceUrl={extractResult.originalUrl}
									title={extractResult.title}
								/>
							</div>
						</div>
					</div>
				</div>
			</section>
		{/if}

		<!-- Intro Text -->
		<section class="py-10 px-6 lg:px-20">
			<div class="max-w-3xl mx-auto">
				<p class="text-base text-plum/80 leading-relaxed font-medium">{config.introText}</p>
			</div>
		</section>

		<!-- USP Cards -->
		<section class="py-8 px-6 lg:px-20 bg-slate-50 border-t border-pink-50">
			<div class="max-w-6xl mx-auto">
				<h2 class="text-2xl font-bold text-plum mb-8 text-center">Why Use Snapvie?</h2>
				<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
					{#each config.uspBullets as usp}
						<div class="bento-card bg-white p-6 rounded-[2rem] shadow-sm border border-pink-50 flex flex-col gap-3">
							<div class="w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center">
								<AppIcon name={usp.icon} class="text-2xl text-primary" />
							</div>
							<h3 class="font-bold text-plum text-base">{usp.title}</h3>
							<p class="text-sm text-plum/70 leading-relaxed">{usp.desc}</p>
						</div>
					{/each}
				</div>
			</div>
		</section>

		<!-- FAQ -->
		<section class="py-10 px-6 lg:px-20 bg-white border-t border-pink-50">
			<div class="max-w-3xl mx-auto">
				<h2 class="text-2xl font-bold text-plum mb-6 text-center">Frequently Asked Questions</h2>
				<div class="divide-y divide-pink-50">
					{#each config.faqItems as item}
						<details class="py-3 group">
							<summary class="cursor-pointer font-semibold text-plum text-sm list-none flex items-center justify-between gap-2">
								{item.q}
								<span class="text-plum/40 text-xs group-open:rotate-180 transition-transform shrink-0 select-none">▼</span>
							</summary>
							<p class="mt-2 text-plum/70 text-sm leading-relaxed">{item.a}</p>
						</details>
					{/each}
				</div>
			</div>
		</section>

		<!-- Related pages / cross-links -->
		<section class="py-8 px-6 lg:px-20 bg-slate-50 border-t border-pink-50">
			<div class="max-w-3xl mx-auto text-center">
				<h2 class="text-lg font-bold text-plum mb-5">Explore More Snapvie Tools</h2>
				<div class="flex flex-wrap justify-center gap-3">
					<a
						href="/"
						class="inline-flex items-center gap-2 rounded-full border border-pink-100 bg-white px-5 py-2 text-sm font-bold text-plum hover:text-primary hover:border-primary transition-colors"
					>
						<AppIcon name="home" class="text-base" />
						Back to Snapvie
					</a>
					{#each config.relatedPages as rel}
						<a
							href={`/${rel.slug}`}
							class="inline-flex items-center gap-2 rounded-full border border-pink-100 bg-white px-5 py-2 text-sm font-bold text-plum hover:text-primary hover:border-primary transition-colors"
						>
							{rel.label}
						</a>
					{/each}
				</div>
			</div>
		</section>
	</main>

	<footer class="bg-white border-t border-pink-100 py-6 px-6">
		<div class="max-w-7xl mx-auto flex flex-col md:flex-row justify-between items-center gap-4">
			<div class="flex items-center gap-2">
				<AppIcon name="smart_toy" class="text-xl text-plum/75 grayscale hover:grayscale-0" />
				<span class="font-bold text-sm text-plum/90">© {new Date().getFullYear()} Snapvie. All rights reserved.</span>
			</div>
			<div class="flex gap-4 text-plum/80 font-semibold text-xs">
				<a class="underline-offset-2 hover:text-primary hover:underline transition-colors" href="/privacy">Privacy Policy</a>
				<a class="underline-offset-2 hover:text-primary hover:underline transition-colors" href="/privacy">Terms of Service</a>
				<a class="underline-offset-2 hover:text-primary hover:underline transition-colors" href="/privacy#contact">Contact</a>
			</div>
		</div>
	</footer>

	<!-- Theme toggle -->
	<button
		type="button"
		class="theme-toggle fixed bottom-5 right-5 z-[70] flex h-12 min-w-[120px] items-center justify-center gap-2 rounded-full px-4 text-sm font-bold shadow-xl hover:scale-105 active:scale-95 transition-all duration-300 backdrop-blur-md"
		onclick={toggleTheme}
		aria-label={isDarkMode ? 'Switch to light mode' : 'Switch to dark mode'}
	>
		<AppIcon name={isDarkMode ? 'light_mode' : 'dark_mode'} class="text-[18px]" />
		<span>{isDarkMode ? 'Light' : 'Dark'}</span>
	</button>
</div>

<style>
	.page-root {
		background-color: #fff5f9;
		color: #2d1b36;
		font-family: 'Nunito', sans-serif;
		transition: background-color 220ms ease, color 220ms ease;
	}

	h1, h2, h3, button {
		font-family: 'Fredoka', sans-serif;
	}

	:global(.glass-header) {
		background: rgba(255, 245, 249, 0.8);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
	}

	:global(.text-gradient) {
		background: linear-gradient(135deg, #ff4d8c 0%, #ffb938 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
	}

	:global(.bg-gradient-primary) {
		background: linear-gradient(135deg, #ff4d8c 0%, #ffb938 100%);
	}

	.bento-card {
		transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1), box-shadow 0.3s ease;
	}

	.bento-card:hover {
		transform: translateY(-8px);
		box-shadow: 0 25px 50px -12px rgba(255, 77, 140, 0.25);
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

	.page-root.theme-dark :global(.glass-header) {
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

	.page-root.theme-dark [class*='text-plum/'],
	.page-root.theme-dark [class*='text-muted'] {
		color: rgba(224, 208, 245, 0.7) !important;
	}

	.page-root.theme-dark .bg-bg-page,
	.page-root.theme-dark [class*='bg-white/30'],
	.page-root.theme-dark [class*='bg-white/40'],
	.page-root.theme-dark .bg-white,
	.page-root.theme-dark [class*='bg-pink-50'],
	.page-root.theme-dark [class*='bg-slate-50'],
	.page-root.theme-dark [class*='bg-slate-100'],
	.page-root.theme-dark [class*='bg-indigo-50'] {
		background-color: rgba(30, 30, 42, 0.6) !important;
		backdrop-filter: blur(12px);
	}

	.page-root.theme-dark .border-white,
	.page-root.theme-dark [class*='border-pink'],
	.page-root.theme-dark [class*='border-slate'],
	.page-root.theme-dark [class*='border-indigo'] {
		border-color: rgba(255, 77, 140, 0.18) !important;
	}

	.page-root.theme-dark footer {
		background-color: #12121a !important;
		border-color: rgba(255, 255, 255, 0.08) !important;
	}

	.page-root.theme-dark footer a {
		color: rgba(224, 208, 245, 0.55) !important;
	}

	.page-root.theme-dark footer a:hover {
		color: #ff4d8c !important;
	}

	html {
		scroll-behavior: smooth;
	}
</style>
