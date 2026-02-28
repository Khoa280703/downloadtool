import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, url }) => {
	const redirectTo = url.searchParams.get('redirectTo') ?? '/account';

	if (locals.session) {
		throw redirect(302, redirectTo);
	}

	throw redirect(302, `/?auth=required&redirectTo=${encodeURIComponent(redirectTo)}`);
};
