import type { PageServerLoad } from './$types';

import { loadAdminActivity, loadAdminJobs, loadAdminProxies } from '$lib/server/admin-dashboard';

export const load: PageServerLoad = async () => {
	const [jobs, proxies, activity] = await Promise.all([
		loadAdminJobs(),
		loadAdminProxies(),
		loadAdminActivity()
	]);

	return {
		jobs,
		proxies,
		activity
	};
};
