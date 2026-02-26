/**
 * Google Analytics 4 tracking utilities
 *
 * Provides event tracking for user interactions and ad performance.
 * All functions are SSR-safe and check for gtag availability.
 */

/** GA4 measurement ID from environment */
const GA_MEASUREMENT_ID = import.meta.env.PUBLIC_GA_MEASUREMENT_ID || '';

/** Global gtag type declaration */
declare global {
	interface Window {
		gtag?: (...args: unknown[]) => void;
		dataLayer?: unknown[];
	}
}

/**
 * Initialize Google Analytics 4
 *
 * Injects the GA4 script tag and configures the tracker.
 * Should be called once during app initialization.
 *
 * @param measurementId - GA4 measurement ID (defaults to env var)
 * @example
 * ```ts
 * initGA('G-XXXXXXXXXX');
 * ```
 */
export function initGA(measurementId: string = GA_MEASUREMENT_ID): void {
	if (typeof window === 'undefined' || !measurementId) return;

	// Prevent duplicate initialization
	if (window.gtag) return;

	// Initialize dataLayer
	window.dataLayer = window.dataLayer || [];

	// Define gtag function
	window.gtag = function (...args: unknown[]) {
		window.dataLayer?.push(args);
	};

	// Configure consent mode (default to denied until user consent)
	window.gtag('consent', 'default', {
		ad_storage: 'denied',
		analytics_storage: 'denied',
		functionality_storage: 'denied',
		personalization_storage: 'denied',
		security_storage: 'granted'
	});

	// Configure GA4
	window.gtag('js', new Date());
	window.gtag('config', measurementId, {
		send_page_view: true,
		anonymize_ip: true,
		allow_google_signals: false,
		restricted_data_processing: true
	});

	// Inject GA4 script
	const script = document.createElement('script');
	script.async = true;
	script.src = `https://www.googletagmanager.com/gtag/js?id=${measurementId}`;
	document.head.appendChild(script);
}

/**
 * Update consent settings for GA4
 *
 * Called when user accepts or rejects cookies.
 *
 * @param hasConsent - Whether user has given consent
 * @example
 * ```ts
 * updateConsent(true); // User accepted
 * ```
 */
export function updateConsent(hasConsent: boolean): void {
	if (typeof window === 'undefined' || !window.gtag) return;

	const consentState = hasConsent ? 'granted' : 'denied';

	window.gtag('consent', 'update', {
		ad_storage: consentState,
		analytics_storage: consentState,
		functionality_storage: consentState,
		personalization_storage: consentState,
		security_storage: 'granted'
	});
}

/**
 * Track a custom event in GA4
 *
 * @param name - Event name (use snake_case)
 * @param params - Optional event parameters
 * @example
 * ```ts
 * trackEvent('download_started', {
 *   platform: 'youtube',
 *   quality: '1080p',
 *   format: 'mp4'
 * });
 * ```
 */
export function trackEvent(
	name: string,
	params?: Record<string, string | number | boolean>
): void {
	if (typeof window === 'undefined' || !window.gtag) return;

	window.gtag('event', name, params);
}

/**
 * Track page view
 *
 * @param path - Page path (defaults to current location)
 * @param title - Page title (defaults to document.title)
 * @example
 * ```ts
 * trackPageView('/privacy-policy', 'Privacy Policy');
 * ```
 */
export function trackPageView(path?: string, title?: string): void {
	if (typeof window === 'undefined' || !window.gtag) return;

	window.gtag('event', 'page_view', {
		page_path: path || window.location.pathname,
		page_title: title || document.title,
		page_location: window.location.href
	});
}

// ============================================
// Predefined Event Trackers
// ============================================

/**
 * Track when user submits a URL for extraction
 *
 * @param platform - Video platform (youtube)
 * @param urlLength - Length of submitted URL
 */
export function trackUrlSubmitted(platform: string, urlLength: number): void {
	trackEvent('url_submitted', {
		platform,
		url_length: urlLength
	});
}

/**
 * Track successful video extraction
 *
 * @param platform - Video platform
 * @param duration - Extraction time in ms
 * @param formatCount - Number of available formats
 */
export function trackExtractSuccess(
	platform: string,
	duration: number,
	formatCount: number
): void {
	trackEvent('extract_success', {
		platform,
		duration_ms: duration,
		format_count: formatCount
	});
}

/**
 * Track extraction error
 *
 * @param platform - Video platform
 * @param errorType - Type of error
 * @param errorCode - Error code if available
 */
export function trackExtractError(
	platform: string,
	errorType: string,
	errorCode?: string
): void {
	trackEvent('extract_error', {
		platform,
		error_type: errorType,
		error_code: errorCode || 'unknown'
	});
}

/**
 * Track when user selects a format
 *
 * @param platform - Video platform
 * @param quality - Selected quality (1080p, 720p, etc.)
 * @param format - File format (mp4, webm, etc.)
 * @param hasAudio - Whether format includes audio
 */
export function trackFormatSelected(
	platform: string,
	quality: string,
	format: string,
	hasAudio: boolean
): void {
	trackEvent('format_selected', {
		platform,
		quality,
		format,
		has_audio: hasAudio
	});
}

/**
 * Track download start
 *
 * @param platform - Video platform
 * @param quality - Download quality
 * @param format - File format
 * @param fileSize - File size in bytes (if known)
 */
export function trackDownloadStarted(
	platform: string,
	quality: string,
	format: string,
	fileSize?: number
): void {
	trackEvent('download_started', {
		platform,
		quality,
		format,
		file_size: fileSize || 0
	});
}

/**
 * Track download completion
 *
 * @param platform - Video platform
 * @param duration - Download duration in ms
 * @param bytesDownloaded - Total bytes downloaded
 */
export function trackDownloadComplete(
	platform: string,
	duration: number,
	bytesDownloaded: number
): void {
	trackEvent('download_complete', {
		platform,
		duration_ms: duration,
		bytes_downloaded: bytesDownloaded
	});
}

/**
 * Track ad impression
 *
 * @param adSlot - Ad slot identifier
 * @param adSize - Ad dimensions
 * @param adNetwork - Ad network name
 */
export function trackAdImpression(
	adSlot: string,
	adSize: string,
	adNetwork: string
): void {
	trackEvent('ad_impression', {
		ad_slot: adSlot,
		ad_size: adSize,
		ad_network: adNetwork
	});
}

/**
 * Track ad click
 *
 * @param adSlot - Ad slot identifier
 * @param adNetwork - Ad network name
 */
export function trackAdClick(adSlot: string, adNetwork: string): void {
	trackEvent('ad_clicked', {
		ad_slot: adSlot,
		ad_network: adNetwork
	});
}

/**
 * Track batch download events
 *
 * @param event - Event type
 * @param urlCount - Number of URLs
 */
export function trackBatchEvent(
	event: 'batch_started' | 'batch_complete' | 'batch_error',
	urlCount: number
): void {
	trackEvent(event, {
		url_count: urlCount
	});
}

/**
 * Track cookie consent interaction
 *
 * @param accepted - Whether user accepted cookies
 * @param source - UI element that triggered the action
 */
export function trackConsent(accepted: boolean, source: string): void {
	trackEvent('cookie_consent', {
		accepted,
		source
	});
}
