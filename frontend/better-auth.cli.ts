import { betterAuth } from 'better-auth';
import { jwt } from 'better-auth/plugins';
import { config } from 'dotenv';
import { Pool } from 'pg';

config({ path: '.env' });
if (!process.env.DATABASE_URL) {
	config({ path: '../.env' });
}

if (!process.env.DATABASE_URL) {
	throw new Error('DATABASE_URL is required for Better Auth CLI migrations');
}

export const auth = betterAuth({
	database: new Pool({
		connectionString: process.env.DATABASE_URL
	}),
	plugins: [jwt()]
});
