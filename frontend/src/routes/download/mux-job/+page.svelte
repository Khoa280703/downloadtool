<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import {
		createMuxedDownloadJob,
		waitForMuxedDownloadJobReady
	} from '$lib/api';

	let stage = $state('Đang chuẩn bị mux job...');
	let detail = $state('Giữ tab này mở cho đến khi tải bắt đầu.');
	let fatalError = $state<string | null>(null);
	let bootstrapped = false;

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
			stage = 'Thiếu tham số tải';
			detail = 'Link launcher không hợp lệ hoặc đã bị cắt mất query.';
			fatalError = 'Missing required video/audio stream URLs';
			return;
		}

		try {
			stage = 'Đang tạo job...';
			detail = 'Frontend sẽ chuyển yêu cầu sang worker pipeline.';
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

			stage = 'Đang đợi worker hoàn tất...';
			detail = `Job ${created.jobId} đang mux và upload artifact.`;
			const downloadUrl = await waitForMuxedDownloadJobReady(created.jobId);
			console.info('[downloadtool] launcher mux job ready', {
				jobId: created.jobId,
				downloadUrl
			});

			stage = 'Đang bắt đầu tải...';
			detail = 'Nếu trình duyệt không tự tải, hãy kiểm tra popup chặn tải xuống.';
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

			stage = 'Không thể bắt đầu tải';
			detail = error instanceof Error ? error.message : 'Unexpected error';
			fatalError = detail;
		}
	}

	onMount(() => {
		void bootstrapDownload();
	});
</script>

<svelte:head>
	<title>Mux Job Launcher</title>
</svelte:head>

<div class="launcher-shell">
	<div class="launcher-card">
		<p class="eyebrow">Mux Jobs</p>
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
