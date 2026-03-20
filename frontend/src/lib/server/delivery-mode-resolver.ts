import { env } from '$env/dynamic/private';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

type DeliveryMode = 'hybrid' | 'direct' | 'proxy';

export interface DeliveryDecision {
	deliveryMode: 'direct' | 'proxy';
	reason: string;
}

function getDeliveryMode(): DeliveryMode {
	return (env.DOWNLOAD_DELIVERY_MODE || 'direct') as DeliveryMode;
}

/**
 * Decide whether to serve a download directly (R2 signed URL) or via proxy.
 *
 * Rules (evaluated in order):
 * 1. config `proxy`  → always proxy
 * 2. non-absolute URL (local-fs path) → proxy (can't redirect externally)
 * 3. every absolute artifact URL goes direct during the R2 rollout
 */
export function resolveDeliveryMode(
	_userAgent: string | null,
	backendDownloadUrl: string
): DeliveryDecision {
	const mode = getDeliveryMode();
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

	return { deliveryMode: 'direct', reason: 'hybrid_direct_r2_rollout' };
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
