<script lang="ts">
	import { onDestroy } from 'svelte';
	import AppIcon from '$components/AppIcon.svelte';
	import {
		buildStreamUrl,
		createMuxedDownloadJob,
		releaseMuxedDownloadJob,
		waitForMuxedDownloadJobReady,
		type MuxJobStatusUpdate
	} from '$lib/api';
	import { currentDownload, setDownloading, downloadProgress } from '$stores/download';
	import { get } from 'svelte/store';
	import { trackDownloadStarted } from '$lib/analytics';
	import { saveDownload } from '$lib/playlist-download-file-saver';
	import * as m from '$lib/paraglide/messages';
	import type { Stream } from '$lib/types';

	interface Props {
		stream: Stream | null;
		/** Audio-only stream to mux with video-only stream. When provided, uses background job flow. */
		audioStream?: Stream | null;
		/** Canonical source/watch URL for refresh-on-auth-failure on backend. */
		sourceUrl?: string | null;
		title: string;
		disabled?: boolean;
	}

	let {
		stream,
		audioStream = null,
		sourceUrl = null,
		title,
		disabled = false
	}: Props = $props();
	let isLoading = $state(false);
	let progressLabel = $state('');
	let progressIndeterminate = $state(false);
	let progressAnimationFrame: number | null = null;
	let lastAnimationFrameAtMs = 0;
	let lastRealProgress = 0;
	let lastRealProgressAtMs = 0;
	let estimatedVelocityPerSecond = 0;

	const MAX_PROGRESS_BEFORE_READY = 99.4;
	const MIN_PROGRESS_VELOCITY = 0.12;
	const MAX_PROGRESS_VELOCITY = 4.8;
	const MIN_DISPLAY_VELOCITY = 0.65;
	const MAX_DISPLAY_VELOCITY = 10;
	const READY_DISPLAY_VELOCITY = 40;
	const MAX_PROGRESS_LEAD = 4.2;

	function nowMs(): number {
		if (typeof performance !== 'undefined' && typeof performance.now === 'function') {
			return performance.now();
		}
		return Date.now();
	}

	function enforceHttps(url: string): string {
		if (
			typeof window !== 'undefined' &&
			window.location.protocol === 'https:' &&
			url.startsWith('http://')
		) {
			return `https://${url.slice('http://'.length)}`;
		}
		return url;
	}

	function formatSize(bytes?: number): string {
		if (!bytes) return '';
		const units = ['B', 'KB', 'MB', 'GB'];
		let size = bytes;
		let unitIndex = 0;
		while (size >= 1024 && unitIndex < units.length - 1) {
			size /= 1024;
			unitIndex += 1;
		}
		return `${size.toFixed(size >= 100 || unitIndex === 0 ? 0 : 1)} ${units[unitIndex]}`;
	}

	function ctaLabel(): string {
		if (!stream) return m.download_btn_select_format();
		const size = formatSize(stream.size);
		if (size) return m.download_btn_download_now_size({ size });
		if (stream.quality) return m.download_btn_download_quality({ quality: stream.quality });
		return m.download_btn_download_now_plain();
	}

	function setProgressState(
		label: string,
		options: { value?: number; indeterminate?: boolean; allowDecrease?: boolean } = {}
	): void {
		progressLabel = label;
		progressIndeterminate = options.indeterminate ?? false;
		if (progressIndeterminate) {
			stopProgressAnimation();
			return;
		}
		if (!progressIndeterminate && typeof options.value === 'number') {
			animateProgressTo(options.value, options.allowDecrease ?? false);
		}
	}

	function resetProgressState(): void {
		stopProgressAnimation();
		progressLabel = '';
		progressIndeterminate = false;
		lastAnimationFrameAtMs = 0;
		lastRealProgress = 0;
		lastRealProgressAtMs = 0;
		estimatedVelocityPerSecond = 0;
		downloadProgress.set(0);
	}

	function stopProgressAnimation(): void {
		if (progressAnimationFrame === null || typeof window === 'undefined') return;
		window.cancelAnimationFrame(progressAnimationFrame);
		progressAnimationFrame = null;
		lastAnimationFrameAtMs = 0;
	}

	function clamp(value: number, min: number, max: number): number {
		return Math.min(max, Math.max(min, value));
	}

	function ensureProgressAnimation(): void {
		if (typeof window === 'undefined' || progressAnimationFrame !== null) return;

		const step = (frameTimeMs: number) => {
			const current = get(downloadProgress);
			const lastFrameAt = lastAnimationFrameAtMs || frameTimeMs;
			const deltaSeconds = Math.min(Math.max((frameTimeMs - lastFrameAt) / 1000, 0.001), 0.1);
			lastAnimationFrameAtMs = frameTimeMs;

			let desired = current;
			let displayVelocity = MIN_DISPLAY_VELOCITY;
			let hasFutureSoftAdvance = false;

			if (lastRealProgress >= 100) {
				desired = 100;
				displayVelocity = READY_DISPLAY_VELOCITY;
			} else if (lastRealProgress > 0 && lastRealProgressAtMs > 0) {
				const elapsedSinceRealSeconds = Math.max(0, (frameTimeMs - lastRealProgressAtMs) / 1000);
				const softLead = clamp(
					Math.max(estimatedVelocityPerSecond * 2.8, 0.85),
					0.85,
					MAX_PROGRESS_LEAD
				);
				const projectedDelta = Math.min(estimatedVelocityPerSecond * elapsedSinceRealSeconds, softLead);
				const projected = lastRealProgress + projectedDelta;
				desired = Math.min(MAX_PROGRESS_BEFORE_READY, Math.min(projected, lastRealProgress + softLead));
				hasFutureSoftAdvance =
					projectedDelta + 0.02 < softLead && desired + 0.02 < MAX_PROGRESS_BEFORE_READY;
				displayVelocity = clamp(
					Math.max(estimatedVelocityPerSecond * 1.7, MIN_DISPLAY_VELOCITY),
					MIN_DISPLAY_VELOCITY,
					MAX_DISPLAY_VELOCITY
				);
			}

			const next = desired > current ? current + Math.min(desired - current, displayVelocity * deltaSeconds) : current;
			downloadProgress.set(clamp(next, 0, 100));

			if (
				lastRealProgress >= 100 &&
				Math.abs(100 - next) <= 0.02
			) {
				downloadProgress.set(100);
				progressAnimationFrame = null;
				lastAnimationFrameAtMs = 0;
				return;
			}

			if (next + 0.01 < desired || hasFutureSoftAdvance) {
				progressAnimationFrame = window.requestAnimationFrame(step);
				return;
			}

			progressAnimationFrame = null;
			lastAnimationFrameAtMs = 0;
		};

		progressAnimationFrame = window.requestAnimationFrame(step);
	}

	function animateProgressTo(value: number, allowDecrease = false): void {
		const clamped = clamp(value, 0, 100);
		const effectiveValue =
			allowDecrease
				? clamped
				: clamped >= 100
					? 100
					: Math.min(MAX_PROGRESS_BEFORE_READY, Math.max(lastRealProgress, clamped));
		const timestampMs = nowMs();

		if (allowDecrease && effectiveValue <= 0) {
			lastRealProgress = 0;
			lastRealProgressAtMs = timestampMs;
			estimatedVelocityPerSecond = 0;
		} else if (effectiveValue > lastRealProgress) {
			if (lastRealProgressAtMs > 0) {
				const deltaProgress = effectiveValue - lastRealProgress;
				const deltaSeconds = Math.max((timestampMs - lastRealProgressAtMs) / 1000, 0.08);
				const observedVelocity = deltaProgress / deltaSeconds;
				estimatedVelocityPerSecond =
					estimatedVelocityPerSecond > 0
						? clamp(
								estimatedVelocityPerSecond * 0.45 + observedVelocity * 0.55,
								MIN_PROGRESS_VELOCITY,
								MAX_PROGRESS_VELOCITY
							)
						: clamp(observedVelocity, MIN_PROGRESS_VELOCITY, MAX_PROGRESS_VELOCITY);
			} else {
				estimatedVelocityPerSecond = clamp(effectiveValue / 6, MIN_PROGRESS_VELOCITY, 1.2);
			}

			lastRealProgress = effectiveValue;
			lastRealProgressAtMs = timestampMs;
		} else if (effectiveValue >= 100) {
			lastRealProgress = 100;
			lastRealProgressAtMs = timestampMs;
		}

		if (typeof window === 'undefined') {
			downloadProgress.set(lastRealProgress);
			return;
		}

		ensureProgressAnimation();
	}

	function formatProgressPercent(value: number): string {
		if (!Number.isFinite(value)) return '0%';
		if (value >= 100) return '100%';
		return `${value.toFixed(2)}%`;
	}

	function buildFilename(extension: string): string {
		const safeTitle = title.replace(/[^a-z0-9]/gi, '_');
		const normalizedExt = extension.trim().replace(/^\.+/, '') || 'mp4';
		return `${safeTitle}.${normalizedExt}`;
	}

	function clampProgressPercent(
		value: number | null | undefined,
		status: MuxJobStatusUpdate['status']
	): number | null {
		if (typeof value !== 'number' || Number.isNaN(value)) return null;
		const max = status === 'ready' ? 100 : 99;
		return Math.max(0, Math.min(max, value));
	}

	function resolveMuxPhaseLabel(update: MuxJobStatusUpdate): string | null {
		switch (update.phase) {
			case 'starting':
				return m.download_btn_mux_status_processing_running();
			case 'fetching_streams':
				return m.download_btn_mux_status_processing_fetching();
			case 'muxing_uploading':
				return m.download_btn_mux_status_processing_muxing();
			case 'completing_upload':
				return m.download_btn_mux_status_processing_finalizing();
			case 'retrying':
				return m.download_btn_mux_status_queued_waiting();
			case 'ready':
				return m.download_btn_mux_status_ready();
			case 'failed':
				return m.download_btn_mux_status_failed();
			default:
				return null;
		}
	}

	function formatMuxStatus(
		update: MuxJobStatusUpdate
	): { label: string; value: number; indeterminate: boolean; allowDecrease?: boolean } {
		const elapsedSeconds = Math.floor(update.elapsedMs / 1000);
		const livePercent = clampProgressPercent(update.percent, update.status);
		const phaseLabel = resolveMuxPhaseLabel(update);

		if (phaseLabel) {
			return {
				label: phaseLabel,
				value:
					update.status === 'ready'
						? 100
						: update.status === 'failed'
							? 0
							: (livePercent ?? 18),
				indeterminate: livePercent === null && update.status !== 'ready' && update.status !== 'failed',
				allowDecrease: update.status === 'failed'
			};
		}

		if (update.status === 'queued') {
			return {
				label:
					elapsedSeconds >= 8
						? m.download_btn_mux_status_queued_waiting()
						: m.download_btn_mux_status_queued(),
				value: livePercent ?? 14,
				indeterminate: livePercent === null
			};
		}

		if (update.status === 'leased') {
			return {
				label:
					elapsedSeconds >= 10
						? m.download_btn_mux_status_leased_waiting()
						: m.download_btn_mux_status_leased(),
				value: livePercent ?? 28,
				indeterminate: livePercent === null
			};
		}

		if (update.status === 'processing') {
			const phase = Math.floor(elapsedSeconds / 6) % 4;
			if (phase === 0) {
				return {
					label: m.download_btn_mux_status_processing_fetching(),
					value: livePercent ?? 46,
					indeterminate: livePercent === null
				};
			}
			if (phase === 1) {
				return {
					label: m.download_btn_mux_status_processing_muxing(),
					value: livePercent ?? 58,
					indeterminate: livePercent === null
				};
			}
			if (phase === 2) {
				return {
					label: m.download_btn_mux_status_processing_finalizing(),
					value: livePercent ?? 68,
					indeterminate: livePercent === null
				};
			}
			return {
				label: m.download_btn_mux_status_processing_running(),
				value: livePercent ?? 76,
				indeterminate: livePercent === null
			};
		}

		if (update.status === 'ready') {
			return { label: m.download_btn_mux_status_ready(), value: 100, indeterminate: false };
		}

		if (update.status === 'failed') {
			return {
				label: m.download_btn_mux_status_failed(),
				value: 0,
				indeterminate: false,
				allowDecrease: true
			};
		}

		return {
			label: m.download_btn_mux_status_expired(),
			value: 0,
			indeterminate: false,
			allowDecrease: true
		};
	}

	/** Trigger browser download */
	async function handleDownload(): Promise<void> {
		if (!stream) return;

		trackDownloadStarted('youtube', stream.quality || 'unknown', stream.format || 'mp4');
		console.info('[downloadtool] download button clicked', {
			title,
			streamUrl: stream.url,
			audioStreamUrl: audioStream?.url ?? null,
			streamFormatId: stream.formatId ?? null,
			audioFormatId: audioStream?.formatId ?? null,
			useMux: Boolean(audioStream && !stream.hasAudio)
		});

		const muxAudioStream = audioStream && !stream.hasAudio ? audioStream : null;
		const useMux = muxAudioStream !== null;
		isLoading = true;
		setDownloading(true);
			resetProgressState();
			setProgressState(
				useMux ? m.download_btn_progress_queueing_mux() : m.download_btn_progress_preparing_browser(),
				{ value: useMux ? 8 : 20, indeterminate: true }
			);

		try {
			const controller = new AbortController();
			let downloadUrl: string;
			let muxJobId: string | null = null;
			let filename: string;

			if (useMux) {
				filename = buildFilename('mp4');
				const created = await createMuxedDownloadJob(
					stream.url,
					muxAudioStream.url,
					title,
					{
						sourceUrl: sourceUrl ?? undefined,
						videoFormatId: stream.formatId,
						audioFormatId: muxAudioStream.formatId
					},
					controller.signal
				);
				muxJobId = created.jobId;
					console.info('[downloadtool] mux job created', {
						jobId: created.jobId,
						statusUrl: created.statusUrl,
						fileUrl: created.fileUrl
					});
					setProgressState(m.download_btn_progress_muxing(), {
						value: 18,
						indeterminate: true
					});
				downloadUrl = await waitForMuxedDownloadJobReady(
					created.jobId,
					{
						onStatus: (update) => {
							const nextState = formatMuxStatus(update);
							setProgressState(nextState.label, {
								value: nextState.value,
								indeterminate: nextState.indeterminate,
								allowDecrease: nextState.allowDecrease
							});
						}
					},
					controller.signal
				);
					console.info('[downloadtool] mux job ready', {
						jobId: created.jobId,
						downloadUrl
					});
					setProgressState(m.download_btn_progress_starting_browser(), {
						value: 92,
						indeterminate: true
					});
			} else {
				filename = buildFilename(stream.format || 'mp4');
				downloadUrl = buildStreamUrl(stream.url, title, stream.format, {
					sourceUrl: sourceUrl ?? undefined,
					formatId: stream.formatId,
					patchInitMetadata: !stream.hasAudio && (stream.format || 'mp4').toLowerCase() === 'mp4'
				});
					console.info('[downloadtool] direct stream download prepared', {
						downloadUrl
					});
					setProgressState(m.download_btn_progress_starting_browser(), {
						value: 92,
						indeterminate: true
					});
			}

			const secureDownloadUrl = enforceHttps(downloadUrl);
			console.info('[downloadtool] invoking saveDownload', {
				muxJobId,
				downloadUrl: secureDownloadUrl,
				filename
			});

			const saveOpts = {
				requireFsaa: false,
				allowAnchorFallback: true
			} as const;

			await saveDownload(secureDownloadUrl, filename, controller.signal, saveOpts);
			console.info('[downloadtool] saveDownload resolved', {
				muxJobId,
				filename
			});
			if (muxJobId) {
				try {
					await releaseMuxedDownloadJob(muxJobId, controller.signal);
					console.info('[downloadtool] mux job released', {
						jobId: muxJobId
					});
				} catch (releaseError) {
					console.warn('Failed to release mux job hint:', releaseError);
				}
			}

				setProgressState(m.download_btn_progress_started(), { value: 100 });
				currentDownload.update((state) => ({ ...state, error: null }));

			isLoading = false;
			setDownloading(false);
			resetProgressState();
			} catch (err) {
				console.error('Download failed:', err);
				currentDownload.update((state) => ({ ...state, error: m.download_btn_error_failed() }));
				isLoading = false;
				setDownloading(false);
			resetProgressState();
		}
	}

	onDestroy(() => {
		stopProgressAnimation();
	});
</script>

<div class="cta-shell">
	<button
		type="button"
		class="download-cta"
		onclick={handleDownload}
		disabled={disabled || !stream || isLoading || $currentDownload.isDownloading}
		aria-label={
			isLoading ? m.download_btn_aria_preparing() : m.download_btn_aria_download_selected()
		}
	>
		{#if isLoading || $currentDownload.isDownloading}
			<span class="spinner"></span>
			<span>{m.download_btn_preparing()}</span>
		{:else}
			<AppIcon name="download" class="icon" />
			<span>{ctaLabel()}</span>
		{/if}
	</button>

	{#if $currentDownload.isDownloading}
		<div class="progress-row" aria-live="polite">
			<div class:indeterminate={progressIndeterminate} class="progress-track">
				<div
					class:indeterminate={progressIndeterminate}
					class="progress-fill"
					style:width={progressIndeterminate ? undefined : `${$downloadProgress}%`}
				></div>
			</div>
				<div class="progress-meta">
					<span class="progress-text">{progressLabel || m.download_btn_preparing()}</span>
				{#if !progressIndeterminate}
					<span class="progress-percent">{formatProgressPercent($downloadProgress)}</span>
				{/if}
			</div>
		</div>
	{:else}
		<p class="legal-note">
			{m.download_btn_legal_prefix()} <a href="/privacy">{m.download_btn_legal_privacy_link()}</a>.
		</p>
	{/if}
</div>

<style>
	.cta-shell {
		display: flex;
		flex-direction: column;
		gap: 0.65rem;
	}

	.download-cta {
		width: 100%;
		height: 3.5rem;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.6rem;
		border: 0;
		border-radius: 999px;
		background: linear-gradient(135deg, #ff2e93, #ec4899);
		color: #fff;
		font-size: 1rem;
		font-weight: 800;
		letter-spacing: 0.01em;
		cursor: pointer;
		box-shadow: 0 10px 22px -12px rgba(255, 46, 147, 0.85);
		transition: transform 0.2s ease, box-shadow 0.2s ease, filter 0.2s ease;
	}

	.download-cta:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 16px 28px -16px rgba(79, 70, 229, 0.75);
		filter: brightness(1.04);
	}

	.download-cta:active:not(:disabled) {
		transform: translateY(0);
	}

	.download-cta:disabled {
		cursor: not-allowed;
		opacity: 0.55;
		background: #cbd5e1;
		color: #475569;
		box-shadow: none;
	}

	:global(.icon) {
		font-size: 1.15rem;
	}

	.spinner {
		width: 1.05rem;
		height: 1.05rem;
		border-radius: 999px;
		border: 2px solid rgba(255, 255, 255, 0.32);
		border-top-color: #fff;
		animation: spin 0.75s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.progress-row {
		display: flex;
		flex-direction: column;
		align-items: stretch;
		gap: 0.45rem;
	}

	.progress-track {
		height: 0.36rem;
		border-radius: 999px;
		background: #e2e8f0;
		overflow: hidden;
	}

	.progress-track.indeterminate {
		position: relative;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(135deg, #4f46e5, #8b5cf6);
		border-radius: 999px;
	}

	.progress-fill.indeterminate {
		width: 32%;
		animation: progress-sweep 1.15s ease-in-out infinite;
	}

	.progress-meta {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.75rem;
	}

	.progress-text {
		font-size: 0.72rem;
		font-weight: 800;
		color: #475569;
		text-align: left;
	}

	.progress-percent {
		flex: 0 0 auto;
		font-size: 0.72rem;
		font-weight: 800;
		color: #475569;
	}

	.legal-note {
		margin: 0;
		text-align: center;
		font-size: 0.68rem;
		font-weight: 600;
		color: #94a3b8;
	}

	.legal-note a {
		color: #64748b;
		text-decoration: underline;
		text-underline-offset: 2px;
	}

	.legal-note a:hover {
		color: #ff2e93;
	}

	@media (max-width: 560px) {
		.download-cta {
			font-size: 0.92rem;
			height: 3.25rem;
			padding-inline: 0.9rem;
		}
	}

	@keyframes progress-sweep {
		0% {
			transform: translateX(-120%);
		}
		100% {
			transform: translateX(320%);
		}
	}
</style>
