<script lang="ts">
	import { onMount } from 'svelte';
	import { consent } from '$stores/consent';

	/**
	 * Ad size variants supported by AdsTerra and other networks
	 */
	type AdSize = '728x90' | '320x50' | '300x250' | '160x600' | '300x600';

	interface Props {
		/** Ad slot identifier from ad network */
		slot?: string;
		/** Ad size format - determines container dimensions */
		size?: AdSize;
		/** Custom CSS class */
		class?: string;
		/** Lazy load ad - only load when in viewport */
		lazy?: boolean;
		/** Ad network type */
		network?: 'adsterra' | 'propellerads' | 'adsense';
	}

	let {
		slot = 'default',
		size = '300x250',
		class: className = '',
		lazy = true,
		network = 'adsterra'
	}: Props = $props();

	/** Container mounted state - prevents SSR hydration issues */
	let mounted = $state(false);

	/** Ad loaded state */
	let adLoaded = $state(false);

	/** Intersection observer for lazy loading */
	let containerRef: HTMLDivElement | undefined = $state(undefined);

	/**
	 * Get dimensions based on ad size
	 */
	function getDimensions(sz: AdSize): { width: number; height: number; class: string } {
		const dims: Record<AdSize, { width: number; height: number; class: string }> = {
			'728x90': { width: 728, height: 90, class: 'leaderboard' },
			'320x50': { width: 320, height: 50, class: 'mobile-banner' },
			'300x250': { width: 300, height: 250, class: 'medium-rectangle' },
			'160x600': { width: 160, height: 600, class: 'wide-skyscraper' },
			'300x600': { width: 300, height: 600, class: 'half-page' }
		};
		return dims[sz];
	}

	const dims = $derived(getDimensions(size));

	/**
	 * Get ad script URL based on network
	 */
	function getAdScriptUrl(net: string): string {
		const adsterraKey = import.meta.env.PUBLIC_ADSTERRA_KEY || '';
		switch (net) {
			case 'adsterra':
				return `//pl${adsterraKey}.highcpmgate.com/${adsterraKey}/invoke.js`;
			case 'propellerads':
				return '//native.propellerads.com/1.js';
			case 'adsense':
				return 'https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js';
			default:
				return '';
		}
	}

	/**
	 * Inject ad script into container
	 */
	function injectAdScript(): void {
		if (!containerRef || !$consent.accepted) return;

		const scriptUrl = getAdScriptUrl(network);
		if (!scriptUrl) return;

		// Check if script already exists
		const existingScript = containerRef.querySelector(`script[src*="${network}"]`);
		if (existingScript) return;

		const script = document.createElement('script');
		script.async = true;
		script.src = scriptUrl;
		script.setAttribute('data-ad-slot', slot);
		script.setAttribute('data-ad-size', size);

		script.onload = () => {
			adLoaded = true;
			trackAdImpression();
		};

		containerRef.appendChild(script);
	}

	/**
	 * Track ad impression via analytics
	 */
	function trackAdImpression(): void {
		if (typeof window !== 'undefined' && (window as any).gtag) {
			(window as any).gtag('event', 'ad_impression', {
				ad_slot: slot,
				ad_size: size,
				ad_network: network
			});
		}
	}

	onMount(() => {
		mounted = true;

		if (lazy && 'IntersectionObserver' in window) {
			const observer = new IntersectionObserver(
				(entries) => {
					entries.forEach((entry) => {
						if (entry.isIntersecting && $consent.accepted) {
							injectAdScript();
							observer.disconnect();
						}
					});
				},
				{ rootMargin: '100px' }
			);

			if (containerRef) {
				observer.observe(containerRef);
			}

			return () => observer.disconnect();
		} else if ($consent.accepted) {
			injectAdScript();
		}
	});

	// React to consent changes
	$effect(() => {
		if ($consent.accepted && mounted && containerRef) {
			injectAdScript();
		}
	});
</script>

{#if mounted && $consent.accepted}
	<div
		bind:this={containerRef}
		class="ad-banner ad-{dims.class} {className}"
		data-ad-slot={slot}
		data-ad-size={size}
		style="--ad-width: {dims.width}px; --ad-height: {dims.height}px;"
		role="region"
		aria-label="Advertisement"
	>
		<div class="ad-container" style="width: {dims.width}px; height: {dims.height}px;">
			<span class="ad-label">Advertisement</span>
			<div class="ad-content">
				{#if !adLoaded}
					<div class="ad-loading">
						<span class="ad-fallback">Loading...</span>
					</div>
				{/if}
			</div>
		</div>
	</div>
{:else if mounted && !$consent.accepted}
	<div
		class="ad-banner ad-placeholder {className}"
		style="--ad-width: {dims.width}px; --ad-height: {dims.height}px;"
	>
		<div class="ad-container" style="width: {dims.width}px; height: {dims.height}px;">
			<span class="ad-label">Advertisement</span>
			<div class="ad-content ad-consent-message">
				<svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor">
					<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
				</svg>
				<span>Please accept cookies to view ads</span>
			</div>
		</div>
	</div>
{/if}

<style>
	.ad-banner {
		width: 100%;
		max-width: var(--ad-width);
		margin: 0 auto;
	}

	.ad-container {
		position: relative;
		background: var(--ad-bg, #f3f4f6);
		border: 1px dashed var(--border-color, #d1d5db);
		border-radius: 0.5rem;
		overflow: hidden;
		/* Fixed size prevents CLS */
		min-width: var(--ad-width);
		min-height: var(--ad-height);
	}

	.ad-label {
		position: absolute;
		top: 0.25rem;
		left: 0.5rem;
		font-size: 0.625rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--text-secondary, #9ca3af);
		z-index: 1;
	}

	.ad-content {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		height: 100%;
	}

	.ad-loading {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		height: 100%;
	}

	.ad-fallback {
		font-size: 0.75rem;
		color: var(--text-secondary, #9ca3af);
	}

	.ad-consent-message {
		flex-direction: column;
		gap: 0.5rem;
		color: var(--text-secondary, #6b7280);
		font-size: 0.75rem;
		text-align: center;
		padding: 1rem;
	}

	.ad-consent-message svg {
		opacity: 0.5;
	}

	/* Hide on mobile for desktop-only sizes */
	@media (max-width: 767px) {
		.ad-banner.leaderboard,
		.ad-banner.wide-skyscraper {
			display: none;
		}
	}

	/* Hide on desktop for mobile-only sizes */
	@media (min-width: 768px) {
		.ad-banner.mobile-banner {
			display: none;
		}
	}

	/* Responsive sizing for fluid containers */
	@media (max-width: 480px) {
		.ad-banner .ad-container {
			transform: scale(0.9);
			transform-origin: center;
		}
	}

	@media (prefers-color-scheme: dark) {
		.ad-container {
			--ad-bg: #1f2937;
			--border-color: #374151;
		}
	}
</style>
