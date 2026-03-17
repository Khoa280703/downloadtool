<script lang="ts">
	import AppIcon from '$components/AppIcon.svelte';

	type SummaryField = {
		label: string;
		value: string | number | null;
	};

	let {
		open = false,
		title,
		subtitle = null,
		summary = [],
		payload = null,
		onClose = () => {}
	}: {
		open?: boolean;
		title: string;
		subtitle?: string | null;
		summary?: SummaryField[];
		payload?: unknown;
		onClose?: () => void;
	} = $props();

	const formattedPayload = $derived(
		payload == null ? null : JSON.stringify(payload, null, 2)
	);

	function handleBackdropClick(event: MouseEvent) {
		if (event.target === event.currentTarget) {
			onClose();
		}
	}
</script>

{#if open}
	<div
		class="fixed inset-0 z-[120] flex items-start justify-center bg-gray-950/45 px-4 py-8"
		onclick={handleBackdropClick}
		onkeydown={(event) => event.key === 'Escape' && onClose()}
		role="presentation"
		tabindex="-1"
	>
		<div class="flex max-h-[90vh] w-full max-w-5xl flex-col overflow-hidden border border-gray-200 bg-white shadow-2xl">
			<div class="flex items-start justify-between border-b border-gray-200 bg-gray-50 px-5 py-4">
				<div class="min-w-0">
					<h3 class="truncate text-sm font-semibold text-gray-900">{title}</h3>
					{#if subtitle}
						<p class="mt-1 truncate text-xs text-gray-500">{subtitle}</p>
					{/if}
				</div>
				<button
					type="button"
					class="inline-flex h-8 w-8 items-center justify-center border border-gray-200 text-gray-500 transition hover:bg-gray-100"
					onclick={onClose}
					aria-label="Close details"
				>
					<AppIcon name="close" class="text-base" />
				</button>
			</div>

			<div class="grid min-h-0 flex-1 grid-cols-1 divide-y divide-gray-200 overflow-hidden lg:grid-cols-[22rem_minmax(0,1fr)] lg:divide-x lg:divide-y-0">
				<div class="overflow-y-auto px-5 py-4">
					<p class="mb-3 text-[10px] font-semibold uppercase tracking-[0.18em] text-gray-400">
						Summary
					</p>
					<div class="space-y-3">
						{#each summary as field}
							<div class="border-b border-gray-100 pb-2">
								<p class="text-[10px] font-medium uppercase tracking-[0.14em] text-gray-400">
									{field.label}
								</p>
								<p class="mt-1 break-all text-[13px] text-gray-700">
									{field.value == null || field.value === '' ? '—' : String(field.value)}
								</p>
							</div>
						{/each}
					</div>
				</div>

				<div class="min-h-0 overflow-y-auto px-5 py-4">
					<div class="mb-3 flex items-center justify-between">
						<p class="text-[10px] font-semibold uppercase tracking-[0.18em] text-gray-400">
							Full Payload
						</p>
					</div>
					{#if formattedPayload}
						<pre class="overflow-x-auto border border-gray-200 bg-gray-950 p-4 text-[12px] leading-5 text-gray-100">{formattedPayload}</pre>
					{:else}
						<div class="border border-dashed border-gray-200 bg-gray-50 px-4 py-10 text-center text-sm text-gray-400">
							No additional payload.
						</div>
					{/if}
				</div>
			</div>
		</div>
	</div>
{/if}
