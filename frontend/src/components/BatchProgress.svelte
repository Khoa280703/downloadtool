<script lang="ts">
	import { onMount } from 'svelte';
	import {
		batchQueue,
		batchProgress,
		isBatchActive,
		setAllPendingBatchItemsSelected,
		setBatchItemSelected
	} from '$stores/batch';
	import { getStatus } from '$lib/playlist-download-worker';

	let workerStatus = $state(getStatus());

	const selectedCount = $derived.by(
		() => $batchQueue.filter((item) => item.selected !== false).length
	);
	const completedCount = $derived.by(
		() => $batchQueue.filter((item) => item.selected !== false && item.status === 'completed').length
	);
	const errorCount = $derived.by(
		() => $batchQueue.filter((item) => item.selected !== false && item.status === 'error').length
	);
	const pendingCount = $derived.by(
		() => $batchQueue.filter((item) => item.selected !== false && item.status === 'pending').length
	);
	const canEditSelection = $derived.by(() => !$isBatchActive && workerStatus.active === 0);

	function truncate(text: string, maxLength: number = 32): string {
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

	function toggleItemSelection(videoId: string, checked: boolean): void {
		if (!canEditSelection) return;
		setBatchItemSelected(videoId, checked);
	}

	function selectAllPending(): void {
		if (!canEditSelection) return;
		setAllPendingBatchItemsSelected(true);
	}

	function clearAllPending(): void {
		if (!canEditSelection) return;
		setAllPendingBatchItemsSelected(false);
	}

	function getItemStatusLabel(item: { status: string; selected?: boolean }): string {
		if (item.status === 'completed') return 'Success';
		if (item.status === 'error') return 'Fail';
		if (item.status === 'downloading') return 'In Progress';
		if (item.selected === false) return 'Skipped';
		return 'Pending';
	}

	function getItemStatusClass(item: { status: string; selected?: boolean }): string {
		if (item.status === 'completed') return 'status-success';
		if (item.status === 'error') return 'status-fail';
		if (item.status === 'downloading') return 'status-in-progress';
		if (item.selected === false) return 'status-skipped';
		return 'status-pending';
	}

	onMount(() => {
		const interval = setInterval(() => {
			workerStatus = getStatus();
		}, 500);
		return () => clearInterval(interval);
	});
</script>

<div class="batch-progress" class:is-idle={!$isBatchActive && $batchQueue.length === 0}>
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
			<span class="summary-pill neutral">Selected: {selectedCount}</span>
			<span class="summary-pill success">Done: {completedCount}</span>
			<span class="summary-pill error">Errors: {errorCount}</span>
		</div>

		{#if canEditSelection}
			<div class="selection-actions">
				<button type="button" class="selection-btn" onclick={selectAllPending}>Select all</button>
				<button type="button" class="selection-btn" onclick={clearAllPending}>Clear all</button>
			</div>
		{/if}

		<div class="queue-list" role="list">
			{#each $batchQueue as item}
				<div class="queue-item" role="listitem">
					{#if item.status === 'pending'}
						<button
							type="button"
							class="item-toggle"
							class:is-selected={item.selected !== false}
							disabled={!canEditSelection}
							onclick={() => toggleItemSelection(item.videoId, item.selected === false)}
							aria-label={item.selected === false ? 'Select video' : 'Deselect video'}
							aria-pressed={item.selected !== false}
							title={item.selected === false ? 'Select video' : 'Deselect video'}
						>
							<span class="material-symbols-outlined toggle-icon">
								{item.selected !== false ? 'check' : 'add'}
							</span>
						</button>
					{:else}
						<span class="item-toggle-spacer"></span>
					{/if}
					<span class="item-title" title={item.title}>{truncate(item.title)}</span>
					<span class={`item-status ${getItemStatusClass(item)}`}>{getItemStatusLabel(item)}</span>
				</div>
			{/each}
		</div>
	{:else}
		<div class="idle-note">
			Fetch Playlist Videos để xem tiến trình tải và trạng thái từng video.
		</div>
	{/if}
</div>

<style>
	.batch-progress {
		padding: 1rem;
		background: linear-gradient(155deg, #2d1b36 0%, #3a2347 55%, #4a2a5f 100%);
		border-radius: 1.5rem;
		border: 1px solid rgba(255, 255, 255, 0.15);
		box-shadow: 0 22px 40px -26px rgba(45, 27, 54, 0.9);
		position: relative;
		overflow: hidden;
		min-height: 240px;
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

	.idle-note {
		margin-top: 0.75rem;
		padding: 0.85rem 0.9rem;
		border-radius: 0.75rem;
		border: 1px solid rgba(255, 255, 255, 0.18);
		background: rgba(255, 255, 255, 0.12);
		color: rgba(255, 242, 250, 0.9);
		font-size: 0.78rem;
		font-weight: 700;
		line-height: 1.45;
		position: relative;
		z-index: 1;
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

	.summary-pill.neutral {
		background: rgba(255, 255, 255, 0.18);
		color: #ffe9f6;
	}

	.selection-actions {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
		position: relative;
		z-index: 1;
	}

	.selection-btn {
		font-size: 0.6875rem;
		font-weight: 700;
		padding: 0.28rem 0.6rem;
		border-radius: 999px;
		border: 1px solid rgba(255, 255, 255, 0.3);
		background: rgba(255, 255, 255, 0.12);
		color: #fff4fb;
		cursor: pointer;
		transition: filter 0.2s ease, transform 0.2s ease;
	}

	.selection-btn:hover {
		filter: brightness(1.08);
		transform: translateY(-1px);
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
		gap: 0.375rem;
		max-height: 190px;
		overflow-y: auto;
		padding-right: 0.25rem;
		position: relative;
		z-index: 1;
	}

	.queue-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.5rem;
		background: rgba(255, 255, 255, 0.1);
		border: 1px solid rgba(255, 255, 255, 0.16);
		border-radius: 0.6rem;
		font-size: 0.75rem;
	}

	.item-toggle {
		width: 1rem;
		height: 1rem;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		border-radius: 0.3rem;
		border: 1px solid rgba(255, 255, 255, 0.35);
		background: rgba(255, 255, 255, 0.1);
		color: rgba(255, 255, 255, 0.75);
		padding: 0;
		cursor: pointer;
		transition: transform 0.2s ease, filter 0.2s ease, background 0.2s ease, border-color 0.2s ease;
	}

	.item-toggle .toggle-icon {
		font-size: 0.74rem;
		font-weight: 700;
		line-height: 1;
	}

	.item-toggle.is-selected {
		background: linear-gradient(135deg, #ff4d8c 0%, #ffb938 100%);
		border-color: rgba(255, 255, 255, 0.5);
		color: #ffffff;
		box-shadow: 0 6px 14px -10px rgba(255, 77, 140, 0.9);
	}

	.item-toggle:hover:not(:disabled) {
		filter: brightness(1.05);
		transform: translateY(-1px);
	}

	.item-toggle:disabled {
		cursor: not-allowed;
		opacity: 0.45;
	}

	.item-toggle-spacer {
		width: 1rem;
		height: 1rem;
		flex: 0 0 auto;
	}

	.item-title {
		flex: 1;
		color: #fff6fc;
		font-weight: 700;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.item-status {
		min-width: 4.8rem;
		text-align: right;
		font-size: 0.625rem;
		font-weight: 800;
		letter-spacing: 0.01em;
	}

	.item-status.status-pending {
		color: rgba(255, 255, 255, 0.82);
	}

	.item-status.status-in-progress {
		color: #ffd27b;
	}

	.item-status.status-success {
		color: #86efac;
	}

	.item-status.status-fail {
		color: #fca5a5;
	}

	.item-status.status-skipped {
		color: rgba(255, 255, 255, 0.62);
	}

	.queue-list::-webkit-scrollbar {
		width: 6px;
	}

	.queue-list::-webkit-scrollbar-thumb {
		background: rgba(255, 255, 255, 0.32);
		border-radius: 999px;
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

	:global(.page-root.theme-dark) .idle-note {
		background: rgba(255, 77, 140, 0.12);
		border-color: rgba(255, 77, 140, 0.24);
		color: rgba(250, 234, 255, 0.95);
	}

	:global(.page-root.theme-dark) .item-toggle {
		border-color: rgba(255, 124, 175, 0.38);
		background: rgba(255, 124, 175, 0.1);
		color: rgba(255, 235, 247, 0.8);
	}

	:global(.page-root.theme-dark) .item-toggle.is-selected {
		border-color: rgba(255, 225, 239, 0.65);
		background: linear-gradient(135deg, #ff5f99 0%, #ffbc47 100%);
		color: #ffffff;
	}

	:global(.page-root.theme-dark) .item-title {
		color: #ffe8f5;
	}
</style>
