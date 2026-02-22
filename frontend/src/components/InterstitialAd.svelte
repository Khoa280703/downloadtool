<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade, scale } from 'svelte/transition';
	import { consent } from '$stores/consent';

	interface Props {
		/** Show/hide the interstitial modal */
		show: boolean;
		/** Countdown duration in seconds */
		countdownSeconds?: number;
		/** Ad network to use */
		network?: 'adsterra' | 'propellerads';
		/** Callback when countdown completes or skipped */
		onComplete: () => void;
		/** Callback when user skips early */
		onSkip?: () => void;
	}

	let {
		show = $bindable(false),
		countdownSeconds = 3,
		network = 'adsterra',
		onComplete,
		onSkip
	}: Props = $props();

	/** Current countdown value */
	let count = $state(countdownSeconds);

	/** Whether countdown is complete */
	let canSkip = $state(false);

	/** Interval reference for cleanup */
	let intervalId: ReturnType<typeof setInterval> | null = $state(null);

	/** Ad container reference */
	let adContainerRef: HTMLDivElement | undefined = $state(undefined);

	/**
	 * Start countdown timer
	 */
	function startCountdown(): void {
		count = countdownSeconds;
		canSkip = false;

		intervalId = setInterval(() => {
			count -= 1;
			if (count <= 0) {
				canSkip = true;
				if (intervalId) {
					clearInterval(intervalId);
					intervalId = null;
				}
			}
		}, 1000);
	}

	/**
	 * Inject interstitial ad script
	 */
	function injectInterstitialAd(): void {
		if (!adContainerRef || !$consent.accepted) return;

		const adsterraKey = import.meta.env.PUBLIC_ADSTERRA_KEY || '';
		const scriptUrl = `//pl${adsterraKey}.highcpmgate.com/${adsterraKey}/invoke.js`;

		// Prevent duplicate injection
		if (adContainerRef.querySelector('script')) return;

		const script = document.createElement('script');
		script.async = true;
		script.src = scriptUrl;
		script.setAttribute('data-ad-format', 'interstitial');

		script.onload = () => {
			trackInterstitialImpression();
		};

		adContainerRef.appendChild(script);
	}

	/**
	 * Track interstitial ad impression
	 */
	function trackInterstitialImpression(): void {
		if (typeof window !== 'undefined' && (window as any).gtag) {
			(window as any).gtag('event', 'ad_impression', {
				ad_format: 'interstitial',
				ad_network: network
			});
		}
	}

	/**
	 * Handle skip/continue action
	 */
	function handleContinue(): void {
		if (intervalId) {
			clearInterval(intervalId);
			intervalId = null;
		}

		if (!canSkip && onSkip) {
			onSkip();
		}

		show = false;
		onComplete();
	}

	// Watch show prop to start/stop countdown
	$effect(() => {
		if (show && $consent.accepted) {
			startCountdown();
			// Small delay to ensure container is rendered
			setTimeout(() => injectInterstitialAd(), 100);
		} else if (!show && intervalId) {
			clearInterval(intervalId);
			intervalId = null;
		}
	});

	onDestroy(() => {
		if (intervalId) {
			clearInterval(intervalId);
		}
	});
</script>

{#if show && $consent.accepted}
	<div
		class="interstitial-overlay"
		transition:fade={{ duration: 200 }}
		role="dialog"
		aria-modal="true"
		aria-labelledby="interstitial-title"
	>
		<div class="interstitial-modal" transition:scale={{ duration: 300, start: 0.95 }}>
			<div class="interstitial-header">
				<h3 id="interstitial-title">Please Wait</h3>
				<div class="countdown-badge" class:ready={canSkip}>
					{#if canSkip}
						<svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
							<path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
						</svg>
					{:else}
						<span>{count}s</span>
					{/if}
				</div>
			</div>

			<div class="interstitial-content">
				<div bind:this={adContainerRef} class="ad-slot" aria-label="Advertisement">
					<span class="ad-label">Advertisement</span>
				</div>

				<p class="interstitial-message">
					Your video is being prepared. Please wait a moment...
				</p>
			</div>

			<div class="interstitial-footer">
				<button
					class="continue-btn"
					class:ready={canSkip}
					onclick={handleContinue}
					disabled={!canSkip}
					aria-label={canSkip ? 'Continue to download' : `Please wait ${count} seconds`}
				>
					{#if canSkip}
						Continue to Download
						<svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor">
							<path d="M12 4l-1.41 1.41L16.17 11H4v2h12.17l-5.58 5.59L12 20l8-8z"/>
						</svg>
					{:else}
						Please wait... {count}s
					{/if}
				</button>
			</div>
		</div>
	</div>
{:else if show && !$consent.accepted}
	<!-- Skip interstitial if no consent, call complete immediately --}
	{#if typeof window !== 'undefined'}{handleContinue()}{/if} -->
{/if}

<style>
	.interstitial-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.75);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
	}

	.interstitial-modal {
		background: var(--bg-color, #ffffff);
		border-radius: 1rem;
		width: 100%;
		max-width: 400px;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
		overflow: hidden;
	}

	.interstitial-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.interstitial-header h3 {
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-color, #111827);
		margin: 0;
	}

	.countdown-badge {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		border-radius: 50%;
		background: var(--primary-alpha, rgba(59, 130, 246, 0.1));
		color: var(--primary-color, #3b82f6);
		font-size: 0.875rem;
		font-weight: 600;
		transition: all 0.3s ease;
	}

	.countdown-badge.ready {
		background: var(--success-color, #22c55e);
		color: white;
	}

	.interstitial-content {
		padding: 1.25rem;
	}

	.ad-slot {
		position: relative;
		width: 100%;
		min-height: 250px;
		background: var(--card-bg, #f9fafb);
		border: 1px dashed var(--border-color, #d1d5db);
		border-radius: 0.5rem;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 1rem;
	}

	.ad-label {
		position: absolute;
		top: 0.5rem;
		left: 0.75rem;
		font-size: 0.625rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--text-secondary, #9ca3af);
	}

	.interstitial-message {
		text-align: center;
		font-size: 0.875rem;
		color: var(--text-secondary, #6b7280);
		margin: 0;
	}

	.interstitial-footer {
		padding: 1rem 1.25rem 1.25rem;
		border-top: 1px solid var(--border-color, #e5e7eb);
	}

	.continue-btn {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.875rem 1.5rem;
		border-radius: 0.75rem;
		font-size: 0.9375rem;
		font-weight: 600;
		border: none;
		cursor: not-allowed;
		transition: all 0.2s ease;
		background: var(--card-bg, #f3f4f6);
		color: var(--text-secondary, #9ca3af);
	}

	.continue-btn.ready {
		background: var(--primary-color, #3b82f6);
		color: white;
		cursor: pointer;
	}

	.continue-btn.ready:hover {
		background: var(--primary-hover, #2563eb);
		transform: translateY(-1px);
	}

	.continue-btn.ready:active {
		transform: translateY(0);
	}

	@media (prefers-color-scheme: dark) {
		.interstitial-modal {
			--bg-color: #111827;
			--card-bg: #1f2937;
			--border-color: #374151;
			--text-color: #f9fafb;
			--text-secondary: #9ca3af;
		}
	}

	@media (max-width: 480px) {
		.interstitial-overlay {
			padding: 0.5rem;
		}

		.interstitial-modal {
			border-radius: 0.75rem;
		}

		.ad-slot {
			min-height: 200px;
		}
	}
</style>
