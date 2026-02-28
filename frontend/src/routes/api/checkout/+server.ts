import { env } from '$env/dynamic/private';
import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

function requireWhopPlanId(): string {
	const planId = env.WHOP_PLAN_ID?.trim();
	if (!planId) {
		throw new Error('WHOP_PLAN_ID is required');
	}
	return planId;
}

export const GET: RequestHandler = async ({ locals }) => {
	if (!locals.user) {
		throw redirect(302, '/login?redirectTo=%2Faccount');
	}

	const planId = requireWhopPlanId();
	const checkoutUrl = new URL(`https://whop.com/checkout/${planId}/`);
	checkoutUrl.searchParams.set('custom_data', locals.user.id);

	throw redirect(302, checkoutUrl.toString());
};
