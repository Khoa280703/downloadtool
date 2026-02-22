<script lang="ts">
	import { extract, isValidVideoUrl } from '$lib/api';
	import {
		currentDownload,
		setVideoUrl,
		setExtracted,
		setError
	} from '$stores/download';
	import { trackUrlSubmitted } from '$lib/analytics';
	import type { ExtractResult } from '$lib/types';

	interface Props {
		onExtract?: (result: ExtractResult) => void;
	}

	let { onExtract }: Props = $props();

	let url = $state('');
	let isPasting = $state(false);
	let validationError = $state('');

	/** Validate URL format */
	function validate(input: string): boolean {
		if (!input.trim()) {
			validationError = '';
			return false;
		}
		if (!isValidVideoUrl(input)) {
			validationError = 'Please enter a valid YouTube or TikTok URL';
			return false;
		}
		validationError = '';
		return true;
	}

	/** Handle paste button click */
	async function handlePaste(): Promise<void> {
		try {
			isPasting = true;
			const text = await navigator.clipboard.readText();
			url = text;
			validate(text);
		} catch (err) {
			console.error('Failed to read clipboard:', err);
			validationError = 'Could not access clipboard. Please paste manually.';
		} finally {
			isPasting = false;
		}
	}

	/** Handle form submission */
	async function handleSubmit(): Promise<void> {
		if (!validate(url)) return;

		// Track URL submission
		const platform = url.includes('tiktok') ? 'tiktok' : 'youtube';
		trackUrlSubmitted(platform, url.length);

		setVideoUrl(url);

		try {
			const result = await extract(url);
			setExtracted(result.streams);
			onExtract?.(result);
		} catch (err) {
			const message = err instanceof Error ? err.message : 'Extraction failed';
			setError(message);
		}
	}

	/** Handle input changes */
	function handleInput(e: Event): void {
		const target = e.target as HTMLInputElement;
		url = target.value;
		validate(url);
	}
</script>

<div class="url-input">
	<form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
		<div class="input-wrapper">
			<input
				type="url"
				placeholder="Paste YouTube or TikTok link..."
				value={url}
				oninput={handleInput}
				aria-label="Video URL"
				aria-invalid={validationError ? 'true' : 'false'}
				aria-describedby={validationError ? 'url-error' : undefined}
				class="url-field"
			/>
			<button
				type="button"
				class="paste-btn"
				onclick={handlePaste}
				disabled={isPasting}
				aria-label="Paste from clipboard"
			>
				{#if isPasting}
					<span class="spinner-small"></span>
				{:else}
					<svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor">
						<path d="M19 2h-4.18C14.4.84 13.3 0 12 0c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm7 18H5V4h2v3h10V4h2v16z"/>
					</svg>
				{/if}
			</button>
		</div>

		{#if validationError}
			<span id="url-error" class="error-text" role="alert">{validationError}</span>
		{/if}

		<button
			type="submit"
			class="submit-btn"
			disabled={!url || !!validationError || $currentDownload.isExtracting}
		>
			{#if $currentDownload.isExtracting}
				<span class="spinner"></span>
				Analyzing...
			{:else}
				Get Video
			{/if}
		</button>
	</form>
</div>

<style>
	.url-input {
		width: 100%;
	}

	form {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.input-wrapper {
		display: flex;
		gap: 0.5rem;
	}

	.url-field {
		flex: 1;
		padding: 0.875rem 1rem;
		font-size: 1rem;
		border: 2px solid var(--border-color, #e5e7eb);
		border-radius: 0.75rem;
		background: var(--input-bg, #ffffff);
		color: var(--text-color, #111827);
		transition: border-color 0.2s, box-shadow 0.2s;
	}

	.url-field:focus {
		outline: none;
		border-color: var(--primary-color, #3b82f6);
		box-shadow: 0 0 0 3px var(--primary-alpha, rgba(59, 130, 246, 0.1));
	}

	.url-field[aria-invalid="true"] {
		border-color: var(--error-color, #ef4444);
	}

	.paste-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		padding: 0;
		border: 2px solid var(--border-color, #e5e7eb);
		border-radius: 0.75rem;
		background: var(--input-bg, #ffffff);
		color: var(--text-secondary, #6b7280);
		cursor: pointer;
		transition: all 0.2s;
	}

	.paste-btn:hover:not(:disabled) {
		border-color: var(--primary-color, #3b82f6);
		color: var(--primary-color, #3b82f6);
	}

	.paste-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.error-text {
		color: var(--error-color, #ef4444);
		font-size: 0.875rem;
	}

	.submit-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 1rem 1.5rem;
		font-size: 1rem;
		font-weight: 600;
		color: white;
		background: var(--primary-color, #3b82f6);
		border: none;
		border-radius: 0.75rem;
		cursor: pointer;
		transition: background 0.2s, transform 0.1s;
		min-height: 48px;
	}

	.submit-btn:hover:not(:disabled) {
		background: var(--primary-hover, #2563eb);
	}

	.submit-btn:active:not(:disabled) {
		transform: scale(0.98);
	}

	.submit-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.spinner {
		width: 18px;
		height: 18px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.spinner-small {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(0, 0, 0, 0.1);
		border-top-color: currentColor;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	@media (prefers-color-scheme: dark) {
		.url-field {
			--input-bg: #1f2937;
			--border-color: #374151;
			--text-color: #f9fafb;
		}

		.paste-btn {
			--input-bg: #1f2937;
			--border-color: #374151;
			--text-secondary: #9ca3af;
		}
	}
</style>
