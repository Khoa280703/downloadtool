import { fail } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';

import { requireAdmin } from '$lib/server/admin-access';
import { loadAdminProxies } from '$lib/server/admin-dashboard';
import {
	createProxy,
	maskProxyUrl,
	updateProxyNotes,
	updateProxyStatus
} from '$lib/server/admin-proxy-management';
import { logAuditEvent } from '$lib/server/audit-log';

export const load: PageServerLoad = async () => {
	return {
		proxies: await loadAdminProxies()
	};
};

export const actions: Actions = {
	createProxy: async ({ request, locals, cookies, url }) => {
		requireAdmin(locals, '/admin/proxies');
		const formData = await request.formData();
		const proxyUrl = formData.get('proxyUrl')?.toString().trim() ?? '';
		const displayName = formData.get('displayName')?.toString().trim() ?? '';
		const notes = formData.get('notes')?.toString().trim() ?? '';

		if (!proxyUrl) {
			await logAuditEvent(
				{ request, locals, cookies, url },
				{
					scope: 'admin',
					eventType: 'proxy_create',
					targetLabel: displayName || null,
					statusCode: 400,
					outcome: 'failure',
					detail: 'Missing proxy URL'
				}
			);
			return fail(400, { error: 'Proxy URL is required.' });
		}

		try {
			await createProxy({ proxyUrl, displayName, notes });
			await logAuditEvent(
				{ request, locals, cookies, url },
				{
					scope: 'admin',
					eventType: 'proxy_create',
					targetLabel: displayName || maskProxyUrl(proxyUrl),
					statusCode: 200,
					outcome: 'success',
					payload: {
						displayName,
						maskedProxyUrl: maskProxyUrl(proxyUrl),
						hasNotes: Boolean(notes)
					}
				}
			);
			return { success: 'Proxy saved successfully.' };
		} catch (error) {
			await logAuditEvent(
				{ request, locals, cookies, url },
				{
					scope: 'admin',
					eventType: 'proxy_create',
					targetLabel: displayName || maskProxyUrl(proxyUrl),
					statusCode: 400,
					outcome: 'failure',
					detail: error instanceof Error ? error.message : 'Failed to save proxy.'
				}
			);
			return fail(400, {
				error: error instanceof Error ? error.message : 'Failed to save proxy.'
			});
		}
	},
	updateProxyStatus: async ({ request, locals, cookies, url }) => {
		requireAdmin(locals, '/admin/proxies');
		const formData = await request.formData();
		const proxyId = formData.get('proxyId')?.toString().trim() ?? '';
		const status = formData.get('status')?.toString().trim() ?? '';

		if (!proxyId || !['active', 'disabled', 'quarantined'].includes(status)) {
			await logAuditEvent(
				{ request, locals, cookies, url },
				{
					scope: 'admin',
					eventType: 'proxy_status_update',
					entityId: proxyId || null,
					targetLabel: proxyId || null,
					statusCode: 400,
					outcome: 'failure',
					detail: 'Invalid proxy update payload'
				}
			);
			return fail(400, { error: 'Invalid proxy update payload.' });
		}

		try {
			const result = await updateProxyStatus({
				proxyId,
				status: status as 'active' | 'disabled' | 'quarantined'
			});

			if (result === 'noop') {
				await logAuditEvent(
					{ request, locals, cookies, url },
					{
						scope: 'admin',
						eventType: 'proxy_status_update',
						entityId: proxyId,
						targetLabel: proxyId,
						statusCode: 200,
						outcome: 'success',
						detail: `Proxy already ${status}`,
						payload: {
							status,
							result
						}
					}
				);
				return { success: `Proxy is already ${status}.` };
			}
		} catch (error) {
			await logAuditEvent(
				{ request, locals, cookies, url },
				{
					scope: 'admin',
					eventType: 'proxy_status_update',
					entityId: proxyId,
					targetLabel: proxyId,
					statusCode: 400,
					outcome: 'failure',
					detail: error instanceof Error ? error.message : 'Failed to update proxy.',
					payload: {
						status
					}
				}
			);
			return fail(400, {
				error: error instanceof Error ? error.message : 'Failed to update proxy.'
			});
		}

		await logAuditEvent(
			{ request, locals, cookies, url },
			{
				scope: 'admin',
				eventType: 'proxy_status_update',
				entityId: proxyId,
				targetLabel: proxyId,
				statusCode: 200,
				outcome: 'success',
				payload: {
					status
				}
			}
		);

		return { success: `Proxy updated to ${status}.` };
	},
	updateProxyNotes: async ({ request, locals, cookies, url }) => {
		requireAdmin(locals, '/admin/proxies');
		const formData = await request.formData();
		const proxyId = formData.get('proxyId')?.toString().trim() ?? '';
		const notes = formData.get('notes')?.toString() ?? '';

		if (!proxyId) {
			await logAuditEvent(
				{ request, locals, cookies, url },
				{
					scope: 'admin',
					eventType: 'proxy_notes_update',
					statusCode: 400,
					outcome: 'failure',
					detail: 'Invalid proxy notes payload'
				}
			);
			return fail(400, { error: 'Invalid proxy notes payload.' });
		}

		try {
			await updateProxyNotes({ proxyId, notes });
		} catch (error) {
			await logAuditEvent(
				{ request, locals, cookies, url },
				{
					scope: 'admin',
					eventType: 'proxy_notes_update',
					entityId: proxyId,
					targetLabel: proxyId,
					statusCode: 400,
					outcome: 'failure',
					detail: error instanceof Error ? error.message : 'Failed to update proxy notes.'
				}
			);
			return fail(400, {
				error: error instanceof Error ? error.message : 'Failed to update proxy notes.'
			});
		}

		await logAuditEvent(
			{ request, locals, cookies, url },
			{
				scope: 'admin',
				eventType: 'proxy_notes_update',
				entityId: proxyId,
				targetLabel: proxyId,
				statusCode: 200,
				outcome: 'success',
				payload: {
					notesLength: notes.trim().length
				}
			}
		);

		return { success: 'Proxy notes updated.' };
	}
};
