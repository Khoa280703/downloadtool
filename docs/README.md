# Documentation Index

Welcome to the downloadtool documentation. This folder contains comprehensive guides for users, developers, and operators.

**Last Updated:** 2026-03-01

## Quick Navigation

### For Users & Project Managers
Start here to understand what the project does and why:
- **[Project Overview & PDR](./project-overview-pdr.md)** - Product vision, requirements, roadmap, and success metrics

### For Developers
Understand how to develop and extend the system:
1. **[Codebase Summary](./codebase-summary.md)** - Start with architecture overview and component responsibilities
2. **[System Architecture](./system-architecture.md)** - Deep dive into data flows, interfaces, and component interactions
3. **[Code Standards](./code-standards.md)** - Development guidelines, naming conventions, patterns, and examples

### For Operations & DevOps
Deploy and maintain the system:
- **[Project Overview & PDR](./project-overview-pdr.md)** - See "Deployment Strategy" section
- **[System Architecture](./system-architecture.md)** - See "Deployment Architecture" and "Performance Characteristics"

## Document Overview

### 1. Codebase Summary (323 lines)
**Purpose:** High-level overview of project structure

**Contains:**
- Project overview and vision
- Complete architecture with diagrams
- 8 major components overview
- Recent changes (2026-02-28): yt-dlp subprocess extractor, batch SSE operations, auth system, Whop webhooks
- Technology stack
- File organization
- Design patterns
- Known limitations

**Start here if:** You want a bird's-eye view of the system

---

### 2. System Architecture (580 lines)
**Purpose:** Detailed architectural flows and technical specifications

**Contains:**
- High-level architecture diagram
- 3 detailed data flow diagrams:
  - Download & extraction flow
  - YouTube n-parameter transform flow
  - GPU transcoding flow
- Component interfaces (API routes, extractors, anti-bot)
- Critical components deep-dive:
  - Muxer architecture (NEW: dual-traf, QuickTime fix, WebM exclusion)
  - Anti-bot layer (timeout fix explanation)
  - N-parameter transform details
  - Extraction engine strategy
- Data structures & types
- Request/response flows (with before/after timeout fix)
- Security & anti-detection measures
- Performance characteristics
- Deployment architecture
- Error handling strategy (WebM handling added)

**Start here if:** You need to understand how data flows through the system

---

### 3. Code Standards (640 lines)
**Purpose:** Development guidelines and best practices

**Contains:**
- Complete directory structure with annotations
- Rust workspace organization (40+ source files across 6 crates)
- Naming conventions (Rust, TypeScript, Svelte)
- Code organization principles
- Error handling & logging patterns
- 8 critical component walkthroughs:
  - yt-dlp subprocess extractor (NEW: 536 LOC, moka cache, semaphore throttle)
  - JWT middleware & claims (NEW: auth system)
  - Whop webhook handler (NEW: subscription integration)
  - Muxer architecture (dual-traf, 3,205 LOC, QuickTime fix)
  - Anti-bot client (timeout fix explanation)
  - YouTube n-transform module (algorithm & regex)
  - GPU pipeline (hardware acceleration)
  - Batch operations with SSE (NEW)
- Testing & quality standards
- Performance optimizations
- Security practices (JWT, HMAC signatures)
- Build & compilation
- Common pitfalls & solutions
- Deployment checklist

**Start here if:** You're implementing features or fixing bugs

---

### 4. Project Overview & PDR (580 lines)
**Purpose:** Product requirements and roadmap

**Contains:**
- Executive summary & product vision
- Target users & platform support matrix
- 8 functional requirements with acceptance criteria
- 6 non-functional requirements (performance, reliability, etc.)
- Architecture & technical constraints
- 8+ implementation phases with status (8.3 = QuickTime/WebM fixes)
- Recent changes (2026-02-24):
  - QuickTime double-duration bug fixed (moov_merger.rs)
  - WebM video-only stream exclusion (API 422 + frontend filter)
  - Dual-track fMP4 muxer architecture (traf_merger.rs)
  - Download timeout fix (still active from 2026-02-23)
- Success metrics & KPIs (updated: QuickTime ✅, WebM handling ✅)
- Risk assessment & mitigation
- Dependencies & external services
- 4-quarter roadmap (Q1-Q4 2026)
- Known limitations
- Testing strategy
- Compliance & legal considerations
- Version history (v2.1 2026-02-24)

**Start here if:** You need to understand requirements or plan features

---

## Key Changes (2026-03-01 — Frontend Auth & Performance)

### 1. Auth Flow Migration to Modal
**Files:** `frontend/src/routes/(auth)/login/` (DELETED), `frontend/src/hooks.server.ts` (updated), `frontend/src/components/AuthModal.svelte` (NEW)

**Changes:**
- Removed dedicated `/login` route (page.svelte + page.server.ts)
- Login now triggered via `?auth=required` URL parameter on homepage
- `hooks.server.ts` detects missing session → redirects to `/?auth=required`
- AuthModal pops up client-side without full page navigation
- BFF pattern: `hooks.server.ts` proxies auth.api calls with secrets, frontend never sees credentials

**Impact:** Reduced page loads, improved UX, centralized auth modal on homepage

### 2. Font Optimization (Material Symbols)
**File:** `frontend/src/app.html`

**Changes:**
- Google Material Symbols font: 1.1 MB → 4.5 KB (subset of 27 icons only)
- Added `font-display: swap` for non-blocking rendering
- Preload CSS: `<link rel="preload" href="..." as="style">`
- Lazy loading on external Google CDN (images): Added `loading="lazy"` + `fetchpriority="low"`

**Impact:** LCP improved (was bottleneck at 7.5s), FCP faster, bandwidth savings 1.09 MB per user

### 3. Homepage Prerendering
**File:** `frontend/src/routes/+page.ts` (NEW)

**Changes:**
- Added `export const prerender = true` to enable build-time prerendering
- Deleted `+page.server.ts` (no server-side data needed for static home)
- Homepage now static HTML (no server request required)
- Auth check moved to `hooks.server.ts` only (fires after static load)

**Impact:** Instant homepage load, no initial server roundtrip, cacheable by CDN

### 4. Cookie Check Optimization
**File:** `frontend/src/hooks.server.ts`

**Changes:**
- Added early return if `better-auth` cookie not present
- Skips `auth.api.getSession()` DB query for unauthenticated users
- Only calls getSession() if cookie exists (user likely authenticated)

**Impact:** DB query savings 95%+ for unauthenticated users, faster auth check

### 5. Batch Download URL Fix
**File:** `frontend/src/lib/playlist-download-worker.ts`

**Changes:**
- Always uses `buildStreamUrl()` to construct proper muxed URLs
- Was using raw CDN URL in some cases, breaking download attribute
- Now consistent URL building for all stream types

**Impact:** Batch downloads work reliably across all format combinations

---

## Key Changes (2026-02-28)

### 1. yt-dlp Subprocess Extractor
**File:** `crates/extractor/src/ytdlp.rs` (NEW, 536 LOC)
- Replaced Deno in-process with `yt-dlp -J --no-playlist` subprocess
- Moka cache (500 items, 300s TTL) for faster repeat lookups
- Tokio Semaphore (max 10 concurrent) prevents resource exhaustion
- Fallback retry with alternate player client on format errors
- Metrics: cache hits/misses, fallback retry count tracked

**Why:** yt-dlp handles PO Token, signature decryption, throttle bypass automatically

### 2. JWT Authentication System
**Files:** `crates/api/src/auth/` (NEW, 3 modules)
- `jwt_claims.rs` — Token structure (user_id, tier, exp)
- `jwt_middleware.rs` — Axum extractor for validation (141 LOC)
- `user_tier.rs` — Enum (Free, Pro, Premium) with rate limits
- HMAC-SHA256 signing via `jsonwebtoken` crate
- BFF pattern: SvelteKit proxies API calls with JWT server-side

### 3. Whop Subscription Integration
**File:** `crates/api/src/routes/whop_webhook.rs` (NEW, 187 LOC)
- HMAC-SHA256 signature verification (X-Whop-Signature header)
- Updates PostgreSQL subscriptions table on webhook
- User tier changes take effect immediately

### 4. Batch Operations with SSE
**File:** `crates/api/src/routes/batch.rs` (updated)
- Server-Sent Events (SSE) stream instead of polling
- Real-time per-download status updates
- Rate limiting per user tier
- Frontend components: BatchInput, BatchProgress, BatchActiveState

### 5. PostgreSQL Integration
- Connection pooling via `sqlx::PgPool`
- Schema: subscriptions(user_id, tier, created_at, expires_at)
- Migrations in `crates/api/migrations/0001_*`

**Documented in:** All core doc files updated (overview, codebase summary, code standards, system architecture)

## Key Changes (2026-02-24)

### 1. QuickTime Double-Duration Bug Fixed
**File:** `crates/muxer/src/moov_merger.rs`

YouTube DASH streams set `mdhd.duration` per track. When muxed, QuickTime summed them (213+213=426s instead of 213s). Now zeros both trak mdhd.duration.

**Documented in:**
- Codebase Summary → Recent Changes
- System Architecture → Critical Components Deep-Dive (Muxer section)
- Code Standards → Muxer Architecture Walkthrough
- Project Overview → Phase 8.3

### 2. WebM Video-Only Stream Exclusion
**Files:** `crates/api/src/routes/stream.rs` + `frontend/src/components/FormatPicker.svelte`

WebM uses EBML (incompatible with ISO BMFF fMP4). Backend returns 422, frontend filters VP9/WebM from options.

**Documented in:**
- Codebase Summary → Recent Changes
- System Architecture → Critical Components (Muxer) + Error Handling
- Code Standards → Muxer Architecture + Common Pitfalls
- Project Overview → Phase 8.3

### 3. Dual-Track fMP4 Muxer Architecture
**Files:** `crates/muxer/src/traf_merger.rs` (416 LOC) + `fmp4_remuxer.rs` (407 LOC)

Video-led grouping with dual traf boxes, QuickTime-compatible, removes legacy fmp4_muxer.

**Documented in:**
- Codebase Summary → Muxer Components (8 modules listed)
- System Architecture → Critical Components (Muxer section)
- Code Standards → Muxer Architecture Walkthrough
- Project Overview → Phase 8.3

### 4. Download Timeout Bug Fix (2026-02-23, Still Active)
**File:** `crates/proxy/src/anti_bot.rs` (Line 99)

Changed `.timeout(30s)` to `.connect_timeout(30s)` so downloads don't timeout mid-transfer.

**Documented in:**
- Codebase Summary → Recent Changes
- System Architecture → Request/Response Flow + Critical Components
- Code Standards → Anti-Bot Client Section
- Project Overview → Phase 8.2

## Documentation Map

```
docs/
├── README.md (this file)
│   └── Navigation & quick reference
├── codebase-summary.md
│   └── Architecture & components overview
├── system-architecture.md
│   └── Data flows & technical details
├── code-standards.md
│   └── Development guidelines & patterns
└── project-overview-pdr.md
    └── Requirements & roadmap
```

## Reading Paths

### Path 1: Quick Understanding (30 minutes)
1. This README
2. Codebase Summary (overview section only)
3. Project Overview → Recent Changes

**Result:** Understand what changed and why

### Path 2: Architecture Deep-Dive (90 minutes)
1. Codebase Summary (full read)
2. System Architecture (full read)
3. Code Standards (skim key sections)

**Result:** Understand how the system works

### Path 3: Development Setup (60 minutes)
1. Project Overview → Deployment Strategy
2. Code Standards (full read)
3. System Architecture (data flows section)

**Result:** Ready to implement features

### Path 4: Feature Planning (120 minutes)
1. Project Overview & PDR (full read)
2. Codebase Summary → Design Patterns
3. System Architecture → Critical Components

**Result:** Ready to plan new features

## Common Questions

**Q: How does YouTube download bypass throttling?**
A: See System Architecture → YouTube N-Transform Flow Diagram

**Q: Why did the timeout fix matter?**
A: See System Architecture → Request/Response Flow (Before/After) or Code Standards → Anti-Bot Client

**Q: How do extractors work?**
A: See System Architecture → Download & Extraction Flow

**Q: What's the GPU pipeline?**
A: See System Architecture → GPU Transcoding Flow

**Q: How do I add a new platform?**
A: See Code Standards → Extractor Engine section, then Project Overview → Future Improvements

**Q: Where's the deployment guide?**
A: See Project Overview & PDR → Deployment Strategy section

**Q: What are the performance targets?**
A: See System Architecture → Performance Characteristics table or Project Overview → Success Metrics

## Reporting & Updates

### Latest Report
- **File:** `/plans/reports/docs-manager-260224-1016-post-implementation-documentation-update.md`
- **Date:** 2026-02-24
- **Status:** In progress (this update)

### Report Contents
- All recent changes (QuickTime fix, WebM exclusion, dual-traf muxer) documented
- Cross-referenced to source code and architecture
- Quality assurance checklist
- Metrics updated (Rust 43 files, 10K LOC; Muxer 3,205 LOC across 8 modules)

## Maintenance & Updates

### Update Frequency
- **Weekly:** Monitor for YouTube player.js changes
- **Monthly:** Review timeout and performance metrics
- **Quarterly:** Update roadmap and KPIs
- **Ad-hoc:** Critical changes or hotfixes

### How to Update Documentation
1. Make code changes
2. Update relevant docs (usually 2-3 files)
3. Update version history in affected documents
4. Run verification (ensure examples still work)
5. Commit with clear messages

### Documentation Owners
- **codebase-summary.md:** Architecture changes, component updates
- **system-architecture.md:** Data flow changes, performance changes
- **code-standards.md:** New patterns, naming/structure changes
- **project-overview-pdr.md:** Requirements, roadmap, metrics

## Resource Links

### Code Directories
- API Server: `/crates/api`
- Extractors: `/extractors`
- Proxy Layer: `/crates/proxy`
- GPU Pipeline: `/crates/gpu-pipeline`
- Frontend: `/frontend`

### Infrastructure
- Docker: `/docker`
- Kubernetes: `/infra`
- CI/CD: `/.github/workflows`

### Development Plans
- Internationalization (i18n): `/plans/260301-1326-i18n-paraglide-claude-api/plan.md`

## Getting Help

1. **Understanding Architecture:** Read System Architecture document
2. **Implementation Questions:** Check Code Standards for patterns
3. **Requirements Tracing:** See Project Overview & PDR
4. **Deployment Issues:** Check Project Overview → Deployment Strategy
5. **Performance Tuning:** See System Architecture → Performance Characteristics

---

**Documentation Version:** 2.3
**Last Generated:** 2026-03-01
**Status:** Complete ✅ (Frontend Auth & Performance ✅, Phase 10 i18n Planned)

For the latest updates, check `/plans/reports/` for implementation reports.
