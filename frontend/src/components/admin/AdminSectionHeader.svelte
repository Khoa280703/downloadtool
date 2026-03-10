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
	<p class="text-[11px] font-bold uppercase tracking-[0.22em] text-slate-500">{eyebrow}</p>
	<h2 class="mt-2 text-2xl font-black tracking-[-0.03em] text-slate-950">{title}</h2>
	<p class="mt-2 text-sm text-slate-600">{description}</p>
</div>
