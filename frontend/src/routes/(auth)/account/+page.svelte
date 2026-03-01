<script lang="ts">
	import { signOutFromBrowser } from '$lib/auth-actions';
	import { getLocale } from '$lib/paraglide/runtime';
	import * as m from '$lib/paraglide/messages';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
	let isSigningOut = $state(false);
	let signOutError = $state('');

	const isPremium = $derived(
		data.subscription.plan === 'premium' && data.subscription.status === 'active'
	);

	function formatDate(value: string | null): string {
		if (!value) return m.account_period_none();
		const parsed = new Date(value);
		if (Number.isNaN(parsed.getTime())) return m.account_period_unknown();
		return parsed.toLocaleDateString(getLocale(), {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}

	async function handleSignOut(): Promise<void> {
		signOutError = '';
		isSigningOut = true;
		try {
			await signOutFromBrowser();
			window.location.assign('/');
		} catch (error) {
			signOutError = error instanceof Error ? error.message : m.account_sign_out_failed();
		} finally {
			isSigningOut = false;
		}
	}
</script>

<svelte:head>
	<title>{m.account_meta_title()}</title>
</svelte:head>

<section class="account-page mx-auto max-w-3xl px-4 py-10">
	<div class="account-card rounded-3xl border border-pink-200 bg-white p-6 shadow-card">
		<div class="flex flex-wrap items-center justify-between gap-4">
			<div>
				<h1 class="text-3xl font-bold text-plum">{m.account_title()}</h1>
				<p class="mt-1 text-sm text-plum/70">{data.user.email}</p>
			</div>
			<button
				type="button"
				class="account-outline-btn rounded-full border border-plum/20 px-4 py-2 text-sm font-bold text-plum transition hover:border-primary/40 hover:text-primary disabled:cursor-not-allowed disabled:opacity-60"
				onclick={handleSignOut}
				disabled={isSigningOut}
			>
				{isSigningOut ? m.account_signing_out() : m.account_sign_out()}
			</button>
		</div>

		{#if data.checkoutStatus === 'success'}
			<p class="status-success mt-4 rounded-2xl border border-emerald-200 bg-emerald-50 px-4 py-3 text-sm text-emerald-700">
				{m.account_checkout_success()}
			</p>
		{/if}

		{#if signOutError}
			<p class="status-error mt-4 rounded-2xl border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700">
				{signOutError}
			</p>
		{/if}

		<div class="mt-6 grid gap-4 md:grid-cols-2">
			<div class="account-info-card rounded-2xl border border-pink-100 bg-pink-50 p-4">
				<p class="text-xs font-bold uppercase tracking-wide text-plum/60">{m.account_current_plan()}</p>
				<div class="mt-2 flex items-center gap-3">
					<span
						class="inline-flex rounded-full px-3 py-1 text-sm font-bold {isPremium
							? 'bg-emerald-100 text-emerald-700'
							: 'bg-slate-200 text-slate-700'}"
					>
						{isPremium ? m.account_plan_premium() : m.account_plan_free()}
					</span>
					<span class="text-sm text-plum/70">{m.account_status_label()}: {data.subscription.status}</span>
				</div>
			</div>

			<div class="account-info-card rounded-2xl border border-pink-100 bg-pink-50 p-4">
				<p class="text-xs font-bold uppercase tracking-wide text-plum/60">{m.account_current_period()}</p>
				<p class="mt-2 text-base font-semibold text-plum">
					{formatDate(data.subscription.currentPeriodEnd)}
				</p>
			</div>
		</div>

		<div class="mt-6 flex flex-wrap gap-3">
			{#if !isPremium}
				<a
					href="/api/checkout"
					class="account-primary-btn rounded-full bg-primary px-5 py-3 text-sm font-bold uppercase tracking-wide text-white transition hover:bg-primary/90"
				>
					{m.account_upgrade_premium()}
				</a>
			{/if}

			<a
				href="https://whop.com/hub/"
				target="_blank"
				rel="noreferrer"
				class="account-outline-btn rounded-full border border-plum/20 px-5 py-3 text-sm font-bold uppercase tracking-wide text-plum transition hover:border-primary/40 hover:text-primary"
			>
				{m.account_manage_subscription()}
			</a>
		</div>
	</div>
</section>

<style>
	:global(.app.theme-dark) .account-card {
		border-color: rgba(255, 77, 140, 0.24);
		background: rgba(28, 27, 40, 0.82);
		box-shadow: 0 20px 34px -28px rgba(255, 77, 140, 0.45);
	}

	:global(.app.theme-dark) .account-page h1,
	:global(.app.theme-dark) .account-page .text-plum {
		color: #ffffff !important;
	}

	:global(.app.theme-dark) .account-page [class*='text-plum/70'],
	:global(.app.theme-dark) .account-page [class*='text-plum/60'] {
		color: rgba(224, 208, 245, 0.78) !important;
	}

	:global(.app.theme-dark) .account-info-card {
		border-color: rgba(255, 77, 140, 0.18);
		background: rgba(40, 35, 53, 0.75);
	}

	:global(.app.theme-dark) .status-success {
		border-color: rgba(52, 211, 153, 0.35);
		background: rgba(6, 78, 59, 0.25);
		color: #a7f3d0 !important;
	}

	:global(.app.theme-dark) .status-error {
		border-color: rgba(248, 113, 113, 0.4);
		background: rgba(127, 29, 29, 0.25);
		color: #fecaca !important;
	}

	:global(.app.theme-dark) .account-outline-btn {
		border-color: rgba(255, 77, 140, 0.28) !important;
		color: rgba(245, 232, 255, 0.94) !important;
	}

	:global(.app.theme-dark) .account-outline-btn:hover {
		border-color: rgba(255, 124, 175, 0.45) !important;
		color: #ff8cbc !important;
	}

	:global(.app.theme-dark) .account-primary-btn {
		background: linear-gradient(135deg, #ff4d8c 0%, #ffb938 100%);
		color: #ffffff !important;
	}
</style>
