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
	healthScore: number;
	autoDisabledAt: string | null;
	autoDisabledReason: string | null;
	lastQuarantinedAt: string | null;
	lastQuarantineReason: string | null;
	quarantineExpiresAt: string | null;
	updatedAt: string;
	eventCount24h: number;
	lastEventType: string | null;
	lastEventAt: string | null;
	extractAttempts24h: number;
	proxyRelevantAttempts24h: number;
	extractSuccesses24h: number;
	fullFormatHits24h: number;
	combined360OnlyHits24h: number;
	timeoutHits24h: number;
	p95ExtractLatencyMs: number | null;
	lastExtractOutcome: string | null;
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
