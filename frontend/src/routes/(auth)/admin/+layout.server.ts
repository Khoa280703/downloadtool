import type { LayoutServerLoad } from './$types';

import { requireAdmin, getAdminGateMode } from '$lib/server/admin-access';
import { loadAdminOverview } from '$lib/server/admin-dashboard';

export const load: LayoutServerLoad = async ({ locals, url }) => {
	const user = requireAdmin(locals, url.pathname);

	return {
		user,
		gateMode: getAdminGateMode(),
		overview: await loadAdminOverview()
	};
};
