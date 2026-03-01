<script lang="ts">
	import type { Stream } from '$lib/types';

	interface Props {
		streams: Stream[];
		onSelect: (videoStream: Stream, audioStream: Stream | null) => void;
	}

	type TabMode = 'video' | 'audio';
	type PreferredFormat = {
		mode: TabMode;
		qualityValue?: number;
		format?: string;
		codecLabel?: string;
		bitrate?: number;
	};

	const PREFERRED_FORMAT_KEY = 'fetchtube.preferred-format.v1';
	const IS_BROWSER = typeof window !== 'undefined';

	let { streams, onSelect }: Props = $props();
	let activeTab = $state<TabMode>('video');
	let selectedStream = $state<Stream | null>(null);
	let lastSelectionKey = $state('');
	let preferredFormat = $state<PreferredFormat | null>(null);
	let hasHydratedPreference = $state(false);

	function qualityScore(label: string): number {
		const clean = label.replace(' (video only)', '').toLowerCase();
		if (clean.includes('8k')) return 4320;
		if (clean.includes('4k')) return 2160;
		if (clean.includes('2k')) return 1440;
		const num = parseInt(clean, 10);
		return Number.isNaN(num) ? 0 : num;
	}

	function codecPriority(stream: Stream): number {
		if (stream.codecLabel === 'H.264' && stream.format === 'mp4') return 0;
		if (stream.codecLabel === 'AV1' && stream.format === 'mp4') return 1;
		if (stream.format === 'mp4') return 2;
		return 3;
	}

	const videoOptions = $derived.by(() =>
		streams
			.filter((s) => !s.isAudioOnly && (s.hasAudio || s.format !== 'webm'))
			.sort((a, b) => {
				const q = qualityScore(b.quality) - qualityScore(a.quality);
				if (q !== 0) return q;
				const c = codecPriority(a) - codecPriority(b);
				if (c !== 0) return c;
				return (b.bitrate ?? 0) - (a.bitrate ?? 0);
			})
	);

	const audioOptions = $derived.by(() =>
		streams
			.filter((s) => s.isAudioOnly)
			.sort((a, b) => (b.bitrate ?? 0) - (a.bitrate ?? 0))
	);

	const bestAudioForMux = $derived.by(() => {
		const mp4Audio = audioOptions.filter((s) => s.format !== 'webm');
		return (mp4Audio.length > 0 ? mp4Audio : audioOptions)[0] ?? null;
	});

	function savePreferredFormat(mode: TabMode, stream: Stream): void {
		const nextPreference: PreferredFormat = {
			mode,
			qualityValue: mode === 'video' ? qualityScore(stream.quality) : undefined,
			format: stream.format,
			codecLabel: stream.codecLabel,
			bitrate: stream.bitrate
		};
		preferredFormat = nextPreference;
		if (!IS_BROWSER) return;
		try {
			window.localStorage.setItem(PREFERRED_FORMAT_KEY, JSON.stringify(nextPreference));
		} catch {
			// Ignore localStorage errors (private mode, quotas, etc.)
		}
	}

	function loadPreferredFormat(): PreferredFormat | null {
		if (!IS_BROWSER) return null;
		try {
			const raw = window.localStorage.getItem(PREFERRED_FORMAT_KEY);
			if (!raw) return null;
			const parsed = JSON.parse(raw) as PreferredFormat;
			if (parsed.mode !== 'video' && parsed.mode !== 'audio') return null;
			return parsed;
		} catch {
			return null;
		}
	}

	function pickPreferredVideo(): Stream | null {
		if (videoOptions.length === 0) return null;
		if (!preferredFormat || preferredFormat.mode !== 'video') return videoOptions[0] ?? null;

		const targetQuality = preferredFormat.qualityValue ?? 0;
		const targetFormat = preferredFormat.format;
		const targetCodec = preferredFormat.codecLabel;

		const exactMatch = videoOptions.find(
			(stream) =>
				qualityScore(stream.quality) === targetQuality &&
				stream.format === targetFormat &&
				(targetCodec ? stream.codecLabel === targetCodec : true)
		);
		if (exactMatch) return exactMatch;

		const qualityAndFormat = videoOptions.find(
			(stream) =>
				qualityScore(stream.quality) === targetQuality &&
				(targetFormat ? stream.format === targetFormat : true)
		);
		if (qualityAndFormat) return qualityAndFormat;

		const closestByQuality = [...videoOptions].sort(
			(a, b) =>
				Math.abs(qualityScore(a.quality) - targetQuality) -
				Math.abs(qualityScore(b.quality) - targetQuality)
		);
		if (closestByQuality[0]) return closestByQuality[0];

		return videoOptions[0] ?? null;
	}

	function pickPreferredAudio(): Stream | null {
		if (audioOptions.length === 0) return null;
		if (!preferredFormat || preferredFormat.mode !== 'audio') return audioOptions[0] ?? null;

		const targetFormat = preferredFormat.format;
		const targetBitrate = preferredFormat.bitrate ?? 0;
		const sameFormat = targetFormat
			? audioOptions.filter((stream) => stream.format === targetFormat)
			: audioOptions;

		if (sameFormat.length > 0) {
			const sortedByBitrate = [...sameFormat].sort(
				(a, b) => Math.abs((a.bitrate ?? 0) - targetBitrate) - Math.abs((b.bitrate ?? 0) - targetBitrate)
			);
			return sortedByBitrate[0] ?? sameFormat[0] ?? null;
		}

		return audioOptions[0] ?? null;
	}

	function emitSelection(videoStream: Stream, audioStream: Stream | null): void {
		const key = `${videoStream.url}::${audioStream?.url ?? ''}`;
		if (key === lastSelectionKey) return;
		lastSelectionKey = key;
		onSelect(videoStream, audioStream);
	}

	$effect(() => {
		if (hasHydratedPreference || !IS_BROWSER) return;
		preferredFormat = loadPreferredFormat();
		hasHydratedPreference = true;
	});

	$effect(() => {
		if (IS_BROWSER && !hasHydratedPreference) return;

		if (selectedStream && !streams.some((s) => s.url === selectedStream!.url)) {
			selectedStream = null;
			lastSelectionKey = '';
		}

		if (!selectedStream) {
			const preferredAudio = pickPreferredAudio();
			const preferredVideo = pickPreferredVideo();

			if (preferredFormat?.mode === 'audio' && preferredAudio) {
				selectedStream = preferredAudio;
				activeTab = 'audio';
				emitSelection(preferredAudio, null);
				return;
			}

			if (preferredVideo) {
				selectedStream = preferredVideo;
				activeTab = 'video';
				emitSelection(preferredVideo, preferredVideo.hasAudio ? null : bestAudioForMux);
				return;
			}

			if (preferredAudio) {
				selectedStream = preferredAudio;
				activeTab = 'audio';
				emitSelection(preferredAudio, null);
			}
		}
	});

	function selectVideo(stream: Stream): void {
		selectedStream = stream;
		activeTab = 'video';
		savePreferredFormat('video', stream);
		emitSelection(stream, stream.hasAudio ? null : bestAudioForMux);
	}

	function selectAudio(stream: Stream): void {
		selectedStream = stream;
		activeTab = 'audio';
		savePreferredFormat('audio', stream);
		emitSelection(stream, null);
	}

	function isSelected(stream: Stream): boolean {
		return selectedStream?.url === stream.url;
	}

	function cleanQuality(quality: string): string {
		return quality.replace(' (video only)', '');
	}

	function formatSize(bytes?: number): string {
		if (!bytes) return 'Unknown size';
		const units = ['B', 'KB', 'MB', 'GB'];
		let value = bytes;
		let unit = 0;
		while (value >= 1024 && unit < units.length - 1) {
			value /= 1024;
			unit += 1;
		}
		return `${value.toFixed(value >= 100 || unit === 0 ? 0 : 1)} ${units[unit]}`;
	}

	function formatQualityMeta(stream: Stream): string {
		if (stream.bitrate) return `${Math.round(stream.bitrate / 1000)}kbps`;
		return stream.isAudioOnly ? 'Audio only' : 'Video';
	}

	function iconFor(stream: Stream): string {
		if (stream.isAudioOnly) return 'headphones';
		const quality = qualityScore(stream.quality);
		if (quality >= 2160) return '4k';
		if (quality >= 1440) return '2k';
		if (quality >= 1080) return 'full_hd';
		if (quality >= 720) return 'hd';
		if (quality >= 480) return 'sd';
		return 'smartphone';
	}

	function accentClass(stream: Stream): string {
		const score = qualityScore(stream.quality);
		if (score >= 2160) return 'accent-indigo';
		if (score >= 1080) return 'accent-pink';
		if (score >= 720) return 'accent-cyan';
		return 'accent-slate';
	}
</script>

<div class="picker-shell">
	<div class="tabs" role="tablist" aria-label="Download type">
		<button
			type="button"
			class={`tab-btn ${activeTab === 'video' ? 'tab-btn-active' : ''}`}
			role="tab"
			aria-selected={activeTab === 'video'}
			onclick={() => (activeTab = 'video')}
		>
			<span class="material-symbols-outlined tab-icon">movie</span>
			<span>Video</span>
		</button>
		<button
			type="button"
			class={`tab-btn ${activeTab === 'audio' ? 'tab-btn-active' : ''}`}
			role="tab"
			aria-selected={activeTab === 'audio'}
			onclick={() => (activeTab = 'audio')}
		>
			<span class="material-symbols-outlined tab-icon">headphones</span>
			<span>Audio</span>
		</button>
	</div>

	<div class="list-scroll" role="radiogroup" aria-label={activeTab === 'audio' ? 'Audio options' : 'Video options'}>
		{#if activeTab === 'video'}
			{#if videoOptions.length === 0}
				<div class="empty-state">
					<span class="material-symbols-outlined">movie</span>
					<p>No compatible video formats found.</p>
				</div>
			{:else}
				{#each videoOptions as stream, idx}
					<label class={`option-card ${isSelected(stream) ? 'option-card-selected' : ''}`} for={`stream-video-${idx}`}>
						<input
							id={`stream-video-${idx}`}
							class="option-radio"
							type="radio"
							name="download-quality"
							checked={isSelected(stream)}
							onchange={() => selectVideo(stream)}
						/>
						{#if idx === 0}
							<span class="badge-best">Best Quality</span>
						{/if}
						<div class="option-left">
							<div class={`option-icon ${accentClass(stream)} ${isSelected(stream) ? 'option-icon-selected' : ''}`}>
								<span class="material-symbols-outlined">{iconFor(stream)}</span>
							</div>
							<div class="option-meta">
								<p class="option-title">{cleanQuality(stream.quality)}</p>
								<div class="option-submeta">
									<span class="fmt-chip">{stream.format.toUpperCase()}</span>
									{#if stream.codecLabel}<span>{stream.codecLabel}</span>{/if}
									{#if !stream.hasAudio}<span>+ Auto audio</span>{/if}
								</div>
							</div>
						</div>
						<div class="option-right">
							<span class="size-pill">{formatSize(stream.size)}</span>
							<span class={`check-pill ${isSelected(stream) ? 'check-pill-active' : ''}`}>✓</span>
						</div>
					</label>
				{/each}
			{/if}
		{:else}
			{#if audioOptions.length === 0}
				<div class="empty-state">
					<span class="material-symbols-outlined">headphones</span>
					<p>No audio-only stream available.</p>
				</div>
			{:else}
				{#each audioOptions as stream, idx}
					<label class={`option-card ${isSelected(stream) ? 'option-card-selected' : ''}`} for={`stream-audio-${idx}`}>
						<input
							id={`stream-audio-${idx}`}
							class="option-radio"
							type="radio"
							name="download-quality"
							checked={isSelected(stream)}
							onchange={() => selectAudio(stream)}
						/>
						<div class="option-left">
							<div class={`option-icon accent-purple ${isSelected(stream) ? 'option-icon-selected' : ''}`}>
								<span class="material-symbols-outlined">{iconFor(stream)}</span>
							</div>
							<div class="option-meta">
								<p class="option-title">Audio</p>
								<div class="option-submeta">
									<span class="fmt-chip">{stream.format.toUpperCase()}</span>
									<span>{formatQualityMeta(stream)}</span>
								</div>
							</div>
						</div>
						<div class="option-right">
							<span class="size-pill">{formatSize(stream.size)}</span>
							<span class={`check-pill ${isSelected(stream) ? 'check-pill-active' : ''}`}>✓</span>
						</div>
					</label>
				{/each}
			{/if}
		{/if}
	</div>
</div>

<style>
	.picker-shell {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.tabs {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 0.5rem;
		padding: 0.35rem;
		background: #f3f4f6;
		border: 1px solid #e5e7eb;
		border-radius: 1rem;
	}

	.tab-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.45rem;
		padding: 0.7rem 0.9rem;
		border: 0;
		border-radius: 0.8rem;
		background: transparent;
		color: #64748b;
		font-size: 0.88rem;
		font-weight: 700;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.tab-btn:hover {
		background: #f8fafc;
		color: #4f46e5;
	}

	.tab-btn-active {
		background: #4f46e5;
		color: #fff;
		box-shadow: 0 8px 20px -10px rgba(79, 70, 229, 0.55);
	}

	.tab-icon {
		font-size: 18px;
	}

	.list-scroll {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		max-height: 480px;
		overflow-y: auto;
		padding-top: 0.7rem;
		padding-right: 0.35rem;
		scrollbar-gutter: stable;
		scrollbar-width: thin;
		scrollbar-color: #a5b4fc #eef2ff;
	}

	.list-scroll::-webkit-scrollbar {
		width: 10px;
	}

	.list-scroll::-webkit-scrollbar-track {
		background: #eef2ff;
		border-radius: 999px;
	}

	.list-scroll::-webkit-scrollbar-thumb {
		background: linear-gradient(180deg, #6366f1 0%, #ec4899 100%);
		border-radius: 999px;
		border: 2px solid #eef2ff;
	}

	.list-scroll::-webkit-scrollbar-thumb:hover {
		background: linear-gradient(180deg, #4f46e5 0%, #db2777 100%);
	}

	.option-card {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.9rem;
		padding: 0.85rem 0.95rem;
		border: 2px solid #e5e7eb;
		border-radius: 1rem;
		background: #fff;
		cursor: pointer;
		transition: border-color 0.2s ease, box-shadow 0.2s ease, background-color 0.2s ease;
	}

	.option-card:hover {
		border-color: #c7d2fe;
		box-shadow: 0 10px 30px -20px rgba(79, 70, 229, 0.7);
	}

	.option-card-selected {
		border-color: #4f46e5;
		background: #eef2ff;
		box-shadow: inset 0 0 0 1px rgba(79, 70, 229, 0.12);
	}

	.option-radio {
		position: absolute;
		opacity: 0;
		pointer-events: none;
	}

	.badge-best {
		position: absolute;
		top: -0.55rem;
		right: 1rem;
		background: linear-gradient(135deg, #4f46e5, #8b5cf6);
		color: #fff;
		border-radius: 999px;
		padding: 0.2rem 0.55rem;
		font-size: 0.6rem;
		font-weight: 800;
		letter-spacing: 0.03em;
		text-transform: uppercase;
		box-shadow: 0 6px 16px -8px rgba(79, 70, 229, 0.7);
	}

	.option-left {
		min-width: 0;
		display: flex;
		align-items: center;
		gap: 0.8rem;
	}

	.option-icon {
		width: 2.75rem;
		height: 2.75rem;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		border-radius: 0.85rem;
		background: #e2e8f0;
		color: #475569;
		transition: transform 0.2s ease;
	}

	.option-card:hover .option-icon {
		transform: scale(1.06);
	}

	.option-icon-selected {
		box-shadow: 0 8px 18px -12px rgba(79, 70, 229, 0.75);
	}

	.option-icon .material-symbols-outlined {
		font-size: 24px;
	}

	.accent-indigo {
		background: #e0e7ff;
		color: #4f46e5;
	}

	.accent-pink {
		background: #fce7f3;
		color: #db2777;
	}

	.accent-cyan {
		background: #cffafe;
		color: #0891b2;
	}

	.accent-purple {
		background: #ede9fe;
		color: #7c3aed;
	}

	.accent-slate {
		background: #f1f5f9;
		color: #64748b;
	}

	.option-meta {
		min-width: 0;
	}

	.option-title {
		margin: 0;
		color: #0f172a;
		font-size: 1rem;
		font-weight: 800;
		line-height: 1.2;
	}

	.option-submeta {
		margin-top: 0.2rem;
		display: flex;
		flex-wrap: wrap;
		gap: 0.3rem 0.5rem;
		color: #64748b;
		font-size: 0.74rem;
		font-weight: 700;
	}

	.fmt-chip {
		background: #e2e8f0;
		color: #475569;
		border-radius: 0.3rem;
		padding: 0.1rem 0.3rem;
		font-size: 0.65rem;
	}

	.option-right {
		flex: none;
		display: flex;
		align-items: center;
		gap: 0.65rem;
	}

	.size-pill {
		white-space: nowrap;
		border-radius: 0.6rem;
		border: 1px solid #e0e7ff;
		background: #fff;
		padding: 0.3rem 0.55rem;
		font-size: 0.74rem;
		font-weight: 800;
		color: #4f46e5;
	}

	.check-pill {
		width: 1.4rem;
		height: 1.4rem;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		border-radius: 999px;
		border: 2px solid #cbd5e1;
		background: #fff;
		color: transparent;
		font-size: 0.72rem;
		font-weight: 900;
		transition: all 0.2s ease;
	}

	.check-pill-active {
		border-color: #4f46e5;
		background: #4f46e5;
		color: #fff;
	}

	.empty-state {
		display: flex;
		align-items: center;
		gap: 0.65rem;
		border: 1px dashed #cbd5e1;
		border-radius: 0.9rem;
		padding: 0.85rem 1rem;
		color: #64748b;
		font-size: 0.88rem;
		font-weight: 700;
	}

	.empty-state p {
		margin: 0;
	}

	:global(.page-root.theme-dark) .tabs {
		background: #1a2130;
		border-color: rgba(129, 140, 248, 0.22);
	}

	:global(.page-root.theme-dark) .tab-btn {
		color: #b8c4da;
	}

	:global(.page-root.theme-dark) .tab-btn:hover {
		background: #263147;
		color: #dbe7ff;
	}

	:global(.page-root.theme-dark) .tab-btn-active {
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: #ffffff;
		box-shadow: 0 10px 24px -14px rgba(129, 140, 248, 0.9);
	}

	:global(.page-root.theme-dark) .list-scroll {
		scrollbar-color: #5b6e99 #182033;
	}

	:global(.page-root.theme-dark) .list-scroll::-webkit-scrollbar-track {
		background: #182033;
	}

	:global(.page-root.theme-dark) .list-scroll::-webkit-scrollbar-thumb {
		background: linear-gradient(180deg, #6366f1 0%, #a855f7 100%);
		border-color: #182033;
	}

	:global(.page-root.theme-dark) .option-card {
		border-color: #33415f;
		background: #151d2b;
	}

	:global(.page-root.theme-dark) .option-card:hover {
		border-color: #6d81bf;
		box-shadow: 0 14px 28px -22px rgba(129, 140, 248, 0.9);
	}

	:global(.page-root.theme-dark) .option-card-selected {
		border-color: #7c8bff;
		background: #1f2a42;
		box-shadow: inset 0 0 0 1px rgba(129, 140, 248, 0.24);
	}

	:global(.page-root.theme-dark) .option-title {
		color: #f8faff;
	}

	:global(.page-root.theme-dark) .option-submeta {
		color: #aab6cd;
	}

	:global(.page-root.theme-dark) .fmt-chip {
		background: #2a344c;
		color: #d8e2ff;
	}

	:global(.page-root.theme-dark) .size-pill {
		border-color: #3d4f7a;
		background: #111828;
		color: #c7d2fe;
	}

	:global(.page-root.theme-dark) .check-pill {
		border-color: #5f6f90;
		background: #111828;
	}

	:global(.page-root.theme-dark) .check-pill-active {
		border-color: #7c8bff;
		background: #6366f1;
	}

	:global(.page-root.theme-dark) .empty-state {
		border-color: #475569;
		color: #aab6cd;
	}

	:global(.page-root.theme-dark) .accent-indigo {
		background: #273560;
		color: #c7d2fe;
	}

	:global(.page-root.theme-dark) .accent-pink {
		background: #4a2846;
		color: #f9a8d4;
	}

	:global(.page-root.theme-dark) .accent-cyan {
		background: #1d3d4a;
		color: #67e8f9;
	}

	:global(.page-root.theme-dark) .accent-purple {
		background: #3a2d5a;
		color: #c4b5fd;
	}

	:global(.page-root.theme-dark) .accent-slate {
		background: #2b364a;
		color: #cbd5e1;
	}

	@media (max-width: 860px) {
		.size-pill {
			display: none;
		}
	}

	@media (max-width: 560px) {
		.tab-btn {
			padding: 0.65rem 0.5rem;
			font-size: 0.8rem;
		}

		.option-card {
			padding: 0.75rem;
		}

		.option-title {
			font-size: 0.92rem;
		}
	}
</style>
