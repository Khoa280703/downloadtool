<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>Admin Capacity</title>
</svelte:head>

<div class="mb-8">
	<h2 class="text-3xl font-black tracking-tight text-slate-900">Capacity</h2>
	<p class="mt-1 text-sm text-slate-500">Đọc nhanh headroom hiện tại của worker, storage cache và proxy fleet.</p>
</div>

<section class="admin-panel mb-6 overflow-hidden border border-slate-200 bg-white">
	<div class="border-b border-slate-200 bg-slate-50 px-6 py-4">
		<h3 class="text-xs font-bold uppercase tracking-wider text-slate-500">Capacity Summary</h3>
	</div>
	<div class="overflow-x-auto">
		<table class="w-full border-collapse text-left">
			<thead>
				<tr class="border-b border-slate-200 bg-slate-50/60">
					<th class="px-6 py-4 text-xs font-bold uppercase tracking-wider text-slate-500">Signal</th>
					<th class="px-6 py-4 text-xs font-bold uppercase tracking-wider text-slate-500">Value</th>
					<th class="px-6 py-4 text-xs font-bold uppercase tracking-wider text-slate-500">Interpretation</th>
				</tr>
			</thead>
			<tbody class="divide-y divide-slate-100 text-sm">
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Queue Backlog</td><td class="px-6 py-4">{data.overview.queuedJobs + data.overview.leasedJobs}</td><td class="px-6 py-4 text-slate-500">Áp lực đang chờ worker xử lý.</td></tr>
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Processing Load</td><td class="px-6 py-4">{data.overview.processingJobs + data.overview.leasedJobs}</td><td class="px-6 py-4 text-slate-500">Mức tải đang chạy trong pipeline.</td></tr>
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Building Artifacts</td><td class="px-6 py-4">{data.overview.buildingArtifacts}</td><td class="px-6 py-4 text-slate-500">Số artifact đang mux/upload.</td></tr>
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Ready Artifacts</td><td class="px-6 py-4">{data.overview.readyArtifacts}</td><td class="px-6 py-4 text-slate-500">Cache hiện có để reuse và cấp ticket.</td></tr>
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Active Proxies</td><td class="px-6 py-4">{data.overview.activeProxies}</td><td class="px-6 py-4 text-slate-500">Độ phủ thực tế của fleet.</td></tr>
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Quarantined Proxies</td><td class="px-6 py-4">{data.overview.quarantinedProxies}</td><td class="px-6 py-4 text-slate-500">Rủi ro thiếu proxy nếu tăng tiếp.</td></tr>
			</tbody>
		</table>
	</div>
</section>

<section class="admin-panel overflow-hidden border border-slate-200 bg-white">
	<div class="border-b border-slate-200 bg-slate-50 px-6 py-4">
		<h3 class="text-sm font-bold text-slate-900">Operational Notes</h3>
		<p class="mt-1 text-sm text-slate-500">Guidance đơn giản cho operator khi capacity thay đổi.</p>
	</div>
	<div class="overflow-x-auto">
		<table class="w-full border-collapse text-left">
			<thead>
				<tr class="border-b border-slate-200 bg-slate-50/60">
					<th class="px-6 py-4 text-xs font-bold uppercase tracking-wider text-slate-500">Condition</th>
					<th class="px-6 py-4 text-xs font-bold uppercase tracking-wider text-slate-500">Action</th>
				</tr>
			</thead>
			<tbody class="divide-y divide-slate-100 text-sm">
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Failed + expired tăng liên tục</td><td class="px-6 py-4 text-slate-500">Kiểm tra proxy health, upstream extract và lease timeout.</td></tr>
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Building artifacts cao nhưng ready thấp</td><td class="px-6 py-4 text-slate-500">Xem worker throughput, upload latency và storage backend.</td></tr>
				<tr><td class="px-6 py-4 font-semibold text-slate-900">Quarantined proxies tăng nhanh</td><td class="px-6 py-4 text-slate-500">Bổ sung proxy mới hoặc tạm giảm load extract.</td></tr>
			</tbody>
		</table>
	</div>
</section>
