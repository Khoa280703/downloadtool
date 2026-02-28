import { building } from '$app/environment';
import { env } from '$env/dynamic/private';
import { Pool } from 'pg';

type JwtTokenProvider = {
	api: {
		getToken: (context: { headers: Headers }) => Promise<{ token: string } | null | undefined>;
	};
};

type SubscriptionQueryRow = {
	plan: string;
	status: string;
	current_period_end: Date | string | null;
	whop_membership_id: string | null;
};

export type SubscriptionSummary = {
	plan: 'free' | 'premium';
	status: 'active' | 'expired';
	currentPeriodEnd: string | null;
	whopMembershipId: string | null;
};

const globalPool = globalThis as typeof globalThis & { __frontendAuthDbPool?: Pool };

function requireEnv(name: keyof typeof env, fallback = ''): string {
	const value = env[name];
	if (!value && !building) {
		throw new Error(`${name} is required`);
	}
	return value ?? fallback;
}

function parseTimestamp(value: Date | string | null): string | null {
	if (!value) return null;
	if (value instanceof Date) return value.toISOString();
	const parsed = new Date(value);
	return Number.isNaN(parsed.getTime()) ? null : parsed.toISOString();
}

function mapSubscriptionRow(row: SubscriptionQueryRow | undefined): SubscriptionSummary {
	if (!row) {
		return {
			plan: 'free',
			status: 'expired',
			currentPeriodEnd: null,
			whopMembershipId: null
		};
	}

	const plan = row.plan === 'premium' ? 'premium' : 'free';
	const status = row.status === 'active' ? 'active' : 'expired';

	return {
		plan,
		status,
		currentPeriodEnd: parseTimestamp(row.current_period_end),
		whopMembershipId: row.whop_membership_id
	};
}

export function getDatabasePool(): Pool {
	if (!globalPool.__frontendAuthDbPool) {
		globalPool.__frontendAuthDbPool = new Pool({
			connectionString: requireEnv(
				'DATABASE_URL',
				'postgres://postgres:postgres@127.0.0.1:5432/postgres'
			)
		});
	}
	return globalPool.__frontendAuthDbPool;
}

export async function getUserTier(userId: string): Promise<'free' | 'premium'> {
	const pool = getDatabasePool();
	try {
		const result = await pool.query<SubscriptionQueryRow>(
			`
			SELECT plan, status, current_period_end, whop_membership_id
			FROM subscriptions
			WHERE user_id = $1
			LIMIT 1
			`,
			[userId]
		);

		const subscription = mapSubscriptionRow(result.rows[0]);
		if (subscription.plan !== 'premium') return 'free';
		if (subscription.status !== 'active') return 'free';
		if (!subscription.currentPeriodEnd) return 'premium';

		return new Date(subscription.currentPeriodEnd) > new Date() ? 'premium' : 'free';
	} catch (error) {
		console.error('getUserTier query failed, fallback free tier:', error);
		return 'free';
	}
}

export async function getSubscriptionForUser(userId: string): Promise<SubscriptionSummary> {
	const pool = getDatabasePool();
	try {
		const result = await pool.query<SubscriptionQueryRow>(
			`
			SELECT plan, status, current_period_end, whop_membership_id
			FROM subscriptions
			WHERE user_id = $1
			LIMIT 1
			`,
			[userId]
		);

		return mapSubscriptionRow(result.rows[0]);
	} catch (error) {
		console.error('getSubscriptionForUser query failed, fallback free plan:', error);
		return {
			plan: 'free',
			status: 'expired',
			currentPeriodEnd: null,
			whopMembershipId: null
		};
	}
}

export async function getJwtForRequest(
	auth: JwtTokenProvider,
	headers: Headers
): Promise<string | null> {
	const tokenResponse = await auth.api.getToken({ headers });
	const token = tokenResponse?.token?.trim();
	return token ? token : null;
}
