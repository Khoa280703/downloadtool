import type { AdminOverview } from './types';

export type StatTone = 'neutral' | 'sky' | 'emerald' | 'amber' | 'rose';
export type AdminSectionId = 'overview' | 'jobs' | 'proxies' | 'activity' | 'capacity';
export type AdminSectionGroup = 'Monitor' | 'Operations';

export type AdminSectionItem = {
	id: AdminSectionId;
	group: AdminSectionGroup;
	label: string;
	icon: string;
	description: string;
	href: string;
};

export type DashboardStat = {
	label: string;
	value: number;
	caption: string;
	tone: StatTone;
};

export const adminSectionItems: AdminSectionItem[] = [
	{
		id: 'overview',
		group: 'Monitor',
		label: 'Overview',
		icon: 'space_dashboard',
		description: 'Tổng quan hệ thống',
		href: '/admin/overview'
	},
	{
		id: 'jobs',
		group: 'Operations',
		label: 'Jobs',
		icon: 'work_history',
		description: 'Queue và tiến trình mux',
		href: '/admin/jobs'
	},
	{
		id: 'proxies',
		group: 'Operations',
		label: 'Proxies',
		icon: 'hub',
		description: 'Fleet, quarantine, add proxy',
		href: '/admin/proxies'
	},
	{
		id: 'activity',
		group: 'Monitor',
		label: 'Activity',
		icon: 'monitoring',
		description: 'Timeline event gần nhất',
		href: '/admin/activity'
	},
	{
		id: 'capacity',
		group: 'Operations',
		label: 'Capacity',
		icon: 'speed',
		description: 'Sức chứa và trạng thái',
		href: '/admin/capacity'
	}
];

export function buildAdminDashboardViewModel(overview: AdminOverview) {
	const queueBacklog = overview.queuedJobs + overview.leasedJobs;
	const activeJobs = overview.processingJobs + overview.leasedJobs;
	const totalArtifacts = overview.buildingArtifacts + overview.readyArtifacts;
	const totalProxies =
		overview.activeProxies + overview.disabledProxies + overview.quarantinedProxies;

	const topStats: DashboardStat[] = [
		{
			label: 'Queue backlog',
			value: queueBacklog,
			caption: `${overview.processingJobs} đang xử lý`,
			tone: queueBacklog > 0 ? 'amber' : 'neutral'
		},
		{
			label: 'Active workers',
			value: activeJobs,
			caption: `${overview.readyJobs} job hoàn tất`,
			tone: activeJobs > 0 ? 'sky' : 'neutral'
		},
		{
			label: 'Failed / expired',
			value: overview.failedJobs + overview.expiredJobs,
			caption: `${overview.failedJobs} failed, ${overview.expiredJobs} expired`,
			tone: overview.failedJobs + overview.expiredJobs > 0 ? 'rose' : 'neutral'
		},
		{
			label: 'Artifacts',
			value: totalArtifacts,
			caption: `${overview.readyArtifacts} file sẵn sàng`,
			tone: overview.readyArtifacts > 0 ? 'emerald' : 'neutral'
		},
		{
			label: 'Proxy fleet',
			value: totalProxies,
			caption: `${overview.quarantinedProxies} quarantined`,
			tone: overview.quarantinedProxies > 0 ? 'rose' : 'sky'
		},
		{
			label: 'Signals / 24h',
			value: overview.eventsLast24h,
			caption: 'job events + proxy events',
			tone: 'neutral'
		}
	];

	return {
		queueBacklog,
		activeJobs,
		totalArtifacts,
		totalProxies,
		topStats,
		headerStats: [
			{ label: 'Queued', value: overview.queuedJobs, caption: 'Chờ worker nhận lease' },
			{ label: 'Processing', value: overview.processingJobs, caption: 'Đang mux và upload' },
			{ label: 'Ready', value: overview.readyJobs, caption: 'Đã có ticket để tải' }
		],
		queueStats: [
			{ label: 'Leased', value: overview.leasedJobs },
			{ label: 'Failed', value: overview.failedJobs },
			{ label: 'Expired', value: overview.expiredJobs }
		],
		snapshotStats: [
			{
				label: 'Queue pressure',
				value: queueBacklog > 0 ? `${queueBacklog} job cần hoàn thành` : 'Ổn định',
				caption: `${overview.processingJobs} processing, ${overview.leasedJobs} leased.`
			},
			{
				label: 'Proxy coverage',
				value: `${overview.activeProxies} active / ${totalProxies}`,
				caption: `${overview.quarantinedProxies} quarantined, ${overview.disabledProxies} disabled.`
			},
			{
				label: 'Artifact cache',
				value: `${overview.readyArtifacts} ready / ${totalArtifacts}`,
				caption: `${overview.buildingArtifacts} artifact đang build.`
			}
		]
	};
}

export function getAdminSectionBadge(sectionId: AdminSectionId, overview: AdminOverview): number | null {
	switch (sectionId) {
		case 'jobs': {
			const queueBacklog = overview.queuedJobs + overview.leasedJobs;
			return queueBacklog || null;
		}
		case 'proxies':
			return overview.quarantinedProxies || null;
		case 'activity':
			return overview.eventsLast24h || null;
		case 'capacity':
			return overview.buildingArtifacts || null;
		default:
			return null;
	}
}
