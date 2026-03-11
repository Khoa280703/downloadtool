import { getRequestEvent } from '$app/server';
import { building } from '$app/environment';
import { env } from '$env/dynamic/private';
import { betterAuth } from 'better-auth';
import { jwt } from 'better-auth/plugins';
import { sveltekitCookies } from 'better-auth/svelte-kit';

import { getDatabasePool, getUserTier } from './auth-utils';

function normalizeOptionalUrl(value: string | undefined): string | undefined {
	const trimmed = value?.trim();
	return trimmed ? trimmed.replace(/\/+$/, '') : undefined;
}

function isLoopbackBaseUrl(value: string): boolean {
	try {
		const hostname = new URL(value).hostname;
		return (
			hostname === 'localhost' ||
			hostname === '127.0.0.1' ||
			hostname === '0.0.0.0' ||
			hostname === '::1'
		);
	} catch {
		return false;
	}
}

function resolveAuthBaseUrl(): string | undefined {
	const configuredBaseUrl =
		normalizeOptionalUrl(env.BETTER_AUTH_URL) ?? normalizeOptionalUrl(env.ORIGIN);

	// In local dev we often access the app via localhost, LAN IP, or Tailscale IP interchangeably.
	// Let Better Auth derive the origin from the current request instead of pinning callbacks to loopback.
	if (!configuredBaseUrl || isLoopbackBaseUrl(configuredBaseUrl)) {
		return undefined;
	}

	return configuredBaseUrl;
}

function resolveAuthSecret(): string {
	const value = env.BETTER_AUTH_SECRET?.trim();

	if (!value || value === 'better-auth-secret') {
		if (!building) {
			throw new Error('BETTER_AUTH_SECRET is required and must not use default value');
		}

		return 'snapvie-build-placeholder-secret-please-set-better-auth-secret-in-runtime';
	}

	return value;
}

const trustedOrigins = (env.BETTER_AUTH_TRUSTED_ORIGINS ?? '')
	.split(',')
	.map((origin) => origin.trim())
	.filter(Boolean);

const socialProviders =
	env.GOOGLE_CLIENT_ID && env.GOOGLE_CLIENT_SECRET
		? {
				google: {
					clientId: env.GOOGLE_CLIENT_ID,
					clientSecret: env.GOOGLE_CLIENT_SECRET
				}
			}
		: undefined;

export const auth = betterAuth({
	baseURL: resolveAuthBaseUrl(),
	database: getDatabasePool(),
	secret: resolveAuthSecret(),
	trustedOrigins,
	emailAndPassword: {
		enabled: true
	},
	socialProviders,
	plugins: [
		sveltekitCookies(() => getRequestEvent()),
		jwt({
			jwt: {
				expirationTime: '15m',
				definePayload: async ({ user }) => ({
					sub: user.id,
					tier: await getUserTier(user.id)
				})
			}
		})
	]
});
