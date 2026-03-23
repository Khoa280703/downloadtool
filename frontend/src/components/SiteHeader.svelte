<script lang="ts">
	import AppIcon from '$components/AppIcon.svelte';
	import LanguageSwitcher from '$components/LanguageSwitcher.svelte';
	import UserMenu from '$components/UserMenu.svelte';
	import * as m from '$lib/paraglide/messages';

	type AuthUser = { name?: string | null; email: string; image?: string | null };

	let {
		authUser,
		onOpenAuthModal,
		homeHref,
		howItWorksHref,
		toolsHref
	}: {
		authUser: AuthUser | null | undefined;
		onOpenAuthModal: () => void;
		homeHref: string;
		howItWorksHref: string;
		toolsHref: string;
	} = $props();

</script>

<header class="glass-header sticky top-0 z-50 border-b border-white/50 px-6 py-3 lg:px-20 transition-all duration-300">
	<div class="max-w-7xl mx-auto flex items-center justify-between">
		<a href={homeHref} class="flex items-center gap-3 group min-w-0">
			<img
				src="/logo.svg"
				alt="Snapvie"
				class="h-10 w-auto shrink-0 drop-shadow-sm transition-transform duration-300 group-hover:scale-105"
				loading="eager"
				decoding="async"
			/>
			<h2 class="text-plum text-2xl font-bold tracking-tight">Snapvie</h2>
		</a>
		<div class="hidden md:flex items-center gap-8">
			<nav class="flex gap-6">
				<a class="text-plum font-semibold hover:text-primary transition-colors text-base" href={homeHref}>{m.header_nav_home()}</a>
				<a class="text-plum font-semibold hover:text-primary transition-colors text-base" href={howItWorksHref}>{m.header_nav_how_it_works()}</a>
				<a class="text-plum font-semibold hover:text-primary transition-colors text-base" href="/guides">{m.header_nav_guides()}</a>
				<a class="text-plum font-semibold hover:text-primary transition-colors text-base" href="/compare">{m.header_nav_compare()}</a>
			</nav>
			<LanguageSwitcher />
			{#if authUser === undefined}
				<div class="h-10 w-24 rounded-full bg-plum/10 animate-pulse"></div>
			{:else if authUser}
				<UserMenu user={authUser} />
			{:else}
				<button
					type="button"
					class="flex h-10 px-6 items-center justify-center rounded-full bg-plum text-white text-sm font-bold shadow-lg hover:bg-plum/90 hover:scale-105 active:scale-95 transition-all duration-300 tracking-wide uppercase"
					onclick={onOpenAuthModal}
				>
					{m.header_login()}
				</button>
			{/if}
		</div>
		<div class="md:hidden flex items-center gap-2">
			<LanguageSwitcher />
			{#if authUser === undefined}
				<div class="h-10 w-10 rounded-full bg-plum/10 animate-pulse"></div>
			{:else if authUser}
				<a href="/account" class="text-plum p-2 rounded-xl hover:bg-white/50 transition-colors flex items-center">
					<AppIcon name="account_circle" class="text-3xl" />
				</a>
			{:else}
				<button
					type="button"
					class="text-plum p-2 rounded-xl hover:bg-white/50 transition-colors flex items-center"
					onclick={onOpenAuthModal}
				>
					<AppIcon name="login" class="text-3xl" />
				</button>
			{/if}
		</div>
	</div>
</header>
