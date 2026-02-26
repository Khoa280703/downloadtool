<script lang="ts">
	import { isValidVideoUrl, subscribeBatch, buildStreamUrl } from '$lib/api';
	import {
		batchQueue,
		batchProgress,
		isBatchActive,
		setEventSource,
		resetBatch,
		addBatchItem,
		setBatchProgress,
		startBatch,
		completeBatch
	} from '$stores/batch';
	import { downloadPool } from '$lib/download-pool';

	interface Props {
		onStart?: () => void;
		onComplete?: () => void;
	}

	let { onStart, onComplete }: Props = $props();

	let url = $state('');
	let error = $state('');

	/** Check if URL is a channel/playlist */
	function isChannelOrPlaylist(input: string): boolean {
		// YouTube playlist/channel patterns
		if (/youtube\.com\/(playlist|channel|@|c\/|user\/)/.test(input)) return true;
		if (/[?&]list=/.test(input)) return true;
		return false;
	}

	/** Validate URL */
	function validate(input: string): boolean {
		if (!input.trim()) {
			error = 'Please enter a URL';
			return false;
		}
		if (!isValidVideoUrl(input) && !isChannelOrPlaylist(input)) {
			error = 'Please enter a valid YouTube URL';
			return false;
		}
		if (!isChannelOrPlaylist(input)) {
			error = 'This appears to be a single video. Use the main input above.';
			return false;
		}
		error = '';
		return true;
	}

	/** Start batch download */
	async function handleSubmit(): Promise<void> {
		if (!validate(url)) return;

		resetBatch();
		startBatch();
		onStart?.();

		const es = subscribeBatch(
			url,
			(data) => {
				if (data.type === 'link') {
					if (!data.url || !data.title || data.index == null || data.total == null) {
						console.error('Invalid batch link payload:', data);
						return;
					}

					addBatchItem({
						url: data.url,
						title: data.title,
						status: 'pending'
					});
					setBatchProgress(data.index, data.total);

					// Add to download pool
					const filename = `${data.title.replace(/[^a-z0-9]/gi, '_')}.mp4`;
					const streamUrl = buildStreamUrl(data.url, data.title);
					downloadPool.add(streamUrl, filename);
				} else if (data.type === 'done') {
					completeBatch();
					es.close();
					onComplete?.();
				} else if (data.type === 'error') {
					console.error('Batch item error:', data.message);
					error = data.message || 'Batch extraction failed';
					completeBatch();
					es.close();
				}
			},
			() => {
				error = 'Connection error. Please try again.';
				completeBatch();
				onComplete?.();
			}
		);
		setEventSource(es);

	}

	/** Cancel batch download */
	function handleCancel(): void {
		resetBatch();
	}
</script>

<div class="batch-input">
	<div class="header">
		<h3>Batch Download</h3>
		<p class="subtitle">Download entire playlists or channels</p>
	</div>

	{#if !$isBatchActive}
		<form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
			<input
				type="url"
				placeholder="Paste playlist or channel URL..."
				bind:value={url}
				aria-label="Playlist or channel URL"
				class="url-field"
			/>

			{#if error}
				<span class="error-text" role="alert">{error}</span>
			{/if}

			<button
				type="submit"
				class="submit-btn"
				disabled={!url}
			>
				Start Batch Download
			</button>
		</form>
	{:else}
		<div class="active-batch">
			<div class="progress-info">
				<span class="progress-text">
					{$batchProgress.received} of {$batchProgress.total} videos
				</span>
				{#if $batchProgress.total > 0}
					<span class="progress-percent">
						{Math.round(($batchProgress.received / $batchProgress.total) * 100)}%
					</span>
				{/if}
			</div>

			<div class="progress-bar">
				<div
					class="progress-fill"
					style:width="{$batchProgress.total > 0 ? ($batchProgress.received / $batchProgress.total) * 100 : 0}%"
				></div>
			</div>

			<div class="pool-status">
				<span class="pool-text">
					{#if downloadPool.getStatus().active > 0}
						Downloading: {downloadPool.getStatus().active} / {downloadPool.getStatus().max} concurrent
					{:else}
						Waiting for downloads...
					{/if}
				</span>
			</div>

			<button
				type="button"
				class="cancel-btn"
				onclick={handleCancel}
			>
				Cancel
			</button>
		</div>
	{/if}
</div>

<style>
	.batch-input {
		padding: 1.5rem;
		background: var(--card-bg, #f9fafb);
		border-radius: 1rem;
		border: 1px solid var(--border-color, #e5e7eb);
	}

	.header {
		margin-bottom: 1rem;
	}

	h3 {
		margin: 0;
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-color, #111827);
	}

	.subtitle {
		margin: 0.25rem 0 0;
		font-size: 0.875rem;
		color: var(--text-secondary, #6b7280);
	}

	form {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.url-field {
		padding: 0.875rem 1rem;
		font-size: 1rem;
		border: 2px solid var(--border-color, #e5e7eb);
		border-radius: 0.75rem;
		background: var(--input-bg, #ffffff);
		color: var(--text-color, #111827);
	}

	.url-field:focus {
		outline: none;
		border-color: var(--primary-color, #3b82f6);
	}

	.error-text {
		color: var(--error-color, #ef4444);
		font-size: 0.875rem;
	}

	.submit-btn {
		padding: 0.875rem 1.5rem;
		font-size: 1rem;
		font-weight: 600;
		color: white;
		background: var(--secondary-color, #8b5cf6);
		border: none;
		border-radius: 0.75rem;
		cursor: pointer;
		transition: background 0.2s;
		min-height: 48px;
	}

	.submit-btn:hover:not(:disabled) {
		background: var(--secondary-hover, #7c3aed);
	}

	.submit-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.active-batch {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.progress-info {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.progress-text {
		font-size: 0.875rem;
		color: var(--text-secondary, #6b7280);
	}

	.progress-percent {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--primary-color, #3b82f6);
	}

	.progress-bar {
		height: 8px;
		background: var(--border-color, #e5e7eb);
		border-radius: 4px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: var(--primary-color, #3b82f6);
		border-radius: 4px;
		transition: width 0.3s ease;
	}

	.pool-status {
		font-size: 0.875rem;
		color: var(--text-secondary, #6b7280);
	}

	.cancel-btn {
		padding: 0.75rem 1.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-secondary, #6b7280);
		background: transparent;
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.cancel-btn:hover {
		background: var(--border-color, #e5e7eb);
	}

	@media (prefers-color-scheme: dark) {
		.batch-input {
			--card-bg: #1f2937;
			--border-color: #374151;
		}

		.url-field {
			--input-bg: #111827;
			--text-color: #f9fafb;
		}
	}
</style>
