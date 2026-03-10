<script lang="ts">
	import type { AdminJobRow } from '$lib/admin/types';

	let { jobs }: { jobs: AdminJobRow[] } = $props();

	function formatBytes(value: number | null): string {
		if (!value) return '—';
		if (value < 1024 * 1024) return `${Math.round(value / 1024)} KB`;
		if (value < 1024 * 1024 * 1024) return `${(value / 1024 / 1024).toFixed(1)} MB`;
		return `${(value / 1024 / 1024 / 1024).toFixed(2)} GB`;
	}
</script>

<div class="overflow-x-auto">
	<table class="min-w-full text-sm">
		<thead class="text-left text-xs uppercase tracking-[0.18em] text-plum/55">
			<tr>
				<th class="pb-3">Job</th>
				<th class="pb-3">Status</th>
				<th class="pb-3">Owner</th>
				<th class="pb-3">Attempts</th>
				<th class="pb-3">Artifact</th>
				<th class="pb-3">Updated</th>
			</tr>
		</thead>
		<tbody class="divide-y divide-pink-100/80">
			{#each jobs as job}
				<tr class="align-top">
					<td class="py-3 pr-4">
						<p class="max-w-[24rem] truncate font-semibold text-plum">{job.title ?? job.id}</p>
						<p class="mt-1 font-mono text-xs text-plum/55">{job.id}</p>
						{#if job.lastError}
							<p class="mt-2 max-w-[28rem] text-xs text-red-700">{job.lastError}</p>
						{/if}
					</td>
					<td class="py-3 pr-4">
						<span class="inline-flex rounded-full bg-pink-100 px-3 py-1 text-xs font-bold uppercase tracking-wide text-pink-700">
							{job.status}
						</span>
					</td>
					<td class="py-3 pr-4 font-mono text-xs text-plum/65">{job.ownerLabel}</td>
					<td class="py-3 pr-4 text-plum/70">{job.attemptLabel}</td>
					<td class="py-3 pr-4">
						<p class="text-plum/75">{job.backend ?? '—'}</p>
						<p class="text-xs text-plum/55">{formatBytes(job.fileSizeBytes)}</p>
					</td>
					<td class="py-3 text-plum/70">{new Date(job.updatedAt).toLocaleString()}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
