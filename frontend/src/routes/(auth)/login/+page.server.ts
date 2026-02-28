import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, url }) => {
	if (locals.session) {
		const redirectTo = url.searchParams.get('redirectTo') ?? '/account';
		throw redirect(302, redirectTo);
	}

	return {
		redirectTo: url.searchParams.get('redirectTo') ?? '/account'
	};
};
