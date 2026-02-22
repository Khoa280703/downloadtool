<script lang="ts">
	import { batchQueue, batchProgress, isBatchActive } from '$stores/batch';

	/** Get status icon based on item status */
	function getStatusIcon(status: string): string {
		switch (status) {
			case 'completed': return '✓';
			case 'error': return '✗';
			case 'downloading': return '↓';
			default: return '○';
		}
	}

	/** Get status color */
	function getStatusColor(status: string): string {
		switch (status) {
			case 'completed': return 'var(--success-color, #22c55e)';
			case 'error': return 'var(--error-color, #ef4444)';
			case 'downloading': return 'var(--primary-color, #3b82f6)';
			default: return 'var(--text-secondary, #6b7280)';
		}
	}

	/** Truncate long titles */
	function truncate(text: string, maxLength: number = 40): string {
		if (text.length <= maxLength) return text;
		return text.slice(0, maxLength) + '...';
	}
</script>

{#if $isBatchActive || $batchQueue.length > 0}
	<div class="batch-progress">
		<div class="header">
			<h4>Batch Progress</h4>
			<span class="count">
				{$batchProgress.received} / {$batchProgress.total}
			</span>
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
					{#each Array(3) as _, i}
						<span
							class="slot"
							class:active={i < Math.min($batchQueue.filter(q => q.status === 'downloading').length, 3)}
							title="Download slot {i + 1}"
						></span>
					{/each}
				</div>
				<span class="pool-text">
					{Math.min($batchQueue.filter(q => q.status === 'downloading').length, 3)} active downloads
				</span>
			</div>
		{/if}

		{#if $batchQueue.length > 0}
			<div class="queue-list" role="list">
				{#each $batchQueue.slice(-5) as item}
					<div class="queue-item" role="listitem">
						<span
							class="status-icon"
							style:color={getStatusColor(item.status)}
						>
							{getStatusIcon(item.status)}
						</span>
						<span class="item-title" title={item.title}>
							{truncate(item.title)}
						</span>
						{#if item.error}
							<span class="item-error" title={item.error}>Error</span>
						{/if}
					</div>
				{/each}
				{#if $batchQueue.length > 5}
					<div class="more-items">
						+{$batchQueue.length - 5} more items
					</div>
				{/if}
			</div>
		{/if}
	</div>
{/if}

<style>
	.batch-progress {
		padding: 1.25rem;
		background: var(--card-bg, #f9fafb);
		border-radius: 1rem;
		border: 1px solid var(--border-color, #e5e7eb);
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
	}

	h4 {
		margin: 0;
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-color, #111827);
	}

	.count {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--primary-color, #3b82f6);
	}

	.progress-bar {
		height: 8px;
		background: var(--border-color, #e5e7eb);
		border-radius: 4px;
		overflow: hidden;
		margin-bottom: 1rem;
	}

	.progress-fill {
		height: 100%;
		background: var(--primary-color, #3b82f6);
		border-radius: 4px;
		transition: width 0.3s ease;
	}

	.progress-fill.complete {
		background: var(--success-color, #22c55e);
	}

	.pool-indicator {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1rem;
		padding: 0.75rem;
		background: var(--input-bg, #ffffff);
		border-radius: 0.5rem;
	}

	.slots {
		display: flex;
		gap: 0.375rem;
	}

	.slot {
		width: 12px;
		height: 12px;
		border-radius: 50%;
		background: var(--border-color, #e5e7eb);
		transition: background 0.3s;
	}

	.slot.active {
		background: var(--success-color, #22c55e);
		animation: pulse 1.5s infinite;
	}

	@keyframes pulse {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.6; }
	}

	.pool-text {
		font-size: 0.75rem;
		color: var(--text-secondary, #6b7280);
	}

	.queue-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		max-height: 200px;
		overflow-y: auto;
	}

	.queue-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem;
		background: var(--input-bg, #ffffff);
		border-radius: 0.375rem;
		font-size: 0.8125rem;
	}

	.status-icon {
		font-weight: 600;
		width: 1rem;
		text-align: center;
	}

	.item-title {
		flex: 1;
		color: var(--text-color, #111827);
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

	.more-items {
		text-align: center;
		font-size: 0.75rem;
		color: var(--text-secondary, #6b7280);
		padding: 0.5rem;
	}

	@media (prefers-color-scheme: dark) {
		.batch-progress {
			--card-bg: #1f2937;
			--border-color: #374151;
		}

		.pool-indicator,
		.queue-item {
			--input-bg: #111827;
		}
	}
</style>
