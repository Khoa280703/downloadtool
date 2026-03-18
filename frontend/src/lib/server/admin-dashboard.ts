import { env } from '$env/dynamic/private';
import { getDatabasePool, getProxyDatabasePool } from './auth-utils';
import { maskProxyUrl } from './admin-proxy-management';
import type { AdminActivityRow, AdminDashboardData, AdminJobRow, AdminOverview, AdminPlaylistJobRow, AdminProxyRow } from '$lib/admin/types';

type OverviewRow = Record<string, number>;

function normalizeDate(value: Date | string | null | undefined): string | null {
	if (!value) return null;
	const parsed = value instanceof Date ? value : new Date(value);
	return Number.isNaN(parsed.getTime()) ? null : parsed.toISOString();
}

const overviewQuery = `
	SELECT
		(SELECT COUNT(*)::int FROM mux_jobs WHERE status = 'queued') AS "queuedJobs",
		(SELECT COUNT(*)::int FROM mux_jobs WHERE status = 'leased') AS "leasedJobs",
		(SELECT COUNT(*)::int FROM mux_jobs WHERE status = 'processing') AS "processingJobs",
		(SELECT COUNT(*)::int FROM mux_jobs WHERE status = 'ready') AS "readyJobs",
		(SELECT COUNT(*)::int FROM mux_jobs WHERE status = 'failed') AS "failedJobs",
		(SELECT COUNT(*)::int FROM mux_jobs WHERE status = 'expired') AS "expiredJobs",
		(SELECT COUNT(*)::int FROM mux_artifacts WHERE status = 'building') AS "buildingArtifacts",
		(SELECT COUNT(*)::int FROM mux_artifacts WHERE status = 'ready') AS "readyArtifacts",
		(SELECT COUNT(*) FROM mux_job_events WHERE created_at >= NOW() - INTERVAL '24 hours')::int AS "jobEventsLast24h"
`;

const playlistOverviewQuery = `
	SELECT
		(SELECT COUNT(*)::int FROM playlist_jobs WHERE status IN ('queued', 'discovering', 'processing')) AS "playlistActiveJobs",
		(SELECT COUNT(*)::int FROM playlist_jobs WHERE status = 'completed' AND updated_at >= NOW() - INTERVAL '24 hours') AS "playlistCompletedJobs24h",
		(SELECT COUNT(*)::int FROM playlist_jobs WHERE status = 'failed' AND updated_at >= NOW() - INTERVAL '24 hours') AS "playlistFailedJobs24h"
`;

const playlistJobsQuery = `
	SELECT
		id,
		source_url AS "sourceUrl",
		title,
		status,
		total_items AS "totalItems",
		completed_items AS "completedItems",
		failed_items AS "failedItems",
		requested_quality AS "requestedQuality",
		requested_mode AS "requestedMode",
		COALESCE(NULLIF(user_id, ''), NULLIF(session_id, ''), NULLIF(request_ip, ''), 'guest') AS "ownerLabel",
		TO_CHAR(TO_TIMESTAMP(created_at_ms / 1000.0), 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS "createdAt",
		TO_CHAR(TO_TIMESTAMP(updated_at_ms / 1000.0), 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS "updatedAt"
	FROM playlist_jobs
	ORDER BY updated_at_ms DESC
	LIMIT 50
`;

const auditOverviewQuery = `
	SELECT COUNT(*)::int AS "auditEventsLast24h"
	FROM audit_events
	WHERE created_at >= NOW() - INTERVAL '24 hours'
`;

const proxyOverviewQuery = `
	SELECT
		(SELECT COUNT(*)::int FROM proxies WHERE status = 'active') AS "activeProxies",
		(SELECT COUNT(*)::int FROM proxies WHERE status = 'quarantined') AS "quarantinedProxies",
		(SELECT COUNT(*)::int FROM proxies WHERE status = 'disabled') AS "disabledProxies",
		(SELECT COUNT(*) FROM proxy_health_events WHERE created_at >= NOW() - INTERVAL '24 hours')::int AS "proxyEventsLast24h"
`;

const jobsQuery = `
	SELECT
		j.id,
		j.title,
		j.status,
		COALESCE(NULLIF(j.user_id, ''), NULLIF(j.session_id, ''), 'guest') AS "ownerLabel",
		(j.attempt_count::text || '/' || GREATEST(j.max_attempts, 1)::text) AS "attemptLabel",
		a.size_bytes AS "fileSizeBytes",
		a.backend,
		j.last_error AS "lastError",
		j.source_url AS "sourceUrl",
		TO_TIMESTAMP(j.updated_at_ms / 1000.0) AS "updatedAt",
		jsonb_build_object(
			'userId', j.user_id,
			'sessionId', j.session_id,
			'requestHash', j.request_hash,
			'dedupeKey', j.dedupe_key,
			'sourceUrl', j.source_url,
			'videoUrl', j.video_url,
			'audioUrl', j.audio_url,
			'videoFormatId', j.video_format_id,
			'audioFormatId', j.audio_format_id,
			'leaseOwner', j.lease_owner,
			'leaseExpiresAtMs', j.lease_expires_at_ms,
			'createdAtMs', j.created_at_ms,
			'updatedAtMs', j.updated_at_ms,
			'createdAt', j.created_at,
			'updatedAt', j.updated_at,
			'completedAt', j.completed_at,
			'deleteAfterAt', j.delete_after_at,
			'artifact', jsonb_build_object(
				'id', a.id,
				'dedupeKey', a.dedupe_key,
				'artifactKey', a.artifact_key,
				'backend', a.backend,
				'localPath', a.local_path,
				'storageBucket', a.storage_bucket,
				'objectKey', a.object_key,
				'contentType', a.content_type,
				'status', a.status,
				'sizeBytes', a.size_bytes,
				'etag', a.etag,
				'sha256', a.sha256,
				'createdAt', a.created_at,
				'readyAt', a.ready_at,
				'expiresAt', a.expires_at,
				'lastAccessedAt', a.last_accessed_at
			),
			'recentEvents', COALESCE(job_events.recent_events, '[]'::jsonb)
		) AS "detailJson"
	FROM mux_jobs j
	LEFT JOIN mux_artifacts a ON a.id = j.artifact_id
	LEFT JOIN LATERAL (
		SELECT jsonb_agg(
			jsonb_build_object(
				'id', e.id,
				'eventType', e.event_type,
				'payload', e.payload_json,
				'createdAt', e.created_at
			)
			ORDER BY e.created_at DESC
		) AS recent_events
		FROM (
			SELECT id, event_type, payload_json, created_at
			FROM mux_job_events
			WHERE job_id = j.id
			ORDER BY created_at DESC
			LIMIT 12
		) e
	) job_events ON TRUE
	ORDER BY j.updated_at_ms DESC
	LIMIT 20
`;

const proxiesQuery = `
	SELECT
		p.id,
		p.proxy_url AS "maskedProxyUrl",
		p.display_name AS "displayName",
		p.status,
		p.source,
		p.notes,
		p.health_score::int AS "healthScore",
		p.auto_disabled_at AS "autoDisabledAt",
		p.auto_disabled_reason AS "autoDisabledReason",
		p.last_quarantined_at AS "lastQuarantinedAt",
		p.last_quarantine_reason AS "lastQuarantineReason",
		CASE
			WHEN p.status = 'quarantined' AND p.last_quarantined_at IS NOT NULL
				THEN p.last_quarantined_at + make_interval(secs => $1::double precision)
			ELSE NULL
		END AS "quarantineExpiresAt",
		p.updated_at AS "updatedAt",
		COALESCE(meta.event_count_24h, 0)::int AS "eventCount24h",
		meta.last_event_type AS "lastEventType",
		meta.last_event_at AS "lastEventAt",
		COALESCE(metrics.extract_attempts_24h, 0)::int AS "extractAttempts24h",
		COALESCE(metrics.proxy_relevant_attempts_24h, 0)::int AS "proxyRelevantAttempts24h",
		COALESCE(metrics.extract_successes_24h, 0)::int AS "extractSuccesses24h",
		COALESCE(metrics.full_format_hits_24h, 0)::int AS "fullFormatHits24h",
		COALESCE(metrics.combined_360_only_hits_24h, 0)::int AS "combined360OnlyHits24h",
		COALESCE(metrics.timeout_hits_24h, 0)::int AS "timeoutHits24h",
		metrics.p95_extract_latency_ms::double precision AS "p95ExtractLatencyMs",
		metrics.last_extract_outcome AS "lastExtractOutcome",
		COALESCE(downloads.download_access_count, 0)::int AS "downloadAccessCount",
		COALESCE(downloads.download_accesses_24h, 0)::int AS "downloadAccesses24h",
		COALESCE(downloads.mux_job_accesses_24h, 0)::int AS "muxJobAccesses24h",
		COALESCE(downloads.direct_stream_accesses_24h, 0)::int AS "directStreamAccesses24h",
		downloads.last_download_access_at AS "lastDownloadAccessAt"
	FROM proxies p
	LEFT JOIN LATERAL (
		SELECT
			COUNT(*) FILTER (WHERE created_at >= NOW() - INTERVAL '24 hours') AS event_count_24h,
			(ARRAY_AGG(event_type ORDER BY created_at DESC))[1] AS last_event_type,
			MAX(created_at) AS last_event_at
		FROM proxy_health_events
		WHERE proxy_id = p.id
	) meta ON TRUE
	LEFT JOIN LATERAL (
		SELECT
			COUNT(*)::bigint AS extract_attempts_24h,
			COUNT(*) FILTER (
				WHERE COALESCE((payload_json->>'success')::boolean, false)
				   OR COALESCE(payload_json->>'failure_kind', '') <> 'not_proxy_related'
			)::bigint AS proxy_relevant_attempts_24h,
			COUNT(*) FILTER (
				WHERE COALESCE((payload_json->>'success')::boolean, false)
			)::bigint AS extract_successes_24h,
			COUNT(*) FILTER (
				WHERE COALESCE((payload_json->>'full_format_available')::boolean, false)
			)::bigint AS full_format_hits_24h,
			COUNT(*) FILTER (
				WHERE COALESCE((payload_json->>'combined_360_only')::boolean, false)
			)::bigint AS combined_360_only_hits_24h,
			COUNT(*) FILTER (
				WHERE COALESCE(payload_json->>'failure_kind', '') IN ('transport_dead', 'subprocess_timeout')
			)::bigint AS timeout_hits_24h,
			percentile_cont(0.95) WITHIN GROUP (
				ORDER BY (payload_json->>'elapsed_ms')::double precision
			) FILTER (
				WHERE payload_json ? 'elapsed_ms'
			) AS p95_extract_latency_ms,
			(ARRAY_AGG(payload_json->>'outcome' ORDER BY created_at DESC))[1] AS last_extract_outcome
		FROM proxy_health_events
		WHERE proxy_id = p.id
		  AND event_type = 'extract_result'
		  AND created_at >= NOW() - INTERVAL '24 hours'
	) metrics ON TRUE
	LEFT JOIN LATERAL (
		SELECT
			COUNT(*)::bigint AS download_access_count,
			COUNT(*) FILTER (
				WHERE created_at >= NOW() - INTERVAL '24 hours'
			)::bigint AS download_accesses_24h,
			COUNT(*) FILTER (
				WHERE created_at >= NOW() - INTERVAL '24 hours'
				  AND COALESCE(payload_json->>'kind', '') = 'mux_job'
			)::bigint AS mux_job_accesses_24h,
			COUNT(*) FILTER (
				WHERE created_at >= NOW() - INTERVAL '24 hours'
				  AND COALESCE(payload_json->>'kind', '') LIKE 'direct_stream%'
			)::bigint AS direct_stream_accesses_24h,
			MAX(created_at) AS last_download_access_at
		FROM proxy_health_events
		WHERE proxy_id = p.id
		  AND event_type = 'download_access'
	) downloads ON TRUE
	ORDER BY
		CASE p.status
			WHEN 'quarantined' THEN 0
			WHEN 'disabled' THEN 1
			ELSE 2
		END,
		p.health_score ASC,
		p.updated_at DESC
	LIMIT 40
`;

function getProxyQuarantineTtlSecs(): number {
	const parsed = Number(env.PROXY_QUARANTINE_TTL_SECS ?? '172800');
	return Number.isFinite(parsed) && parsed > 0 ? parsed : 172800;
}

const jobActivityQuery = `
	SELECT
		e.id,
		'job'::text AS source,
		'job'::text AS scope,
		e.job_id AS "entityId",
		COALESCE(j.title, j.id) AS label,
		e.event_type AS "eventType",
		LEFT(COALESCE(e.payload_json::text, ''), 180) AS detail,
		NULL::text AS "actorLabel",
		NULL::text AS "clientIp",
		NULL::text AS "routePath",
		NULL::text AS method,
		NULL::int AS "statusCode",
		NULL::text AS outcome,
		e.created_at AS "createdAt",
		jsonb_build_object(
			'jobStatus', j.status,
			'title', j.title,
			'userId', j.user_id,
			'sessionId', j.session_id,
			'lastError', j.last_error,
			'sourceUrl', j.source_url,
			'payload', e.payload_json
		) AS "detailJson"
	FROM mux_job_events e
	JOIN mux_jobs j ON j.id = e.job_id
	ORDER BY e.created_at DESC
	LIMIT 40
`;

const proxyActivityQuery = `
	SELECT
		e.id,
		'proxy'::text AS source,
		'proxy'::text AS scope,
		p.id AS "entityId",
		COALESCE(NULLIF(p.display_name, ''), p.proxy_url) AS label,
		e.event_type AS "eventType",
		e.reason AS detail,
		NULL::text AS "actorLabel",
		NULL::text AS "clientIp",
		NULL::text AS "routePath",
		NULL::text AS method,
		NULL::int AS "statusCode",
		NULL::text AS outcome,
		e.created_at AS "createdAt",
		jsonb_build_object(
			'proxyUrl', p.proxy_url,
			'displayName', p.display_name,
			'status', p.status,
			'source', p.source,
			'notes', p.notes,
			'healthScore', p.health_score,
			'autoDisabledAt', p.auto_disabled_at,
			'autoDisabledReason', p.auto_disabled_reason,
			'lastQuarantinedAt', p.last_quarantined_at,
			'lastQuarantineReason', p.last_quarantine_reason,
			'reason', e.reason,
			'payload', e.payload_json
		) AS "detailJson"
	FROM proxy_health_events e
	JOIN proxies p ON p.id = e.proxy_id
	ORDER BY e.created_at DESC
	LIMIT 40
`;

const auditActivityQuery = `
	SELECT
		id,
		'audit'::text AS source,
		scope,
		entity_id AS "entityId",
		COALESCE(NULLIF(target_label, ''), NULLIF(route_path, ''), event_type) AS label,
		event_type AS "eventType",
		detail,
		COALESCE(NULLIF(user_email, ''), NULLIF(user_id, ''), NULLIF(download_session_id, ''), NULLIF(client_ip, ''), actor_type) AS "actorLabel",
		client_ip AS "clientIp",
		route_path AS "routePath",
		method,
		status_code AS "statusCode",
		outcome,
		created_at AS "createdAt",
		jsonb_build_object(
			'scope', scope,
			'entityId', entity_id,
			'targetLabel', target_label,
			'routePath', route_path,
			'method', method,
			'statusCode', status_code,
			'outcome', outcome,
			'actorType', actor_type,
			'userId', user_id,
			'userEmail', user_email,
			'authSessionId', auth_session_id,
			'downloadSessionId', download_session_id,
			'clientIp', client_ip,
			'userAgent', user_agent,
			'detail', detail,
			'payload', payload_json
		) AS "detailJson"
	FROM audit_events
	ORDER BY created_at DESC
	LIMIT 80
`;

type QueryError = Error & {
	code?: string;
};

async function safeLoadPlaylistOverview(): Promise<Record<string, number>> {
	const pool = getDatabasePool();
	try {
		const result = await pool.query<OverviewRow>(playlistOverviewQuery);
		return result.rows[0] ?? {};
	} catch (error) {
		if ((error as QueryError)?.code !== '42P01') {
			console.error('Failed to query playlist overview:', error);
		}
		return {};
	}
}

async function safeLoadAuditEventsLast24h(): Promise<number> {
	const pool = getDatabasePool();
	try {
		const result = await pool.query<OverviewRow>(auditOverviewQuery);
		return Number(result.rows[0]?.auditEventsLast24h ?? 0);
	} catch (error) {
		if ((error as QueryError)?.code !== '42P01') {
			console.error('Failed to query audit event totals:', error);
		}
		return 0;
	}
}

async function safeLoadAuditActivity(): Promise<AdminActivityRow[]> {
	const pool = getDatabasePool();
	try {
		const result = await pool.query<AdminActivityRow>(auditActivityQuery);
		return result.rows;
	} catch (error) {
		if ((error as QueryError)?.code !== '42P01') {
			console.error('Failed to query audit activity:', error);
		}
		return [];
	}
}

export async function loadAdminOverview(): Promise<AdminOverview> {
	const appPool = getDatabasePool();
	const proxyPool = getProxyDatabasePool();
	const [overviewResult, proxyOverviewResult, auditEventsLast24h, playlistOverviewResult] = await Promise.all([
		appPool.query<OverviewRow>(overviewQuery),
		proxyPool.query<OverviewRow>(proxyOverviewQuery),
		safeLoadAuditEventsLast24h(),
		safeLoadPlaylistOverview()
	]);
	const jobOverview = overviewResult.rows[0] ?? {};
	const proxyOverview = proxyOverviewResult.rows[0] ?? {};

	return {
		...(jobOverview as AdminOverview),
		activeProxies: Number(proxyOverview.activeProxies ?? 0),
		quarantinedProxies: Number(proxyOverview.quarantinedProxies ?? 0),
		disabledProxies: Number(proxyOverview.disabledProxies ?? 0),
		eventsLast24h:
			Number(jobOverview.jobEventsLast24h ?? 0) +
			auditEventsLast24h +
			Number(proxyOverview.proxyEventsLast24h ?? 0),
		playlistActiveJobs: Number(playlistOverviewResult.playlistActiveJobs ?? 0),
		playlistCompletedJobs24h: Number(playlistOverviewResult.playlistCompletedJobs24h ?? 0),
		playlistFailedJobs24h: Number(playlistOverviewResult.playlistFailedJobs24h ?? 0)
	} as AdminOverview;
}

export async function loadAdminJobs(): Promise<AdminJobRow[]> {
	const pool = getDatabasePool();
	const jobsResult = await pool.query<AdminJobRow>(jobsQuery);

	return jobsResult.rows.map((row) => ({ ...row, updatedAt: normalizeDate(row.updatedAt) ?? row.updatedAt }));
}

export async function loadAdminProxies(): Promise<AdminProxyRow[]> {
	const pool = getProxyDatabasePool();
	const proxiesResult = await pool.query<AdminProxyRow>(proxiesQuery, [getProxyQuarantineTtlSecs()]);

	return proxiesResult.rows.map((row) => ({
		...row,
		maskedProxyUrl: maskProxyUrl(row.maskedProxyUrl),
		autoDisabledAt: normalizeDate(row.autoDisabledAt),
		lastQuarantinedAt: normalizeDate(row.lastQuarantinedAt),
		quarantineExpiresAt: normalizeDate(row.quarantineExpiresAt),
		updatedAt: normalizeDate(row.updatedAt) ?? row.updatedAt,
		lastEventAt: normalizeDate(row.lastEventAt),
		lastDownloadAccessAt: normalizeDate(row.lastDownloadAccessAt)
	}));
}

export async function loadAdminPlaylistJobs(): Promise<AdminPlaylistJobRow[]> {
	const pool = getDatabasePool();
	try {
		const result = await pool.query<AdminPlaylistJobRow>(playlistJobsQuery);
		return result.rows;
	} catch (error) {
		if ((error as QueryError)?.code !== '42P01') {
			console.error('Failed to query playlist jobs:', error);
		}
		return [];
	}
}

export async function loadAdminActivity(): Promise<AdminActivityRow[]> {
	const appPool = getDatabasePool();
	const proxyPool = getProxyDatabasePool();
	const [jobActivityResult, auditActivityResult, proxyActivityResult] = await Promise.all([
		appPool.query<AdminActivityRow>(jobActivityQuery),
		safeLoadAuditActivity(),
		proxyPool.query<AdminActivityRow>(proxyActivityQuery)
	]);

	return [...jobActivityResult.rows, ...proxyActivityResult.rows, ...auditActivityResult]
		.sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime())
		.slice(0, 80)
		.map((row) => ({
			...row,
			createdAt: normalizeDate(row.createdAt) ?? row.createdAt
		}));
}

export async function loadAdminDashboard(): Promise<AdminDashboardData> {
	const [overview, jobs, proxies, activity] = await Promise.all([
		loadAdminOverview(),
		loadAdminJobs(),
		loadAdminProxies(),
		loadAdminActivity()
	]);

	return {
		overview,
		jobs,
		proxies,
		activity
	};
}
