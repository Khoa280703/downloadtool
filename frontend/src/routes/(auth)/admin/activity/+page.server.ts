import type { PageServerLoad } from './$types';

import { loadAdminActivity } from '$lib/server/admin-dashboard';

export const load: PageServerLoad = async () => {
	return {
		activity: await loadAdminActivity()
	};
};
