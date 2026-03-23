/**
 * JSON-LD schema builders for content detail and hub pages.
 * Reuses entity anchors from structured-data.ts (Organization, WebSite).
 * Returns serialized JSON-LD strings ready for <script type="application/ld+json">.
 */

import type { ContentEntry, ContentHubConfig } from './content-types';
import { hubPath } from './content-types';
import { contentPageUrl } from './build-page-seo';
import { SITE_URL } from '$lib/seo/public-pages';

const ORG_ID = `${SITE_URL}/#organization`;
const WEBSITE_ID = `${SITE_URL}/#website`;

/** Article schema node for a content entry */
export function buildArticleSchema(entry: ContentEntry) {
	return {
		'@type': 'Article',
		'@id': `${contentPageUrl(entry)}/#article`,
		headline: entry.h1,
		description: entry.metaDescription,
		datePublished: entry.datePublished,
		dateModified: entry.dateModified,
		author: { '@type': 'Organization', '@id': ORG_ID },
		publisher: { '@type': 'Organization', '@id': ORG_ID },
		isPartOf: { '@id': WEBSITE_ID }
	};
}

/** FAQPage schema node — only included when faqItems are present */
function buildFaqNode(faqItems: { q: string; a: string }[]) {
	return {
		'@type': 'FAQPage',
		mainEntity: faqItems.map(({ q, a }) => ({
			'@type': 'Question',
			name: q,
			acceptedAnswer: { '@type': 'Answer', text: a }
		}))
	};
}

/** BreadcrumbList schema node for a content detail page */
function buildContentBreadcrumb(entry: ContentEntry) {
	const hubLabel = entry.pageType === 'guide' ? 'Guides' : 'Compare';
	const hubUrl = `${SITE_URL}${hubPath(entry.pageType)}`;

	return {
		'@type': 'BreadcrumbList',
		itemListElement: [
			{ '@type': 'ListItem', position: 1, name: 'Snapvie', item: SITE_URL },
			{ '@type': 'ListItem', position: 2, name: hubLabel, item: hubUrl },
			{ '@type': 'ListItem', position: 3, name: entry.h1, item: contentPageUrl(entry) }
		]
	};
}

/**
 * Full JSON-LD @graph for a content detail page.
 * Includes: Article + BreadcrumbList + FAQPage (when faqItems present).
 */
export function buildContentPageJsonLd(entry: ContentEntry): string {
	const graph: object[] = [buildArticleSchema(entry), buildContentBreadcrumb(entry)];

	if (entry.faqItems && entry.faqItems.length > 0) {
		graph.push(buildFaqNode(entry.faqItems));
	}

	return JSON.stringify({ '@context': 'https://schema.org', '@graph': graph }, null, 0);
}

/**
 * Full JSON-LD @graph for a content hub index page (e.g. /guides).
 * Includes: ItemList of entries + BreadcrumbList.
 */
export function buildHubPageJsonLd(config: ContentHubConfig, entries: ContentEntry[]): string {
	const hubUrl = `${SITE_URL}${hubPath(config.pageType)}`;

	const itemList = {
		'@type': 'ItemList',
		name: config.h1,
		description: config.metaDescription,
		url: hubUrl,
		numberOfItems: entries.length,
		itemListElement: entries.map((entry, i) => ({
			'@type': 'ListItem',
			position: i + 1,
			name: entry.h1,
			url: contentPageUrl(entry)
		}))
	};

	const breadcrumb = {
		'@type': 'BreadcrumbList',
		itemListElement: [
			{ '@type': 'ListItem', position: 1, name: 'Snapvie', item: SITE_URL },
			{ '@type': 'ListItem', position: 2, name: config.h1, item: hubUrl }
		]
	};

	return JSON.stringify(
		{ '@context': 'https://schema.org', '@graph': [itemList, breadcrumb] },
		null,
		0
	);
}
