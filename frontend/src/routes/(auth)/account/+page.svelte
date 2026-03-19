<script lang="ts">
	import { signOutFromBrowser } from '$lib/auth-actions';
	import * as m from '$lib/paraglide/messages';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
	let isSigningOut = $state(false);
	let signOutError = $state('');

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

		{#if signOutError}
			<p class="status-error mt-4 rounded-2xl border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700">
				{signOutError}
			</p>
		{/if}

		<div class="mt-6 flex flex-wrap gap-3">
			{#if data.isAdmin}
				<a
					href="/admin/overview"
					class="account-outline-btn rounded-full border border-plum/20 px-5 py-3 text-sm font-bold uppercase tracking-wide text-plum transition hover:border-primary/40 hover:text-primary"
				>
					{m.account_admin_dashboard()}
				</a>
			{/if}
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

	:global(.app.theme-dark) .account-page [class*='text-plum/70'] {
		color: rgba(224, 208, 245, 0.78) !important;
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

</style>
