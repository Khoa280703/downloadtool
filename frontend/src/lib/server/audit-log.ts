import type { RequestEvent } from '@sveltejs/kit';

import { getDatabasePool } from './auth-utils';

const DOWNLOAD_SESSION_COOKIE = 'downloadtool_session';
const MAX_DETAIL_LENGTH = 4000;
const MAX_LABEL_LENGTH = 512;
const MAX_IP_LENGTH = 128;
const MAX_USER_AGENT_LENGTH = 512;

type AuditActorType = 'guest' | 'user' | 'system';

type AuditPayload = Record<string, unknown> | null | undefined;

export type AuditEventInput = {
	scope: string;
	eventType: string;
	entityId?: string | null;
	targetLabel?: string | null;
	routePath?: string | null;
	method?: string | null;
	statusCode?: number | null;
	outcome?: string | null;
	detail?: string | null;
	payload?: AuditPayload;
	actorType?: AuditActorType;
};

function truncate(value: string | null | undefined, limit: number): string | null {
	if (!value) return null;
	const normalized = value.trim();
	if (!normalized) return null;
	return normalized.length > limit ? normalized.slice(0, limit) : normalized;
}

function resolveClientIp(event: Pick<RequestEvent, 'request'>): string | null {
	const headers = event.request.headers;
	const direct =
		headers.get('cf-connecting-ip') ??
		headers.get('x-real-ip') ??
		headers.get('fly-client-ip') ??
		headers.get('x-client-ip');

	if (direct?.trim()) {
		return truncate(direct, MAX_IP_LENGTH);
	}

	const forwarded = headers.get('x-forwarded-for')?.trim();
	if (!forwarded) return null;

	const firstHop = forwarded
		.split(',')
		.map((value) => value.trim())
		.find(Boolean);

	return truncate(firstHop, MAX_IP_LENGTH);
}

function resolveDownloadSessionId(
	event: Pick<RequestEvent, 'cookies' | 'request'>
): string | null {
	return (
		truncate(event.cookies.get(DOWNLOAD_SESSION_COOKIE), MAX_LABEL_LENGTH) ??
		truncate(event.request.headers.get('x-download-session-id'), MAX_LABEL_LENGTH)
	);
}

function normalizePayload(payload: AuditPayload): Record<string, unknown> {
	if (!payload) return {};

	try {
		return JSON.parse(JSON.stringify(payload)) as Record<string, unknown>;
	} catch {
		return {
			unserializable: true
		};
	}
}

export function deriveAuditOutcome(statusCode: number | null | undefined): string {
	if (!statusCode) return 'info';
	if (statusCode >= 200 && statusCode < 300) return 'success';
	if (statusCode === 401 || statusCode === 403) return 'denied';
	if (statusCode === 404) return 'not_found';
	if (statusCode === 408 || statusCode === 429) return 'rate_limited';
	if (statusCode >= 500) return 'error';
	return 'failure';
}

export function sanitizeAuditUrl(raw: string | null | undefined): string | null {
	const trimmed = truncate(raw, 2048);
	if (!trimmed) return null;

	try {
		const url = new URL(trimmed);
		url.username = '';
		url.password = '';
		url.hash = '';

		const keepParams =
			url.hostname.includes('youtube.com') || url.hostname.includes('youtu.be')
				? ['v', 'list', 'index', 't']
				: [];
		const nextParams = new URLSearchParams();
		for (const key of keepParams) {
			for (const value of url.searchParams.getAll(key)) {
				nextParams.append(key, value);
			}
		}
		url.search = nextParams.toString();
		return truncate(url.toString(), 2048);
	} catch {
		return trimmed;
	}
}

export async function logAuditEvent(
	event: Pick<RequestEvent, 'request' | 'locals' | 'cookies' | 'url'>,
	input: AuditEventInput
): Promise<void> {
	const pool = getDatabasePool();
	const actorType: AuditActorType =
		input.actorType ?? (event.locals.user ? 'user' : 'guest');
	const routePath = truncate(input.routePath ?? event.url.pathname, MAX_LABEL_LENGTH);
	const method = truncate(input.method ?? event.request.method, 32);
	const clientIp = resolveClientIp(event);
	const userAgent = truncate(event.request.headers.get('user-agent'), MAX_USER_AGENT_LENGTH);
	const payload = normalizePayload(input.payload);

	try {
		await pool.query(
			`
				INSERT INTO audit_events (
					scope,
					event_type,
					entity_id,
					target_label,
					route_path,
					method,
					status_code,
					outcome,
					actor_type,
					user_id,
					user_email,
					auth_session_id,
					download_session_id,
					client_ip,
					user_agent,
					detail,
					payload_json
				)
				VALUES (
					$1, $2, $3, $4, $5, $6, $7, $8, $9,
					$10, $11, $12, $13, $14, $15, $16, $17::jsonb
				)
			`,
			[
				input.scope,
				input.eventType,
				truncate(input.entityId, MAX_LABEL_LENGTH),
				truncate(input.targetLabel, MAX_LABEL_LENGTH),
				routePath,
				method,
				input.statusCode ?? null,
				truncate(input.outcome ?? 'info', 64),
				actorType,
				truncate(event.locals.user?.id, MAX_LABEL_LENGTH),
				truncate(event.locals.user?.email ?? null, MAX_LABEL_LENGTH),
				truncate(event.locals.session?.id ?? null, MAX_LABEL_LENGTH),
				resolveDownloadSessionId(event),
				clientIp,
				userAgent,
				truncate(input.detail, MAX_DETAIL_LENGTH),
				JSON.stringify(payload)
			]
		);
	} catch (error) {
		console.error('Failed to persist audit event:', error);
	}
}
