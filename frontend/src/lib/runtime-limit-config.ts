import JSON5 from 'json5';
import runtimeLimitProfilesRaw from '../../../config/runtime-limit-profiles.json?raw';

const DISABLED_NUMERIC_LIMIT = Number.MAX_SAFE_INTEGER;

type FrontendLimitProfile = {
	extract_max_retry_attempts?: number | null;
	extract_retry_base_delay_ms?: number | null;
	extract_retry_max_delay_ms?: number | null;
	batch_max_reconnect_attempts?: number | null;
	batch_reconnect_base_delay_ms?: number | null;
	batch_reconnect_max_delay_ms?: number | null;
	mux_job_poll_interval_ms?: number | null;
	mux_job_max_wait_ms?: number | null;
	playlist_worker_max_concurrent?: number | null;
	playlist_worker_ready_queue_max?: number | null;
	playlist_worker_extract_jitter_min_ms?: number | null;
	playlist_worker_extract_jitter_range_ms?: number | null;
	playlist_worker_circuit_cooldown_ms?: number | null;
};

type RuntimeLimitProfiles = {
	local?: { frontend?: FrontendLimitProfile };
	production?: { frontend?: FrontendLimitProfile };
};

function toSafeNumber(
	value: number | undefined | null,
	fallback: number,
	min = 0,
	integer = true
): number {
	const parsed = typeof value === 'number' && Number.isFinite(value) ? value : fallback;
	const normalized = integer ? Math.trunc(parsed) : parsed;
	if (normalized < min) return fallback;
	return normalized;
}

function toCapNumber(value: number | undefined | null, fallback: number, min = 1): number {
	if (value === null) return DISABLED_NUMERIC_LIMIT;
	return toSafeNumber(value, fallback, min);
}

function toThresholdNumber(
	value: number | undefined | null,
	fallback: number,
	min = 1,
	integer = true
): number {
	if (value === null) return Number.POSITIVE_INFINITY;
	return toSafeNumber(value, fallback, min, integer);
}

function toZeroWhenNull(value: number | undefined | null, fallback: number, min = 0): number {
	if (value === null) return 0;
	return toSafeNumber(value, fallback, min);
}

function getFrontendLimitProfile(): FrontendLimitProfile {
	const profiles = JSON5.parse(runtimeLimitProfilesRaw) as RuntimeLimitProfiles;
	const active = import.meta.env.PROD ? profiles.production : profiles.local;
	return active?.frontend ?? {};
}

const profile = getFrontendLimitProfile();

export const muxJobClientLimitConfig = {
	pollIntervalMs: toSafeNumber(profile.mux_job_poll_interval_ms, 1_200, 100),
	maxWaitMs: toThresholdNumber(profile.mux_job_max_wait_ms, 10 * 60 * 1_000, 1_000)
} as const;

export const playlistWorkerLimitConfig = {
	maxConcurrent: toCapNumber(profile.playlist_worker_max_concurrent, 1, 1),
	readyQueueMax: toCapNumber(profile.playlist_worker_ready_queue_max, 1, 1),
	extractJitterMinMs: toZeroWhenNull(profile.playlist_worker_extract_jitter_min_ms, 2_000, 0),
	extractJitterRangeMs: toZeroWhenNull(profile.playlist_worker_extract_jitter_range_ms, 3_000, 0),
	circuitCooldownMs: toZeroWhenNull(
		profile.playlist_worker_circuit_cooldown_ms,
		5 * 60 * 1_000,
		0
	)
} as const;

export const apiClientLimitConfig = {
	extractMaxRetryAttempts: toCapNumber(profile.extract_max_retry_attempts, 4, 1),
	extractRetryBaseDelayMs: toSafeNumber(profile.extract_retry_base_delay_ms, 500, 0),
	extractRetryMaxDelayMs: toSafeNumber(profile.extract_retry_max_delay_ms, 8_000, 1),
	batchMaxReconnectAttempts: toCapNumber(profile.batch_max_reconnect_attempts, 8, 1),
	batchReconnectBaseDelayMs: toSafeNumber(profile.batch_reconnect_base_delay_ms, 1_000, 0),
	batchReconnectMaxDelayMs: toSafeNumber(profile.batch_reconnect_max_delay_ms, 12_000, 1)
} as const;
