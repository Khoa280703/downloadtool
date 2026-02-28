<script lang="ts">
	import { onMount } from 'svelte';
	import { batchQueue, batchProgress, isBatchActive } from '$stores/batch';
	import { getStatus } from '$lib/playlist-download-worker';

	let workerStatus = $state(getStatus());

	const completedCount = $derived.by(
		() => $batchQueue.filter((item) => item.status === 'completed').length
	);
	const errorCount = $derived.by(() => $batchQueue.filter((item) => item.status === 'error').length);
	const pendingCount = $derived.by(() => $batchQueue.filter((item) => item.status === 'pending').length);

	function getStatusIcon(status: string): string {
		switch (status) {
			case 'completed':
				return '✓';
			case 'error':
				return '✗';
			case 'downloading':
				return '↓';
			default:
				return '○';
		}
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'completed':
				return 'var(--success-color, #22c55e)';
			case 'error':
				return 'var(--error-color, #ef4444)';
			case 'downloading':
				return '#ff4d8c';
			default:
				return 'rgba(255, 255, 255, 0.56)';
		}
	}

	function truncate(text: string, maxLength: number = 40): string {
		if (text.length <= maxLength) return text;
		return text.slice(0, maxLength) + '...';
	}

	function formatCooldown(ms: number): string {
		const totalSeconds = Math.max(0, Math.ceil(ms / 1000));
		const minutes = Math.floor(totalSeconds / 60);
		const seconds = totalSeconds % 60;
		return `${minutes}:${String(seconds).padStart(2, '0')}`;
	}

	function formatSummary(): string {
		const waiting = workerStatus.pending + workerStatus.ready;
		if (workerStatus.active > 0) return `${workerStatus.active} active • ${waiting} waiting`;
		if (pendingCount > 0) return `${pendingCount} queued • click Start Playlist Download`;
		if (waiting > 0) return `${waiting} waiting`;
		return 'Finalizing playlist';
	}

	onMount(() => {
		const interval = setInterval(() => {
			workerStatus = getStatus();
		}, 500);
		return () => clearInterval(interval);
	});
</script>

{#if $isBatchActive || $batchQueue.length > 0}
	<div class="batch-progress">
		<div class="header">
			<h4>Playlist Progress</h4>
			<span class="count">{$batchProgress.received} / {$batchProgress.total}</span>
		</div>

		<div class="progress-bar">
			<div
				class="progress-fill"
				style:width="{$batchProgress.total > 0 ? ($batchProgress.received / $batchProgress.total) * 100 : 0}%"
				class:complete={$batchProgress.received >= $batchProgress.total && $batchProgress.total > 0}
			></div>
		</div>

		{#if $batchQueue.length > 0}
			<div class="pool-indicator">
				<div class="slots">
					{#each Array(workerStatus.max) as _, i}
						<span class="slot" class:active={i < workerStatus.active} title="Download slot {i + 1}"></span>
					{/each}
				</div>
				<span class="pool-text">{formatSummary()}</span>
			</div>
			{#if workerStatus.circuitOpen}
				<div class="cooldown-note">Rate-limited, retry in {formatCooldown(workerStatus.cooldownMs)}</div>
			{/if}

			<div class="summary-row">
				<span class="summary-pill success">Done: {completedCount}</span>
				<span class="summary-pill error">Errors: {errorCount}</span>
			</div>
		{/if}

		{#if $batchQueue.length > 0}
			<div class="queue-list" role="list">
				{#each $batchQueue as item}
					<div class="queue-item" role="listitem">
						<span class="status-icon" style:color={getStatusColor(item.status)}>
							{getStatusIcon(item.status)}
						</span>
						<span class="item-title" title={item.title}>{truncate(item.title)}</span>
						{#if item.error}
							<span class="item-error" title={item.error}>Error</span>
						{/if}
					</div>
				{/each}
			</div>
		{/if}
	</div>
{/if}

<style>
	.batch-progress {
		padding: 1.25rem;
		background: linear-gradient(155deg, #2d1b36 0%, #3a2347 55%, #4a2a5f 100%);
		border-radius: 1.5rem;
		border: 1px solid rgba(255, 255, 255, 0.15);
		box-shadow: 0 22px 40px -26px rgba(45, 27, 54, 0.9);
		position: relative;
		overflow: hidden;
	}

	.batch-progress::after {
		content: '';
		position: absolute;
		inset: 0;
		pointer-events: none;
		background:
			radial-gradient(circle at 8% 15%, rgba(255, 77, 140, 0.25), transparent 42%),
			radial-gradient(circle at 90% 90%, rgba(255, 185, 56, 0.18), transparent 35%);
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
		position: relative;
		z-index: 1;
	}

	h4 {
		margin: 0;
		font-size: 0.95rem;
		font-weight: 800;
		color: #fff6fc;
		letter-spacing: 0.01em;
	}

	.count {
		font-size: 0.875rem;
		font-weight: 800;
		color: #ffd1e5;
	}

	.progress-bar {
		height: 8px;
		background: rgba(255, 255, 255, 0.18);
		border-radius: 4px;
		overflow: hidden;
		margin-bottom: 1rem;
		position: relative;
		z-index: 1;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #ff4d8c 0%, #ffb938 100%);
		border-radius: 4px;
		transition: width 0.3s ease;
	}

	.progress-fill.complete {
		background: linear-gradient(90deg, #22c55e 0%, #86efac 100%);
	}

	.pool-indicator {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 0.75rem;
		padding: 0.75rem;
		background: rgba(255, 255, 255, 0.12);
		border-radius: 0.8rem;
		border: 1px solid rgba(255, 255, 255, 0.2);
		position: relative;
		z-index: 1;
	}

	.slots {
		display: flex;
		gap: 0.375rem;
	}

	.slot {
		width: 12px;
		height: 12px;
		border-radius: 50%;
		background: rgba(255, 255, 255, 0.24);
		transition: background 0.3s;
	}

	.slot.active {
		background: var(--success-color, #22c55e);
		animation: pulse 1.5s infinite;
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.6;
		}
	}

	.pool-text {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.82);
		font-weight: 700;
	}

	.cooldown-note {
		margin-bottom: 0.75rem;
		font-size: 0.75rem;
		font-weight: 600;
		color: #fcd34d;
		position: relative;
		z-index: 1;
	}

	.summary-row {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
		position: relative;
		z-index: 1;
	}

	.summary-pill {
		font-size: 0.75rem;
		font-weight: 600;
		padding: 0.25rem 0.5rem;
		border-radius: 999px;
	}

	.summary-pill.success {
		background: rgba(34, 197, 94, 0.22);
		color: #dcfce7;
	}

	.summary-pill.error {
		background: rgba(239, 68, 68, 0.18);
		color: #fee2e2;
	}

	.queue-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		position: relative;
		z-index: 1;
	}

	.queue-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem;
		background: rgba(255, 255, 255, 0.1);
		border: 1px solid rgba(255, 255, 255, 0.16);
		border-radius: 0.6rem;
		font-size: 0.8125rem;
	}

	.status-icon {
		font-weight: 600;
		width: 1rem;
		text-align: center;
	}

	.item-title {
		flex: 1;
		color: #fff6fc;
		font-weight: 700;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.item-error {
		font-size: 0.6875rem;
		padding: 0.125rem 0.375rem;
		background: var(--error-bg, rgba(239, 68, 68, 0.1));
		color: var(--error-color, #ef4444);
		border-radius: 0.25rem;
	}

	:global(.page-root.theme-dark) .batch-progress {
		background: linear-gradient(180deg, rgba(30, 30, 42, 0.92) 0%, rgba(25, 25, 35, 0.92) 100%);
		border-color: rgba(255, 77, 140, 0.22);
	}

	:global(.page-root.theme-dark) h4 {
		color: #f3e8ff;
	}

	:global(.page-root.theme-dark) .pool-text {
		color: rgba(224, 208, 245, 0.82);
	}

	:global(.page-root.theme-dark) .pool-indicator,
	:global(.page-root.theme-dark) .queue-item {
		background: rgba(255, 77, 140, 0.1);
		border-color: rgba(255, 77, 140, 0.22);
	}

	:global(.page-root.theme-dark) .item-title {
		color: #ffe8f5;
	}
</style>
