/**
 * Cookie consent store
 *
 * Manages user consent state for GDPR compliance.
 * Persists consent choice to localStorage.
 */

import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';
import { updateConsent as updateGAConsent } from '$lib/analytics';

/** Consent storage key */
const CONSENT_KEY = 'videodl_cookie_consent';

/** Consent state interface */
interface ConsentState {
	/** Whether user has made a choice */
	decided: boolean;
	/** Whether user has given consent */
	accepted: boolean;
	/** Timestamp of consent decision */
	timestamp?: number;
}

/** Default consent state */
const defaultConsent: ConsentState = {
	decided: false,
	accepted: false
};

/**
 * Load consent from localStorage
 */
function loadConsent(): ConsentState {
	if (!browser) return defaultConsent;

	try {
		const stored = localStorage.getItem(CONSENT_KEY);
		if (stored) {
			return JSON.parse(stored);
		}
	} catch {
		// Ignore localStorage errors
	}

	return defaultConsent;
}

/**
 * Save consent to localStorage
 */
function saveConsent(state: ConsentState): void {
	if (!browser) return;

	try {
		localStorage.setItem(CONSENT_KEY, JSON.stringify(state));
	} catch {
		// Ignore localStorage errors
	}
}

/** Base consent store */
function createConsentStore() {
	const initial = loadConsent();
	const { subscribe, set, update } = writable<ConsentState>(initial);

	// Update GA consent on initialization
	if (browser && initial.decided) {
		updateGAConsent(initial.accepted);
	}

	return {
		subscribe,

		/**
		 * Accept cookies and enable tracking
		 */
		accept: () => {
			const state: ConsentState = {
				decided: true,
				accepted: true,
				timestamp: Date.now()
			};
			saveConsent(state);
			set(state);
			updateGAConsent(true);
		},

		/**
		 * Reject cookies and disable tracking
		 */
		reject: () => {
			const state: ConsentState = {
				decided: true,
				accepted: false,
				timestamp: Date.now()
			};
			saveConsent(state);
			set(state);
			updateGAConsent(false);
		},

		/**
		 * Reset consent (for testing)
		 */
		reset: () => {
			if (browser) {
				localStorage.removeItem(CONSENT_KEY);
			}
			set(defaultConsent);
			updateGAConsent(false);
		}
	};
}

/** Global consent store instance */
export const consent = createConsentStore();

/** Derived store for quick consent check */
export const hasConsent = derived(consent, ($consent) => $consent.accepted);

/** Derived store to check if user has decided */
export const hasDecided = derived(consent, ($consent) => $consent.decided);
