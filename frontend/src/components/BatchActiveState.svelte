<script lang="ts">
	import { batchProgress } from '$stores/batch';
	import { getStatus } from '$lib/playlist-download-worker';
	import { onMount } from 'svelte';

	interface Props {
		fsaaSupported: boolean;
		dirPicked: boolean;
		onPickDirectory: () => Promise<void>;
		onCancel: () => void;
	}

	let { fsaaSupported, dirPicked, onPickDirectory, onCancel }: Props = $props();
	let workerStatus = $state(getStatus());

	onMount(() => {
		const timer = setInterval(() => {
			workerStatus = getStatus();
		}, 500);
		return () => clearInterval(timer);
	});
</script>

<div class="active-batch">
	<div class="progress-info">
		<span class="progress-text">{$batchProgress.received} / {$batchProgress.total} videos</span>
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
			{#if workerStatus.active > 0}
				Downloading: {workerStatus.active} / {workerStatus.max}
			{:else if workerStatus.ready > 0}
				Prepared: {workerStatus.ready} video(s), starting soon...
			{:else}
				Waiting for playlist links...
			{/if}
		</span>
	</div>

	<div class="actions">
		{#if fsaaSupported}
			<button type="button" class="folder-btn" onclick={onPickDirectory}>
				{dirPicked ? 'Save folder selected' : 'Choose save folder'}
			</button>
		{/if}

		<button type="button" class="cancel-btn" onclick={onCancel}>Cancel run</button>
	</div>
</div>

<style>
	.active-batch {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		padding: 1rem;
		border-radius: 1.2rem;
		background: rgba(255, 255, 255, 0.78);
		border: 1px solid #ffd7e8;
	}

	.progress-info {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.progress-text {
		font-size: 0.875rem;
		color: rgba(45, 27, 54, 0.72);
		font-weight: 700;
	}

	.progress-percent {
		font-size: 0.875rem;
		font-weight: 800;
		color: #ff4d8c;
	}

	.progress-bar {
		height: 8px;
		background: #ffe8f2;
		border-radius: 4px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #ff4d8c 0%, #ffb938 100%);
		border-radius: 4px;
		transition: width 0.3s ease;
	}

	.pool-status {
		font-size: 0.875rem;
		color: rgba(45, 27, 54, 0.74);
		font-weight: 600;
	}

	.actions {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.folder-btn,
	.cancel-btn {
		padding: 0.75rem 1rem;
		font-size: 0.875rem;
		font-weight: 700;
		border-radius: 999px;
		cursor: pointer;
		transition: transform 0.2s ease, filter 0.2s ease, background 0.2s ease;
	}

	.folder-btn {
		color: #2d1b36;
		background: #fff2f9;
		border: 1px solid #ffcce1;
	}

	.folder-btn:hover {
		background: #ffe0ef;
		transform: translateY(-1px);
	}

	.cancel-btn {
		color: white;
		background: #2d1b36;
		border: 1px solid #2d1b36;
	}

	.cancel-btn:hover {
		filter: brightness(1.08);
		transform: translateY(-1px);
	}

	:global(.page-root.theme-dark) .active-batch {
		background: rgba(20, 20, 29, 0.72);
		border-color: rgba(255, 77, 140, 0.25);
	}

	:global(.page-root.theme-dark) .progress-text,
	:global(.page-root.theme-dark) .pool-status {
		color: rgba(224, 208, 245, 0.86);
	}

	:global(.page-root.theme-dark) .folder-btn {
		background: rgba(255, 77, 140, 0.15);
		border-color: rgba(255, 77, 140, 0.25);
		color: #ffe2f0;
	}
</style>
