/**
 * Universal content entry types for the Snapvie content registry.
 * Adding a new page = adding a new entry, no route boilerplate needed.
 */

export type ContentCategory =
	| 'how-to'
	| 'troubleshooting'
	| 'format-quality'
	| 'workflow'
	| 'device'
	| 'comparison'
	| 'best-for'
	| 'education';

export type ContentIntent = 'informational' | 'comparison' | 'transactional';

export type ContentPageType = 'guide' | 'compare';

export interface ContentSection {
	heading: string;
	/** HTML or plain text content for this section */
	content: string;
	type?: 'steps' | 'table' | 'text' | 'faq';
}

export interface ContentEntry {
	slug: string;
	pageType: ContentPageType;
	category: ContentCategory;
	intent: ContentIntent;
	/** BCP-47 locale code, defaults to 'en' */
	locale: string;
	title: string;
	metaDescription: string;
	h1: string;
	subtitle?: string;
	/** Concise answer targeting featured snippets and AI Search */
	quickAnswer: string;
	sections: ContentSection[];
	faqItems?: { q: string; a: string }[];
	/** Slug of the money page this content supports (for internal linking) */
	relatedMoneyPage: string;
	/** Sibling content slugs for related-content widget */
	relatedSlugs: string[];
	/** ISO 8601 date string, e.g. '2026-03-23' */
	datePublished: string;
	/** ISO 8601 date string */
	dateModified: string;
	/** Previous slug to 301-redirect from old URL */
	legacySlug?: string;
	/** Defaults to self (slug within its hub path) */
	canonicalSlug?: string;
}

export interface ContentHubConfig {
	pageType: ContentPageType;
	title: string;
	metaDescription: string;
	h1: string;
	subtitle: string;
}
