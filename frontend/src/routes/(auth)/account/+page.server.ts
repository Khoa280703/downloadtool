import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

import { getSubscriptionForUser } from '$lib/server/auth-utils';

export const load: PageServerLoad = async ({ locals, url }) => {
	if (!locals.user || !locals.session) {
		throw redirect(302, `/login?redirectTo=${encodeURIComponent('/account')}`);
	}

	const subscription = await getSubscriptionForUser(locals.user.id);

	return {
		user: locals.user,
		subscription,
		checkoutStatus: url.searchParams.get('checkout')
	};
};
