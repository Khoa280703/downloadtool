---
title: "Phase 03 - Below-fold Render and Bundle Tuning"
status: completed
priority: P2
effort: 15m
---

# Phase 03 - Below-fold Render and Bundle Tuning

## Context Links

- `frontend/src/routes/+page.svelte`

## Overview

Tinh chỉnh tải tài nguyên dưới fold để giảm tranh chấp băng thông với phần trên fold.

## Implementation

1. Giữ `content-visibility` cho section dài (`.defer-section`) để giảm render cost ban đầu.
2. Thêm `fetchpriority="low"` cho ảnh dưới fold đã dùng `loading="lazy"`.
3. Giữ nguyên hành vi tương tác ở vùng fetch đầu trang (không delay input/fetch).

## Success Criteria

- Ảnh dưới fold không cạnh tranh tải với tài nguyên quan trọng.
- Không thay đổi bố cục/flow người dùng.
