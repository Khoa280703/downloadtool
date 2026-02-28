import { createAuthClient } from 'better-auth/svelte';

const baseURL = typeof window === 'undefined' ? '' : window.location.origin;

export const authClient = createAuthClient({
	baseURL
});
