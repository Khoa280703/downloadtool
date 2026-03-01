function normalizeSignOutError(status: number, payload: unknown): string {
	if (payload && typeof payload === 'object' && 'message' in payload) {
		const message = payload.message;
		if (typeof message === 'string' && message.trim()) {
			return message;
		}
	}

	return `Đăng xuất thất bại (HTTP ${status}).`;
}

export async function signOutFromBrowser(): Promise<void> {
	const response = await fetch('/api/auth/sign-out', {
		method: 'POST',
		credentials: 'include'
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
}
