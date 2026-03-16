import { getDatabasePool } from './auth-utils';
import type { ProxyStatus } from '$lib/admin/types';

type ProxyMutationInput = {
	proxyId: string;
	status: ProxyStatus;
	reason?: string;
};

type ProxyStatusRow = {
	status: ProxyStatus;
};

export type ProxyMutationResult = 'updated' | 'noop';

export function maskProxyUrl(raw: string): string {
	try {
		const url = new URL(raw);
		const auth = url.username || url.password ? '***:***@' : '';
		return `${url.protocol}//${auth}${url.host}`;
	} catch {
		return raw.replace(/\/\/[^@]+@/, '//***:***@');
	}
}

function normalizeProxyUrl(raw: string): string {
	const value = raw.trim();
	if (/^(socks5h?|https?):\/\//i.test(value)) return value;

	const parts = value.split(':');
	if (parts.length === 4) {
		const [host, port, username, password] = parts;
		return `socks5h://${encodeURIComponent(username)}:${encodeURIComponent(password)}@${host}:${port}`;
	}

	throw new Error('Proxy phải là URL đầy đủ hoặc dạng host:port:user:pass');
}

export async function createProxy(input: {
	proxyUrl: string;
	displayName?: string;
	notes?: string;
}): Promise<void> {
	const pool = getDatabasePool();
	await pool.query(
		`
			INSERT INTO proxies (proxy_url, display_name, status, source, notes)
			VALUES ($1, NULLIF($2, ''), 'active', 'admin', NULLIF($3, ''))
			ON CONFLICT (proxy_url) DO UPDATE
			SET display_name = COALESCE(NULLIF(EXCLUDED.display_name, ''), proxies.display_name),
				notes = COALESCE(NULLIF(EXCLUDED.notes, ''), proxies.notes),
				updated_at = NOW()
		`,
		[normalizeProxyUrl(input.proxyUrl), input.displayName?.trim() ?? '', input.notes?.trim() ?? '']
	);
}

export async function updateProxyStatus(input: ProxyMutationInput): Promise<ProxyMutationResult> {
	const pool = getDatabasePool();
	const client = await pool.connect();
	const reason = input.reason?.trim() || `admin set ${input.status}`;

	try {
		await client.query('BEGIN');
		const currentResult = await client.query<ProxyStatusRow>(
			`SELECT status FROM proxies WHERE id = $1 LIMIT 1`,
			[input.proxyId]
		);
		const current = currentResult.rows[0]?.status;

		if (!current) {
			throw new Error('Proxy not found.');
		}

		if (current === input.status) {
			await client.query('ROLLBACK');
			return 'noop';
		}

		if (current === 'quarantined' && input.status === 'active') {
			throw new Error(
				'Cannot reactivate a quarantined proxy from dashboard yet. Disable it or replace it with a new proxy.'
			);
		}

		await client.query(
			`
				UPDATE proxies
				SET status = $2,
					auto_disabled_at = NULL,
					auto_disabled_reason = NULL,
					last_quarantined_at = CASE WHEN $2 = 'quarantined' THEN NOW() ELSE last_quarantined_at END,
					last_quarantine_reason = CASE WHEN $2 = 'quarantined' THEN $3 ELSE last_quarantine_reason END,
					updated_at = NOW()
				WHERE id = $1
			`,
			[input.proxyId, input.status, reason]
		);
		await client.query(
			`
				INSERT INTO proxy_health_events (proxy_id, event_type, reason, payload_json)
				VALUES ($1, $2, $3, jsonb_build_object('status', $2, 'source', 'admin-dashboard'))
			`,
			[input.proxyId, `admin_${input.status}`, reason]
		);
		await client.query('COMMIT');
		return 'updated';
	} catch (error) {
		await client.query('ROLLBACK');
		throw error;
	} finally {
		client.release();
	}
}
