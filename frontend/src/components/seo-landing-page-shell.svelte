<script lang="ts">
	/**
	 * Shared shell for EN long-tail SEO landing pages.
	 * Provides: downloader input, USP cards, FAQ accordion, cross-links.
	 * Keeps each page file under 200 lines by centralising all shared logic here.
	 */
	import AppIcon from '$components/AppIcon.svelte';
	import FormatPicker from '$components/FormatPicker.svelte';
	import DownloadBtn from '$components/DownloadBtn.svelte';
	import ExploreMoreSnapvieTools from '$components/explore-more-snapvie-tools.svelte';
	import FrequentlyAskedQuestionsSection from '$components/frequently-asked-questions-section.svelte';
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
</script>

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
		<FrequentlyAskedQuestionsSection items={config.faqItems} />

		<!-- Related pages / cross-links -->
		<ExploreMoreSnapvieTools
			showHomeLink={true}
			links={config.relatedPages.map((rel) => ({ href: `/${rel.slug}`, label: rel.label }))}
		/>

<style>
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
		background-clip: text;
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

	:global(.page-root.theme-dark .text-plum),
	:global(.page-root.theme-dark .text-text-main) {
		color: #ffffff !important;
	}

	:global(.page-root.theme-dark [class*='text-plum/']),
	:global(.page-root.theme-dark [class*='text-muted']) {
		color: rgba(224, 208, 245, 0.7) !important;
	}

	:global(.page-root.theme-dark .bg-bg-page),
	:global(.page-root.theme-dark [class*='bg-white/30']),
	:global(.page-root.theme-dark [class*='bg-white/40']),
	:global(.page-root.theme-dark .bg-white),
	:global(.page-root.theme-dark [class*='bg-pink-50']),
	:global(.page-root.theme-dark [class*='bg-slate-50']),
	:global(.page-root.theme-dark [class*='bg-slate-100']),
	:global(.page-root.theme-dark [class*='bg-indigo-50']) {
		background-color: rgba(30, 30, 42, 0.6) !important;
		backdrop-filter: blur(12px);
	}

	:global(.page-root.theme-dark .border-white),
	:global(.page-root.theme-dark [class*='border-pink']),
	:global(.page-root.theme-dark [class*='border-slate']),
	:global(.page-root.theme-dark [class*='border-indigo']) {
		border-color: rgba(255, 77, 140, 0.18) !important;
	}
</style>
