/**
 * SEO-specific event tracking for landing pages and content clusters.
 *
 * Complements the core analytics.ts with page-group metadata so GA4
 * can segment organic traffic by intent (money, guide, compare, trust).
 * All functions are SSR-safe (delegated to trackEvent which guards on window).
 */

import { trackEvent } from '../analytics';

/** SEO page groups matching landing-page content taxonomy */
export type SeoPageGroup = 'homepage' | 'money' | 'guide' | 'compare' | 'trust' | 'other';

export interface SeoEventParams {
	page_group: SeoPageGroup;
	page_slug: string;
	cluster?: string;
	locale: string;
}

/** Build a flat GA4-compatible params object, omitting undefined optional fields */
function buildParams(
	params: SeoEventParams,
	extra?: Record<string, string | number | boolean>
): Record<string, string | number | boolean> {
	const base: Record<string, string | number | boolean> = {
		page_group: params.page_group,
		page_slug: params.page_slug,
		locale: params.locale
	};
	if (params.cluster !== undefined) base.cluster = params.cluster;
	if (extra) Object.assign(base, extra);
	return base;
}

/**
 * Track SEO page view with group metadata.
 *
 * @param params - Page group, slug, optional cluster, and locale
 */
export function trackSeoPageView(params: SeoEventParams): void {
	trackEvent('seo_page_view', buildParams(params));
}

/**
 * Track when user focuses on URL input from an SEO landing page.
 * Signals intent — user arrived from organic search and is about to use the tool.
 *
 * @param params - Page group, slug, optional cluster, and locale
 */
export function trackSeoInputFocus(params: SeoEventParams): void {
	trackEvent('seo_input_focus', buildParams(params));
}

/**
 * Track URL extract submission from an SEO landing page.
 *
 * @param params - Page group, slug, optional cluster, and locale
 */
export function trackSeoExtractSubmit(params: SeoEventParams): void {
	trackEvent('seo_extract_submit', buildParams(params));
}

/**
 * Track successful extraction from an SEO landing page.
 *
 * @param params - Page metadata plus number of returned formats
 */
export function trackSeoExtractSuccess(params: SeoEventParams & { format_count: number }): void {
	trackEvent('seo_extract_success', buildParams(params, { format_count: params.format_count }));
}

/**
 * Track download start initiated from an SEO landing page.
 *
 * @param params - Page metadata plus optional quality string (e.g. "4K", "1080p")
 */
export function trackSeoDownloadStart(params: SeoEventParams & { quality?: string }): void {
	const extra: Record<string, string | number | boolean> = {};
	if (params.quality !== undefined) extra.quality = params.quality;
	trackEvent('seo_download_start', buildParams(params, extra));
}

/**
 * Track playlist/batch download start initiated from an SEO landing page.
 *
 * @param params - Page metadata plus number of URLs in the batch
 */
export function trackSeoPlaylistStart(params: SeoEventParams & { url_count: number }): void {
	trackEvent('seo_playlist_start', buildParams(params, { url_count: params.url_count }));
}
