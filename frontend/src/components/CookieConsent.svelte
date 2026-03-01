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

	onMount(() => {
		// Avoid SSR -> CSR flicker where banner appears briefly then closes
		// when consent state is restored from localStorage on the client.
		isClientReady = true;
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
	<div
		class="cookie-banner"
		role="region"
		aria-label="Cookie consent"
	>
		<div class="cookie-content">
			<div class="cookie-text">
				<h2 class="cookie-title">We value your privacy</h2>
				<p>
					We use cookies to enhance your browsing experience,
					and analyze our traffic. By clicking "Accept All", you consent to our use of cookies.
					<a href="/privacy" aria-label="Read our privacy policy">Read our Privacy Policy</a>
				</p>
			</div>

			{#if showDetails}
				<div class="cookie-details">
					<ul>
						<li>
							<strong>Essential:</strong> Required for the site to function properly.
							Always enabled.
						</li>
						<li>
							<strong>Analytics:</strong> Helps us understand how visitors interact
							with our website.
						</li>
						<li>
							<strong>Preferences:</strong> Remembers your choices for a better
							site experience.
						</li>
					</ul>
				</div>
			{/if}

			<div class="cookie-actions">
				<button class="btn-accept" onclick={handleAccept}>
					Accept All
				</button>
				<button class="btn-reject" onclick={handleReject}>
					Reject Non-Essential
				</button>
				<button
					class="btn-details"
					onclick={() => showDetails = !showDetails}
					aria-expanded={showDetails}
				>
					{showDetails ? 'Hide Details' : 'Show Details'}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.cookie-banner {
		position: fixed;
		bottom: 0;
		left: 0;
		right: 0;
		background: var(--bg-color, #ffffff);
		border-top: 1px solid var(--border-color, #e5e7eb);
		box-shadow: 0 -4px 6px -1px rgba(0, 0, 0, 0.1);
		z-index: 999;
		padding: 1rem;
		will-change: transform, opacity;
		animation: cookie-banner-enter 220ms ease-out;
	}

	.cookie-content {
		max-width: 1200px;
		margin: 0 auto;
	}

	.cookie-title {
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-color, #111827);
		margin: 0 0 0.5rem;
	}

	.cookie-text p {
		font-size: 0.875rem;
		color: var(--text-secondary, #4b5563);
		margin: 0 0 1rem;
		line-height: 1.5;
	}

	.cookie-text a {
		color: var(--primary-color, #1d4ed8);
		font-weight: 600;
		text-decoration: underline;
		text-underline-offset: 2px;
	}

	.cookie-text a:hover {
		color: var(--primary-hover, #1e40af);
	}

	.cookie-details {
		background: var(--card-bg, #f9fafb);
		border-radius: 0.5rem;
		padding: 1rem;
		margin-bottom: 1rem;
	}

	.cookie-details ul {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.cookie-details li {
		font-size: 0.875rem;
		color: var(--text-secondary, #6b7280);
		margin-bottom: 0.5rem;
		padding-left: 1.25rem;
		position: relative;
	}

	.cookie-details li:last-child {
		margin-bottom: 0;
	}

	.cookie-details li::before {
		content: '';
		position: absolute;
		left: 0;
		top: 0.5rem;
		width: 6px;
		height: 6px;
		background: var(--primary-color, #3b82f6);
		border-radius: 50%;
	}

	.cookie-actions {
		display: flex;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	.cookie-actions button {
		padding: 0.625rem 1.25rem;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
		border: none;
	}

	.btn-accept {
		background: var(--primary-color, #1d4ed8);
		color: white;
	}

	.btn-accept:hover {
		background: var(--primary-hover, #1e40af);
	}

	.btn-reject {
		background: transparent;
		color: var(--text-secondary, #6b7280);
		border: 1px solid var(--border-color, #e5e7eb) !important;
	}

	.btn-reject:hover {
		background: var(--card-bg, #f9fafb);
		color: var(--text-color, #111827);
	}

	.btn-details {
		background: transparent;
		color: var(--text-secondary, #6b7280);
		text-decoration: underline;
		padding-left: 0.5rem !important;
		padding-right: 0.5rem !important;
	}

	.btn-details:hover {
		color: var(--text-color, #111827);
	}

	@media (min-width: 640px) {
		.cookie-banner {
			padding: 1.5rem;
		}

		.cookie-content {
			display: flex;
			align-items: flex-start;
			gap: 2rem;
		}

		.cookie-text {
			flex: 1;
		}

		.cookie-text p {
			margin-bottom: 0;
		}

		.cookie-actions {
			flex-direction: column;
			min-width: 160px;
		}

		.cookie-actions button {
			width: 100%;
		}
	}

	@media (prefers-color-scheme: dark) {
		.cookie-banner {
			--bg-color: #111827;
			--card-bg: #1f2937;
			--border-color: #374151;
			--text-color: #f9fafb;
			--text-secondary: #d1d5db;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.cookie-banner {
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
