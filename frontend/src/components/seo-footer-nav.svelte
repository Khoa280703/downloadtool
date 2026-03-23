<script lang="ts">
	/**
	 * SEO footer navigation component for internal link architecture.
	 * Provides link equity flow to all money pages, hubs, and trust pages.
	 * Visible on all non-home, non-admin pages via +layout.svelte.
	 */
	import * as m from '$lib/paraglide/messages';
</script>

<footer class="seo-footer border-t border-pink-100 bg-slate-50 py-10 px-6">
	<nav aria-label={m.footer_nav_aria()} class="max-w-7xl mx-auto">
		<div class="grid grid-cols-1 sm:grid-cols-3 gap-8">

			<!-- Product links: money pages -->
			<div>
				<h3 class="seo-footer-heading text-xs font-bold uppercase tracking-widest text-plum/50 mb-3">
					{m.footer_nav_download_tools()}
				</h3>
				<ul class="flex flex-col gap-2">
					<li>
						<a class="seo-footer-link" href="/download-youtube-4k">
							{m.footer_nav_download_4k()}
						</a>
					</li>
					<li>
						<a class="seo-footer-link" href="/download-youtube-8k-hdr">
							{m.footer_nav_download_8k_hdr()}
						</a>
					</li>
					<li>
						<a class="seo-footer-link" href="/download-youtube-playlist">
							{m.footer_nav_download_playlists()}
						</a>
					</li>
					<li>
						<a class="seo-footer-link" href="/download-youtube-shorts">
							{m.footer_nav_download_shorts()}
						</a>
					</li>
					<li>
						<a class="seo-footer-link" href="/download-youtube-mp3">
							{m.footer_nav_download_mp3()}
						</a>
					</li>
				</ul>
			</div>

			<!-- Resources: content hubs -->
			<div>
				<h3 class="seo-footer-heading text-xs font-bold uppercase tracking-widest text-plum/50 mb-3">
					{m.footer_nav_resources()}
				</h3>
				<ul class="flex flex-col gap-2">
					<li>
						<a class="seo-footer-link" href="/guides">
							{m.footer_nav_guides()}
						</a>
					</li>
					<li>
						<a class="seo-footer-link" href="/compare">
							{m.footer_nav_comparisons()}
						</a>
					</li>
				</ul>
			</div>

			<!-- Company / trust pages -->
			<div>
				<h3 class="seo-footer-heading text-xs font-bold uppercase tracking-widest text-plum/50 mb-3">
					{m.footer_nav_company()}
				</h3>
				<ul class="flex flex-col gap-2">
					<li>
						<a class="seo-footer-link" href="/about">{m.footer_nav_about()}</a>
					</li>
					<li>
						<a class="seo-footer-link" href="/contact">{m.footer_nav_contact()}</a>
					</li>
					<li>
						<a class="seo-footer-link" href="/privacy">{m.footer_nav_privacy()}</a>
					</li>
					<li>
						<a class="seo-footer-link" href="/terms">{m.footer_nav_terms()}</a>
					</li>
					<li>
						<a class="seo-footer-link" href="/dmca">{m.footer_nav_dmca()}</a>
					</li>
				</ul>
			</div>
		</div>
	</nav>
</footer>

<style>
	.seo-footer-heading {
		font-family: 'Fredoka', sans-serif;
	}

	.seo-footer-link {
		font-family: 'Nunito', sans-serif;
		font-size: 0.875rem;
		font-weight: 600;
		color: #2d1b36;
		opacity: 0.7;
		text-decoration: none;
		transition: color 0.15s ease, opacity 0.15s ease;
	}

	.seo-footer-link:hover {
		color: #ff4d8c;
		opacity: 1;
	}

	/* Dark mode */
	:global(.page-root.theme-dark) .seo-footer {
		background-color: rgba(18, 18, 26, 0.72);
		border-top-color: rgba(255, 77, 140, 0.14);
	}

	:global(.page-root.theme-dark) .seo-footer-heading {
		color: rgba(224, 208, 245, 0.4);
	}

	:global(.page-root.theme-dark) .seo-footer-link {
		color: rgba(224, 208, 245, 0.6);
		opacity: 1;
	}

	:global(.page-root.theme-dark) .seo-footer-link:hover {
		color: #ff8fb7;
	}
</style>
