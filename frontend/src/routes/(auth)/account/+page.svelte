<script lang="ts">
	import { goto } from '$app/navigation';

	import { authClient } from '$lib/auth-client';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
	let isSigningOut = $state(false);

	const isPremium = $derived(
		data.subscription.plan === 'premium' && data.subscription.status === 'active'
	);

	function formatDate(value: string | null): string {
		if (!value) return 'Không có';
		const parsed = new Date(value);
		if (Number.isNaN(parsed.getTime())) return 'Không xác định';
		return parsed.toLocaleDateString('vi-VN', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}

	async function handleSignOut(): Promise<void> {
		isSigningOut = true;
		try {
			await authClient.signOut();
		} finally {
			isSigningOut = false;
		}
		await goto('/login', { invalidateAll: true });
	}
</script>

<svelte:head>
	<title>Tài khoản - FetchTube</title>
</svelte:head>

<section class="mx-auto max-w-3xl px-4 py-10">
	<div class="rounded-3xl border border-pink-200 bg-white p-6 shadow-card">
		<div class="flex flex-wrap items-center justify-between gap-4">
			<div>
				<h1 class="text-3xl font-bold text-plum">Tài khoản của bạn</h1>
				<p class="mt-1 text-sm text-plum/70">{data.user.email}</p>
			</div>
			<button
				type="button"
				class="rounded-full border border-plum/20 px-4 py-2 text-sm font-bold text-plum transition hover:border-primary/40 hover:text-primary disabled:cursor-not-allowed disabled:opacity-60"
				onclick={handleSignOut}
				disabled={isSigningOut}
			>
				{isSigningOut ? 'Đang đăng xuất...' : 'Đăng xuất'}
			</button>
		</div>

		{#if data.checkoutStatus === 'success'}
			<p class="mt-4 rounded-2xl border border-emerald-200 bg-emerald-50 px-4 py-3 text-sm text-emerald-700">
				Thanh toán đã hoàn tất. Nếu plan chưa cập nhật ngay, hãy đợi vài giây để webhook đồng bộ.
			</p>
		{/if}

		<div class="mt-6 grid gap-4 md:grid-cols-2">
			<div class="rounded-2xl border border-pink-100 bg-pink-50 p-4">
				<p class="text-xs font-bold uppercase tracking-wide text-plum/60">Gói hiện tại</p>
				<div class="mt-2 flex items-center gap-3">
					<span
						class="inline-flex rounded-full px-3 py-1 text-sm font-bold {isPremium
							? 'bg-emerald-100 text-emerald-700'
							: 'bg-slate-200 text-slate-700'}"
					>
						{isPremium ? 'Premium' : 'Free'}
					</span>
					<span class="text-sm text-plum/70">Status: {data.subscription.status}</span>
				</div>
			</div>

			<div class="rounded-2xl border border-pink-100 bg-pink-50 p-4">
				<p class="text-xs font-bold uppercase tracking-wide text-plum/60">Kỳ hiện tại</p>
				<p class="mt-2 text-base font-semibold text-plum">
					{formatDate(data.subscription.currentPeriodEnd)}
				</p>
			</div>
		</div>

		<div class="mt-6 flex flex-wrap gap-3">
			{#if !isPremium}
				<a
					href="/api/checkout"
					class="rounded-full bg-primary px-5 py-3 text-sm font-bold uppercase tracking-wide text-white transition hover:bg-primary/90"
				>
					Upgrade lên Premium
				</a>
			{/if}

			<a
				href="https://whop.com/hub/"
				target="_blank"
				rel="noreferrer"
				class="rounded-full border border-plum/20 px-5 py-3 text-sm font-bold uppercase tracking-wide text-plum transition hover:border-primary/40 hover:text-primary"
			>
				Manage Subscription
			</a>
		</div>
	</div>
</section>
