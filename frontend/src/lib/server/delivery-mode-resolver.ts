import { env } from '$env/dynamic/private';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

type DeliveryMode = 'hybrid' | 'direct' | 'proxy';

export interface DeliveryDecision {
	deliveryMode: 'direct' | 'proxy';
	reason: string;
}

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

function getDeliveryConfig(): { mode: DeliveryMode; proxyUaRegex: RegExp } {
	const mode = (env.DOWNLOAD_DELIVERY_MODE || 'hybrid') as DeliveryMode;
	const pattern = env.DOWNLOAD_PROXY_UA_REGEX || 'Safari|iPhone|iPad';
	return { mode, proxyUaRegex: new RegExp(pattern, 'i') };
}

// ---------------------------------------------------------------------------
// Core resolver
// ---------------------------------------------------------------------------

/**
 * Decide whether to serve a download directly (R2 signed URL) or via proxy.
 *
 * Rules (evaluated in order):
 * 1. config `proxy`  → always proxy
 * 2. non-absolute URL (local-fs path) → proxy (can't redirect externally)
 * 3. config `direct` → always direct
 * 4. hybrid: if UA matches proxy pattern AND is NOT Chrome/Edge → proxy
 * 5. hybrid default → direct
 */
export function resolveDeliveryMode(
	userAgent: string | null,
	backendDownloadUrl: string
): DeliveryDecision {
	const { mode, proxyUaRegex } = getDeliveryConfig();
	const isAbsoluteUrl = /^https?:\/\//i.test(backendDownloadUrl);

	if (mode === 'proxy') {
		return { deliveryMode: 'proxy', reason: 'config_force_proxy' };
	}

	if (!isAbsoluteUrl) {
		return { deliveryMode: 'proxy', reason: 'backend_local_path' };
	}

	if (mode === 'direct') {
		return { deliveryMode: 'direct', reason: 'config_force_direct' };
	}

	// Hybrid: check UA
	// Chrome and Edge both include "Safari" in their UA — exclude them from proxy.
	if (userAgent && proxyUaRegex.test(userAgent)) {
		const isChrome = /Chrome\//i.test(userAgent);
		const isEdge = /Edg\//i.test(userAgent);
		if (!isChrome && !isEdge) {
			return { deliveryMode: 'proxy', reason: 'ua_match_proxy_pattern' };
		}
	}

	return { deliveryMode: 'direct', reason: 'hybrid_default_direct' };
}

// ---------------------------------------------------------------------------
// UA family classifier (used for audit enrichment)
// ---------------------------------------------------------------------------

export function classifyUserAgentFamily(ua: string | null): string {
	if (!ua) return 'unknown';
	if (/Edg\//i.test(ua)) return 'edge';
	if (/Chrome\//i.test(ua)) return 'chrome';
	if (/Firefox\//i.test(ua)) return 'firefox';
	if (/Safari\//i.test(ua)) return 'safari';
	return 'other';
}
