import { getRequestEvent } from '$app/server';
import { building } from '$app/environment';
import { env } from '$env/dynamic/private';
import { betterAuth } from 'better-auth';
import { jwt } from 'better-auth/plugins';
import { sveltekitCookies } from 'better-auth/svelte-kit';

import { getDatabasePool, getUserTier } from './auth-utils';

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
	baseURL: env.ORIGIN ?? env.BETTER_AUTH_URL ?? 'http://localhost:5168',
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
