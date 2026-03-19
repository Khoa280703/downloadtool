import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

import { isAdminEmail } from '$lib/server/admin-access';

export const load: PageServerLoad = async ({ locals }) => {
	if (!locals.user || !locals.session) {
		throw redirect(302, `/?auth=required&redirectTo=${encodeURIComponent('/account')}`);
	}

	return {
		user: locals.user,
		isAdmin: isAdminEmail(locals.user.email)
	};
};
