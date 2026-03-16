<script lang="ts">
	let {
		eyebrow,
		title,
		description
	}: {
		eyebrow: string;
		title: string;
		description: string;
	} = $props();
</script>

<div>
	<p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">{eyebrow}</p>
	<h2 class="mt-1 text-base font-semibold text-gray-900">{title}</h2>
	<p class="mt-0.5 text-[12px] leading-5 text-gray-500">{description}</p>
</div>
