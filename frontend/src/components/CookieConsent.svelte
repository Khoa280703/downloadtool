<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { consent, hasDecided } from '$stores/consent';
	import { trackConsent } from '$lib/analytics';

	/**
	 * Show/hide detailed info
	 */
	let showDetails = $state(false);
	let isClientReady = $state(false);
	let isDarkMode = $state(false);

	function syncThemeFromDom(): void {
		if (!browser) return;
		isDarkMode = document.querySelector('.page-root.theme-dark') !== null;
	}

	onMount(() => {
		// Avoid SSR -> CSR flicker where banner appears briefly then closes
		// when consent state is restored from localStorage on the client.
		isClientReady = true;
		syncThemeFromDom();

		const observer = new MutationObserver(() => {
			syncThemeFromDom();
		});

		observer.observe(document.body, {
			attributes: true,
			subtree: true,
			attributeFilter: ['class']
		});

		const storageHandler = (event: StorageEvent) => {
			if (event.key !== 'fetchtube-theme') return;
			syncThemeFromDom();
		};

		window.addEventListener('storage', storageHandler);

		return () => {
			observer.disconnect();
			window.removeEventListener('storage', storageHandler);
		};
	});

	/**
	 * Handle accept action
	 */
	function handleAccept(): void {
		consent.accept();
		trackConsent(true, 'banner');
	}

	/**
	 * Handle reject action
	 */
	function handleReject(): void {
		consent.reject();
		trackConsent(false, 'banner');
	}
</script>

{#if browser && isClientReady && !$hasDecided}
	<div class="cookie-shell" class:theme-dark={isDarkMode} role="region" aria-label="Cookie consent">
		<div class="cookie-banner">
			<div class="cookie-main">
				<span class="cookie-badge">Privacy</span>
				<h2 class="cookie-title">We value your privacy</h2>
				<p class="cookie-description">
					We use cookies to improve your browsing experience and analyze traffic.
					By clicking "Accept All", you consent to our use of cookies.
					<a href="/privacy" aria-label="Read our privacy policy">Read Privacy Policy</a>
				</p>
			</div>

			<div class="cookie-actions">
				<button class="btn-accept" onclick={handleAccept}>Accept All</button>
				<button class="btn-reject" onclick={handleReject}>Reject Non-Essential</button>
				<button
					class="btn-details"
					onclick={() => (showDetails = !showDetails)}
					aria-expanded={showDetails}
				>
					{showDetails ? 'Hide Details' : 'Show Details'}
				</button>
			</div>

			{#if showDetails}
				<div class="cookie-details">
					<div class="detail-item">
						<strong>Essential:</strong> Required for core site functionality. Always enabled.
					</div>
					<div class="detail-item">
						<strong>Analytics:</strong> Helps us measure performance and improve the product.
					</div>
					<div class="detail-item">
						<strong>Preferences:</strong> Stores interface choices for a smoother experience.
					</div>
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	.cookie-shell {
		position: fixed;
		bottom: 14px;
		left: 0;
		right: 0;
		z-index: 120;
		padding: 0 14px;
		will-change: transform, opacity;
		animation: cookie-banner-enter 260ms ease-out;
		pointer-events: none;
	}

	.cookie-banner {
		max-width: 1040px;
		margin: 0 auto;
		padding: 1rem;
		border-radius: 1.35rem;
		border: 1px solid rgba(255, 77, 140, 0.2);
		background:
			linear-gradient(145deg, rgba(255, 255, 255, 0.97), rgba(255, 246, 250, 0.96)),
			radial-gradient(circle at 88% 10%, rgba(255, 185, 56, 0.18), transparent 45%);
		backdrop-filter: blur(16px);
		-webkit-backdrop-filter: blur(16px);
		box-shadow: 0 22px 44px -24px rgba(45, 27, 54, 0.5);
		pointer-events: auto;
		display: grid;
		grid-template-columns: 1fr;
		gap: 0.85rem;
	}

	.cookie-main {
		min-width: 0;
	}

	.cookie-badge {
		display: inline-flex;
		align-items: center;
		height: 24px;
		padding: 0 0.75rem;
		border-radius: 999px;
		background: rgba(255, 77, 140, 0.12);
		color: #d92572;
		font-size: 0.7rem;
		font-weight: 800;
		letter-spacing: 0.08em;
		text-transform: uppercase;
	}

	.cookie-title {
		margin: 0.45rem 0 0;
		font-size: 1.05rem;
		line-height: 1.2;
		font-weight: 700;
		color: #2d1b36;
	}

	.cookie-description {
		margin: 0.4rem 0 0;
		font-size: 0.875rem;
		line-height: 1.55;
		color: rgba(45, 27, 54, 0.76);
		font-weight: 600;
	}

	.cookie-description a {
		color: #d92572;
		font-weight: 800;
		text-decoration: underline;
		text-underline-offset: 2px;
	}

	.cookie-description a:hover {
		color: #ff4d8c;
	}

	.cookie-actions {
		display: flex;
		flex-wrap: wrap;
		gap: 0.55rem;
	}

	.cookie-actions button {
		border: none;
		cursor: pointer;
		font-size: 0.76rem;
		font-weight: 800;
		letter-spacing: 0.03em;
		text-transform: uppercase;
		height: 2.3rem;
		padding: 0 0.9rem;
		border-radius: 999px;
		transition: transform 0.2s ease, filter 0.2s ease, box-shadow 0.2s ease;
	}

	.cookie-actions button:hover {
		transform: translateY(-1px);
	}

	.btn-accept {
		background: linear-gradient(135deg, #ff4d8c 0%, #ffb938 100%);
		color: #ffffff;
		box-shadow: 0 12px 24px -16px rgba(255, 77, 140, 0.8);
	}

	.btn-accept:hover {
		filter: brightness(1.05);
	}

	.btn-reject {
		background: rgba(45, 27, 54, 0.06);
		color: #2d1b36;
		border: 1px solid rgba(45, 27, 54, 0.14) !important;
	}

	.btn-reject:hover {
		background: rgba(45, 27, 54, 0.11);
	}

	.btn-details {
		background: transparent;
		color: rgba(45, 27, 54, 0.72);
		text-decoration: underline;
		text-underline-offset: 2px;
		padding-left: 0.55rem !important;
		padding-right: 0.55rem !important;
	}

	.btn-details:hover {
		color: #2d1b36;
	}

	.cookie-details {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		padding: 0.85rem;
		border-radius: 1rem;
		border: 1px solid rgba(255, 77, 140, 0.16);
		background: rgba(255, 255, 255, 0.72);
	}

	.detail-item {
		font-size: 0.82rem;
		line-height: 1.45;
		color: rgba(45, 27, 54, 0.78);
		padding-left: 0.85rem;
		position: relative;
	}

	.detail-item::before {
		content: '';
		position: absolute;
		left: 0;
		top: 0.55rem;
		width: 0.35rem;
		height: 0.35rem;
		border-radius: 999px;
		background: #ff4d8c;
	}

	.detail-item strong {
		color: #2d1b36;
	}

	.cookie-shell.theme-dark .cookie-banner {
		border-color: rgba(255, 77, 140, 0.26);
		background:
			linear-gradient(145deg, rgba(28, 27, 40, 0.96), rgba(20, 20, 30, 0.96)),
			radial-gradient(circle at 88% 10%, rgba(255, 185, 56, 0.14), transparent 45%);
		box-shadow: 0 20px 44px -26px rgba(0, 0, 0, 0.8);
	}

	.cookie-shell.theme-dark .cookie-badge {
		background: rgba(255, 77, 140, 0.2);
		color: #ff9dc7;
	}

	.cookie-shell.theme-dark .cookie-title {
		color: #f7ecff;
	}

	.cookie-shell.theme-dark .cookie-description {
		color: rgba(224, 208, 245, 0.78);
	}

	.cookie-shell.theme-dark .cookie-description a {
		color: #ff8cbc;
	}

	.cookie-shell.theme-dark .btn-reject {
		background: rgba(255, 255, 255, 0.05);
		color: rgba(243, 232, 255, 0.9);
		border-color: rgba(255, 255, 255, 0.18) !important;
	}

	.cookie-shell.theme-dark .btn-reject:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.cookie-shell.theme-dark .btn-details {
		color: rgba(224, 208, 245, 0.78);
	}

	.cookie-shell.theme-dark .cookie-details {
		background: rgba(255, 77, 140, 0.08);
		border-color: rgba(255, 77, 140, 0.26);
	}

	.cookie-shell.theme-dark .detail-item {
		color: rgba(224, 208, 245, 0.8);
	}

	.cookie-shell.theme-dark .detail-item strong {
		color: #f7ecff;
	}

	@media (min-width: 900px) {
		.cookie-banner {
			grid-template-columns: minmax(0, 1fr) auto;
			grid-template-areas:
				'main actions'
				'details details';
			align-items: center;
			column-gap: 1rem;
			padding: 1rem 1.15rem;
		}

		.cookie-main {
			grid-area: main;
		}

		.cookie-actions {
			grid-area: actions;
			justify-content: flex-end;
			flex-wrap: nowrap;
		}

		.cookie-details {
			grid-area: details;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.cookie-shell {
			animation: none;
		}
	}

	@keyframes cookie-banner-enter {
		from {
			transform: translateY(14px);
			opacity: 0;
		}

		to {
			transform: translateY(0);
			opacity: 1;
		}
	}
</style>
