/**
 * JSON-LD structured data builders for Snapvie SEO.
 * All schemas use @id anchors for entity linking.
 */

import { SITE_URL } from '$lib/seo/public-pages';
const SITE_NAME = 'Snapvie';
const LOGO_URL = `${SITE_URL}/logo.svg`;
const OG_IMAGE_URL = `${SITE_URL}/og-image.png`;

/** Organization entity — root anchor for all schemas */
export function buildOrganizationSchema() {
	return {
		'@context': 'https://schema.org',
		'@type': 'Organization',
		'@id': `${SITE_URL}/#organization`,
		name: SITE_NAME,
		url: SITE_URL,
		description:
			'Snapvie is a free, ad-free YouTube video downloader supporting 4K, 8K HDR, playlists, Shorts, and audio-only downloads.',
		foundingDate: '2025',
		logo: {
			'@type': 'ImageObject',
			url: LOGO_URL,
			width: 512,
			height: 512
		},
		contactPoint: {
			'@type': 'ContactPoint',
			email: 'support@snapvie.com',
			contactType: 'customer support'
		},
		sameAs: [] as string[]
	};
}

/** WebSite entity — links to Organization, enables SearchAction */
export function buildWebSiteSchema() {
	return {
		'@context': 'https://schema.org',
		'@type': 'WebSite',
		'@id': `${SITE_URL}/#website`,
		url: SITE_URL,
		name: SITE_NAME,
		publisher: { '@id': `${SITE_URL}/#organization` }
	};
}

/** WebApplication entity — describes the tool itself */
export function buildWebApplicationSchema() {
	return {
		'@context': 'https://schema.org',
		'@type': 'WebApplication',
		'@id': `${SITE_URL}/#webapp`,
		name: SITE_NAME,
		url: SITE_URL,
		applicationCategory: 'MultimediaApplication',
		operatingSystem: 'Any',
		offers: {
			'@type': 'Offer',
			price: '0',
			priceCurrency: 'USD'
		},
		screenshot: OG_IMAGE_URL,
		author: { '@id': `${SITE_URL}/#organization` }
	};
}

/** FAQPage entity — inlines Q&A pairs for content relevance */
export function buildFaqSchema(faqs: Array<{ q: string; a: string }>) {
	return {
		'@context': 'https://schema.org',
		'@type': 'FAQPage',
		mainEntity: faqs.map(({ q, a }) => ({
			'@type': 'Question',
			name: q,
			acceptedAnswer: {
				'@type': 'Answer',
				text: a
			}
		}))
	};
}

/** Serialize an array of schema objects as a JSON-LD @graph */
export function buildHomepageJsonLd(): string {
	const graph = [buildOrganizationSchema(), buildWebSiteSchema(), buildWebApplicationSchema()];
	return JSON.stringify({ '@context': 'https://schema.org', '@graph': graph }, null, 0);
}

/** BreadcrumbList entity — improves SERP display with breadcrumb trail */
export function buildBreadcrumbSchema(items: Array<{ name: string; url: string }>) {
	return {
		'@context': 'https://schema.org',
		'@type': 'BreadcrumbList',
		itemListElement: items.map((item, i) => ({
			'@type': 'ListItem',
			position: i + 1,
			name: item.name,
			item: item.url
		}))
	};
}

/** HowTo entity — enables rich snippets for step-by-step guides */
export function buildHowToSchema(name: string, steps: Array<{ name: string; text: string }>) {
	return {
		'@context': 'https://schema.org',
		'@type': 'HowTo',
		name,
		step: steps.map((s, i) => ({
			'@type': 'HowToStep',
			position: i + 1,
			name: s.name,
			text: s.text
		}))
	};
}

/** Serialize JSON-LD @graph for a landing page — includes org, site, webapp, breadcrumb, faq */
export function buildLandingPageJsonLd(
	pageName: string,
	pageUrl: string,
	breadcrumbs: Array<{ name: string; url: string }>,
	faqs: Array<{ q: string; a: string }>
): string {
	const graph = [
		buildOrganizationSchema(),
		buildWebSiteSchema(),
		{ ...buildWebApplicationSchema(), url: pageUrl, name: `${pageName} - Snapvie` },
		buildBreadcrumbSchema(breadcrumbs),
		buildFaqSchema(faqs)
	];
	return JSON.stringify({ '@context': 'https://schema.org', '@graph': graph }, null, 0);
}

/**
 * Article + ItemList schemas for content pages are in:
 * $lib/seo/content/build-page-schema.ts
 * (single source of truth for content page schemas)
 */
