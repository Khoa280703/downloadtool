<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import {
		cookieDomain,
		cookieMaxAge,
		cookieName,
		getLocale,
		locales,
		localizeHref
	} from '$lib/paraglide/runtime';
	import * as m from '$lib/paraglide/messages';

	let open = $state(false);

	const LANGUAGE_LABELS: Record<string, string> = {
		ar: 'العربية',
		bg: 'Български',
		cs: 'Čeština',
		da: 'Dansk',
		de: 'Deutsch',
		el: 'Ελληνικά',
		en: 'English',
		es: 'Español',
		et: 'Eesti',
		fi: 'Suomi',
		fr: 'Français',
		hu: 'Magyar',
		id: 'Bahasa Indonesia',
		it: 'Italiano',
		ja: '日本語',
		ko: '한국어',
		lt: 'Lietuvių',
		lv: 'Latviešu',
		nb: 'Norsk Bokmål',
		nl: 'Nederlands',
		pl: 'Polski',
		pt: 'Português',
		'pt-BR': 'Português (Brasil)',
		ro: 'Română',
		ru: 'Русский',
		sk: 'Slovenčina',
		sl: 'Slovenščina',
		sv: 'Svenska',
		tr: 'Türkçe',
		uk: 'Українська',
		vi: 'Tiếng Việt',
		zh: '简体中文',
		'zh-TW': '繁體中文'
	};

	const currentLocale = $derived(getLocale());

	function closeMenu(): void {
		open = false;
	}

	function persistLocalePreference(locale: string): void {
		window.localStorage.setItem('preferred-lang', locale);

		const domainPart = cookieDomain ? `; Domain=${cookieDomain}` : '';
		const securePart = window.location.protocol === 'https:' ? '; Secure' : '';

		document.cookie = `${cookieName}=${encodeURIComponent(locale)}; Path=/; Max-Age=${cookieMaxAge}; SameSite=Lax${domainPart}${securePart}`;
	}

	async function switchLanguage(locale: string): Promise<void> {
		closeMenu();
		if (!browser) return;

		persistLocalePreference(locale);

		const currentHref = `${$page.url.pathname}${$page.url.search}${$page.url.hash}`;
		const targetHref = localizeHref(currentHref, { locale });

		if (targetHref !== `${window.location.pathname}${window.location.search}${window.location.hash}`) {
			await goto(targetHref, { replaceState: true, invalidateAll: true });
		}
	}
</script>

<div class="relative">
	<button
		type="button"
		class="language-switcher-trigger flex h-10 items-center gap-2 rounded-full border border-plum/20 bg-white/70 px-3 text-xs font-bold uppercase tracking-wide text-plum transition hover:border-primary/45 hover:text-primary"
		onclick={() => (open = !open)}
		aria-label={m.header_language_aria()}
		aria-expanded={open}
	>
		<span class="material-symbols-outlined text-base">language</span>
		<span>{currentLocale}</span>
	</button>

	{#if open}
		<button
			type="button"
			class="fixed inset-0 z-40 cursor-default bg-transparent"
			onclick={closeMenu}
			aria-label={m.header_language_close_menu()}
		></button>

		<div class="language-switcher-menu absolute right-0 top-full z-50 mt-2 max-h-80 w-64 overflow-y-auto rounded-2xl border border-pink-100 bg-white p-2 shadow-xl">
			{#each locales as locale}
				<button
					type="button"
					class="flex w-full items-center justify-between rounded-xl px-3 py-2 text-left text-sm font-semibold text-plum transition hover:bg-pink-50"
					class:text-primary={locale === currentLocale}
					onclick={() => switchLanguage(locale)}
				>
					<span>{LANGUAGE_LABELS[locale] ?? locale}</span>
					{#if locale === currentLocale}
						<span class="material-symbols-outlined text-base">check</span>
					{/if}
				</button>
			{/each}
		</div>
	{/if}
</div>

<style>
	:global(.app.theme-dark) .language-switcher-trigger {
		border-color: rgba(255, 77, 140, 0.28);
		background: rgba(18, 18, 26, 0.72);
		color: rgba(245, 232, 255, 0.95);
	}

	:global(.app.theme-dark) .language-switcher-trigger:hover {
		border-color: rgba(255, 124, 175, 0.45);
		color: #ff8cbc;
	}

	:global(.app.theme-dark) .language-switcher-menu {
		border-color: rgba(255, 77, 140, 0.28);
		background: rgba(28, 27, 40, 0.95);
	}

	:global(.app.theme-dark) .language-switcher-menu button {
		color: rgba(245, 232, 255, 0.9);
	}

	:global(.app.theme-dark) .language-switcher-menu button:hover {
		background: rgba(255, 77, 140, 0.16);
	}

	:global(.app.theme-dark) .language-switcher-menu button.text-primary {
		color: #ff8cbc !important;
	}
</style>
