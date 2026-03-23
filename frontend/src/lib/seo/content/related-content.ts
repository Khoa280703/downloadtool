/**
 * Related content resolver for content pages.
 * Priority: explicitly listed relatedSlugs → same-category fill.
 */

import type { ContentEntry } from './content-types';
import {
	getContentBySlug,
	getContentByCategory
} from './content-registry';

export function getRelatedContent(slug: string, limit = 3): ContentEntry[] {
	const entry = getContentBySlug(slug);
	if (!entry) return [];

	// First: explicitly listed related slugs
	const explicit = entry.relatedSlugs
		.map((s) => getContentBySlug(s, entry.locale))
		.filter((e): e is ContentEntry => e !== undefined);

	if (explicit.length >= limit) return explicit.slice(0, limit);

	// Fill remaining with same-category content, excluding self and already listed
	const excluded = new Set([slug, ...entry.relatedSlugs]);
	const sameCategory = getContentByCategory(entry.category, entry.locale).filter(
		(e) => !excluded.has(e.slug)
	);

	return [...explicit, ...sameCategory].slice(0, limit);
}
