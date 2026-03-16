<script lang="ts">
	let {
		label,
		value,
		caption = '',
		inverted = false
	}: {
		label: string;
		value: string | number;
		caption?: string;
		inverted?: boolean;
	} = $props();
</script>

<div
	class={`rounded-md border px-3 py-2 ${inverted ? 'border-gray-700 bg-gray-800 text-white' : 'border-gray-200 bg-white text-gray-900'}`}
>
	<p
		class={`text-[10px] font-medium uppercase tracking-wider ${inverted ? 'text-gray-400' : 'text-gray-400'}`}
	>
		{label}
	</p>
	<p class={`mt-1 text-lg font-semibold tabular-nums tracking-tight ${inverted ? 'text-white' : 'text-gray-900'}`}>
		{value}
	</p>
	{#if caption}
		<p class={`mt-0.5 text-[11px] leading-4 ${inverted ? 'text-gray-300' : 'text-gray-500'}`}>{caption}</p>
	{/if}
</div>
