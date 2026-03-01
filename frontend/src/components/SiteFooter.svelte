<script lang="ts">
	import * as m from '$lib/paraglide/messages';
</script>

<footer class="bg-white border-t border-pink-100 py-6 px-6">
	<div class="max-w-7xl mx-auto flex flex-col md:flex-row justify-between items-center gap-4">
		<div class="flex items-center gap-2 opacity-50 grayscale hover:grayscale-0 transition-all">
			<span class="material-symbols-outlined text-xl">smart_toy</span>
			<span class="font-bold text-sm">{m.footer_copyright({ year: String(new Date().getFullYear()) })}</span>
		</div>
		<div class="flex gap-4 text-plum/60 font-semibold text-xs">
			<a class="hover:text-primary transition-colors" href="/privacy">{m.footer_privacy_policy()}</a>
			<a class="hover:text-primary transition-colors" href="/privacy">{m.footer_terms_of_service()}</a>
			<a class="hover:text-primary transition-colors" href="/privacy#contact">{m.footer_contact()}</a>
		</div>
	</div>
</footer>
