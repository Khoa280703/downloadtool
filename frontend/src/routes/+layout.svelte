<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import favicon from '$lib/assets/favicon.svg';
	import CookieConsent from '$components/CookieConsent.svelte';
	import { initGA } from '$lib/analytics';
	import { browser } from '$app/environment';

	let { children } = $props();

	/** GA4 Measurement ID from env */
	const GA_MEASUREMENT_ID = import.meta.env.PUBLIC_GA_MEASUREMENT_ID || '';

	/** Deferred install prompt for PWA "Add to Home Screen" */
	let deferredInstallPrompt = $state<BeforeInstallPromptEvent | null>(null);
	let showInstallBtn = $state(false);

	onMount(() => {
		if (!browser) return;

		// GA
		if (GA_MEASUREMENT_ID) initGA(GA_MEASUREMENT_ID);

		// Register service worker (production only — skip in dev to avoid breaking HMR)
		if (import.meta.env.PROD && 'serviceWorker' in navigator) {
			navigator.serviceWorker.register('/service-worker.js').catch(() => {
				// SW registration failed silently — PWA features degrade gracefully
			});

			// Handle Background Fetch completion: trigger <a> download
			navigator.serviceWorker.addEventListener('message', (e) => {
				if (e.data?.type === 'bg-fetch-complete' && e.data.url) {
					const a = document.createElement('a');
					a.href = e.data.url;
					a.download = e.data.title ?? 'video.mp4';
					a.click();
				}
			});
		}

		// PWA install prompt
		const installHandler = (e: Event) => {
			e.preventDefault();
			deferredInstallPrompt = e as BeforeInstallPromptEvent;
			showInstallBtn = true;
		};
		window.addEventListener('beforeinstallprompt', installHandler);

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
			window.removeEventListener('beforeinstallprompt', installHandler);
			document.removeEventListener('visibilitychange', visibilityHandler);
		};
	});

	async function handleInstall() {
		if (!deferredInstallPrompt) return;
		await deferredInstallPrompt.prompt();
		deferredInstallPrompt = null;
		showInstallBtn = false;
	}
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<meta name="theme-color" content="#3b82f6" />
	<meta name="color-scheme" content="light dark" />

	{#if GA_MEASUREMENT_ID}
		<!-- Google Analytics 4 -->
		<script
			async
			src="https://www.googletagmanager.com/gtag/js?id={GA_MEASUREMENT_ID}"
		></script>
	{/if}
</svelte:head>

<div class="app">
	{#if $page.url.pathname !== '/'}
		<header class="header">
			<nav class="nav">
				<a href="/" class="logo">
					<svg viewBox="0 0 24 24" width="28" height="28" fill="currentColor">
						<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 14.5v-9l6 4.5-6 4.5z"/>
					</svg>
					<span>VideoDL</span>
				</a>
				{#if showInstallBtn}
					<button class="install-btn" onclick={handleInstall}>⬇ Install App</button>
				{/if}
			</nav>
		</header>
	{/if}

	<main class={$page.url.pathname === '/' ? 'main-home' : 'main'}>
		{@render children()}
	</main>

	{#if $page.url.pathname !== '/'}
		<footer class="footer">
			<p>&copy; {new Date().getFullYear()} VideoDL. Free video downloader.</p>
			<div class="footer-links">
				<a href="/privacy">Privacy Policy</a>
				<a href="#terms">Terms</a>
			</div>
		</footer>
	{/if}
</div>

<!-- Cookie Consent Banner -->
<CookieConsent />

<style>
	:global(*) {
		box-sizing: border-box;
		margin: 0;
		padding: 0;
	}

	:global(:root) {
		--primary-color: #3b82f6;
		--primary-hover: #2563eb;
		--primary-alpha: rgba(59, 130, 246, 0.1);
		--secondary-color: #8b5cf6;
		--secondary-hover: #7c3aed;
		--success-color: #22c55e;
		--success-hover: #16a34a;
		--error-color: #ef4444;
		--error-bg: rgba(239, 68, 68, 0.1);
		--text-color: #111827;
		--text-secondary: #6b7280;
		--bg-color: #ffffff;
		--card-bg: #f9fafb;
		--input-bg: #ffffff;
		--border-color: #e5e7eb;
		--shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	:global(body) {
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
		background: var(--bg-color);
		color: var(--text-color);
		line-height: 1.5;
		min-height: 100vh;
	}

	.app {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	.header {
		border-bottom: 1px solid var(--border-color);
		background: var(--bg-color);
		position: sticky;
		top: 0;
		z-index: 100;
	}

	.nav {
		max-width: 1200px;
		margin: 0 auto;
		padding: 1rem 1.5rem;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.logo {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: var(--primary-color);
		text-decoration: none;
		font-size: 1.25rem;
		font-weight: 700;
	}

	.install-btn {
		padding: 0.4rem 1rem;
		background: var(--primary-color);
		color: #fff;
		border: none;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
	}
	.install-btn:hover { background: var(--primary-hover); }

	.main {
		flex: 1;
		max-width: 800px;
		width: 100%;
		margin: 0 auto;
		padding: 2rem 1.5rem;
	}

	.main-home {
		flex: 1;
		width: 100%;
		margin: 0;
		padding: 0;
	}

	.footer {
		border-top: 1px solid var(--border-color);
		padding: 1.5rem;
		text-align: center;
		color: var(--text-secondary);
		font-size: 0.875rem;
		background: var(--bg-color);
	}

	.footer p {
		margin-bottom: 0.5rem;
	}

	.footer-links {
		display: flex;
		justify-content: center;
		gap: 1rem;
	}

	.footer-links a {
		color: var(--text-secondary);
		text-decoration: none;
	}

	.footer-links a:hover {
		color: var(--primary-color);
	}

	@media (min-width: 768px) {
		.main {
			padding-bottom: 2rem;
		}
	}

	@media (max-width: 640px) {
		.main {
			padding: 1rem;
			padding-bottom: 2rem;
		}

		.nav {
			padding: 0.75rem 1rem;
		}
	}

	@media (prefers-color-scheme: dark) {
		:global(:root) {
			--text-color: #f9fafb;
			--text-secondary: #9ca3af;
			--bg-color: #111827;
			--card-bg: #1f2937;
			--input-bg: #1f2937;
			--border-color: #374151;
		}
	}
</style>
