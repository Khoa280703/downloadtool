<script lang="ts">
	import { getLocale, locales, localizeHref } from '$lib/paraglide/runtime';
	import * as m from '$lib/paraglide/messages';
	import '../app.css';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { browser } from '$app/environment';
	import AppIcon from '$components/AppIcon.svelte';
	import SiteHeader from '$components/SiteHeader.svelte';
	import CookieConsent from '$components/CookieConsent.svelte';
	import { initGA } from '$lib/analytics';

	let { children } = $props();

	type AuthUser = { name?: string | null; email: string; image?: string | null };
	type AuthModalComponentType = typeof import('$components/AuthModal.svelte').default;

	/** GA4 Measurement ID from env */
	const GA_MEASUREMENT_ID = import.meta.env.PUBLIC_GA_MEASUREMENT_ID || '';

	let isDarkMode = $state(false);
	let authModalOpen = $state(false);
	let AuthModalComponent = $state<AuthModalComponentType | null>(null);
	let authUser = $state<AuthUser | null | undefined>(undefined);
	let redirectTo = $state('/');

	function syncThemeFromStorage(): void {
		if (!browser) return;

		isDarkMode = window.localStorage.getItem('snapvie-theme') === 'dark';
	}

	function broadcastThemeChange(): void {
		if (!browser) return;
		window.dispatchEvent(new CustomEvent('snapvie-theme-change', { detail: { isDarkMode } }));
	}

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

	function hasExplicitLocalePrefix(pathname: string): boolean {
		return locales.some((locale) => {
			if (locale === 'en') return false;
			return pathname === `/${locale}` || pathname.startsWith(`/${locale}/`);
		});
	}

	function isLocalizedHomePath(pathname: string): boolean {
		if (pathname === '/') return true;
		return locales.some((locale) => {
			if (locale === 'en') return false;
			return pathname === `/${locale}` || pathname === `/${locale}/`;
		});
	}

	function isAdminPath(pathname: string): boolean {
		return pathname === '/admin' || pathname.startsWith('/admin/');
	}

	function isWideMarketingPath(pathname: string): boolean {
		return [
			'/download-youtube-8k-hdr',
			'/download-youtube-playlist',
			'/download-youtube-shorts',
			'/download-youtube-4k',
			'/download-youtube-mp3'
		].includes(pathname);
	}

	function isKnownLocale(value: string): value is (typeof locales)[number] {
		return (locales as readonly string[]).includes(value);
	}

	function applyPreferredLanguageRedirect(): boolean {
		if (!browser) return false;

		const preferred = window.localStorage.getItem('preferred-lang');
		if (!preferred || !isKnownLocale(preferred)) return false;
		if (hasExplicitLocalePrefix(window.location.pathname)) return false;

		const currentLocale = getLocale();
		if (preferred === currentLocale) return false;

		const currentHref = `${$page.url.pathname}${$page.url.search}${$page.url.hash}`;
		const targetHref = localizeHref(currentHref, { locale: preferred });
		const activeHref = `${window.location.pathname}${window.location.search}${window.location.hash}`;

		if (targetHref !== activeHref) {
			void goto(targetHref, { replaceState: true, noScroll: true, invalidateAll: false });
			return true;
		}

		return false;
	}

	onMount(() => {
		if (!browser) return;

		syncThemeFromStorage();
		applyPreferredLanguageRedirect();

		if (GA_MEASUREMENT_ID) {
			const startGA = () => initGA(GA_MEASUREMENT_ID);
			const requestIdleCallbackFn = (window as Window & {
				requestIdleCallback?: (cb: () => void, options?: { timeout: number }) => number;
			}).requestIdleCallback;

			if (requestIdleCallbackFn) {
				requestIdleCallbackFn(startGA, { timeout: 2500 });
			} else {
				window.setTimeout(startGA, 1200);
			}
		}

		let serviceWorkerMessageHandler: ((event: MessageEvent) => void) | null = null;

		// Register service worker (production only — skip in dev to avoid breaking HMR)
		if (import.meta.env.PROD && 'serviceWorker' in navigator) {
			navigator.serviceWorker.register('/service-worker.js').catch(() => {
				// SW registration failed silently — PWA features degrade gracefully
			});

			// Handle Background Fetch completion: trigger <a> download
			serviceWorkerMessageHandler = (event) => {
				if (event.data?.type === 'bg-fetch-complete' && event.data.url) {
					const anchor = document.createElement('a');

					anchor.href = event.data.url;
					anchor.download = event.data.title ?? 'video.mp4';
					anchor.click();
				}
			};

			navigator.serviceWorker.addEventListener('message', serviceWorkerMessageHandler);
		}

		// Homepage resolves auth in +page.svelte; avoid duplicate fetch here.
		if (!isLocalizedHomePath(window.location.pathname)) {
			void (async () => {
				await refreshAuthUser();

				const params = new URLSearchParams(window.location.search);

				if (params.get('auth') === 'required' && !authUser) {
					redirectTo = normalizeRedirectTo(params.get('redirectTo'));
					void ensureAuthModalLoaded();
					authModalOpen = true;
				}
			})();
		} else {
			authUser = null;
		}

		const storageHandler = (event: StorageEvent) => {
			if (event.key !== 'snapvie-theme') return;

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
		window.addEventListener('snapvie-theme-change', themeChangeHandler as EventListener);

		// Clipboard auto-read: detect YouTube URL when user returns to tab
		const visibilityHandler = async () => {
			if (document.visibilityState !== 'visible') return;
			if (!navigator.clipboard?.readText) return;

			try {
				const text = await navigator.clipboard.readText();

				if (text && (text.includes('youtube.com/watch') || text.includes('youtu.be/'))) {
					window.dispatchEvent(new CustomEvent('yturl-detected', { detail: { url: text } }));
				}
			} catch {
				// Clipboard permission denied — ignore silently
			}
		};

		document.addEventListener('visibilitychange', visibilityHandler);

			return () => {
				window.removeEventListener('storage', storageHandler);
				window.removeEventListener('snapvie-theme-change', themeChangeHandler as EventListener);
				document.removeEventListener('visibilitychange', visibilityHandler);

			if (serviceWorkerMessageHandler && 'serviceWorker' in navigator) {
				navigator.serviceWorker.removeEventListener('message', serviceWorkerMessageHandler);
			}
		};
	});

	// Keep layout theme in sync when navigating from pages that write localStorage in-tab.
	$effect(() => {
		if (!browser) return;

		$page.url.pathname;
		syncThemeFromStorage();
	});

	function toggleTheme(): void {
		isDarkMode = !isDarkMode;

		if (!browser) return;

		window.localStorage.setItem('snapvie-theme', isDarkMode ? 'dark' : 'light');
		broadcastThemeChange();
	}

	function openAuthModal(): void {
		redirectTo = $page.url.pathname;
		void ensureAuthModalLoaded();
		authModalOpen = true;
	}

	async function closeAuthModal(): Promise<void> {
		authModalOpen = false;

		const params = new URLSearchParams(window.location.search);

		if (params.get('auth') === 'required' && !authUser) {
			const homeHref = localizeHref('/', { locale: getLocale() });
			await goto(homeHref, { replaceState: true, noScroll: true, invalidateAll: false });
		}
	}

async function handleAuthSuccess(target: string): Promise<void> {
	authModalOpen = false;
	await refreshAuthUser();
	await goto(target, { invalidateAll: true, replaceState: true });
}
</script>

<svelte:head>
	<link
		rel="stylesheet"
		href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200&display=swap"
	/>
	<meta name="color-scheme" content="light dark" />
</svelte:head>

<div
	class="app page-root bg-bg-page min-h-screen flex flex-col overflow-x-hidden text-plum selection:bg-primary/20"
	class:theme-dark={isDarkMode}
	class:theme-light={!isDarkMode}
	class:admin-route={isAdminPath($page.url.pathname)}
>
	{#if !isLocalizedHomePath($page.url.pathname) && !isAdminPath($page.url.pathname)}
		<SiteHeader
			authUser={authUser}
			onOpenAuthModal={openAuthModal}
			homeHref="/#home"
			howItWorksHref="/#how-it-works"
			toolsHref="/#download-options"
		/>
	{/if}

	<main
		class={isLocalizedHomePath($page.url.pathname)
			? 'main-home'
			: isAdminPath($page.url.pathname)
				? 'main-admin'
				: isWideMarketingPath($page.url.pathname)
					? 'main-home'
					: 'main'}
	>
		{#key $page.url.pathname}
			{@render children()}
		{/key}
	</main>

	{#if !isLocalizedHomePath($page.url.pathname) && !isAdminPath($page.url.pathname)}
		<footer class="bg-white border-t border-pink-100 py-6 px-6">
			<div class="max-w-7xl mx-auto flex flex-col md:flex-row justify-between items-center gap-4 md:pr-40">
				<div class="flex items-center gap-2 transition-all">
					<img
						src="/logo.svg"
						alt="Snapvie"
						class="h-7 w-7 rounded-md object-contain"
						loading="eager"
						decoding="async"
					/>
					<span class="font-bold text-sm text-plum/90">{m.footer_copyright({ year: String(new Date().getFullYear()) })}</span>
				</div>
				<div class="flex gap-4 text-plum/80 font-semibold text-xs">
					<a class="underline-offset-2 hover:text-primary hover:underline transition-colors" href="/privacy">{m.footer_privacy_policy()}</a>
					<a class="underline-offset-2 hover:text-primary hover:underline transition-colors" href="/terms">{m.footer_terms_of_service()}</a>
					<a class="underline-offset-2 hover:text-primary hover:underline transition-colors" href="/contact">{m.footer_contact()}</a>
				</div>
			</div>
		</footer>

		{#if authModalOpen && AuthModalComponent}
			<AuthModalComponent
				open={authModalOpen}
				redirectTo={redirectTo}
				onClose={closeAuthModal}
				onSuccess={handleAuthSuccess}
			/>
		{/if}

		<button
			type="button"
			class="theme-toggle fixed bottom-4 right-4 z-[70] flex h-11 w-11 min-w-0 items-center justify-center rounded-full px-0 text-sm font-bold shadow-xl transition-all duration-300 backdrop-blur-md hover:scale-105 active:scale-95 sm:bottom-5 sm:right-5 sm:h-12 sm:w-auto sm:min-w-[120px] sm:gap-2 sm:px-4"
			onclick={toggleTheme}
			aria-label={isDarkMode ? m.common_theme_switch_to_light() : m.common_theme_switch_to_dark()}
		>
			<AppIcon name={isDarkMode ? 'light_mode' : 'dark_mode'} class="text-[18px]" />
			<span class="hidden sm:inline">{isDarkMode ? m.common_theme_light_mode() : m.common_theme_dark_mode()}</span>
		</button>
	{/if}
</div>

{#if !isAdminPath($page.url.pathname)}
	<CookieConsent />
{/if}

<style>
	:global(body) {
		margin: 0;
		font-family: 'Nunito', sans-serif;
		line-height: 1.5;
		min-height: 100vh;
	}

	.main {
		flex: 1;
		max-width: 1100px;
		width: 100%;
		margin: 0 auto;
		padding: 2rem 1.5rem 2.6rem;
	}

	.main-home {
		flex: 1;
		width: 100%;
		margin: 0;
		padding: 0;
	}

	.main-admin {
		flex: 1;
		width: 100%;
		max-width: none;
		margin: 0;
		padding: 0;
	}

	.page-root {
		position: relative;
		isolation: isolate;
		background-color: #fff5f9;
		color: #2d1b36;
		transition: background-color 220ms ease, color 220ms ease;
	}

	.page-root::before {
		content: '';
		position: absolute;
		inset: 0;
		z-index: 0;
		pointer-events: none;
		opacity: 0.88;
		filter: blur(18px);
		background-image:
			radial-gradient(circle at center, rgba(255, 185, 56, 0.14) 0, rgba(255, 185, 56, 0.14) 38%, transparent 70%),
			radial-gradient(circle at center, rgba(255, 77, 140, 0.12) 0, rgba(255, 77, 140, 0.12) 40%, transparent 72%),
			radial-gradient(circle at center, rgba(255, 185, 56, 0.1) 0, rgba(255, 185, 56, 0.1) 34%, transparent 66%);
		background-size: 18rem 18rem, 24rem 24rem, 14rem 14rem;
		background-position:
			6% 5rem,
			88% 18rem,
			78% 6rem;
		background-repeat: repeat-y;
	}

	.page-root.theme-dark {
		background-color: #12121a;
		color: #e0d0f5;
	}

	.page-root.theme-dark::before {
		opacity: 0.34;
		filter: blur(26px);
		background-image:
			radial-gradient(circle at center, rgba(255, 185, 56, 0.16) 0, rgba(255, 185, 56, 0.16) 38%, transparent 70%),
			radial-gradient(circle at center, rgba(255, 77, 140, 0.16) 0, rgba(255, 77, 140, 0.16) 40%, transparent 72%),
			radial-gradient(circle at center, rgba(129, 140, 248, 0.12) 0, rgba(129, 140, 248, 0.12) 34%, transparent 66%);
	}

	.page-root.admin-route,
	.page-root.theme-dark.admin-route {
		background: #f3f4f6;
		color: #0f172a;
	}

	:global(.glass-header) {
		background: rgba(255, 245, 249, 0.8);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
	}

	:global(.bg-gradient-primary) {
		background: linear-gradient(135deg, #ff4d8c 0%, #ffb938 100%);
	}

	.theme-toggle {
		background: rgba(45, 27, 54, 0.08);
		border: 1px solid rgba(45, 27, 54, 0.14);
		color: #2d1b36;
	}

	:global(.page-root.theme-dark .glass-header) {
		background: rgba(18, 18, 26, 0.7);
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.page-root.theme-dark .theme-toggle {
		background: rgba(255, 255, 255, 0.1);
		border-color: rgba(255, 255, 255, 0.12);
		color: #ffffff;
	}

	:global(.page-root.theme-dark .text-plum),
	:global(.page-root.theme-dark .text-text-main) {
		color: #ffffff !important;
	}

	:global(.page-root.theme-dark footer) {
		border-top-color: transparent !important;
		background: rgba(18, 18, 26, 0.72);
	}

	:global(.page-root.theme-dark footer a) {
		color: rgba(224, 208, 245, 0.55) !important;
	}

	:global(.page-root.theme-dark footer a:hover) {
		color: #ff4d8c !important;
	}

	@media (min-width: 768px) {
		.main {
			padding-bottom: 2rem;
		}
	}

	@media (max-width: 640px) {
		.main {
			padding: 1rem 0.9rem 1.7rem;
			padding-bottom: 2rem;
		}
	}
</style>
