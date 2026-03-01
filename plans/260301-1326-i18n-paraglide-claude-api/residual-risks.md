# Residual Risks — i18n Paraglide Implementation

**Created:** 2026-03-01

---

## Risk 1: OAuth/API Routes Getting Locale Prefix

**Severity:** Critical
**Routes at risk:** `/api/auth/*`, `/api/proxy/*`

Paraglide's `i18n.handle()` intercepts requests and may attempt to locale-wrap API routes if not configured to exclude them.

**Symptoms if triggered:**
- better-auth OAuth callbacks fail (redirect URI mismatch)
- Proxy requests return 404 or redirect loops
- Session cookies set on wrong path

**Mitigation:**
- Check Paraglide's `i18n` config for an `exclude` option. If available, add:
  ```ts
  // src/lib/i18n.ts
  export const i18n = createI18n(routing, {
    exclude: [/^\/api\//]
  });
  ```
- After merging `i18n.handle()` into `hooks.server.ts`, verify manually:
  ```bash
  curl -I http://localhost:5168/api/auth/get-session
  # Expect: 200 JSON response, NOT a redirect to /en/api/auth/get-session
  ```
- Add to Phase 1 success criteria (already done).

---

## Risk 2: hreflang Not Present in Prerendered Homepage HTML

**Severity:** High (SEO impact)

`transformPageChunk` in `hooks.server.ts` only runs for SSR responses. Prerendered pages are served as static HTML — hooks are bypassed entirely at request time.

**Verification method:**
```bash
# After deploy, fetch the raw static file — NOT through SSR
curl https://download.khoadangbui.online/ | grep 'hreflang'
# Must show all 33 locale alternate links (+ x-default)

# Also verify SSR routes work via hooks injection (use /privacy since /vi/ is prerendered):
curl https://download.khoadangbui.online/vi/privacy | grep 'hreflang'
```

**Note:** The homepage `/vi/` is prerendered (static HTML), so hreflang is set statically in `+page.svelte`'s `<svelte:head>` (Phase 4.5). SSR routes like `/vi/privacy` are tested via hooks injection.

**Mitigation:**
- Section 4.5 in phase-04 adds hardcoded `<link>` tags directly in `+page.svelte`'s `<svelte:head>`.
- These are compiled into the static HTML at build time — no runtime dependency.
- If domain changes, update the hardcoded URLs in `+page.svelte` AND `static/robots.txt`.

---

## Risk 3: Translation Key Parity Drift (CI Guard)

**Severity:** Medium
**When:** If `messages/en.json` is updated after initial translation run, other language files may become stale (missing keys → runtime errors or silent fallback).

**Mitigation — Add CI check script:**

Create `scripts/check-message-key-parity.ts`:

```ts
import fs from 'fs';
import path from 'path';

const MESSAGES_DIR = path.resolve('messages');
const sourceFile = path.join(MESSAGES_DIR, 'en.json');
const source: Record<string, string> = JSON.parse(fs.readFileSync(sourceFile, 'utf-8'));
const sourceKeys = new Set(Object.keys(source));

let failed = false;

for (const file of fs.readdirSync(MESSAGES_DIR)) {
  if (file === 'en.json' || !file.endsWith('.json')) continue;
  const lang = file.replace('.json', '');
  const target: Record<string, string> = JSON.parse(
    fs.readFileSync(path.join(MESSAGES_DIR, file), 'utf-8')
  );
  const targetKeys = new Set(Object.keys(target));

  const missing = [...sourceKeys].filter((k) => !targetKeys.has(k));
  const extra = [...targetKeys].filter((k) => !sourceKeys.has(k));

  if (missing.length || extra.length) {
    console.error(`[${lang}] Key mismatch:`);
    if (missing.length) console.error(`  Missing: ${missing.join(', ')}`);
    if (extra.length) console.error(`  Extra: ${extra.join(', ')}`);
    failed = true;
  }
}

if (failed) {
  console.error('\nFix: re-run translate script for affected languages.');
  process.exit(1);
} else {
  console.log('All message files have matching keys.');
}
```

**Add to package.json scripts:**
```json
{
  "scripts": {
    "check:i18n": "tsx scripts/check-message-key-parity.ts",
    "prebuild": "pnpm check:i18n"
  }
}
```

Running `pnpm build` will now fail fast if any language file has key drift — preventing silent translation gaps from reaching production.
