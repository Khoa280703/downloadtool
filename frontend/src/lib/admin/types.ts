export type ProxyStatus = 'active' | 'disabled' | 'quarantined';
export type AdminJobStatus = 'queued' | 'leased' | 'processing' | 'ready' | 'failed' | 'expired';

export type AdminOverview = {
	queuedJobs: number;
	leasedJobs: number;
	processingJobs: number;
	readyJobs: number;
	failedJobs: number;
	expiredJobs: number;
	buildingArtifacts: number;
	readyArtifacts: number;
	activeProxies: number;
	quarantinedProxies: number;
	disabledProxies: number;
	eventsLast24h: number;
};

export type AdminJobRow = {
	id: string;
	title: string | null;
	status: AdminJobStatus;
	ownerLabel: string;
	attemptLabel: string;
	fileSizeBytes: number | null;
	backend: string | null;
	lastError: string | null;
	sourceUrl: string | null;
	updatedAt: string;
};

export type AdminProxyRow = {
	id: string;
	maskedProxyUrl: string;
	displayName: string | null;
	status: ProxyStatus;
	source: string;
	notes: string | null;
	lastQuarantinedAt: string | null;
	lastQuarantineReason: string | null;
	updatedAt: string;
	eventCount24h: number;
	lastEventType: string | null;
	lastEventAt: string | null;
};

export type AdminActivityRow = {
	id: string;
	scope: 'job' | 'proxy';
	entityId: string;
	label: string;
	eventType: string;
	detail: string | null;
	createdAt: string;
};

export type AdminDashboardData = {
	overview: AdminOverview;
	jobs: AdminJobRow[];
	proxies: AdminProxyRow[];
	activity: AdminActivityRow[];
};
