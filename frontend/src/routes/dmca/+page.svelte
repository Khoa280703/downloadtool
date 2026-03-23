<script lang="ts">
	import { trackPageView } from '$lib/analytics';
	import { onMount } from 'svelte';
	import * as m from '$lib/paraglide/messages';

	onMount(() => {
		trackPageView('/dmca', m.dmca_meta_title());
	});
</script>

<svelte:head>
	<title>{m.dmca_meta_title()}</title>
	<meta
		name="description"
		content={m.dmca_meta_description()}
	/>
	<link rel="canonical" href="https://snapvie.com/dmca" />
</svelte:head>

<div class="legal-page">
	<h1>{m.dmca_title()}</h1>
	<p class="tagline">{m.dmca_last_updated()}</p>

	<section>
		<h2>{m.dmca_overview_title()}</h2>
		<p>{m.dmca_overview_p1()}</p>
		<p>{m.dmca_overview_p2()}</p>
	</section>

	<section>
		<h2>{m.dmca_notice_title()}</h2>
		<p>{m.dmca_notice_p1()}</p>
		<ul>
			<li>{m.dmca_notice_item1()}</li>
			<li>{m.dmca_notice_item2()}</li>
			<li>{m.dmca_notice_item3()}</li>
			<li>{m.dmca_notice_item4()}</li>
			<li>{m.dmca_notice_item5()}</li>
			<li>{m.dmca_notice_item6()}</li>
		</ul>
		<p>{m.dmca_notice_p2()}</p>
	</section>

	<section>
		<h2>{m.dmca_counter_title()}</h2>
		<p>{m.dmca_counter_p1()}</p>
		<ul>
			<li>{m.dmca_counter_item1()}</li>
			<li>{m.dmca_counter_item2()}</li>
			<li>{m.dmca_counter_item3()}</li>
			<li>{m.dmca_counter_item4()}</li>
		</ul>
		<p>{m.dmca_counter_p2()}</p>
	</section>

	<section>
		<h2>{m.dmca_repeat_title()}</h2>
		<p>{m.dmca_repeat_p1()}</p>
	</section>

	<section>
		<h2>{m.dmca_contact_title()}</h2>
		<p>
			{m.dmca_contact_agent_text()}<a href="mailto:dmca@snapvie.com">dmca@snapvie.com</a>
		</p>
		<p>
			{m.dmca_contact_general_text()}<a href="/contact">{m.dmca_contact_page_link()}</a>.
		</p>
	</section>

	<div class="back-link">
		<a href="/">{m.common_back_to_snapvie()}</a>
	</div>
</div>

<style>
	.legal-page {
		max-width: 980px;
		margin: 0 auto;
		padding: 0.25rem 0.25rem 1.6rem;
	}

	.legal-page h1 {
		font-size: clamp(1.95rem, 4vw, 2.6rem);
		font-weight: 800;
		color: #2d1b36;
		letter-spacing: -0.02em;
		margin-bottom: 0.5rem;
	}

	.tagline {
		display: inline-flex;
		align-items: center;
		border-radius: 999px;
		border: 1px solid rgba(255, 77, 140, 0.2);
		background: rgba(255, 255, 255, 0.8);
		padding: 0.35rem 0.8rem;
		font-size: 0.83rem;
		font-weight: 700;
		color: #7b3f61;
		margin-bottom: 1.4rem;
	}

	.legal-page section {
		margin-bottom: 0.95rem;
		padding: 1rem 1.1rem;
		border-radius: 1.05rem;
		border: 1px solid rgba(255, 77, 140, 0.12);
		background: rgba(255, 255, 255, 0.72);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		box-shadow: 0 20px 28px -26px rgba(255, 77, 140, 0.45);
	}

	.legal-page h2 {
		font-size: 1.14rem;
		font-weight: 800;
		color: #2d1b36;
		margin-bottom: 1rem;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid rgba(255, 77, 140, 0.14);
	}

	.legal-page p {
		font-size: 0.9375rem;
		line-height: 1.7;
		color: #6d4f64;
		margin-bottom: 1rem;
	}

	.legal-page ul {
		list-style: disc;
		padding-left: 1.5rem;
		margin-bottom: 1rem;
	}

	.legal-page li {
		font-size: 0.9375rem;
		line-height: 1.7;
		color: #6d4f64;
		margin-bottom: 0.5rem;
	}

	.legal-page li strong {
		color: #2d1b36;
	}

	.legal-page a {
		color: #ff4d8c;
		font-weight: 700;
		text-decoration: none;
	}

	.legal-page a:hover {
		text-decoration: underline;
	}

	.back-link {
		margin-top: 1.5rem;
		font-size: 0.9rem;
		font-weight: 700;
	}

	@media (max-width: 640px) {
		.legal-page {
			padding: 0.15rem 0 1rem;
		}

		.legal-page h1 {
			font-size: 1.5rem;
		}

		.legal-page h2 {
			font-size: 1.125rem;
		}
	}

	:global(.app.theme-dark) .legal-page h1 {
		color: #ffffff;
	}

	:global(.app.theme-dark) .tagline {
		border-color: rgba(255, 77, 140, 0.3);
		background: rgba(29, 26, 43, 0.86);
		color: rgba(255, 218, 233, 0.92);
	}

	:global(.app.theme-dark) .legal-page section {
		border-color: rgba(255, 77, 140, 0.2);
		background: rgba(28, 27, 40, 0.72);
		box-shadow: 0 16px 24px -22px rgba(255, 77, 140, 0.42);
	}

	:global(.app.theme-dark) .legal-page h2 {
		color: #ffffff;
		border-bottom-color: rgba(255, 77, 140, 0.22);
	}

	:global(.app.theme-dark) .legal-page p,
	:global(.app.theme-dark) .legal-page li {
		color: rgba(224, 208, 245, 0.85);
	}

	:global(.app.theme-dark) .legal-page li strong {
		color: #ffffff;
	}

	:global(.app.theme-dark) .legal-page a {
		color: #ff7caf;
	}
</style>
