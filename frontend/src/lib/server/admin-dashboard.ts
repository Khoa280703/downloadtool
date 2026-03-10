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
		p.last_quarantined_at AS "lastQuarantinedAt",
		p.last_quarantine_reason AS "lastQuarantineReason",
		p.updated_at AS "updatedAt",
		COALESCE(meta.event_count_24h, 0)::int AS "eventCount24h",
		meta.last_event_type AS "lastEventType",
		meta.last_event_at AS "lastEventAt"
	FROM proxies p
	LEFT JOIN LATERAL (
		SELECT
			COUNT(*) FILTER (WHERE created_at >= NOW() - INTERVAL '24 hours') AS event_count_24h,
			(ARRAY_AGG(event_type ORDER BY created_at DESC))[1] AS last_event_type,
			MAX(created_at) AS last_event_at
		FROM proxy_health_events
		WHERE proxy_id = p.id
	) meta ON TRUE
	ORDER BY
		CASE p.status
			WHEN 'quarantined' THEN 0
			WHEN 'active' THEN 1
			ELSE 2
		END,
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
