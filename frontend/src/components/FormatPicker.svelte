<script lang="ts">
	import type { Stream } from '$lib/types';

	interface Props {
		streams: Stream[];
		onSelect: (videoStream: Stream, audioStream: Stream | null) => void;
	}

	let { streams, onSelect }: Props = $props();

	// --- Data preparation ---

	/** Unique resolutions from video streams (exclude audio-only and WebM-only), sorted low→high.
	 * Resolutions where all video streams are WebM are excluded — WebM uses EBML container
	 * and cannot be muxed into fMP4. */
	const resolutions = $derived.by(() => {
		const seen = new Set<string>();
		const list: string[] = [];

		for (const s of streams) {
			if (s.isAudioOnly) continue;
			// Exclude VP9/WebM video-only streams — not muxable into fMP4
			if (!s.hasAudio && s.format === 'webm') continue;
			const res = s.quality.replace(' (video only)', '');
			if (!seen.has(res)) {
				seen.add(res);
				list.push(res);
			}
		}

		return list.sort((a, b) => (parseInt(a) || 0) - (parseInt(b) || 0));
	});

	/** Codec/format variants for the currently selected resolution.
	 * Excludes WebM video-only streams — EBML container, not ISO BMFF, cannot be muxed.
	 * Streams with built-in audio (progressive) are allowed regardless of container. */
	const codecOptions = $derived.by(() => {
		if (!selectedResolution) return [];
		return streams.filter(
			(s) =>
				!s.isAudioOnly &&
				s.quality.replace(' (video only)', '') === selectedResolution &&
				(s.hasAudio || s.format !== 'webm')
		);
	});

	/** Audio-only streams sorted by bitrate desc */
	const audioOnlyStreams = $derived.by(() =>
		streams.filter((s) => s.isAudioOnly).sort((a, b) => (b.bitrate ?? 0) - (a.bitrate ?? 0))
	);

	/** Best audio-only stream for muxing with MP4 video.
	 * Prefer M4A/MP4 (fMP4/AAC) over WebM (Opus) — WebM uses EBML container,
	 * not ISO BMFF, so it cannot be remuxed into fMP4. */
	const bestAudioStream = $derived.by(() => {
		// Prefer non-WebM (M4A/AAC) for fMP4 compatibility
		const fmp4Audio = audioOnlyStreams.filter((s) => s.format !== 'webm');
		return (fmp4Audio.length > 0 ? fmp4Audio : audioOnlyStreams)[0] ?? null;
	});

	// --- Default selection logic ---

	/** Pick default resolution: highest available */
	function getDefaultResolution(resList: string[]): string | null {
		if (!resList.length) return null;
		return [...resList].sort((a, b) => (parseInt(b) || 0) - (parseInt(a) || 0))[0];
	}

	/** Pick default codec: H.264 MP4 → AV1 MP4 → any MP4 → first available.
	 * VP9/WebM intentionally excluded — cannot be muxed into fMP4. */
	function getDefaultCodec(options: Stream[]): Stream | null {
		if (!options.length) return null;
		const priority: ((s: Stream) => boolean)[] = [
			(s) => s.codecLabel === 'H.264' && s.format === 'mp4',
			(s) => s.codecLabel === 'AV1' && s.format === 'mp4',
			(s) => s.format === 'mp4',
			() => true
		];
		for (const pred of priority) {
			const match = options.find(pred);
			if (match) return match;
		}
		return options[0];
	}

	/** Format audio stream label: "AAC 128kbps" */
	function formatAudioLabel(audio: Stream): string {
		const codec = audio.codecLabel || 'Audio';
		const br = audio.bitrate ? `${Math.round(audio.bitrate / 1000)}kbps` : '';
		return [codec, br].filter(Boolean).join(' ');
	}

	// --- State ---

	let selectedResolution = $state<string | null>(null);
	let selectedStream = $state<Stream | null>(null);
	/** Whether user selected an audio-only stream */
	let audioMode = $state(false);

	// Initialize default resolution once streams load
	$effect(() => {
		if (!selectedResolution && resolutions.length) {
			selectedResolution = getDefaultResolution(resolutions);
		}
	});

	// Auto-select default codec only when current selection is no longer valid
	// (e.g., resolution changed). Do NOT override user's manual codec selection.
	$effect(() => {
		if (!audioMode && codecOptions.length) {
			const currentIsValid =
				selectedStream !== null && codecOptions.some((o) => o.url === selectedStream!.url);
			if (!currentIsValid) {
				const newDefault = getDefaultCodec(codecOptions);
				if (newDefault) {
					selectedStream = newDefault;
					onSelect(newDefault, newDefault.hasAudio ? null : (bestAudioStream ?? null));
				}
			}
		}
	});

	function selectResolution(res: string) {
		selectedResolution = res;
		audioMode = false;
	}

	function selectCodec(stream: Stream) {
		selectedStream = stream;
		audioMode = false;
		onSelect(stream, stream.hasAudio ? null : (bestAudioStream ?? null));
	}

	function selectAudio(audio: Stream) {
		selectedStream = audio;
		audioMode = true;
		onSelect(audio, null);
	}

	/** Quality badge color */
	function getQualityColor(res: string): string {
		const n = parseInt(res) || 0;
		if (n >= 2160) return '#ef4444';
		if (n >= 1080) return '#f97316';
		if (n >= 720) return '#eab308';
		return '#6b7280';
	}
</script>

<div class="format-picker">
	<h4>Select Quality</h4>

	<!-- Row 1: Resolution buttons -->
	<div class="row-label">Resolution</div>
	<div class="stream-list" role="radiogroup" aria-label="Video resolution">
		{#each resolutions as res}
			<button
				class="res-option"
				class:selected={!audioMode && selectedResolution === res}
				onclick={() => selectResolution(res)}
				role="radio"
				aria-checked={!audioMode && selectedResolution === res}
			>
				<span class="quality-badge" style:background-color={getQualityColor(res)}>
					{res}
				</span>
			</button>
		{/each}
	</div>

	<!-- Row 2: Codec/format options for selected resolution -->
	{#if codecOptions.length > 0}
		<div class="row-label">Format</div>
		<div class="stream-list" role="radiogroup" aria-label="Video format">
			{#each codecOptions as stream}
				<button
					class="codec-option"
					class:selected={!audioMode && selectedStream?.url === stream.url}
					onclick={() => selectCodec(stream)}
					role="radio"
					aria-checked={!audioMode && selectedStream?.url === stream.url}
				>
					{#if stream.codecLabel}
						<span class="codec-label">{stream.codecLabel}</span>
					{/if}
					<span class="format-ext">{stream.format.toUpperCase()}</span>
					{#if !stream.hasAudio}
						<span class="mux-badge" title="Audio will be added automatically">+🔊</span>
					{/if}
				</button>
			{/each}
		</div>
	{/if}

	<!-- Audio-only section -->
	{#if audioOnlyStreams.length > 0}
		<div class="row-label">Audio Only</div>
		<div class="stream-list" role="radiogroup" aria-label="Audio only">
			{#each audioOnlyStreams as audio}
				<button
					class="codec-option"
					class:selected={audioMode && selectedStream?.url === audio.url}
					onclick={() => selectAudio(audio)}
					role="radio"
					aria-checked={audioMode && selectedStream?.url === audio.url}
				>
					<span class="codec-label">{formatAudioLabel(audio)}</span>
					<span class="format-ext">{audio.format.toUpperCase()}</span>
				</button>
			{/each}
		</div>
	{/if}

</div>

<style>
	.format-picker {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	h4 {
		margin: 0;
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-color, #111827);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.row-label {
		font-size: 0.7rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.08em;
		color: var(--text-secondary, #6b7280);
		margin-bottom: -0.25rem;
	}

	.stream-list {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.res-option,
	.codec-option {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.4rem 0.65rem;
		background: var(--input-bg, #ffffff);
		border: 2px solid var(--border-color, #e5e7eb);
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.15s;
		font-size: 0.8rem;
	}

	.res-option:hover,
	.codec-option:hover {
		border-color: var(--primary-color, #3b82f6);
	}

	.res-option.selected,
	.codec-option.selected {
		border-color: var(--primary-color, #3b82f6);
		background: var(--primary-alpha, rgba(59, 130, 246, 0.1));
	}

	.quality-badge {
		padding: 0.1rem 0.35rem;
		font-size: 0.75rem;
		font-weight: 700;
		color: white;
		border-radius: 0.25rem;
	}

	.codec-label {
		font-weight: 600;
		color: var(--text-color, #111827);
	}

	.format-ext {
		font-size: 0.7rem;
		color: var(--text-secondary, #6b7280);
	}

	.mux-badge {
		font-size: 0.65rem;
		opacity: 0.7;
	}

	@media (prefers-color-scheme: dark) {
		.res-option,
		.codec-option {
			--input-bg: #1f2937;
			--border-color: #374151;
		}
	}
</style>
