import { getDatabasePool } from './auth-utils';
import { maskProxyUrl } from './admin-proxy-management';
import type { AdminActivityRow, AdminDashboardData, AdminJobRow, AdminOverview, AdminProxyRow } from '$lib/admin/types';

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
		(SELECT COUNT(*)::int FROM proxies WHERE status = 'active') AS "activeProxies",
		(SELECT COUNT(*)::int FROM proxies WHERE status = 'quarantined') AS "quarantinedProxies",
		(SELECT COUNT(*)::int FROM proxies WHERE status = 'disabled') AS "disabledProxies",
		(
			(SELECT COUNT(*) FROM mux_job_events WHERE created_at >= NOW() - INTERVAL '24 hours')
			+
			(SELECT COUNT(*) FROM proxy_health_events WHERE created_at >= NOW() - INTERVAL '24 hours')
		)::int AS "eventsLast24h"
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
		TO_TIMESTAMP(j.updated_at_ms / 1000.0) AS "updatedAt"
	FROM mux_jobs j
	LEFT JOIN mux_artifacts a ON a.id = j.artifact_id
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
		metrics.last_extract_outcome AS "lastExtractOutcome"
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

const activityQuery = `
	SELECT *
	FROM (
		SELECT
			e.id,
			'job'::text AS scope,
			e.job_id AS "entityId",
			COALESCE(j.title, j.id) AS label,
			e.event_type AS "eventType",
			LEFT(COALESCE(e.payload_json::text, ''), 180) AS detail,
			e.created_at AS "createdAt"
		FROM mux_job_events e
		JOIN mux_jobs j ON j.id = e.job_id
		UNION ALL
		SELECT
			e.id,
			'proxy'::text AS scope,
			p.id AS "entityId",
			COALESCE(NULLIF(p.display_name, ''), p.proxy_url) AS label,
			e.event_type AS "eventType",
			e.reason AS detail,
			e.created_at AS "createdAt"
		FROM proxy_health_events e
		JOIN proxies p ON p.id = e.proxy_id
	) activity
	ORDER BY "createdAt" DESC
	LIMIT 24
`;

export async function loadAdminOverview(): Promise<AdminOverview> {
	const pool = getDatabasePool();
	const overviewResult = await pool.query<OverviewRow>(overviewQuery);

	return (overviewResult.rows[0] ?? {}) as AdminOverview;
}

export async function loadAdminJobs(): Promise<AdminJobRow[]> {
	const pool = getDatabasePool();
	const jobsResult = await pool.query<AdminJobRow>(jobsQuery);

	return jobsResult.rows.map((row) => ({ ...row, updatedAt: normalizeDate(row.updatedAt) ?? row.updatedAt }));
}

export async function loadAdminProxies(): Promise<AdminProxyRow[]> {
	const pool = getDatabasePool();
	const proxiesResult = await pool.query<AdminProxyRow>(proxiesQuery);

	return proxiesResult.rows.map((row) => ({
		...row,
		maskedProxyUrl: maskProxyUrl(row.maskedProxyUrl),
		autoDisabledAt: normalizeDate(row.autoDisabledAt),
		lastQuarantinedAt: normalizeDate(row.lastQuarantinedAt),
		updatedAt: normalizeDate(row.updatedAt) ?? row.updatedAt,
		lastEventAt: normalizeDate(row.lastEventAt)
	}));
}

export async function loadAdminActivity(): Promise<AdminActivityRow[]> {
	const pool = getDatabasePool();
	const activityResult = await pool.query<AdminActivityRow>(activityQuery);

	return activityResult.rows.map((row) => ({
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
