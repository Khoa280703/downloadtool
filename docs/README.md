# Documentation Index

Welcome to the downloadtool documentation. This folder contains comprehensive guides for users, developers, and operators.

**Last Updated:** 2026-02-23

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

### 1. Codebase Summary (219 lines)
**Purpose:** High-level overview of project structure

**Contains:**
- Project overview and vision
- Complete architecture with diagrams
- 8 major components overview
- Recent changes (2026-02-23)
- Technology stack
- File organization
- Design patterns
- Known limitations

**Start here if:** You want a bird's-eye view of the system

---

### 2. System Architecture (548 lines)
**Purpose:** Detailed architectural flows and technical specifications

**Contains:**
- High-level architecture diagram
- 3 detailed data flow diagrams:
  - Download & extraction flow
  - YouTube n-parameter transform flow
  - GPU transcoding flow
- Component interfaces (API routes, extractors, anti-bot)
- Critical components deep-dive
- Data structures & types
- Request/response flows (with before/after timeout fix)
- Security & anti-detection measures
- Performance characteristics
- Deployment architecture
- Error handling strategy

**Start here if:** You need to understand how data flows through the system

---

### 3. Code Standards (544 lines)
**Purpose:** Development guidelines and best practices

**Contains:**
- Complete directory structure with annotations
- Rust workspace organization
- Naming conventions (Rust, TypeScript, Svelte)
- Code organization principles
- Error handling & logging patterns
- 5 critical component walkthroughs:
  - Anti-bot client (with timeout fix explanation)
  - YouTube n-transform module (algorithm & regex)
  - YouTube extractor (dual strategy)
  - GPU pipeline (hardware acceleration)
  - fMP4 muxer (container format)
- Testing & quality standards
- Performance optimizations
- Security practices
- Build & compilation
- Common pitfalls & solutions
- Deployment checklist

**Start here if:** You're implementing features or fixing bugs

---

### 4. Project Overview & PDR (510 lines)
**Purpose:** Product requirements and roadmap

**Contains:**
- Executive summary & product vision
- Target users & platform support matrix
- 8 functional requirements with acceptance criteria
- 6 non-functional requirements (performance, reliability, etc.)
- Architecture & technical constraints
- 8 implementation phases with status
- Recent changes (2026-02-23):
  - YouTube n-parameter transform (new feature)
  - Download timeout fix (critical bug)
- Success metrics & KPIs
- Risk assessment & mitigation
- Dependencies & external services
- 4-quarter roadmap (Q1-Q4 2026)
- Known limitations
- Testing strategy
- Compliance & legal considerations
- Version history

**Start here if:** You need to understand requirements or plan features

---

## Key Changes (2026-02-23)

### 1. YouTube N-Parameter Transform
**File:** `extractors/youtube-n-transform.ts` (NEW)

Downloads YouTube videos at full CDN speed instead of being throttled to 100 KB/s.

**Documented in:**
- Codebase Summary → Recent Changes
- System Architecture → N-Transform Flow Diagram
- Code Standards → YouTube N-Transform Module
- Project Overview → Phase 8.1

### 2. Download Timeout Bug Fix
**File:** `crates/proxy/src/anti_bot.rs` (Line 99)

Changed `.timeout(30s)` to `.connect_timeout(30s)` so downloads don't timeout mid-transfer.

**Documented in:**
- Codebase Summary → Recent Changes
- System Architecture → Request/Response Flow
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
- **File:** `/plans/reports/docs-manager-260223-1357-post-implementation-documentation-update.md`
- **Date:** 2026-02-23
- **Status:** Complete ✅

### Report Contents
- All documentation created and verified
- All code changes documented
- Cross-referenced to source code
- Quality assurance checklist

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
- Main Plan: `/plans/260222-1238-video-downloader/plan.md`
- N-Param Fix: `/plans/260223-1345-youtube-download-timeout-and-n-param-fix/plan.md`

## Getting Help

1. **Understanding Architecture:** Read System Architecture document
2. **Implementation Questions:** Check Code Standards for patterns
3. **Requirements Tracing:** See Project Overview & PDR
4. **Deployment Issues:** Check Project Overview → Deployment Strategy
5. **Performance Tuning:** See System Architecture → Performance Characteristics

---

**Documentation Version:** 2.0
**Last Generated:** 2026-02-23 13:57 UTC
**Status:** Complete ✅

For the latest updates, check `/plans/reports/` for implementation reports.
