# Plan: i18n - Paraglide JS + Translation Pipeline

**Status:** completed (implementation scope)  
**Created:** 2026-03-01  
**Goal:** SEO-focused i18n with Paraglide, locale-prefixed routes, hreflang/sitemap, language switcher, and production-safe checks

## Context

- Framework: SvelteKit + adapter-node
- Locale model: `en` is default (no prefix), non-`en` use `/{locale}/...`
- Current locale set: **33 locales total** (`en` + 32 non-en)
- Translation pipeline: parallel sub-agent workflow (no Claude runtime integration in app path)
- SEO hreflang: injected in `hooks.server.ts` for SSR pages + static hreflang in prerendered homepage head

## Phases

| Phase | Status | Description |
|-------|--------|-------------|
| [Phase 1](phase-01-install-paraglide.md) | completed | Install & configure Paraglide |
| [Phase 2](phase-02-extract-strings.md) | completed | Extract hardcoded UI strings to `messages/en.json` |
| [Phase 3](phase-03-translate-with-claude.md) | completed | Base translation files + translation polish for newly added keys |
| [Phase 4](phase-04-seo-hreflang-sitemap.md) | completed | hreflang injection + sitemap + robots |
| [Phase 5](phase-05-language-switcher.md) | completed | Language switcher + preferred language persistence |
| [Phase 6](phase-06-test-deploy.md) | completed | `check`/`build` + preview curl validations |

## Locale Set (33)

`ar, bg, cs, da, de, el, en, es, et, fi, fr, hu, id, it, ja, ko, lt, lv, nb, nl, pl, pt, pt-BR, ro, ru, sk, sl, sv, tr, uk, vi, zh, zh-TW`

## Residual Risks

1. Legal text quality should be reviewed manually for target markets before production SEO push.
2. Full manual UI spot-check across representative locales (`vi`, `ja`, `ar`, `de`, `zh`, `zh-TW`) is still recommended after deploy.
3. Post-deploy checks (Search Console + Lighthouse) remain manual.
