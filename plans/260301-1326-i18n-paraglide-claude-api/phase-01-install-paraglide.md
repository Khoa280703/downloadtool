# Phase 1: Install & Configure Paraglide JS

**Status:** completed | **Priority:** critical (blocks all other phases)

## Overview

Install `@inlang/paraglide-js`, wire `hooks.server.ts` + `hooks.ts` + `vite.config.ts`, then expand language list from 1 to 33 locales total.

## Implementation Notes (2026-03-01)

- Installed Paraglide via `sv add`.
- Kept existing Better Auth handle logic intact, wrapped with `sequence(handleParaglide, existingHandle)`.
- Added route strategy exclude for `/api/:path(.*)?` and `/health` so API paths are not locale-wrapped.
- Added compile script (`paraglide:compile`) and chained it into `dev`, `check`, `build` to avoid missing `$lib/paraglide/*` on fresh machines.
- `pnpm --filter frontend check` passes.

## Implementation Steps

### 1.1 Install via CLI

```bash
cd frontend
npx sv add paraglide
# CLI will ask for languages — enter: en (start with just English)
# Auto-modifies: svelte.config.js, src/hooks.server.ts, src/routes/+layout.svelte
# Creates: messages/en.json, project.inlang/settings.json
```

### 1.2 Verify auto-generated files

**vite.config.ts** should have:
```js
import { paraglideVitePlugin } from '@inlang/paraglide-js';

export default defineConfig({
  plugins: [
    sveltekit(),
    paraglideVitePlugin({ project: './project.inlang', outdir: './src/lib/paraglide' })
  ]
});
```

**src/hooks.server.ts** should have `paraglideMiddleware()` integrated into the handle chain (use `sequence()` if merging with existing better-auth handle).

**src/hooks.ts** should export `reroute` with `deLocalizeUrl()`.

### 1.3 Update project.inlang/settings.json with all 33 locales

```json
{
  "$schema": "https://inlang.com/schema/project-settings",
  "baseLocale": "en",
  "locales": [
    "ar", "bg", "cs", "da", "de", "el", "en", "es", "et", "fi",
    "fr", "hu", "id", "it", "ja", "ko", "lt", "lv", "nb", "nl",
    "pl", "pt", "pt-BR", "ro", "ru", "sk", "sl", "sv", "tr", "uk",
    "vi", "zh", "zh-TW"
  ],
  "modules": ["https://cdn.jsdelivr.net/npm/@inlang/plugin-message-format@4/dist/index.js"],
  "plugin.inlang.messageFormat": {
    "pathPattern": "./messages/{locale}.json"
  }
}
```

### 1.4 Merge Paraglide handle with existing hooks.server.ts

> **WARNING:** The existing `hooks.server.ts` contains complex logic that must be preserved intact:
> - Cookie-based session check (`getSession`)
> - `/login` redirect for unauthenticated routes
> - `svelteKitHandler` (better-auth)
> - `cache-control` headers injection
>
> **DO NOT** destructure or refactor the existing handle function. Wrap it wholesale using `sequence()`.

```ts
// src/hooks.server.ts
import { sequence } from '@sveltejs/kit/hooks';
import { paraglideMiddleware } from '$lib/paraglide/server';
import type { Handle } from '@sveltejs/kit';

// Keep ALL existing handle logic — cookie check, getSession, /login redirect,
// svelteKitHandler, cache-control headers — as a single named handle:
const existingHandle: Handle = async ({ event, resolve }) => {
  // ... entire previous handle body goes here unchanged ...
};

export const handle = sequence(
  handleParaglide,
  existingHandle  // preserves ALL existing logic intact
);
```

> **NOTE on API routes:** Verify that middleware config does NOT add locale prefix to `/api/*` routes.
> Check Paraglide config — if needed, add `exclude: [/^\/api\//]` to the i18n config so `/api/auth/*` and `/api/proxy/*` are never locale-wrapped.

### 1.5 Verify dev server starts without errors

```bash
pnpm dev
# Check: localhost:5168 loads, no TS errors
```

## Files Modified

- `svelte.config.js` (auto)
- `vite.config.ts` (auto — paraglide vite plugin)
- `src/hooks.server.ts` (merge i18n.handle() with sequence())
- `src/routes/+layout.svelte` (auto — ParaglideJS wrapper)
- `project.inlang/settings.json` (expand language list)
- `messages/en.json` (auto-created, empty)
- `src/lib/paraglide/` (auto-generated, gitignore this)

## Success Criteria

- [x] `pnpm dev` starts without errors
- [x] `pnpm check` 0 errors
- [x] `localhost:5168/vi` resolves correctly
- [x] `/api/auth/get-session` still returns JSON (not locale-wrapped)
- [x] `/api/proxy/*` routes unaffected by i18n middleware strategy
- [x] `messages/en.json` exists
