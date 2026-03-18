import type { PageServerLoad } from './$types';

import { loadAdminPlaylistJobs } from '$lib/server/admin-dashboard';

export const load: PageServerLoad = async () => {
	return {
		playlistJobs: await loadAdminPlaylistJobs()
	};
};
