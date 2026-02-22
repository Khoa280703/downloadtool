<script lang="ts">
	import type { Stream, Platform } from '$lib/types';

	interface Props {
		streams: Stream[];
		platform: Platform;
		selectedStream: Stream | null;
		onSelect: (stream: Stream) => void;
	}

	let { streams, platform, selectedStream, onSelect }: Props = $props();

	let removeWatermark = $state(false);
	let addBranding = $state(false);

	/** Get quality badge color based on quality */
	function getQualityColor(quality: string): string {
		if (quality.includes('4K') || quality.includes('2160')) return '#ef4444';
		if (quality.includes('1080')) return '#f97316';
		if (quality.includes('720')) return '#eab308';
		if (quality.includes('MP3') || quality.includes('audio')) return '#8b5cf6';
		return '#6b7280';
	}

	/** Sort streams by quality (highest first) */
	function sortStreams(list: Stream[]): Stream[] {
		const qualityOrder = ['4K', '2160', '1440', '1080', '720', '480', '360', '240', '144'];
		return [...list].sort((a, b) => {
			const aIndex = qualityOrder.findIndex((q) => a.quality.includes(q));
			const bIndex = qualityOrder.findIndex((q) => b.quality.includes(q));
			if (aIndex !== -1 && bIndex !== -1) return aIndex - bIndex;
			if (a.quality.includes('MP3')) return 1;
			if (b.quality.includes('MP3')) return -1;
			return 0;
		});
	}

	const sortedStreams = $derived(sortStreams(streams));
</script>

<div class="format-picker">
	<h4>Select Quality</h4>

	<div class="stream-list" role="radiogroup" aria-label="Video quality options">
		{#each sortedStreams as stream}
			<button
				class="stream-option"
				class:selected={selectedStream?.url === stream.url}
				onclick={() => onSelect(stream)}
				role="radio"
				aria-checked={selectedStream?.url === stream.url}
			>
				<span
					class="quality-badge"
					style:background-color={getQualityColor(stream.quality)}
				>
					{stream.quality}
				</span>
				<span class="format">{stream.format.toUpperCase()}</span>
				{#if stream.hasAudio}
					<span class="audio-badge"><svg viewBox="0 0 24 24" width="12" height="12" fill="currentColor">
						<path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/>
					</svg></span>
				{/if}
			</button>
		{/each}
	</div>

	{#if platform === 'tiktok'}
		<label class="toggle-option">
			<input
				type="checkbox"
				bind:checked={removeWatermark}
			/>
			<span class="toggle-slider"></span>
			<span class="toggle-label">Remove watermark</span>
		</label>
	{/if}

	<label class="toggle-option">
		<input
			type="checkbox"
			bind:checked={addBranding}
			disabled
		/>
		<span class="toggle-slider"></span>
		<span class="toggle-label">Add branding (GPU)</span>
		<span class="coming-soon">Soon</span>
	</label>
</div>

<style>
	.format-picker {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	h4 {
		margin: 0;
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-color, #111827);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.stream-list {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.stream-option {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: var(--input-bg, #ffffff);
		border: 2px solid var(--border-color, #e5e7eb);
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.stream-option:hover {
		border-color: var(--primary-color, #3b82f6);
	}

	.stream-option.selected {
		border-color: var(--primary-color, #3b82f6);
		background: var(--primary-alpha, rgba(59, 130, 246, 0.1));
	}

	.quality-badge {
		padding: 0.125rem 0.375rem;
		font-size: 0.75rem;
		font-weight: 600;
		color: white;
		border-radius: 0.25rem;
	}

	.format {
		font-size: 0.75rem;
		color: var(--text-secondary, #6b7280);
	}

	.audio-badge {
		display: flex;
		align-items: center;
		color: var(--success-color, #22c55e);
	}

	.toggle-option {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
		padding: 0.5rem 0;
	}

	.toggle-option input {
		display: none;
	}

	.toggle-slider {
		width: 44px;
		height: 24px;
		background: var(--border-color, #e5e7eb);
		border-radius: 12px;
		position: relative;
		transition: background 0.2s;
		flex-shrink: 0;
	}

	.toggle-slider::after {
		content: '';
		position: absolute;
		width: 20px;
		height: 20px;
		background: white;
		border-radius: 50%;
		top: 2px;
		left: 2px;
		transition: transform 0.2s;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
	}

	.toggle-option input:checked + .toggle-slider {
		background: var(--primary-color, #3b82f6);
	}

	.toggle-option input:checked + .toggle-slider::after {
		transform: translateX(20px);
	}

	.toggle-option input:disabled + .toggle-slider {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.toggle-label {
		font-size: 0.875rem;
		color: var(--text-color, #111827);
	}

	.coming-soon {
		font-size: 0.625rem;
		padding: 0.125rem 0.375rem;
		background: var(--border-color, #e5e7eb);
		color: var(--text-secondary, #6b7280);
		border-radius: 0.25rem;
		margin-left: auto;
	}

	@media (prefers-color-scheme: dark) {
		.stream-option {
			--input-bg: #1f2937;
			--border-color: #374151;
		}

		.toggle-slider {
			--border-color: #4b5563;
		}
	}
</style>
