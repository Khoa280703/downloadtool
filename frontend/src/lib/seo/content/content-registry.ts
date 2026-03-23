/**
 * Central registry of all content pages (guides + comparisons).
 * Adding a new page = adding an entry to guide-entries.ts or compare-entries.ts.
 */

import type { ContentEntry, ContentCategory, ContentPageType } from './content-types';
import { GUIDE_ENTRIES } from './guide-entries';
import { COMPARE_ENTRIES } from './compare-entries';

export const CONTENT_REGISTRY: ContentEntry[] = [...GUIDE_ENTRIES, ...COMPARE_ENTRIES];

// ─── Query helpers ────────────────────────────────────────────────────────────

export function getContentBySlug(slug: string, locale = 'en'): ContentEntry | undefined {
	return CONTENT_REGISTRY.find((e) => e.slug === slug && e.locale === locale);
}

export function getContentByType(pageType: ContentPageType, locale = 'en'): ContentEntry[] {
	return CONTENT_REGISTRY.filter((e) => e.pageType === pageType && e.locale === locale);
}

export function getContentByCategory(category: ContentCategory, locale = 'en'): ContentEntry[] {
	return CONTENT_REGISTRY.filter((e) => e.category === category && e.locale === locale);
}
