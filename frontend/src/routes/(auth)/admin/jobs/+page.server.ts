import type { PageServerLoad } from './$types';

import { loadAdminJobs } from '$lib/server/admin-dashboard';

export const load: PageServerLoad = async () => {
	return {
		jobs: await loadAdminJobs()
	};
};
