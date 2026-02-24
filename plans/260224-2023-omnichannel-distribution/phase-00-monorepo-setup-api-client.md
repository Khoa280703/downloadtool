# Phase 00: Monorepo Setup & API Client

## Context Links
- Parent plan: [plan.md](./plan.md)
- Backend: `backend/crates/api/src/main.rs`

## Overview

| Field | Value |
|-------|-------|
| Priority | P1 |
| Status | ✅ completed |
| Description | Initialize pnpm workspace + generate type-safe TS API client from Rust backend via utoipa + openapi-ts |
| Effort | 2d |

## Key Insights

- pnpm workspaces: zero config beyond `pnpm-workspace.yaml` — no Turborepo yet
- utoipa annotates Rust handlers → generates `/openapi.json` at runtime
- openapi-ts reads `openapi.json` → generates `packages/api-client/src/` (types + fetch client)
- All apps (`web`, `extension`, `injector`) import from `@downloadtool/api-client` — type-safe, no duplication
- Regenerate client: run `generate.sh` whenever backend API changes

## Requirements

**Functional:**
- `pnpm install` from root installs all workspaces
- `GET /openapi.json` returns OpenAPI 3.x spec from Rust backend
- `packages/api-client/generate.sh` regenerates TS client from live spec
- All apps can import `@downloadtool/api-client`

**Non-functional:**
- Root `package.json` only contains workspace tooling (no app deps)
- `api-client` package has no runtime deps (generated types only)

## Architecture

```
backend (Rust + utoipa)
  └── GET /openapi.json → openapi.json

packages/api-client/
  ├── generate.sh          → curl /openapi.json | openapi-ts → src/
  ├── src/
  │   ├── types.gen.ts     → VideoInfo, Stream, ExtractRequest, etc.
  │   └── sdk.gen.ts       → typed fetch functions
  └── package.json         → name: "@downloadtool/api-client"

apps/web, apps/extension, apps/injector
  └── import { extractVideo } from '@downloadtool/api-client'
```

## Related Code Files

- Create: `pnpm-workspace.yaml`
- Create: `package.json` (root)
- Create: `packages/api-client/package.json`
- Create: `packages/api-client/generate.sh`
- Modify: `backend/crates/api/src/main.rs` — add utoipa, `/openapi.json` route
- Create: `backend/crates/api/src/routes/openapi.rs`

## Implementation Steps

1. **Create `pnpm-workspace.yaml`**
   ```yaml
   packages:
     - 'packages/*'
     - 'apps/*'
   ```

2. **Create root `package.json`**
   - `name`: `downloadtool-monorepo`
   - `private: true`
   - `engines.pnpm`: `>=9`
   - Scripts: `build:all`, `dev:web`, `dev:extension`

3. **Add utoipa to Rust backend**
   - Add `utoipa`, `utoipa-axum` to `backend/crates/api/Cargo.toml`
   - Annotate existing handlers with `#[utoipa::path(...)]`
   - Create `backend/crates/api/src/routes/openapi.rs`: `GET /openapi.json` returns `ApiDoc::openapi()` as JSON
   - Register route in `main.rs`

4. **Create `packages/api-client/package.json`**
   - `name`: `@downloadtool/api-client`
   - `version`: `0.0.1`
   - `main`: `src/index.ts`
   - `devDependencies`: `@hey-api/openapi-ts`

5. **Create `packages/api-client/generate.sh`**
   - Option A (requires running server): `curl http://localhost:3068/openapi.json -o openapi.json && pnpm openapi-ts ...`
   - Option B (recommended — no server needed): Create `backend/crates/api/src/bin/export_openapi.rs`:
     ```rust
     fn main() {
         let doc = ApiDoc::openapi();
         std::fs::write("../../packages/api-client/openapi.json", doc.to_pretty_json().unwrap()).unwrap();
     }
     ```
     Then: `"generate": "cargo run --manifest-path backend/Cargo.toml --bin export_openapi && pnpm openapi-ts ..."`
   - Commit generated files (no build step needed at runtime)

6. **Run initial generation**
   - Option A: Start backend `cargo run`, run `./generate.sh`
   - Option B: `pnpm generate` from root (no server needed, works in CI)
   - Verify `src/types.gen.ts` contains `ExtractRequest`, `VideoInfo`, `Stream`

7. **Create `packages/api-client/src/index.ts`**
   - Re-export everything from generated files

8. **Verify workspace linking**
   - Run `pnpm install` from root
   - Confirm `node_modules/@downloadtool/api-client` symlink exists in apps

## Todo List

- [ ] Create `pnpm-workspace.yaml`
- [ ] Create root `package.json`
- [ ] Add utoipa to Rust backend + `/openapi.json` route
- [ ] Create `packages/api-client/package.json`
- [ ] Create `packages/api-client/generate.sh`
- [ ] Run `generate.sh`, verify generated types
- [ ] Create `packages/api-client/src/index.ts`
- [ ] Run `pnpm install` from root, verify symlinks

## Success Criteria

- `GET /openapi.json` returns valid OpenAPI 3.x spec
- `generate.sh` produces `types.gen.ts` with correct types
- `pnpm install` resolves all workspace packages
- Any app can `import { ... } from '@downloadtool/api-client'`

## Risk Assessment

| Risk | Likelihood | Mitigation |
|------|-----------|------------|
| utoipa annotation effort on existing handlers | Medium | Annotate incrementally; existing tests still pass |
| openapi-ts generated code style conflicts | Low | Generated files are not hand-edited; regenerate as needed |

## Next Steps

- Phase 1: Bookmarklet via `apps/injector/` (uses api-client)
- Phase 2: PWA via `apps/web/` (uses api-client)
