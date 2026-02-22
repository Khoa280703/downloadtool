<script lang="ts">
	import { trackPageView } from '$lib/analytics';
	import { onMount } from 'svelte';

	onMount(() => {
		trackPageView('/privacy', 'Privacy Policy');
	});
</script>

<svelte:head>
	<title>Privacy Policy | VideoDL</title>
	<meta name="description" content="Privacy policy for VideoDL - free online video downloader. Learn how we handle your data and protect your privacy." />
</svelte:head>

<div class="privacy-page">
	<h1>Privacy Policy</h1>
	<p class="last-updated">Last updated: February 22, 2026</p>

	<section>
		<h2>1. Introduction</h2>
		<p>
			Welcome to VideoDL. We respect your privacy and are committed to protecting your personal data.
			This privacy policy explains how we collect, use, and safeguard your information when you use our video downloader service.
		</p>
	</section>

	<section>
		<h2>2. Information We Collect</h2>
		<h3>2.1 Information You Provide</h3>
		<ul>
			<li>
				<strong>Video URLs:</strong> When you use our service, we temporarily process the video URLs you submit
				to extract download information. These URLs are not stored permanently on our servers.
			</li>
		</ul>

		<h3>2.2 Information Collected Automatically</h3>
		<ul>
			<li>
				<strong>Usage Data:</strong> We collect anonymous usage statistics including pages visited,
				features used, and error reports to improve our service.
			</li>
			<li>
				<strong>Device Information:</strong> We collect technical information such as browser type,
				operating system, and device type to optimize your experience.
			</li>
			<li>
				<strong>Cookies:</strong> We use cookies and similar technologies to enhance your browsing
				experience and serve relevant advertisements. See our Cookie Policy below for details.
			</li>
		</ul>
	</section>

	<section>
		<h2>3. How We Use Your Information</h2>
		<p>We use the collected information for the following purposes:</p>
		<ul>
			<li>To provide and maintain our video downloading service</li>
			<li>To improve and optimize our website performance</li>
			<li>To analyze usage patterns and trends</li>
			<li>To serve relevant advertisements (with your consent)</li>
			<li>To detect and prevent abuse of our service</li>
			<li>To comply with legal obligations</li>
		</ul>
	</section>

	<section>
		<h2>4. Cookies and Tracking Technologies</h2>
		<p>
			We use cookies and similar tracking technologies to track activity on our service and hold certain information.
			Cookies are files with a small amount of data that may include an anonymous unique identifier.
		</p>

		<h3>Types of Cookies We Use:</h3>
		<ul>
			<li>
				<strong>Essential Cookies:</strong> Required for the website to function properly.
				These cannot be disabled.
			</li>
			<li>
				<strong>Analytics Cookies:</strong> Help us understand how visitors interact with our website
				by collecting and reporting information anonymously.
			</li>
			<li>
				<strong>Advertising Cookies:</strong> Used to deliver relevant advertisements and track
				ad performance. These are only set with your explicit consent.
			</li>
		</ul>

		<p>
			You can manage your cookie preferences through the consent banner that appears when you first visit our site,
			or by clearing your browser cookies.
		</p>
	</section>

	<section>
		<h2>5. Third-Party Services</h2>
		<p>We use the following third-party services:</p>
		<ul>
			<li>
				<strong>Google Analytics:</strong> For website analytics and usage statistics.
				<a href="https://policies.google.com/privacy" target="_blank" rel="noopener">Google Privacy Policy</a>
			</li>
			<li>
				<strong>Advertising Partners:</strong> We display advertisements from third-party networks
				including AdsTerra and PropellerAds. These partners may use cookies to personalize ads.
			</li>
			<li>
				<strong>Cloudflare:</strong> For website security and performance optimization.
				<a href="https://www.cloudflare.com/privacypolicy/" target="_blank" rel="noopener">Cloudflare Privacy Policy</a>
			</li>
		</ul>
	</section>

	<section>
		<h2>6. Data Security</h2>
		<p>
			We implement appropriate technical and organizational measures to protect your personal data
			against unauthorized access, alteration, disclosure, or destruction. However, no method of
			transmission over the Internet is 100% secure, and we cannot guarantee absolute security.
		</p>
	</section>

	<section>
		<h2>7. Data Retention</h2>
		<p>
			We retain your information only for as long as necessary to fulfill the purposes outlined in this policy:
		</p>
		<ul>
			<li>Video URLs are processed in real-time and not stored permanently</li>
			<li>Analytics data is retained for up to 26 months</li>
			<li>Cookie consent preferences are stored until you clear your browser data</li>
		</ul>
	</section>

	<section>
		<h2>8. Your Rights</h2>
		<p>Depending on your location, you may have the following rights regarding your personal data:</p>
		<ul>
			<li><strong>Right to Access:</strong> Request a copy of your personal data</li>
			<li><strong>Right to Rectification:</strong> Request correction of inaccurate data</li>
			<li><strong>Right to Erasure:</strong> Request deletion of your personal data</li>
			<li><strong>Right to Restrict Processing:</strong> Request limitation of data processing</li>
			<li><strong>Right to Data Portability:</strong> Request transfer of your data</li>
			<li><strong>Right to Object:</strong> Object to processing of your personal data</li>
		</ul>
		<p>
			To exercise these rights, please contact us using the information provided below.
		</p>
	</section>

	<section>
		<h2>9. Children's Privacy</h2>
		<p>
			Our service is not intended for use by children under the age of 13. We do not knowingly
			collect personal information from children under 13. If you become aware that a child
			has provided us with personal information, please contact us.
		</p>
	</section>

	<section>
		<h2>10. Changes to This Policy</h2>
		<p>
			We may update our Privacy Policy from time to time. We will notify you of any changes by
			posting the new Privacy Policy on this page and updating the "Last updated" date.
			You are advised to review this Privacy Policy periodically for any changes.
		</p>
	</section>

	<section>
		<h2>11. Contact Us</h2>
		<p>
			If you have any questions about this Privacy Policy or our data practices, please contact us:
		</p>
		<ul>
			<li>By email: privacy@videodl.app</li>
			<li>Through our website contact form</li>
		</ul>
	</section>

	<section>
		<h2>12. GDPR Compliance</h2>
		<p>
			For users in the European Economic Area (EEA), we comply with the General Data Protection
			Regulation (GDPR). Our legal basis for processing personal data is:
		</p>
		<ul>
			<li>
				<strong>Legitimate Interest:</strong> For essential service functionality and security
			</li>
			<li>
				<strong>Consent:</strong> For analytics and advertising cookies
			</li>
		</ul>
		<p>
			You have the right to lodge a complaint with a supervisory authority if you believe
			our processing of your personal data violates applicable law.
		</p>
	</section>
</div>

<style>
	.privacy-page {
		max-width: 800px;
		margin: 0 auto;
		padding: 2rem 1.5rem;
	}

	h1 {
		font-size: 2rem;
		font-weight: 700;
		color: var(--text-color, #111827);
		margin-bottom: 0.5rem;
	}

	.last-updated {
		font-size: 0.875rem;
		color: var(--text-secondary, #6b7280);
		margin-bottom: 2rem;
	}

	section {
		margin-bottom: 2rem;
	}

	h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-color, #111827);
		margin-bottom: 1rem;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	h3 {
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-color, #111827);
		margin: 1.5rem 0 0.75rem;
	}

	p {
		font-size: 0.9375rem;
		line-height: 1.7;
		color: var(--text-secondary, #4b5563);
		margin-bottom: 1rem;
	}

	ul {
		list-style: disc;
		padding-left: 1.5rem;
		margin-bottom: 1rem;
	}

	li {
		font-size: 0.9375rem;
		line-height: 1.7;
		color: var(--text-secondary, #4b5563);
		margin-bottom: 0.5rem;
	}

	li strong {
		color: var(--text-color, #111827);
	}

	a {
		color: var(--primary-color, #3b82f6);
		text-decoration: none;
	}

	a:hover {
		text-decoration: underline;
	}

	@media (max-width: 640px) {
		.privacy-page {
			padding: 1.5rem 1rem;
		}

		h1 {
			font-size: 1.5rem;
		}

		h2 {
			font-size: 1.125rem;
		}
	}

	@media (prefers-color-scheme: dark) {
		.privacy-page {
			--text-color: #f9fafb;
			--text-secondary: #d1d5db;
			--border-color: #374151;
		}
	}
</style>
