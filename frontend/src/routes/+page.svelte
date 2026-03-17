<script lang="ts">
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { onDestroy, onMount } from 'svelte';
	import BatchProgress from '$components/BatchProgress.svelte';
	import AppIcon from '$components/AppIcon.svelte';
	import SiteHeader from '$components/SiteHeader.svelte';
	import DownloadBtn from '$components/DownloadBtn.svelte';
	import FormatPicker from '$components/FormatPicker.svelte';
	import { extract, extractYouTubeVideoId, isValidVideoUrl, subscribeBatch } from '$lib/api';
	import mikeAvatar from '$lib/assets/testimonials/mike.webp';
	import * as m from '$lib/paraglide/messages';
	import {
		PLAYLIST_DOWNLOAD_MODE_OPTIONS,
		PLAYLIST_QUALITY_OPTIONS,
		type PlaylistDownloadMode,
		type PlaylistQuality,
		getStoredPlaylistDownloadMode,
		getStoredPlaylistQuality
	} from '$lib/playlist-download-stream-selection';
	import {
		type QueueEntry,
		enqueueDownload,
		pickSaveDirectory,
		resetWorkerState,
		setPreferredDownloadMode,
		setPreferredQuality
	} from '$lib/playlist-download-worker';
	import sarahAvatar from '$lib/assets/testimonials/sarah.webp';
	import {
		addBatchItem,
		batchProgress,
		batchQueue,
		completeBatch,
		resetBatch,
		setBatchProgress,
		setEventSource,
		startBatch
	} from '$stores/batch';
	import type { ExtractResult, Stream } from '$lib/types';
	import user1Avatar from '$lib/assets/testimonials/user-1.webp';
	import user2Avatar from '$lib/assets/testimonials/user-2.webp';
	import user3Avatar from '$lib/assets/testimonials/user-3.webp';
	import { currentDownload } from '$stores/download';

	type AuthUser = { name?: string | null; email: string; image?: string | null };
	type AuthModalComponentType = typeof import('$components/AuthModal.svelte').default;

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
	let AuthModalComponent = $state<AuthModalComponentType | null>(null);
	let playlistModeEnabled = $state(false);
	let playlistPhase = $state<'idle' | 'fetching' | 'ready' | 'downloading'>('idle');
	let fsaaSupported = $state(false);
	let dirPicked = $state(false);
	let selectedDownloadMode = $state<PlaylistDownloadMode>(getStoredPlaylistDownloadMode());
	let selectedQuality = $state<PlaylistQuality>(getStoredPlaylistQuality());
	let stagedEntries = $state<QueueEntry[]>([]);
	let playlistCompletionNotified = $state(false);
	/** undefined = loading skeleton, null = unauthenticated, object = authenticated */
	let authUser = $state<AuthUser | null | undefined>(undefined);
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

	const showPlaylistPanel = $derived.by(
		() => playlistPhase !== 'idle' || $batchQueue.length > 0
	);
	const playlistBusy = $derived.by(
		() => playlistPhase === 'fetching' || playlistPhase === 'downloading'
	);
	const playlistReadyCount = $derived.by(() => $batchQueue.length);
	const playlistSelectedCount = $derived.by(
		() => $batchQueue.filter((item) => item.selected !== false).length
	);
	const playlistDiscoveryPercent = $derived.by(() => {
		if ($batchProgress.total <= 0) return 0;
		return Math.min(100, Math.round(($batchProgress.received / $batchProgress.total) * 100));
	});
	const playlistLeadThumbnail = $derived.by(
		() => $batchQueue.find((item) => item.thumbnail)?.thumbnail ?? null
	);
	const playlistDownloadModeLabel = $derived.by(
		() =>
			PLAYLIST_DOWNLOAD_MODE_OPTIONS.find((option) => option.value === selectedDownloadMode)?.label ??
			selectedDownloadMode
	);
	const playlistQualityLabel = $derived.by(
		() =>
			PLAYLIST_QUALITY_OPTIONS.find((option) => option.value === selectedQuality)?.label ??
			selectedQuality
	);
	const playlistStageTitle = $derived.by(() =>
		playlistPhase === 'fetching'
			? m.home_playlist_stage_fetching_title()
			: playlistPhase === 'ready'
				? m.home_playlist_stage_ready_title({ count: String(playlistReadyCount) })
				: playlistPhase === 'downloading'
					? m.home_playlist_stage_downloading_title()
					: m.home_playlist_stage_enabled_title()
	);
	const playlistStageDescription = $derived.by(() =>
		playlistPhase === 'idle'
			? m.home_playlist_stage_idle_desc()
			: playlistPhase === 'fetching'
				? m.home_playlist_stage_fetching_desc({
						received: String($batchProgress.received),
						total: $batchProgress.total ? String($batchProgress.total) : '...'
					})
				: playlistPhase === 'ready'
					? m.home_playlist_stage_ready_desc({
							selected: String(playlistSelectedCount),
							total: String(playlistReadyCount)
						})
					: m.home_playlist_stage_downloading_desc()
	);

	function normalizeRedirectTo(value: string | null): string {
		if (!value || !value.startsWith('/') || value.startsWith('//')) return '/';
		return value;
	}

	async function ensureAuthModalLoaded(): Promise<void> {
		if (AuthModalComponent) return;

		const module = await import('$components/AuthModal.svelte');

		AuthModalComponent = module.default;
	}

	async function refreshAuthUser(): Promise<void> {
		try {
			const resp = await fetch('/api/auth/get-session', { credentials: 'include' });
			authUser = resp.ok ? ((await resp.json())?.user ?? null) : null;
		} catch {
			authUser = null;
		}
	}

	function syncThemeFromStorage(): void {
		if (!browser) return;
		isDarkMode = window.localStorage.getItem('fetchtube-theme') === 'dark';
	}

	function broadcastThemeChange(): void {
		if (!browser) return;
		window.dispatchEvent(new CustomEvent('fetchtube-theme-change', { detail: { isDarkMode } }));
	}

	onMount(() => {
		syncThemeFromStorage();
		const picker = (window as Window & { showDirectoryPicker?: unknown }).showDirectoryPicker;
		fsaaSupported = typeof picker === 'function';

		void (async () => {
			await refreshAuthUser();

			// Handle ?auth=required&redirectTo= after auth state resolves
			const params = new URLSearchParams(window.location.search);
			if (params.get('auth') === 'required' && !authUser) {
				redirectTo = normalizeRedirectTo(params.get('redirectTo'));
				await ensureAuthModalLoaded();
				authModalOpen = true;
			}
		})();

		const storageHandler = (event: StorageEvent) => {
			if (event.key !== 'fetchtube-theme') return;
			isDarkMode = event.newValue === 'dark';
		};

		const themeChangeHandler = (event: Event) => {
			const customEvent = event as CustomEvent<{ isDarkMode?: boolean }>;
			if (typeof customEvent.detail?.isDarkMode === 'boolean') {
				isDarkMode = customEvent.detail.isDarkMode;
				return;
			}

			syncThemeFromStorage();
		};

		window.addEventListener('storage', storageHandler);
		window.addEventListener('fetchtube-theme-change', themeChangeHandler as EventListener);

		return () => {
			window.removeEventListener('storage', storageHandler);
			window.removeEventListener('fetchtube-theme-change', themeChangeHandler as EventListener);
		};
	});

	onDestroy(() => {
		resetWorkerState();
		resetBatch();
	});

	$effect(() => {
		if (playlistPhase !== 'downloading' || playlistCompletionNotified) return;
		if ($batchQueue.length === 0) return;

		const selectedItems = $batchQueue.filter((item) => item.selected !== false);
		const settled = selectedItems.every(
			(item) => item.status === 'completed' || item.status === 'error'
		);
		if (!settled) return;

		playlistCompletionNotified = true;
		playlistPhase = 'ready';
		completeBatch();
	});

	function handleFormatSelect(videoStream: Stream, audioStream: Stream | null): void {
		currentDownload.update((state) => ({ ...state, selectedStream: videoStream }));
		selectedAudioStream = audioStream;
	}

	function isPlaylistUrl(input: string): boolean {
		if (!input.includes('youtube.com') && !input.includes('youtu.be')) return false;
		if (/[?&]list=/.test(input)) return true;
		if (/youtube\.com\/playlist/.test(input)) return true;
		return false;
	}

	function setPlaylistMode(enabled: boolean): void {
		if (playlistBusy || isExtracting) return;
		if (playlistModeEnabled === enabled) return;

		playlistModeEnabled = enabled;
		extractError = '';
		if (enabled) {
			extractResult = null;
			selectedAudioStream = null;
			currentDownload.update((state) => ({ ...state, selectedStream: null, error: null }));
			return;
		}

		resetPlaylistComposer();
	}

	function persistPlaylistQuality(quality: PlaylistQuality): void {
		if (!browser) return;
		try {
			window.localStorage.setItem('fetchtube.playlist-quality.v1', quality);
		} catch {
			// Ignore localStorage failures.
		}
	}

	function persistPlaylistDownloadMode(mode: PlaylistDownloadMode): void {
		if (!browser) return;
		try {
			window.localStorage.setItem('fetchtube.playlist-download-mode.v1', mode);
		} catch {
			// Ignore localStorage failures.
		}
	}

	function pushStagedEntry(entry: QueueEntry): void {
		if (stagedEntries.some((item) => item.videoId === entry.videoId)) return;
		stagedEntries = [...stagedEntries, entry];
	}

	function resetPlaylistComposer(options: { preserveQueue?: boolean } = {}): void {
		resetWorkerState();
		if (!options.preserveQueue) {
			resetBatch();
			stagedEntries = [];
		}
		playlistPhase = 'idle';
		playlistCompletionNotified = false;
	}

	async function handleFetchPlaylist(): Promise<void> {
		if (playlistBusy) return;

		const url = inputUrl.trim();
		if (!url) {
			extractError = m.home_playlist_error_paste_url();
			return;
		}
		if (!isValidVideoUrl(url)) {
			extractError = m.home_playlist_error_invalid_url();
			return;
		}
		if (!isPlaylistUrl(url)) {
			extractError = m.home_playlist_error_use_playlist_url();
			return;
		}

		extractError = '';
		extractResult = null;
		selectedAudioStream = null;
		currentDownload.update((state) => ({ ...state, selectedStream: null, error: null }));
		playlistCompletionNotified = false;
		stagedEntries = [];
		resetWorkerState();
		resetBatch();
		startBatch();
		playlistPhase = 'fetching';

		const es = subscribeBatch(
			url,
			(data) => {
				if (data.type === 'link') {
					if (!data.videoId || !data.title) return;

					const entry: QueueEntry = {
						videoId: data.videoId,
						title: data.title,
						thumbnail: data.thumbnail
					};

					pushStagedEntry(entry);
					addBatchItem({
						videoId: entry.videoId,
						title: entry.title,
						thumbnail: entry.thumbnail,
						status: 'pending'
					});

					if (data.index != null && data.total != null) {
						setBatchProgress(data.index, data.total);
					}
					return;
				}

				if (data.type === 'progress') {
					if (data.current != null && data.total != null) {
						setBatchProgress(data.current, data.total);
					}
					return;
				}

				if (data.type === 'done') {
					es.close();
					completeBatch();
					playlistPhase = stagedEntries.length > 0 ? 'ready' : 'idle';
					if (stagedEntries.length === 0) {
						extractError = m.home_playlist_error_no_videos();
					}
					return;
				}

				if (data.type === 'error') {
					extractError = data.message || m.home_playlist_error_process_failed();
					es.close();
					completeBatch();
					playlistPhase = stagedEntries.length > 0 ? 'ready' : 'idle';
				}
			},
			() => {
				extractError = m.home_playlist_error_sse_failed();
				completeBatch();
				playlistPhase = stagedEntries.length > 0 ? 'ready' : 'idle';
			}
		);

		setEventSource(es);
		queueMicrotask(() => {
			document
				.getElementById('playlist-panel')
				?.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
		});
	}

	function handleStartPlaylistDownload(): void {
		if (playlistPhase === 'downloading') return;

		if (stagedEntries.length === 0) {
			extractError = m.home_playlist_error_empty();
			return;
		}

		const selectedVideoIds = new Set(
			$batchQueue.filter((item) => item.selected !== false).map((item) => item.videoId)
		);
		const selectedEntries = stagedEntries.filter((entry) => selectedVideoIds.has(entry.videoId));
		if (selectedEntries.length === 0) {
			extractError = m.home_playlist_error_select_one();
			return;
		}

		extractError = '';
		playlistCompletionNotified = false;
		resetWorkerState();
		setPreferredDownloadMode(selectedDownloadMode);
		setPreferredQuality(selectedQuality);
		persistPlaylistDownloadMode(selectedDownloadMode);
		persistPlaylistQuality(selectedQuality);
		startBatch();
		playlistPhase = 'downloading';

		for (const entry of selectedEntries) {
			enqueueDownload(entry);
		}
	}

	async function handlePickPlaylistDirectory(): Promise<void> {
		dirPicked = await pickSaveDirectory();
		if (!dirPicked) {
			extractError = m.home_playlist_error_folder_picker();
		}
	}

	function handleCancelPlaylist(): void {
		playlistModeEnabled = false;
		resetPlaylistComposer();
		extractError = '';
	}

	async function handlePrimarySubmit(event?: SubmitEvent): Promise<void> {
		event?.preventDefault();
		if (playlistModeEnabled) {
			await handleFetchPlaylist();
			return;
		}
		await handleFetch();
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
		if (!browser) return;
		window.localStorage.setItem('fetchtube-theme', isDarkMode ? 'dark' : 'light');
		broadcastThemeChange();
	}

	function openAuthModal(): void {
		void ensureAuthModalLoaded();
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
		await refreshAuthUser();
		await goto(target, { invalidateAll: true, replaceState: true });
	}
</script>

<svelte:head>
	<title>{m.home_meta_title()}</title>
	<meta name="description" content={m.home_meta_description()} />

	{#each HOMEPAGE_HREFLANG_LINKS as link}
		<link rel="alternate" hreflang={link.hreflang} href={link.href} />
	{/each}

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

		.page-root.theme-dark [class*='text-plum/'],
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

		.page-root.theme-dark #download-options {
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

		.defer-render-how-it-works {
			content-visibility: auto;
			contain-intrinsic-size: 580px;
		}

		.defer-render-tools {
			content-visibility: auto;
			contain-intrinsic-size: 760px;
		}

		.defer-render-testimonials {
			content-visibility: auto;
			contain-intrinsic-size: 640px;
		}

		.playlist-mode-switch {
			margin: 0 auto 1rem;
			display: inline-flex;
			align-items: center;
			gap: 0.25rem;
			border: 1px solid rgba(255, 77, 140, 0.18);
			border-radius: 999px;
			background: rgba(255, 255, 255, 0.96);
			padding: 0.3rem;
			box-shadow: 0 16px 28px -22px rgba(45, 27, 54, 0.65);
		}

		.playlist-mode-option {
			display: inline-flex;
			min-width: 8rem;
			align-items: center;
			justify-content: center;
			gap: 0.4rem;
			border: 0;
			border-radius: 999px;
			background: transparent;
			padding: 0.58rem 1rem;
			font-size: 0.78rem;
			font-weight: 800;
			color: #7c2d12;
			transition:
				transform 160ms ease,
				background-color 160ms ease,
				color 160ms ease,
				opacity 160ms ease;
		}

		.playlist-mode-option:hover:not(:disabled) {
			transform: translateY(-1px);
		}

		.playlist-mode-option:disabled {
			cursor: not-allowed;
			opacity: 0.6;
		}

		.playlist-mode-option-active {
			background: linear-gradient(135deg, #ff4d8c, #ffb938);
			color: #fff;
			box-shadow: 0 12px 24px -18px rgba(255, 77, 140, 0.9);
		}

		.playlist-panel-close {
			border: 1px solid #fbcfe8;
			border-radius: 999px;
			background: #fff;
			padding: 0.55rem 0.9rem;
			font-size: 0.74rem;
			font-weight: 800;
			color: #be185d;
		}

		.playlist-panel-close:disabled {
			opacity: 0.5;
			cursor: not-allowed;
		}

		.playlist-field {
			display: flex;
			flex-direction: column;
			gap: 0.45rem;
		}

		.playlist-field span {
			font-size: 0.72rem;
			font-weight: 800;
			text-transform: uppercase;
			letter-spacing: 0.08em;
			color: rgba(45, 27, 54, 0.66);
		}

		.playlist-field select {
			height: 2.8rem;
			border: 1px solid #fbcfe8;
			border-radius: 1rem;
			background: #fff;
			padding: 0 0.9rem;
			font-size: 0.92rem;
			font-weight: 700;
			color: #2d1b36;
		}

		.playlist-field select:disabled {
			opacity: 0.55;
			cursor: not-allowed;
		}

		.playlist-primary-btn,
		.playlist-ghost-btn {
			display: inline-flex;
			align-items: center;
			justify-content: center;
			border-radius: 999px;
			padding: 0.8rem 1.15rem;
			font-size: 0.82rem;
			font-weight: 800;
			letter-spacing: 0.01em;
		}

		.playlist-primary-btn {
			border: 0;
			background: linear-gradient(135deg, #ff4d8c, #ffb938);
			color: #fff;
			box-shadow: 0 18px 30px -22px rgba(255, 77, 140, 0.8);
		}

		.playlist-ghost-btn {
			border: 1px solid #fbcfe8;
			background: #fff;
			color: #9d174d;
		}

		.playlist-result-meta {
			gap: 1rem;
		}

		.playlist-result-thumbnail img {
			display: block;
		}

		.playlist-result-summary {
			display: flex;
			flex-direction: column;
			gap: 0.35rem;
		}

		.playlist-summary-line {
			display: flex;
			align-items: center;
			gap: 0.55rem;
			font-weight: 600;
			color: #475569;
		}

		.playlist-summary-line span:last-child {
			min-width: 0;
			flex: 1;
		}

		.playlist-result-toolbar {
			background: linear-gradient(180deg, rgba(248, 250, 252, 0.92), rgba(255, 255, 255, 0.98));
		}

		.playlist-fetch-progress {
			box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.6);
		}

		.playlist-result-control-grid {
			align-items: end;
		}

		.playlist-result-action-group {
			justify-content: flex-start;
		}

		.playlist-result-list {
			background: linear-gradient(180deg, rgba(248, 250, 252, 0.72), rgba(255, 255, 255, 0.96));
			min-height: 0;
		}

		.page-root.theme-dark .playlist-mode-switch {
			background: rgba(28, 29, 40, 0.94);
			border-color: rgba(255, 77, 140, 0.3);
		}

		.page-root.theme-dark .playlist-mode-option {
			color: #fbcfe8;
		}

		.page-root.theme-dark .playlist-mode-option-active {
			background: linear-gradient(135deg, #ff4d8c, #ffb938);
			color: #fff;
		}

		.page-root.theme-dark .playlist-panel-close,
		.page-root.theme-dark .playlist-ghost-btn,
		.page-root.theme-dark .playlist-field select {
			background: rgba(255, 255, 255, 0.04);
			border-color: rgba(255, 77, 140, 0.22);
			color: #fbcfe8;
		}

		.page-root.theme-dark .playlist-field span {
			color: rgba(224, 208, 245, 0.72);
		}

		.page-root.theme-dark .playlist-result-toolbar {
			background: linear-gradient(180deg, rgba(20, 21, 30, 0.9), rgba(16, 17, 26, 0.92));
			border-bottom-color: rgba(255, 77, 140, 0.2) !important;
		}

		.page-root.theme-dark .playlist-result-list {
			background: rgba(19, 20, 29, 0.92) !important;
		}

		.page-root.theme-dark .playlist-result-summary {
			background: rgba(255, 255, 255, 0.05) !important;
			border-color: rgba(255, 77, 140, 0.16) !important;
		}

		.page-root.theme-dark .playlist-summary-line {
			color: rgba(224, 208, 245, 0.84) !important;
		}

		@media (max-width: 768px) {
			.playlist-mode-switch {
				width: 100%;
				max-width: 22rem;
			}

			.playlist-mode-option {
				flex: 1;
				min-width: 0;
			}

			.playlist-result-action-group {
				width: 100%;
			}

			.playlist-summary-line {
				align-items: flex-start;
			}
		}
	</style>
</svelte:head>

<div class="page-root bg-bg-page min-h-screen flex flex-col overflow-x-hidden text-plum selection:bg-primary/20" class:theme-dark={isDarkMode} class:theme-light={!isDarkMode}>
	<SiteHeader
		{authUser}
		onOpenAuthModal={openAuthModal}
		homeHref="#home"
		howItWorksHref="#how-it-works"
		toolsHref="#download-options"
	/>

	<main class="flex-1 w-full">
		<section class="relative pt-12 pb-8 px-6 overflow-visible" id="home">
			<div class="hero-orb absolute top-[10%] left-[5%] w-24 h-24 rounded-full bg-accent/20 blur-xl animate-bob"></div>
			<div class="hero-orb absolute bottom-[20%] right-[10%] w-32 h-32 rounded-3xl rotate-12 bg-primary/10 blur-xl animate-bob-delayed"></div>
			<div class="hero-orb absolute top-[20%] right-[15%] w-16 h-16 rounded-full bg-secondary/30 blur-lg animate-bob"></div>
			<div class="relative z-10 w-full max-w-7xl mx-auto">
				<div class="mx-auto w-full max-w-4xl text-center">
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

					<div
						class="playlist-mode-switch"
						role="tablist"
						aria-label={m.home_download_mode_aria()}
					>
						<button
							type="button"
							class={`playlist-mode-option ${!playlistModeEnabled ? 'playlist-mode-option-active' : ''}`}
							onclick={() => setPlaylistMode(false)}
							aria-pressed={!playlistModeEnabled}
							disabled={playlistBusy || isExtracting}
						>
							<AppIcon name="smart_display" class="text-[16px]" />
							<span>{m.home_playlist_mode_single()}</span>
						</button>
						<button
							type="button"
							class={`playlist-mode-option ${playlistModeEnabled ? 'playlist-mode-option-active' : ''}`}
							onclick={() => setPlaylistMode(true)}
							aria-pressed={playlistModeEnabled}
							disabled={playlistBusy || isExtracting}
						>
							<AppIcon name="playlist_play" class="text-[16px]" />
							<span>{m.home_playlist_mode_playlist()}</span>
						</button>
					</div>

					<form
						id="download-options"
						class="relative mx-auto mb-9 w-full max-w-[700px] group lg:mb-5"
						onsubmit={handlePrimarySubmit}
					>
						<div class="absolute -inset-1 rounded-full bg-gradient-to-r from-primary to-secondary blur opacity-25 transition duration-500 group-hover:opacity-50"></div>
						<div class="relative flex h-[64px] items-center rounded-full bg-white p-2 shadow-float transition-all duration-300 group-focus-within:ring-4 group-focus-within:ring-primary/20">
							<div class="pl-6 text-plum/30">
								<AppIcon name="link" class="text-2xl" />
							</div>
							<input
								id="video-url-input"
								class="h-full w-full border-none bg-transparent px-4 text-lg font-semibold text-plum placeholder:text-muted/50 focus:ring-0 md:text-xl"
								placeholder={playlistModeEnabled ? m.home_playlist_input_placeholder() : m.home_input_placeholder()}
								type="text"
								bind:value={inputUrl}
								disabled={isExtracting || playlistBusy}
							/>
							<button
								class="absolute right-1.5 top-1.5 bottom-1.5 flex items-center justify-center rounded-full bg-gradient-primary px-3 text-sm font-bold text-white shadow-candy transition-all hover:scale-105 hover:brightness-110 active:scale-95 disabled:cursor-not-allowed disabled:opacity-60 disabled:hover:scale-100 md:gap-2 md:px-10 md:text-lg"
								type="submit"
								disabled={isExtracting || playlistBusy}
							>
								<span class="hidden md:inline">
									{isExtracting || playlistBusy
										? m.home_button_fetching()
										: m.home_button_fetch()}
								</span>
								<AppIcon
									name={isExtracting || playlistBusy ? 'progress_activity' : 'bolt'}
									class="text-base font-bold md:text-lg"
								/>
							</button>
						</div>

						{#if extractError}
							<p class="mt-3 text-sm font-bold text-red-500">{extractError}</p>
						{/if}

						{#if isExtracting}
							<p class="mt-3 inline-flex items-center gap-2 rounded-full bg-white/90 px-4 py-2 text-sm font-bold text-primary shadow-sm">
								<AppIcon name="progress_activity" class="animate-spin text-base" />
								{m.home_analyzing_link()}
							</p>
						{/if}

						<div class="mt-6 flex flex-wrap justify-center gap-3 opacity-80 transition-opacity hover:opacity-100">
							<div class="flex items-center gap-2 rounded-xl border border-white/50 bg-white/60 px-3 py-1.5">
								<AppIcon name="check_circle" class="text-lg text-green-500" />
								<span class="text-xs font-bold text-plum/70">{m.home_chip_ad_free()}</span>
							</div>
							<div class="flex items-center gap-2 rounded-xl border border-white/50 bg-white/60 px-3 py-1.5">
								<AppIcon name="verified_user" class="text-lg text-blue-500" />
								<span class="text-xs font-bold text-plum/70">{m.home_chip_safe_secure()}</span>
							</div>
							<div class="flex items-center gap-2 rounded-xl border border-white/50 bg-white/60 px-3 py-1.5">
								<AppIcon name="rocket_launch" class="text-lg text-purple-500" />
								<span class="text-xs font-bold text-plum/70">{m.home_chip_super_fast()}</span>
							</div>
						</div>
					</form>
				</div>

				{#if showPlaylistPanel}
					<section class="py-5 lg:py-3" id="playlist-panel">
						<div class="download-result-card playlist-result-card mx-auto max-w-7xl rounded-[2rem] border border-indigo-50 bg-white shadow-card overflow-hidden flex flex-col lg:flex-row">
							<div class="download-result-meta playlist-result-meta w-full lg:w-[40%] p-6 md:p-7 flex flex-col gap-4 bg-gradient-to-b from-indigo-50/50 to-white lg:border-r border-indigo-50">
								<div class="playlist-result-thumbnail relative w-full aspect-video rounded-3xl overflow-hidden shadow-lg border-4 border-white bg-slate-100">
									{#if playlistLeadThumbnail}
										<img class="absolute inset-0 h-full w-full object-cover" src={playlistLeadThumbnail} alt={playlistStageTitle}/>
									{:else}
										<div class="absolute inset-0 grid place-items-center text-slate-400">
											<AppIcon name="queue_music" class="text-6xl" />
										</div>
									{/if}
									<div class="absolute inset-0 bg-gradient-to-t from-black/65 via-black/10 to-transparent"></div>
									<div class="absolute left-4 top-4 rounded-full border border-white/20 bg-black/55 px-3 py-1.5 text-[11px] font-bold uppercase tracking-[0.14em] text-white backdrop-blur-md">
										{m.home_playlist_panel_label()}
									</div>
									<div class="absolute bottom-4 right-4 inline-flex items-center gap-2 rounded-full border border-white/20 bg-black/60 px-3 py-1.5 text-xs font-bold text-white backdrop-blur-md">
										<AppIcon name="queue_music" class="text-[15px]" />
										{playlistReadyCount}
									</div>
								</div>
								<div class="space-y-2 text-left">
									<h3 class="download-result-title text-2xl md:text-3xl font-bold text-slate-900 leading-tight">{playlistStageTitle}</h3>
									<p class="text-sm font-semibold leading-relaxed text-slate-600">{playlistStageDescription}</p>
								</div>
								<div class="flex flex-wrap items-center gap-3">
									<div class="download-result-chip-duration flex items-center gap-2 bg-indigo-50 px-3 py-1.5 rounded-full text-indigo-600 text-sm font-bold">
										<AppIcon name="queue_music" class="text-[18px]" />
										{playlistReadyCount}
									</div>
									<div class="download-result-chip-view flex items-center gap-2 bg-pink-50 px-3 py-1.5 rounded-full text-pink-600 text-sm font-bold">
										<AppIcon name="task_alt" class="text-[18px]" />
										{m.playlist_progress_selected({ count: String(playlistSelectedCount) })}
									</div>
									<div class="download-result-chip-channel flex items-center gap-2 bg-slate-100 px-3 py-1.5 rounded-full text-slate-600 text-sm font-bold">
										<AppIcon
											name={selectedDownloadMode === 'audio'
												? 'graphic_eq'
												: selectedDownloadMode === 'video-only'
													? 'movie_edit'
													: 'movie'}
											class="text-[18px]"
										/>
										{playlistDownloadModeLabel}
									</div>
									{#if selectedDownloadMode !== 'audio'}
										<div class="download-result-chip-channel flex items-center gap-2 bg-slate-100 px-3 py-1.5 rounded-full text-slate-600 text-sm font-bold">
											<AppIcon name="high_quality" class="text-[18px]" />
											{playlistQualityLabel}
										</div>
									{/if}
								</div>
								<div class="download-result-description playlist-result-summary p-4 rounded-2xl border border-slate-100 bg-slate-50 text-sm text-slate-600 leading-relaxed">
									<span class="font-bold text-slate-800">{m.home_playlist_mode_playlist()}</span>
									<div class="mt-2 flex flex-col gap-2">
										<div class="playlist-summary-line">
											<AppIcon name="link" class="text-[16px]" />
											<span class="truncate">{inputUrl}</span>
										</div>
										{#if fsaaSupported}
											<div class="playlist-summary-line">
												<AppIcon name="folder_open" class="text-[16px]" />
												<span>{dirPicked ? m.home_playlist_save_folder_selected() : m.home_playlist_choose_save_folder()}</span>
											</div>
										{/if}
									</div>
								</div>
								<p class="download-result-hint text-slate-500 font-semibold">
									{playlistPhase === 'downloading' ? m.home_playlist_stage_downloading_desc() : m.home_choose_format_hint()}
								</p>
							</div>
							<div class="download-result-actions playlist-result-actions flex-1 flex flex-col bg-white">
								<div class="playlist-result-toolbar p-5 md:p-6 border-b border-indigo-50">
									<div class="flex flex-col gap-4">
										<div class="flex flex-wrap items-start justify-between gap-3">
											<div class="space-y-1 text-left">
												<p class="text-[11px] font-bold uppercase tracking-[0.16em] text-primary">{m.home_playlist_mode_playlist()}</p>
												<h4 class="text-xl font-bold text-slate-900">{playlistStageTitle}</h4>
											</div>
											<button
												type="button"
												class="playlist-panel-close playlist-result-reset"
												onclick={handleCancelPlaylist}
												disabled={playlistPhase === 'fetching'}
											>
												{m.home_playlist_reset()}
											</button>
										</div>

										{#if playlistPhase === 'fetching'}
											<div class="playlist-fetch-progress rounded-[1.5rem] border border-pink-100 bg-indigo-50/60 p-4">
												<div class="h-2 overflow-hidden rounded-full bg-pink-100">
													<div
														class="h-full rounded-full bg-gradient-primary transition-all duration-300"
														style:width={`${playlistDiscoveryPercent}%`}
													></div>
												</div>
												<div class="mt-3 flex items-center justify-between text-xs font-bold text-plum/65">
													<span>{m.home_playlist_discovery_reading()}</span>
													<span>{playlistDiscoveryPercent}%</span>
												</div>
											</div>
										{:else if playlistPhase === 'ready' || playlistPhase === 'downloading'}
											<div class="playlist-result-control-grid grid gap-4 xl:grid-cols-[minmax(0,1fr)_minmax(0,1fr)_auto]">
												<label class="playlist-field">
													<span>{m.home_playlist_download_type()}</span>
													<select bind:value={selectedDownloadMode} disabled={playlistPhase === 'downloading'}>
														{#each PLAYLIST_DOWNLOAD_MODE_OPTIONS as option}
															<option value={option.value}>{option.label}</option>
														{/each}
													</select>
												</label>

												<label class="playlist-field">
													<span>{m.home_playlist_preferred_quality()}</span>
													<select bind:value={selectedQuality} disabled={selectedDownloadMode === 'audio' || playlistPhase === 'downloading'}>
														{#each PLAYLIST_QUALITY_OPTIONS as option}
															<option value={option.value}>{option.label}</option>
														{/each}
													</select>
												</label>

												<div class="playlist-result-action-group flex flex-wrap items-end gap-3">
													{#if fsaaSupported}
														<button type="button" class="playlist-ghost-btn" onclick={() => void handlePickPlaylistDirectory()}>
															{dirPicked ? m.home_playlist_save_folder_selected() : m.home_playlist_choose_save_folder()}
														</button>
													{/if}
													{#if playlistPhase === 'ready'}
														<button type="button" class="playlist-primary-btn" onclick={handleStartPlaylistDownload}>
															{m.home_playlist_start_download()}
														</button>
													{/if}
												</div>
											</div>
										{/if}
									</div>
								</div>

								<div class="playlist-result-list p-5 md:p-6 bg-white">
									<BatchProgress />
								</div>
							</div>
						</div>
					</section>
				{/if}
			</div>
		</section>

		{#if isExtracting}
			<section class="py-6 lg:py-4 px-6 lg:px-20 lg:-mt-4" id="download-results">
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
										<AppIcon name="movie" class="text-6xl" />
									</div>
								{/if}
								<div class="absolute inset-0 bg-gradient-to-t from-black/40 to-transparent"></div>
								<div class="absolute bottom-4 right-4 bg-black/60 backdrop-blur-md px-3 py-1.5 rounded-full text-xs font-bold text-white border border-white/20 flex items-center gap-1">
									<AppIcon name="progress_activity" class="animate-spin text-sm" />
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
			<section class="py-5 lg:py-3 px-6 lg:px-20 lg:-mt-5" id="download-results">
				<div class="max-w-7xl mx-auto">
					<div class="download-result-card bg-white rounded-[2rem] shadow-card border border-indigo-50 overflow-hidden flex flex-col lg:flex-row">
						<div class="download-result-meta w-full lg:w-[42%] p-6 md:p-7 flex flex-col gap-4 bg-gradient-to-b from-indigo-50/50 to-white lg:border-r border-indigo-50">
							<div class="relative w-full aspect-video rounded-3xl overflow-hidden shadow-lg border-4 border-white bg-slate-100">
								{#if extractResult.thumbnail}
									<img class="absolute inset-0 w-full h-full object-cover" src={extractResult.thumbnail} alt={extractResult.title}/>
								{:else}
									<div class="absolute inset-0 grid place-items-center text-slate-400">
										<AppIcon name="movie" class="text-6xl" />
									</div>
								{/if}
								<div class="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent"></div>
								<div class="absolute bottom-4 right-4 bg-black/60 backdrop-blur-md px-3 py-1.5 rounded-full text-xs font-bold text-white border border-white/20">{m.home_video_available()}</div>
							</div>
							<h3 class="download-result-title text-2xl md:text-3xl font-bold text-slate-900 leading-tight">{extractResult.title}</h3>
							<div class="flex flex-wrap items-center gap-3">
								<div class="download-result-chip-duration flex items-center gap-2 bg-indigo-50 px-3 py-1.5 rounded-full text-indigo-600 text-sm font-bold">
									<AppIcon name="schedule" class="text-[18px]" />
									{formatDuration(extractResult.duration)}
								</div>
								{#if extractResult.viewCount}
									<div class="download-result-chip-view flex items-center gap-2 bg-pink-50 px-3 py-1.5 rounded-full text-pink-600 text-sm font-bold">
										<AppIcon name="visibility" class="text-[18px]" />
										{formatViews(extractResult.viewCount)}
									</div>
								{/if}
								{#if extractResult.channel}
									<div class="download-result-chip-channel flex items-center gap-2 bg-slate-100 px-3 py-1.5 rounded-full text-slate-600 text-sm font-bold">
										<AppIcon name="person" class="text-[18px]" />
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
							<div class="p-5 md:p-6 pb-4 md:pb-5">
								<FormatPicker streams={extractResult.streams} onSelect={handleFormatSelect}/>
							</div>
							<div class="download-result-download p-5 pt-4 md:p-6 md:pt-5 border-t border-indigo-50 bg-indigo-50/20">
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

		<section class="defer-render-how-it-works py-8 px-6 lg:px-20 relative z-20" id="how-it-works">
			<div class="max-w-6xl mx-auto">
				<div class="grid grid-cols-1 md:grid-cols-3 gap-6 relative">
					<div class="relative group">
						<div class="bg-white p-6 rounded-[2rem] shadow-sm border border-pink-50 hover:shadow-float transition-all duration-300 h-full flex flex-col items-center text-center relative overflow-hidden">
							<div class="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-primary to-transparent opacity-50"></div>
							<div class="w-16 h-16 rounded-full bg-primary/10 flex items-center justify-center mb-4 group-hover:scale-110 transition-transform duration-300">
								<AppIcon name="search" class="text-3xl text-primary" />
							</div>
							<h3 class="text-xl font-bold text-plum mb-2">{m.home_step1_title()}</h3>
							<p class="text-plum/75 text-sm font-medium leading-snug">{m.home_step1_desc()}</p>
						</div>
						<div class="md:hidden flex justify-center py-2 text-plum/20">
							<AppIcon name="arrow_downward" />
						</div>
					</div>
					<div class="relative group md:-translate-y-4">
						<div class="step-paste-card bg-gradient-to-br from-white to-pink-50 p-6 rounded-[2rem] shadow-candy border border-pink-100 hover:-translate-y-1 transition-all duration-300 h-full flex flex-col items-center text-center relative overflow-hidden z-10">
							<div class="absolute -right-10 -top-10 w-24 h-24 bg-secondary/10 rounded-full blur-xl"></div>
							<div class="w-16 h-16 rounded-full bg-secondary text-white shadow-lg flex items-center justify-center mb-4 animate-bob">
								<AppIcon name="content_paste_go" class="text-3xl" />
							</div>
							<h3 class="text-xl font-bold text-plum mb-2">{m.home_step2_title()}</h3>
							<p class="text-plum/75 text-sm font-medium leading-snug">{m.home_step2_desc()}</p>
						</div>
						<div class="md:hidden flex justify-center py-2 text-plum/20">
							<AppIcon name="arrow_downward" />
						</div>
					</div>
					<div class="relative group">
						<div class="bg-white p-6 rounded-[2rem] shadow-sm border border-pink-50 hover:shadow-float transition-all duration-300 h-full flex flex-col items-center text-center relative overflow-hidden">
							<div class="absolute top-0 right-0 w-full h-1 bg-gradient-to-l from-accent to-transparent opacity-50"></div>
							<div class="w-16 h-16 rounded-full bg-accent/10 flex items-center justify-center mb-4 group-hover:scale-110 transition-transform duration-300">
								<AppIcon name="download_for_offline" class="text-3xl text-accent" />
							</div>
							<h3 class="text-xl font-bold text-plum mb-2">{m.home_step3_title()}</h3>
							<p class="text-plum/75 text-sm font-medium leading-snug">{m.home_step3_desc()}</p>
						</div>
					</div>
				</div>
			</div>
		</section>

		<section class="defer-render-testimonials py-12 px-6 lg:px-20 relative overflow-hidden">
			<div class="absolute top-0 left-0 w-full h-full overflow-hidden pointer-events-none">
				<div class="absolute top-20 left-[10%] w-64 h-64 bg-primary/5 rounded-full blur-3xl"></div>
				<div class="absolute bottom-10 right-[10%] w-64 h-64 bg-secondary/5 rounded-full blur-3xl"></div>
			</div>
			<div class="max-w-7xl mx-auto relative z-10">
				<div class="flex flex-col md:flex-row items-center gap-8 mb-10">
					<div class="flex-1 text-center md:text-left">
						<span class="inline-block py-1 px-3 rounded-full bg-green-100 text-green-700 font-bold text-xs uppercase mb-3 tracking-wider">{m.home_testimonials_badge()}</span>
						<h2 class="text-3xl md:text-4xl font-bold text-plum mb-3">{m.home_testimonials_title()}</h2>
						<p class="text-base text-plum/75 font-semibold max-w-md mx-auto md:mx-0">{m.home_testimonials_subtitle()}</p>
						<div class="hidden md:flex mt-6 -space-x-3">
								<div class="w-10 h-10 rounded-full border-2 border-white bg-gray-200 flex items-center justify-center overflow-hidden" title={m.home_testimonial_user_avatar({ index: '1' })}><img alt={m.home_testimonial_user_avatar({ index: '1' })} class="w-full h-full object-cover" loading="lazy" decoding="async" width="40" height="40" src={user1Avatar}/></div>
								<div class="w-10 h-10 rounded-full border-2 border-white bg-gray-200 flex items-center justify-center overflow-hidden" title={m.home_testimonial_user_avatar({ index: '2' })}><img alt={m.home_testimonial_user_avatar({ index: '2' })} class="w-full h-full object-cover" loading="lazy" decoding="async" width="40" height="40" src={user2Avatar}/></div>
								<div class="w-10 h-10 rounded-full border-2 border-white bg-gray-200 flex items-center justify-center overflow-hidden" title={m.home_testimonial_user_avatar({ index: '3' })}><img alt={m.home_testimonial_user_avatar({ index: '3' })} class="w-full h-full object-cover" loading="lazy" decoding="async" width="40" height="40" src={user3Avatar}/></div>
							<div class="w-10 h-10 rounded-full border-2 border-white bg-plum text-white text-xs font-bold flex items-center justify-center">+9k</div>
						</div>
					</div>
					<div class="flex-1 w-full grid grid-cols-1 sm:grid-cols-2 gap-4">
						<div class="bg-white p-5 rounded-2xl shadow-sm border border-pink-50 hover:shadow-float hover:-translate-y-1 transition-all duration-300">
							<div class="flex items-center gap-3 mb-3">
									<div class="w-8 h-8 rounded-full bg-purple-100 overflow-hidden"><img alt={m.home_testimonial_sarah_name()} class="w-full h-full object-cover" loading="lazy" decoding="async" width="32" height="32" src={sarahAvatar}/></div>
									<div>
										<p class="font-bold text-plum text-sm">{m.home_testimonial_sarah_name()}</p>
									<div class="flex text-yellow-400 text-[10px]">★★★★★</div>
								</div>
							</div>
							<p class="text-plum/75 font-medium text-xs leading-relaxed">{m.home_testimonial_sarah_quote()}</p>
						</div>
						<div class="bg-white p-5 rounded-2xl shadow-sm border border-pink-50 hover:shadow-float hover:-translate-y-1 transition-all duration-300 sm:translate-y-4">
							<div class="flex items-center gap-3 mb-3">
									<div class="w-8 h-8 rounded-full bg-blue-100 overflow-hidden"><img alt={m.home_testimonial_mike_name()} class="w-full h-full object-cover" loading="lazy" decoding="async" width="32" height="32" src={mikeAvatar}/></div>
									<div>
										<p class="font-bold text-plum text-sm">{m.home_testimonial_mike_name()}</p>
									<div class="flex text-yellow-400 text-[10px]">★★★★★</div>
								</div>
							</div>
							<p class="text-plum/75 font-medium text-xs leading-relaxed">{m.home_testimonial_mike_quote()}</p>
						</div>
						<div class="col-span-1 sm:col-span-2 mt-2 bg-gradient-to-r from-primary/10 to-secondary/10 p-4 rounded-2xl border border-white/50 flex items-center justify-between">
							<div class="flex-1">
								<p class="font-bold text-plum text-sm mb-1">{m.home_join_party_title()}</p>
								<p class="text-plum/75 text-xs">{m.home_join_party_subtitle()}</p>
							</div>
							<button class="bg-plum text-white font-bold text-xs px-5 py-2.5 rounded-full shadow-lg hover:bg-plum/90 hover:scale-105 active:scale-95 transition-all duration-300 flex items-center gap-2">
								<span>{m.home_join_party_cta()}</span>
								<AppIcon name="arrow_forward" class="text-sm" />
							</button>
							<div class="w-12 h-12 ml-4 animate-wiggle"><span class="text-4xl">🎉</span></div>
						</div>
					</div>
				</div>
			</div>
		</section>
		</main>

		{#if authModalOpen && AuthModalComponent}
			<AuthModalComponent
				open={authModalOpen}
				redirectTo={redirectTo}
				onClose={closeAuthModal}
				onSuccess={handleAuthSuccess}
			/>
		{/if}

	<footer class="bg-white border-t border-pink-100 py-6 px-6">
		<div class="max-w-7xl mx-auto flex flex-col md:flex-row justify-between items-center gap-4">
			<div class="flex items-center gap-2 transition-all">
				<AppIcon name="smart_toy" class="text-xl text-plum/75 grayscale hover:grayscale-0" />
				<span class="font-bold text-sm text-plum/90">{m.footer_copyright({ year: String(new Date().getFullYear()) })}</span>
			</div>
			<div class="flex gap-4 text-plum/80 font-semibold text-xs">
				<a class="underline-offset-2 hover:text-primary hover:underline transition-colors" href="/privacy">{m.footer_privacy_policy()}</a>
				<a class="underline-offset-2 hover:text-primary hover:underline transition-colors" href="/privacy">{m.footer_terms_of_service()}</a>
				<a class="underline-offset-2 hover:text-primary hover:underline transition-colors" href="/privacy#contact">{m.footer_contact()}</a>
			</div>
		</div>
	</footer>

	<button
		type="button"
		class="theme-toggle fixed bottom-5 right-5 z-[70] flex h-12 min-w-[120px] items-center justify-center gap-2 rounded-full px-4 text-sm font-bold shadow-xl hover:scale-105 active:scale-95 transition-all duration-300 backdrop-blur-md"
		onclick={toggleTheme}
		aria-label={isDarkMode ? m.common_theme_switch_to_light() : m.common_theme_switch_to_dark()}
	>
		<AppIcon name={isDarkMode ? 'light_mode' : 'dark_mode'} class="text-[18px]" />
		<span>{isDarkMode ? m.common_theme_light_mode() : m.common_theme_dark_mode()}</span>
	</button>
</div>
