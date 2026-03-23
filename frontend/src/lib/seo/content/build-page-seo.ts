/**
 * SEO metadata builders for content hub and detail pages.
 * Matches the base URL and OG conventions used in structured-data.ts.
 */

import type { ContentEntry, ContentHubConfig } from './content-types';
import { hubPath } from './content-types';
import { SITE_URL } from '$lib/seo/public-pages';

const DEFAULT_OG_IMAGE = `${SITE_URL}/og-image.png`;

/** Full canonical URL for a content entry */
export function contentPageUrl(entry: ContentEntry): string {
	return `${SITE_URL}${hubPath(entry.pageType)}/${entry.canonicalSlug ?? entry.slug}`;
}

/** SEO metadata object for a content detail page */
export function buildContentPageMeta(entry: ContentEntry) {
	const url = contentPageUrl(entry);
	// TODO: generate per-page OG images, then switch to `/og/${entry.pageType}-${entry.slug}.png`
	const ogImage = DEFAULT_OG_IMAGE;

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
