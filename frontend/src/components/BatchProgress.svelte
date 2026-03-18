<script lang="ts">
	import AppIcon from '$components/AppIcon.svelte';
	import * as m from '$lib/paraglide/messages';
	import {
		batchQueue,
		batchProgress,
		isBatchActive,
		setAllPendingBatchItemsSelected,
		setBatchItemSelected
	} from '$stores/batch';

	const selectedCount = $derived.by(
		() => $batchQueue.filter((item) => item.selected !== false).length
	);
	const completedCount = $derived.by(
		() => $batchQueue.filter((item) => item.selected !== false && item.status === 'completed').length
	);
	const settledCount = $derived.by(
		() =>
			$batchQueue.filter(
				(item) => item.selected !== false && (item.status === 'completed' || item.status === 'error')
			).length
	);
	const errorCount = $derived.by(
		() => $batchQueue.filter((item) => item.selected !== false && item.status === 'error').length
	);
	const pendingCount = $derived.by(
		() => $batchQueue.filter((item) => item.selected !== false && item.status === 'pending').length
	);
	const downloadingCount = $derived.by(
		() => $batchQueue.filter((item) => item.status === 'downloading').length
	);
	const canEditSelection = $derived.by(() => !$isBatchActive && downloadingCount === 0);
	const progressTotal = $derived.by(() => (selectedCount > 0 ? selectedCount : $batchProgress.total));
	const progressDone = $derived.by(() => (selectedCount > 0 ? settledCount : $batchProgress.received));
	const progressPercent = $derived.by(() => {
		if (progressTotal <= 0) return 0;
		return Math.min(100, Math.round((progressDone / progressTotal) * 100));
	});

	function truncate(text: string, maxLength: number = 56): string {
		if (text.length <= maxLength) return text;
		return text.slice(0, maxLength) + '...';
	}

	function formatProgressPercent(value: number | null | undefined): string {
		if (typeof value !== 'number' || Number.isNaN(value)) return '0%';
		if (value >= 100) return '100%';
		return `${value.toFixed(2)}%`;
	}

	function formatSummary(): string {
		if (downloadingCount > 0) {
			return m.playlist_progress_summary_active_waiting({
				active: String(downloadingCount),
				waiting: String(pendingCount)
			});
		}
		if (pendingCount > 0) {
			return m.playlist_progress_summary_queued({
				pending: String(pendingCount)
			});
		}
		return m.playlist_progress_summary_finalizing();
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

	function getItemStatusLabel(item: { status: string; selected?: boolean; progressLabel?: string }): string {
		if (item.status === 'completed') return m.playlist_progress_status_success();
		if (item.status === 'error') return m.playlist_progress_status_fail();
		if (item.status === 'downloading' || item.progressLabel) return m.playlist_progress_status_in_progress();
		if (item.selected === false) return m.playlist_progress_status_skipped();
		return m.playlist_progress_status_pending();
	}

	function getItemStatusClass(item: { status: string; selected?: boolean; progressLabel?: string }): string {
		if (item.status === 'completed') return 'status-success';
		if (item.status === 'error') return 'status-fail';
		if (item.status === 'downloading' || item.progressLabel) return 'status-in-progress';
		if (item.selected === false) return 'status-skipped';
		return 'status-pending';
	}

	function getDisplayPriority(item: {
		status: string;
		selected?: boolean;
		progressLabel?: string;
	}): number {
		if (item.status === 'downloading' || item.progressLabel) return 0;
		if (item.selected !== false && item.status === 'pending') return 1;
		if (item.selected === false) return 2;
		if (item.status === 'error') return 3;
		if (item.status === 'completed') return 4;
		return 5;
	}

	const orderedBatchQueue = $derived.by(() =>
		$batchQueue
			.map((item, index) => ({ item, index }))
			.sort((left, right) => {
				const priorityDiff =
					getDisplayPriority(left.item) - getDisplayPriority(right.item);
				if (priorityDiff !== 0) return priorityDiff;
				return left.index - right.index;
			})
			.map(({ item }) => item)
	);

</script>

<div class="batch-progress" class:is-idle={!$isBatchActive && $batchQueue.length === 0}>
	<div class="header">
		<h4>{m.playlist_progress_title()}</h4>
		<span class="count">{progressDone} / {progressTotal}</span>
	</div>

	<div class="progress-bar">
		<div
			class="progress-fill"
			style:width={`${progressPercent}%`}
			class:complete={progressDone >= progressTotal && progressTotal > 0}
		></div>
	</div>

	{#if $batchQueue.length > 0}
		<div class="pool-indicator">
			<span class="pool-text">{formatSummary()}</span>
		</div>

		<div class="summary-row">
			<span class="summary-pill neutral">{m.playlist_progress_selected({ count: String(selectedCount) })}</span>
			<span class="summary-pill success">{m.playlist_progress_done({ count: String(completedCount) })}</span>
			<span class="summary-pill error">{m.playlist_progress_errors({ count: String(errorCount) })}</span>
		</div>

		{#if canEditSelection}
			<div class="selection-actions">
				<button type="button" class="selection-btn" onclick={selectAllPending}>{m.playlist_progress_select_all()}</button>
				<button type="button" class="selection-btn" onclick={clearAllPending}>{m.playlist_progress_clear_all()}</button>
			</div>
		{/if}

		<div class="queue-list" role="list">
			{#each orderedBatchQueue as item}
				<div class="queue-item" role="listitem">
					{#if item.status === 'pending' && !item.progressLabel}
						<button
							type="button"
							class="item-toggle"
							class:is-selected={item.selected !== false}
							disabled={!canEditSelection}
							onclick={() => toggleItemSelection(item.videoId, item.selected === false)}
							aria-label={item.selected === false ? m.playlist_progress_select_video() : m.playlist_progress_deselect_video()}
							aria-pressed={item.selected !== false}
							title={item.selected === false ? m.playlist_progress_select_video() : m.playlist_progress_deselect_video()}
						>
							<AppIcon
								name={item.selected !== false ? 'check' : 'add'}
								class="toggle-icon"
							/>
						</button>
					{:else}
						<span class="item-toggle-spacer"></span>
					{/if}
					{#if item.thumbnail}
						<img class="item-thumb" src={item.thumbnail} alt={item.title}/>
					{:else}
						<span class="item-thumb item-thumb-fallback">
							<AppIcon name="movie" class="text-[18px]" />
						</span>
					{/if}
					<div class="item-copy">
						<span class="item-title" title={item.title}>{truncate(item.title)}</span>
						{#if item.error}
							<span class="item-subtitle" title={item.error}>{truncate(item.error, 80)}</span>
						{:else if item.progressLabel}
							<div class="item-progress-stack">
								<div class="item-progress-meta">
									<span class="item-progress-label">{item.progressLabel}</span>
									{#if !item.progressIndeterminate && typeof item.progressPercent === 'number'}
										<span class="item-progress-percent">{formatProgressPercent(item.progressPercent)}</span>
									{/if}
								</div>
								<div
									class:item-progress-track-indeterminate={item.progressIndeterminate}
									class="item-progress-track"
								>
									<div
										class:item-progress-fill-indeterminate={item.progressIndeterminate}
										class="item-progress-fill"
										style:width={item.progressIndeterminate ? undefined : `${Math.max(0, Math.min(100, item.progressPercent ?? 0))}%`}
									></div>
								</div>
							</div>
						{/if}
					</div>
					<span class={`item-status ${getItemStatusClass(item)}`}>{getItemStatusLabel(item)}</span>
				</div>
			{/each}
		</div>
	{:else}
		<div class="idle-note">
			{m.playlist_progress_idle_note()}
		</div>
	{/if}
</div>

<style>
	.batch-progress {
		padding: 0;
		background: transparent;
		border: 0;
		box-shadow: none;
		position: relative;
		min-height: 240px;
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.85rem;
	}

	h4 {
		margin: 0;
		font-size: 1rem;
		font-weight: 800;
		color: #0f172a;
		letter-spacing: 0.01em;
	}

	.count {
		font-size: 0.8rem;
		font-weight: 800;
		color: #64748b;
		padding: 0.35rem 0.7rem;
		border-radius: 999px;
		background: #f8fafc;
		border: 1px solid #e2e8f0;
	}

	.progress-bar {
		height: 0.55rem;
		background: #e2e8f0;
		border-radius: 999px;
		overflow: hidden;
		margin-bottom: 1rem;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #ff4d8c 0%, #ffb938 100%);
		border-radius: 999px;
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
		background: #fff7fb;
		border-radius: 1rem;
		border: 1px solid #fbcfe8;
	}

	.slots {
		display: flex;
		gap: 0.375rem;
	}

	.slot {
		width: 12px;
		height: 12px;
		border-radius: 50%;
		background: #fbcfe8;
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
		font-size: 0.8rem;
		color: #6b213f;
		font-weight: 700;
	}

	.idle-note {
		margin-top: 0.75rem;
		padding: 0.85rem 0.9rem;
		border-radius: 1rem;
		border: 1px solid #e2e8f0;
		background: #f8fafc;
		color: #64748b;
		font-size: 0.78rem;
		font-weight: 700;
		line-height: 1.45;
	}

	.cooldown-note {
		margin-bottom: 0.75rem;
		font-size: 0.75rem;
		font-weight: 600;
		color: #b45309;
	}

	.summary-row {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.summary-pill {
		font-size: 0.75rem;
		font-weight: 700;
		padding: 0.35rem 0.65rem;
		border-radius: 999px;
		border: 1px solid transparent;
	}

	.summary-pill.neutral {
		background: #f8fafc;
		border-color: #e2e8f0;
		color: #475569;
	}

	.selection-actions {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.selection-btn {
		font-size: 0.75rem;
		font-weight: 700;
		padding: 0.45rem 0.8rem;
		border-radius: 999px;
		border: 1px solid #fbcfe8;
		background: #fff;
		color: #be185d;
		cursor: pointer;
		transition: filter 0.2s ease, transform 0.2s ease;
	}

	.selection-btn:hover {
		filter: brightness(1.08);
		transform: translateY(-1px);
	}

	.summary-pill.success {
		background: #f0fdf4;
		border-color: #bbf7d0;
		color: #15803d;
	}

	.summary-pill.error {
		background: #fef2f2;
		border-color: #fecaca;
		color: #b91c1c;
	}

	.queue-list {
		display: flex;
		flex-direction: column;
		gap: 0.55rem;
		max-height: 360px;
		overflow-y: auto;
		padding-right: 0.35rem;
	}

	.queue-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.7rem 0.8rem;
		background: #fff;
		border: 1px solid #e2e8f0;
		border-radius: 1rem;
		font-size: 0.8rem;
	}

	.item-toggle {
		width: 1.2rem;
		height: 1.2rem;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		border-radius: 0.35rem;
		border: 1px solid #cbd5e1;
		background: #fff;
		color: #64748b;
		padding: 0;
		cursor: pointer;
		transition: transform 0.2s ease, filter 0.2s ease, background 0.2s ease, border-color 0.2s ease;
	}

	.item-toggle :global(.toggle-icon) {
		font-size: 0.74rem;
		font-weight: 700;
		line-height: 1;
	}

	.item-toggle.is-selected {
		background: linear-gradient(135deg, #ff4d8c 0%, #ffb938 100%);
		border-color: rgba(255, 77, 140, 0.4);
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
		width: 1.2rem;
		height: 1.2rem;
		flex: 0 0 auto;
	}

	.item-thumb {
		width: 3.4rem;
		height: 2rem;
		flex: 0 0 auto;
		border-radius: 0.7rem;
		object-fit: cover;
		border: 1px solid #e2e8f0;
		background: #f8fafc;
	}

	.item-thumb-fallback {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		color: #94a3b8;
	}

	.item-copy {
		min-width: 0;
		display: flex;
		flex: 1;
		flex-direction: column;
		gap: 0.18rem;
	}

	.item-title {
		color: #0f172a;
		font-weight: 700;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.item-subtitle {
		color: #64748b;
		font-size: 0.72rem;
		font-weight: 600;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.item-progress-stack {
		display: flex;
		flex-direction: column;
		gap: 0.35rem;
		margin-top: 0.18rem;
	}

	.item-progress-meta {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.75rem;
		min-width: 0;
	}

	.item-progress-label {
		min-width: 0;
		color: #475569;
		font-size: 0.72rem;
		font-weight: 700;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.item-progress-percent {
		color: #0f172a;
		font-size: 0.7rem;
		font-weight: 800;
		flex: 0 0 auto;
	}

	.item-progress-track {
		height: 0.36rem;
		border-radius: 999px;
		background: #e2e8f0;
		overflow: hidden;
		position: relative;
	}

	.item-progress-track.item-progress-track-indeterminate {
		background: #e9d5ff;
	}

	.item-progress-fill {
		height: 100%;
		border-radius: 999px;
		background: linear-gradient(90deg, #4f46e5 0%, #8b5cf6 100%);
		transition: width 0.2s ease;
	}

	.item-progress-fill.item-progress-fill-indeterminate {
		width: 34%;
		animation: item-progress-sweep 1.1s ease-in-out infinite;
	}

	@keyframes item-progress-sweep {
		0% {
			transform: translateX(-115%);
		}
		100% {
			transform: translateX(300%);
		}
	}

	.item-status {
		min-width: 5.9rem;
		text-align: center;
		font-size: 0.68rem;
		font-weight: 800;
		letter-spacing: 0.01em;
		padding: 0.35rem 0.55rem;
		border-radius: 999px;
		border: 1px solid transparent;
	}

	.item-status.status-pending {
		background: #f8fafc;
		border-color: #e2e8f0;
		color: #475569;
	}

	.item-status.status-in-progress {
		background: #fff7ed;
		border-color: #fdba74;
		color: #c2410c;
	}

	.item-status.status-success {
		background: #f0fdf4;
		border-color: #bbf7d0;
		color: #15803d;
	}

	.item-status.status-fail {
		background: #fef2f2;
		border-color: #fecaca;
		color: #b91c1c;
	}

	.item-status.status-skipped {
		background: #f8fafc;
		border-color: #e2e8f0;
		color: #94a3b8;
	}

	.queue-list::-webkit-scrollbar {
		width: 6px;
	}

	.queue-list::-webkit-scrollbar-thumb {
		background: rgba(148, 163, 184, 0.45);
		border-radius: 999px;
	}

	:global(.page-root.theme-dark) .batch-progress {
		background: transparent;
	}

	:global(.page-root.theme-dark) h4 {
		color: #f3e8ff;
	}

	:global(.page-root.theme-dark) .count,
	:global(.page-root.theme-dark) .summary-pill.neutral,
	:global(.page-root.theme-dark) .item-status.status-pending,
	:global(.page-root.theme-dark) .item-status.status-skipped,
	:global(.page-root.theme-dark) .idle-note {
		background: rgba(255, 255, 255, 0.04);
		border-color: rgba(255, 77, 140, 0.16);
		color: rgba(224, 208, 245, 0.82);
	}

	:global(.page-root.theme-dark) .pool-text {
		color: rgba(224, 208, 245, 0.82);
	}

	:global(.page-root.theme-dark) .pool-indicator {
		background: rgba(255, 77, 140, 0.08);
		border-color: rgba(255, 77, 140, 0.18);
	}

	:global(.page-root.theme-dark) .queue-item {
		background: rgba(255, 255, 255, 0.03);
		border-color: rgba(255, 77, 140, 0.16);
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

	:global(.page-root.theme-dark) .item-subtitle {
		color: rgba(224, 208, 245, 0.62);
	}

	:global(.page-root.theme-dark) .item-progress-label {
		color: rgba(224, 208, 245, 0.82);
	}

	:global(.page-root.theme-dark) .item-progress-percent {
		color: #ffffff;
	}

	:global(.page-root.theme-dark) .item-progress-track {
		background: rgba(255, 255, 255, 0.1);
	}

	:global(.page-root.theme-dark) .item-progress-track.item-progress-track-indeterminate {
		background: rgba(139, 92, 246, 0.18);
	}

	:global(.page-root.theme-dark) .item-thumb {
		border-color: rgba(255, 77, 140, 0.12);
		background: rgba(255, 255, 255, 0.04);
	}

	@media (max-width: 640px) {
		.queue-item {
			display: grid;
			grid-template-columns: auto auto minmax(0, 1fr);
			grid-template-areas:
				'toggle thumb status'
				'toggle copy copy';
			align-items: center;
		}

		.item-toggle,
		.item-toggle-spacer {
			grid-area: toggle;
		}

		.item-thumb {
			grid-area: thumb;
		}

		.item-copy {
			grid-area: copy;
		}

		.item-status {
			grid-area: status;
			min-width: 0;
			justify-self: end;
		}
	}
</style>
