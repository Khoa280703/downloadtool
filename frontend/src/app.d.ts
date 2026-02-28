// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
import type { Session, User } from 'better-auth';

declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			user: User | null;
			session: Session | null;
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}

	// PWA install prompt event (not yet in standard lib.dom.d.ts)
	interface BeforeInstallPromptEvent extends Event {
		prompt(): Promise<{ outcome: 'accepted' | 'dismissed' }>;
	}
}

export {};
