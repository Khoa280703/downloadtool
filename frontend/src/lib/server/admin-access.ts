import { env } from '$env/dynamic/private';
import { error, redirect } from '@sveltejs/kit';

function configuredAdminEmails(): string[] {
	return (env.ADMIN_EMAILS ?? '')
		.split(',')
		.map((value) => value.trim().toLowerCase())
		.filter(Boolean);
}

export function isAdminEmail(email: string | null | undefined): boolean {
	if (!email) return false;
	const admins = configuredAdminEmails();
	return admins.includes(email.trim().toLowerCase());
}

export function getAdminGateMode(): 'configured' | 'disabled' {
	return configuredAdminEmails().length > 0 ? 'configured' : 'disabled';
}

export function requireAdmin(locals: App.Locals, redirectTo = '/admin') {
	if (!locals.user || !locals.session) {
		throw redirect(302, `/?auth=required&redirectTo=${encodeURIComponent(redirectTo)}`);
	}

	if (isAdminEmail(locals.user.email)) {
		return locals.user;
	}

	if (getAdminGateMode() === 'disabled') {
		throw error(403, 'Admin access is not configured.');
	}

	throw error(403, 'Admin access denied.');
}
