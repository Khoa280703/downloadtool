<script lang="ts">
	import { trackPageView } from '$lib/analytics';
	import * as m from '$lib/paraglide/messages';
	import { onMount } from 'svelte';

	onMount(() => {
		trackPageView('/privacy', m.privacy_meta_title());
	});
</script>

<svelte:head>
	<title>{m.privacy_meta_title()}</title>
	<meta name="description" content={m.privacy_meta_description()} />
</svelte:head>

<div class="privacy-page">
	<h1>{m.privacy_title()}</h1>
	<p class="last-updated">{m.privacy_last_updated()}</p>

	<section>
		<h2>{m.privacy_section_1_title()}</h2>
		<p>{m.privacy_section_1_body()}</p>
	</section>

	<section>
		<h2>{m.privacy_section_2_title()}</h2>
		<h3>{m.privacy_section_2_subtitle_1()}</h3>
		<ul>
			<li>
				<strong>{m.privacy_info_video_urls_label()}</strong>
				{m.privacy_info_video_urls_body()}
			</li>
		</ul>

		<h3>{m.privacy_section_2_subtitle_2()}</h3>
		<ul>
			<li>
				<strong>{m.privacy_info_usage_data_label()}</strong>
				{m.privacy_info_usage_data_body()}
			</li>
			<li>
				<strong>{m.privacy_info_device_info_label()}</strong>
				{m.privacy_info_device_info_body()}
			</li>
			<li>
				<strong>{m.privacy_info_cookies_label()}</strong>
				{m.privacy_info_cookies_body()}
			</li>
		</ul>
	</section>

	<section>
		<h2>{m.privacy_section_3_title()}</h2>
		<p>{m.privacy_section_3_intro()}</p>
		<ul>
			<li>{m.privacy_section_3_item_1()}</li>
			<li>{m.privacy_section_3_item_2()}</li>
			<li>{m.privacy_section_3_item_3()}</li>
			<li>{m.privacy_section_3_item_4()}</li>
			<li>{m.privacy_section_3_item_5()}</li>
		</ul>
	</section>

	<section>
		<h2>{m.privacy_section_4_title()}</h2>
		<p>{m.privacy_section_4_body_1()}</p>

		<h3>{m.privacy_section_4_subtitle()}</h3>
		<ul>
			<li>
				<strong>{m.privacy_cookie_essential_label()}</strong>
				{m.privacy_cookie_essential_body()}
			</li>
			<li>
				<strong>{m.privacy_cookie_analytics_label()}</strong>
				{m.privacy_cookie_analytics_body()}
			</li>
			<li>
				<strong>{m.privacy_cookie_preference_label()}</strong>
				{m.privacy_cookie_preference_body()}
			</li>
		</ul>

		<p>{m.privacy_section_4_body_2()}</p>
	</section>

	<section>
		<h2>{m.privacy_section_5_title()}</h2>
		<p>{m.privacy_section_5_intro()}</p>
		<ul>
			<li>
				<strong>{m.privacy_service_google_label()}</strong>
				{m.privacy_service_google_body()}
				<a href="https://policies.google.com/privacy" target="_blank" rel="noopener"
					>{m.privacy_service_google_link()}</a
				>
			</li>
			<li>
				<strong>{m.privacy_service_cloudflare_label()}</strong>
				{m.privacy_service_cloudflare_body()}
				<a href="https://www.cloudflare.com/privacypolicy/" target="_blank" rel="noopener"
					>{m.privacy_service_cloudflare_link()}</a
				>
			</li>
		</ul>
	</section>

	<section>
		<h2>{m.privacy_section_6_title()}</h2>
		<p>{m.privacy_section_6_body()}</p>
	</section>

	<section>
		<h2>{m.privacy_section_7_title()}</h2>
		<p>{m.privacy_section_7_intro()}</p>
		<ul>
			<li>{m.privacy_section_7_item_1()}</li>
			<li>{m.privacy_section_7_item_2()}</li>
			<li>{m.privacy_section_7_item_3()}</li>
		</ul>
	</section>

	<section>
		<h2>{m.privacy_section_8_title()}</h2>
		<p>{m.privacy_section_8_intro()}</p>
		<ul>
			<li>
				<strong>{m.privacy_right_access_label()}</strong>
				{m.privacy_right_access_body()}
			</li>
			<li>
				<strong>{m.privacy_right_rectification_label()}</strong>
				{m.privacy_right_rectification_body()}
			</li>
			<li>
				<strong>{m.privacy_right_erasure_label()}</strong>
				{m.privacy_right_erasure_body()}
			</li>
			<li>
				<strong>{m.privacy_right_restrict_label()}</strong>
				{m.privacy_right_restrict_body()}
			</li>
			<li>
				<strong>{m.privacy_right_portability_label()}</strong>
				{m.privacy_right_portability_body()}
			</li>
			<li>
				<strong>{m.privacy_right_object_label()}</strong>
				{m.privacy_right_object_body()}
			</li>
		</ul>
		<p>{m.privacy_section_8_outro()}</p>
	</section>

	<section>
		<h2>{m.privacy_section_9_title()}</h2>
		<p>{m.privacy_section_9_body()}</p>
	</section>

	<section>
		<h2>{m.privacy_section_10_title()}</h2>
		<p>{m.privacy_section_10_body()}</p>
	</section>

	<section id="contact">
		<h2>{m.privacy_section_11_title()}</h2>
		<p>{m.privacy_section_11_intro()}</p>
		<ul>
			<li>{m.privacy_contact_email()}</li>
			<li>{m.privacy_contact_form()}</li>
		</ul>
	</section>

	<section>
		<h2>{m.privacy_section_12_title()}</h2>
		<p>{m.privacy_section_12_intro()}</p>
		<ul>
			<li>
				<strong>{m.privacy_gdpr_legitimate_interest_label()}</strong>
				{m.privacy_gdpr_legitimate_interest_body()}
			</li>
			<li>
				<strong>{m.privacy_gdpr_consent_label()}</strong>
				{m.privacy_gdpr_consent_body()}
			</li>
		</ul>
		<p>{m.privacy_section_12_outro()}</p>
	</section>
</div>

<style>
	.privacy-page {
		max-width: 980px;
		margin: 0 auto;
		padding: 0.25rem 0.25rem 1.6rem;
	}

	.privacy-page h1 {
		font-size: clamp(1.95rem, 4vw, 2.6rem);
		font-weight: 800;
		color: #2d1b36;
		letter-spacing: -0.02em;
		margin-bottom: 0.75rem;
	}

	.last-updated {
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

	.privacy-page section {
		margin-bottom: 0.95rem;
		padding: 1rem 1.1rem;
		border-radius: 1.05rem;
		border: 1px solid rgba(255, 77, 140, 0.12);
		background: rgba(255, 255, 255, 0.72);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		box-shadow: 0 20px 28px -26px rgba(255, 77, 140, 0.45);
	}

	.privacy-page h2 {
		font-size: 1.14rem;
		font-weight: 800;
		color: #2d1b36;
		margin-bottom: 1rem;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid rgba(255, 77, 140, 0.14);
	}

	.privacy-page h3 {
		font-size: 0.98rem;
		font-weight: 800;
		color: #5b2547;
		margin: 1.5rem 0 0.75rem;
	}

	.privacy-page p {
		font-size: 0.9375rem;
		line-height: 1.7;
		color: #6d4f64;
		margin-bottom: 1rem;
	}

	.privacy-page ul {
		list-style: disc;
		padding-left: 1.5rem;
		margin-bottom: 1rem;
	}

	.privacy-page li {
		font-size: 0.9375rem;
		line-height: 1.7;
		color: #6d4f64;
		margin-bottom: 0.5rem;
	}

	.privacy-page li strong {
		color: #2d1b36;
	}

	.privacy-page a {
		color: #ff4d8c;
		font-weight: 700;
		text-decoration: none;
	}

	.privacy-page a:hover {
		text-decoration: underline;
	}

	@media (max-width: 640px) {
		.privacy-page {
			padding: 0.15rem 0 1rem;
		}

		.privacy-page h1 {
			font-size: 1.5rem;
		}

		.privacy-page h2 {
			font-size: 1.125rem;
		}
	}

	:global(.app.theme-dark) .privacy-page h1 {
		color: #ffffff;
	}

	:global(.app.theme-dark) .privacy-page .last-updated {
		border-color: rgba(255, 77, 140, 0.3);
		background: rgba(29, 26, 43, 0.86);
		color: rgba(255, 218, 233, 0.92);
	}

	:global(.app.theme-dark) .privacy-page section {
		border-color: rgba(255, 77, 140, 0.2);
		background: rgba(28, 27, 40, 0.72);
		box-shadow: 0 16px 24px -22px rgba(255, 77, 140, 0.42);
	}

	:global(.app.theme-dark) .privacy-page h2 {
		color: #ffffff;
		border-bottom-color: rgba(255, 77, 140, 0.22);
	}

	:global(.app.theme-dark) .privacy-page h3 {
		color: #ffc1dd;
	}

	:global(.app.theme-dark) .privacy-page p,
	:global(.app.theme-dark) .privacy-page li {
		color: rgba(224, 208, 245, 0.85);
	}

	:global(.app.theme-dark) .privacy-page li strong {
		color: #ffffff;
	}

	:global(.app.theme-dark) .privacy-page a {
		color: #ff7caf;
	}

	@media (prefers-color-scheme: dark) {
		.privacy-page {
			color-scheme: dark;
		}
	}
</style>
