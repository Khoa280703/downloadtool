<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import * as m from '$lib/paraglide/messages';
	import {
		createMuxedDownloadJob,
		waitForMuxedDownloadJobReady,
		type MuxJobStatusUpdate
	} from '$lib/api';

	let stage = $state<string>(m.mux_job_stage_preparing());
	let detail = $state<string>(m.mux_job_detail_keep_tab_open());
	let fatalError = $state<string | null>(null);
	let bootstrapped = false;

	function describeMuxStatus(update: MuxJobStatusUpdate): { stage: string; detail: string } {
		const elapsedSeconds = Math.floor(update.elapsedMs / 1000);

		if (update.status === 'queued') {
			return {
				stage: m.mux_job_stage_queued(),
				detail:
					elapsedSeconds >= 8
						? m.mux_job_detail_queued_waiting({ jobId: update.jobId })
						: m.mux_job_detail_queued_received({ jobId: update.jobId })
			};
		}

		if (update.status === 'leased') {
			return {
				stage: m.mux_job_stage_leased(),
				detail: m.mux_job_detail_leased({ jobId: update.jobId })
			};
		}

		if (update.status === 'processing') {
			const phase = Math.floor(elapsedSeconds / 6) % 4;
			if (phase === 0) {
				return {
					stage: m.mux_job_stage_processing_fetching_streams(),
					detail: m.mux_job_detail_processing_fetching_streams({ jobId: update.jobId })
				};
			}
			if (phase === 1) {
				return {
					stage: m.mux_job_stage_processing_muxing(),
					detail: m.mux_job_detail_processing_muxing({ jobId: update.jobId })
				};
			}
			if (phase === 2) {
				return {
					stage: m.mux_job_stage_processing_finalizing(),
					detail: m.mux_job_detail_processing_finalizing({ jobId: update.jobId })
				};
			}
			return {
				stage: m.mux_job_stage_processing_still_running(),
				detail: m.mux_job_detail_processing_still_running({ jobId: update.jobId })
			};
		}

		if (update.status === 'ready') {
			return {
				stage: m.mux_job_stage_ready(),
				detail: m.mux_job_detail_ready({ jobId: update.jobId })
			};
		}

		if (update.status === 'failed') {
			return {
				stage: m.mux_job_stage_failed(),
				detail: m.mux_job_detail_failed({ jobId: update.jobId })
			};
		}

		return {
			stage: m.mux_job_stage_expired(),
			detail: m.mux_job_detail_expired({ jobId: update.jobId })
		};
	}

	function readQuery(): {
		videoUrl: string | null;
		audioUrl: string | null;
		title: string;
		sourceUrl?: string;
		videoFormatId?: string;
		audioFormatId?: string;
	} {
		const params = new URLSearchParams(window.location.search);
		return {
			videoUrl: params.get('video_url'),
			audioUrl: params.get('audio_url'),
			title: params.get('title') || 'video',
			sourceUrl: params.get('source_url') || undefined,
			videoFormatId: params.get('video_format_id') || undefined,
			audioFormatId: params.get('audio_format_id') || undefined
		};
	}

	async function redirectToLogin(): Promise<void> {
		const redirectTo = `/download/mux-job${window.location.search}`;
		await goto(`/?auth=required&redirectTo=${encodeURIComponent(redirectTo)}`);
	}

	async function bootstrapDownload(): Promise<void> {
		if (bootstrapped) return;
		bootstrapped = true;

		const payload = readQuery();
		if (!payload.videoUrl || !payload.audioUrl) {
			stage = m.mux_job_stage_missing_params();
			detail = m.mux_job_detail_missing_params();
			fatalError = m.mux_job_error_missing_required_urls();
			return;
		}

		try {
			stage = m.mux_job_stage_creating();
			detail = m.mux_job_detail_creating();
			console.info('[downloadtool] launcher bootstrap start', payload);
			const created = await createMuxedDownloadJob(
				payload.videoUrl,
				payload.audioUrl,
				payload.title,
				{
					sourceUrl: payload.sourceUrl,
					videoFormatId: payload.videoFormatId,
					audioFormatId: payload.audioFormatId
				}
			);
			console.info('[downloadtool] launcher mux job created', created);

			stage = m.mux_job_stage_waiting_worker();
			detail = m.mux_job_detail_waiting_worker({ jobId: created.jobId });
			const downloadUrl = await waitForMuxedDownloadJobReady(created.jobId, {
				onStatus: (update) => {
					const next = describeMuxStatus(update);
					stage = next.stage;
					detail = next.detail;
				}
			});
			console.info('[downloadtool] launcher mux job ready', {
				jobId: created.jobId,
				downloadUrl
			});

			stage = m.mux_job_stage_starting_download();
			detail = m.mux_job_detail_starting_download();
			console.info('[downloadtool] launcher redirecting to download url', downloadUrl);
			window.location.replace(downloadUrl);
		} catch (error) {
			console.error('[downloadtool] launcher failed', error);
			const status = typeof error === 'object' && error !== null && 'status' in error
				? Number((error as { status?: number }).status)
				: undefined;
			if (status === 401) {
				await redirectToLogin();
				return;
			}

			stage = m.mux_job_stage_cannot_start_download();
			detail = error instanceof Error ? error.message : m.mux_job_error_unexpected();
			fatalError = detail;
		}
	}

	onMount(() => {
		void bootstrapDownload();
	});
</script>

<svelte:head>
	<title>{m.mux_job_page_title()}</title>
</svelte:head>

<div class="launcher-shell">
	<div class="launcher-card">
		<p class="eyebrow">{m.mux_job_page_eyebrow()}</p>
		<h1>{stage}</h1>
		<p>{detail}</p>
		{#if fatalError}
			<pre>{fatalError}</pre>
		{/if}
	</div>
</div>

<style>
	.launcher-shell {
		min-height: 100vh;
		display: grid;
		place-items: center;
		padding: 2rem;
		background:
			radial-gradient(circle at top, rgba(255, 99, 132, 0.12), transparent 35%),
			linear-gradient(180deg, #f8fafc 0%, #eef2ff 100%);
	}

	.launcher-card {
		width: min(100%, 34rem);
		padding: 2rem;
		border-radius: 1.5rem;
		background: rgba(255, 255, 255, 0.9);
		box-shadow: 0 24px 60px rgba(15, 23, 42, 0.12);
	}

	.eyebrow {
		margin: 0 0 0.75rem;
		font-size: 0.8rem;
		font-weight: 700;
		letter-spacing: 0.12em;
		text-transform: uppercase;
		color: #e11d48;
	}

	h1 {
		margin: 0 0 0.75rem;
		font-size: clamp(1.4rem, 3vw, 2rem);
		color: #0f172a;
	}

	p {
		margin: 0;
		line-height: 1.6;
		color: #334155;
	}

	pre {
		margin: 1rem 0 0;
		padding: 0.85rem 1rem;
		border-radius: 1rem;
		background: #0f172a;
		color: #e2e8f0;
		white-space: pre-wrap;
		word-break: break-word;
		font: inherit;
	}
</style>
