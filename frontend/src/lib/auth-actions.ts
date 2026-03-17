import * as m from '$lib/paraglide/messages';

export const AUTH_USER_STATE_EVENT = 'downloadtool-auth-user-state';

export type BrowserAuthUser = {
	name?: string | null;
	email: string;
	image?: string | null;
};

export function broadcastAuthUserState(user: BrowserAuthUser | null): void {
	if (typeof window === 'undefined') return;

	window.dispatchEvent(
		new CustomEvent<{ user: BrowserAuthUser | null }>(AUTH_USER_STATE_EVENT, {
			detail: { user }
		})
	);
}

function normalizeSignOutError(status: number, payload: unknown): string {
	if (payload && typeof payload === 'object' && 'message' in payload) {
		const message = payload.message;
		if (typeof message === 'string' && message.trim()) {
			return message;
		}
	}

	return m.user_menu_sign_out_failed_status({ status: String(status) });
}

export async function signOutFromBrowser(): Promise<void> {
	const response = await fetch('/api/auth/sign-out', {
		method: 'POST',
		credentials: 'include',
		cache: 'no-store'
	});

	if (!response.ok) {
		let payload: unknown = null;
		try {
			payload = await response.json();
		} catch {
			// Ignore JSON parse errors for non-JSON responses
		}
		throw new Error(normalizeSignOutError(response.status, payload));
	}

	broadcastAuthUserState(null);
}
