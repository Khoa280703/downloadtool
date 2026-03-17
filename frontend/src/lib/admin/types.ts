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
	detailJson: Record<string, unknown> | null;
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
	downloadAccessCount: number;
	downloadAccesses24h: number;
	muxJobAccesses24h: number;
	directStreamAccesses24h: number;
	lastDownloadAccessAt: string | null;
};

export type AdminActivityRow = {
	id: string;
	source: 'job' | 'proxy' | 'audit';
	scope: string;
	entityId: string | null;
	label: string;
	eventType: string;
	detail: string | null;
	actorLabel: string | null;
	clientIp: string | null;
	routePath: string | null;
	method: string | null;
	statusCode: number | null;
	outcome: string | null;
	createdAt: string;
	detailJson: Record<string, unknown> | null;
};

export type AdminDashboardData = {
	overview: AdminOverview;
	jobs: AdminJobRow[];
	proxies: AdminProxyRow[];
	activity: AdminActivityRow[];
};
