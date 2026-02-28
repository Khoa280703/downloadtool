import { toSvelteKitHandler } from 'better-auth/svelte-kit';
import type { RequestHandler } from './$types';

import { auth } from '$lib/server/auth';

const handler = toSvelteKitHandler(auth);

export const GET: RequestHandler = (event) => handler(event);
export const POST: RequestHandler = (event) => handler(event);
