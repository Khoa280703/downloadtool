import { fail } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';

import { requireAdmin } from '$lib/server/admin-access';
import { loadAdminProxies } from '$lib/server/admin-dashboard';
import { createProxy, updateProxyStatus } from '$lib/server/admin-proxy-management';

export const load: PageServerLoad = async () => {
	return {
		proxies: await loadAdminProxies()
	};
};

export const actions: Actions = {
	createProxy: async ({ request, locals }) => {
		requireAdmin(locals, '/admin/proxies');
		const formData = await request.formData();
		const proxyUrl = formData.get('proxyUrl')?.toString().trim() ?? '';
		const displayName = formData.get('displayName')?.toString().trim() ?? '';
		const notes = formData.get('notes')?.toString().trim() ?? '';

		if (!proxyUrl) {
			return fail(400, { error: 'Proxy URL is required.' });
		}

		try {
			await createProxy({ proxyUrl, displayName, notes });
			return { success: 'Proxy saved successfully.' };
		} catch (error) {
			return fail(400, {
				error: error instanceof Error ? error.message : 'Failed to save proxy.'
			});
		}
	},
	updateProxyStatus: async ({ request, locals }) => {
		requireAdmin(locals, '/admin/proxies');
		const formData = await request.formData();
		const proxyId = formData.get('proxyId')?.toString().trim() ?? '';
		const status = formData.get('status')?.toString().trim() ?? '';
		const reason = formData.get('reason')?.toString().trim() ?? '';

		if (!proxyId || !['active', 'disabled', 'quarantined'].includes(status)) {
			return fail(400, { error: 'Invalid proxy update payload.' });
		}

		try {
			const result = await updateProxyStatus({
				proxyId,
				status: status as 'active' | 'disabled' | 'quarantined',
				reason
			});

			if (result === 'noop') {
				return { success: `Proxy is already ${status}.` };
			}
		} catch (error) {
			return fail(400, {
				error: error instanceof Error ? error.message : 'Failed to update proxy.'
			});
		}

		return { success: `Proxy updated to ${status}.` };
	}
};
