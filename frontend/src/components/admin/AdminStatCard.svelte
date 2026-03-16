<script lang="ts">
	let {
		label,
		value,
		caption = ''
	}: {
		label: string;
		value: string | number;
		caption?: string;
	} = $props();
</script>

<article class="rounded-md border border-gray-200 bg-white p-3.5">
	<p class="text-[11px] font-medium uppercase tracking-wider text-gray-400">{label}</p>
	<p class="mt-1.5 text-2xl font-semibold tabular-nums tracking-tight text-gray-900">{value}</p>
	{#if caption}
		<p class="mt-1 text-[11px] leading-4 text-gray-500">{caption}</p>
	{/if}
</article>
