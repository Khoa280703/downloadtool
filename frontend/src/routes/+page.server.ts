import type { PageServerLoad } from './$types';

function normalizeRedirectTo(value: string | null): string {
	if (!value || !value.startsWith('/') || value.startsWith('//')) {
		return '/';
	}

	return value;
}

export const load: PageServerLoad = async ({ locals, url }) => {
	return {
		user: locals.user,
		authRequired: url.searchParams.get('auth') === 'required',
		redirectTo: normalizeRedirectTo(url.searchParams.get('redirectTo'))
	};
};
