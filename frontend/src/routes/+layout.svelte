<script lang="ts">
	import { onMount } from 'svelte';
	import favicon from '$lib/assets/favicon.svg';
	import AdBanner from '$components/AdBanner.svelte';
	import CookieConsent from '$components/CookieConsent.svelte';
	import { initGA } from '$lib/analytics';
	import { browser } from '$app/environment';

	let { children } = $props();

	/** GA4 Measurement ID from env */
	const GA_MEASUREMENT_ID = import.meta.env.PUBLIC_GA_MEASUREMENT_ID || '';

	/** Enable banners flag */
	const enableBanners = import.meta.env.PUBLIC_ENABLE_BANNERS !== 'false';

	onMount(() => {
		if (browser && GA_MEASUREMENT_ID) {
			initGA(GA_MEASUREMENT_ID);
		}
	});
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
	<header class="header">
		<nav class="nav">
			<a href="/" class="logo">
				<svg viewBox="0 0 24 24" width="28" height="28" fill="currentColor">
					<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 14.5v-9l6 4.5-6 4.5z"/>
				</svg>
				<span>VideoDL</span>
			</a>
		</nav>

		{#if enableBanners}
			<!-- Desktop Header Banner -->
			<div class="header-ad-desktop">
				<AdBanner size="728x90" slot="header-desktop" network="adsterra" lazy={false} />
			</div>

			<!-- Mobile Header Banner -->
			<div class="header-ad-mobile">
				<AdBanner size="320x50" slot="header-mobile" network="adsterra" lazy={false} />
			</div>
		{/if}
	</header>

	<main class="main">
		{@render children()}
	</main>

	{#if enableBanners}
		<!-- Footer Banner (Mobile Sticky) -->
		<div class="footer-ad-mobile">
			<AdBanner size="320x50" slot="footer-mobile" network="adsterra" />
		</div>
	{/if}

	<footer class="footer">
		<p>&copy; {new Date().getFullYear()} VideoDL. Free video downloader.</p>
		<div class="footer-links">
			<a href="/privacy">Privacy Policy</a>
			<a href="#terms">Terms</a>
		</div>
	</footer>
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

	.header-ad-desktop {
		display: none;
		padding: 0.5rem 1.5rem;
		background: var(--card-bg);
		border-top: 1px solid var(--border-color);
	}

	.header-ad-mobile {
		display: block;
		padding: 0.5rem;
		background: var(--card-bg);
		border-top: 1px solid var(--border-color);
	}

	.main {
		flex: 1;
		max-width: 800px;
		width: 100%;
		margin: 0 auto;
		padding: 2rem 1.5rem;
		padding-bottom: 5rem; /* Space for mobile sticky ad */
	}

	.footer-ad-mobile {
		position: fixed;
		bottom: 0;
		left: 0;
		right: 0;
		background: var(--bg-color);
		border-top: 1px solid var(--border-color);
		padding: 0.5rem;
		z-index: 50;
		display: block;
	}

	.footer {
		border-top: 1px solid var(--border-color);
		padding: 1.5rem;
		text-align: center;
		color: var(--text-secondary);
		font-size: 0.875rem;
		background: var(--bg-color);
		margin-bottom: 70px; /* Space for mobile sticky ad */
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
		.header-ad-desktop {
			display: block;
		}

		.header-ad-mobile {
			display: none;
		}

		.footer-ad-mobile {
			display: none;
		}

		.main {
			padding-bottom: 2rem;
		}

		.footer {
			margin-bottom: 0;
		}
	}

	@media (max-width: 640px) {
		.main {
			padding: 1rem;
			padding-bottom: 5rem;
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
