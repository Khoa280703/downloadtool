<script lang="ts">
	import { signOutFromBrowser } from '$lib/auth-actions';

	type MenuUser = {
		name?: string | null;
		email: string;
		image?: string | null;
	};

	let { user }: { user: MenuUser } = $props();

	let open = $state(false);
	let signingOut = $state(false);
	let signOutError = $state('');

	const displayName = $derived(user.name?.trim() || user.email.split('@')[0]);
	const avatarLabel = $derived(displayName.slice(0, 1).toUpperCase());

	async function handleSignOut(): Promise<void> {
		signOutError = '';
		signingOut = true;
		try {
			await signOutFromBrowser();
			open = false;
			window.location.assign('/');
		} catch (error) {
			signOutError = error instanceof Error ? error.message : 'Đăng xuất thất bại.';
		} finally {
			signingOut = false;
		}
	}
</script>

<div class="relative">
	<button
		type="button"
		class="flex h-10 items-center gap-2 rounded-full border border-plum/20 bg-white px-3 text-sm font-bold text-plum transition hover:border-primary/40"
		onclick={() => (open = !open)}
		aria-expanded={open}
		aria-haspopup="menu"
	>
		{#if user.image}
			<img src={user.image} alt={displayName} class="h-7 w-7 rounded-full object-cover" />
		{:else}
			<span class="inline-flex h-7 w-7 items-center justify-center rounded-full bg-primary text-xs text-white">
				{avatarLabel}
			</span>
		{/if}
		<span class="max-w-[140px] truncate">{displayName}</span>
	</button>

	{#if open}
		<div class="absolute right-0 top-12 z-50 w-56 rounded-2xl border border-pink-100 bg-white p-2 shadow-card">
			<div class="px-2 py-2">
				<p class="truncate text-sm font-bold text-plum">{displayName}</p>
				<p class="truncate text-xs text-plum/60">{user.email}</p>
			</div>
			<a
				href="/account"
				class="block rounded-xl px-3 py-2 text-sm font-semibold text-plum transition hover:bg-pink-50"
				onclick={() => (open = false)}
			>
				Account
			</a>
				<button
					type="button"
					class="mt-1 block w-full rounded-xl px-3 py-2 text-left text-sm font-semibold text-plum transition hover:bg-pink-50 disabled:cursor-not-allowed disabled:opacity-60"
					onclick={handleSignOut}
					disabled={signingOut}
				>
					{signingOut ? 'Đang đăng xuất...' : 'Logout'}
				</button>
				{#if signOutError}
					<p class="mt-2 rounded-lg bg-red-50 px-3 py-2 text-xs text-red-700">{signOutError}</p>
				{/if}
			</div>
		{/if}
	</div>
