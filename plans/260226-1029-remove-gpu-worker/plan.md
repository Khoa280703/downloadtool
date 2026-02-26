---
title: "Remove GPU Worker Infrastructure"
description: "Delete gpu-worker/gpu-pipeline crates and all GPU-related code, config, and Docker artifacts (YAGNI)."
status: pending
priority: P2
effort: 1.5h
branch: main
tags: [cleanup, rust, docker, yagni]
created: 2026-02-26
---

# Remove GPU Worker Infrastructure

## Overview

Remove all GPU worker infrastructure — crates, routes, Docker, CI steps, frontend toggle — since the watermark feature was dropped and there is no practical use case.

## Phases

| # | Phase | Status | ETA |
|---|-------|--------|-----|
| 1 | [Delete GPU crates & proto](./phase-01-delete-gpu-crates.md) | pending | 15m |
| 2 | [Clean workspace Cargo.toml](./phase-02-clean-workspace-cargo.md) | pending | 10m |
| 3 | [Clean crates/api](./phase-03-clean-api-crate.md) | pending | 20m |
| 4 | [Clean Docker & Compose](./phase-04-clean-docker.md) | pending | 15m |
| 5 | [Clean CI/CD](./phase-05-clean-ci.md) | pending | 10m |
| 6 | [Clean Frontend](./phase-06-clean-frontend.md) | pending | 10m |
| 7 | [Verify build & tests](./phase-07-verify.md) | pending | 20m |

## Key Dependencies

- Phase 3 depends on Phase 1 (gpu-pipeline crate must be gone before removing the feature flag reference)
- Phase 7 must be last — full compile + test run to confirm clean removal
