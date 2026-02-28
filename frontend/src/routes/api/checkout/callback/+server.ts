import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ locals }) => {
	if (!locals.user) {
		throw redirect(302, '/?auth=required&redirectTo=%2Faccount');
	}

	throw redirect(302, '/account?checkout=success');
};
