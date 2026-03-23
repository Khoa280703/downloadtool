/**
 * SEO metadata builders for content hub and detail pages.
 * Matches the base URL and OG conventions used in structured-data.ts.
 */

import type { ContentEntry, ContentHubConfig, ContentPageType } from './content-types';

const SITE_URL = 'https://snapvie.com';
const DEFAULT_OG_IMAGE = `${SITE_URL}/og-image.png`;

/** Base path for each content page type */
function hubPath(pageType: ContentPageType): string {
	return pageType === 'guide' ? '/guides' : '/compare';
}

/** Full canonical URL for a content entry */
export function contentPageUrl(entry: ContentEntry): string {
	return `${SITE_URL}${hubPath(entry.pageType)}/${entry.canonicalSlug ?? entry.slug}`;
}

/** SEO metadata object for a content detail page */
export function buildContentPageMeta(entry: ContentEntry) {
	const url = contentPageUrl(entry);
	const ogImage = `${SITE_URL}/og/${entry.pageType}-${entry.slug}.png`;

	return {
		title: entry.title,
		description: entry.metaDescription,
		canonical: url,
		og: {
			title: entry.title,
			description: entry.metaDescription,
			url,
			type: 'article' as const,
			image: ogImage
		},
		twitter: {
			card: 'summary_large_image' as const,
			title: entry.title,
			description: entry.metaDescription
		}
	};
}

/** SEO metadata object for a hub index page (e.g. /guides, /compare) */
export function buildHubPageMeta(config: ContentHubConfig) {
	const url = `${SITE_URL}${hubPath(config.pageType)}`;

	return {
		title: config.title,
		description: config.metaDescription,
		canonical: url,
		og: {
			title: config.title,
			description: config.metaDescription,
			url,
			type: 'website' as const,
			image: DEFAULT_OG_IMAGE
		},
		twitter: {
			card: 'summary_large_image' as const,
			title: config.title,
			description: config.metaDescription
		}
	};
}
