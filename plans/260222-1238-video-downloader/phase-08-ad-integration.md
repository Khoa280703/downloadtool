# Phase 08 — Ad Integration + Monetization

## Context
- Plan: [plan.md](plan.md)
- Prev: [phase-07-frontend.md](phase-07-frontend.md)

## Overview
- **Priority**: P1
- **Status**: pending
- **Effort**: 1d
- Integrate ad networks for banner + interstitial revenue. Maximize RPM without destroying UX. Setup analytics for conversion tracking.

## Key Insights
- Google AdSense: easiest approval, $1-3 RPM for downloader niche; requires site review
- AdsTerra/PropellerAds: higher RPM ($3-8) for download sites, instant approval
- Placement strategy: above-the-fold banner + between-step interstitial = highest CTR
- Avoid pop-unders on iOS (blocked by Safari); use in-page push or sticky banners
- Ad networks often block on localhost — test with deployed domain

## Architecture

```
Ad placements:
  ├── Header banner (728x90 desktop / 320x50 mobile)  — always visible
  ├── Between steps: after extract → before download  — interstitial/square
  └── Footer banner (sticky on mobile)

Analytics:
  ├── Google Analytics 4 (page views, events)
  └── Custom events: url_submitted, download_started, format_selected
```

## Related Code Files
- `frontend/src/components/AdBanner.svelte` — reusable ad slot component
- `frontend/src/lib/analytics.ts` — GA4 event helpers
- `frontend/src/routes/+layout.svelte` — global ad + analytics injection

## Implementation Steps

1. **Ad network selection** — prioritized order:
   - Primary: **AdsTerra** (instant approval, downloader-friendly, high RPM)
   - Secondary: **Google AdSense** (apply after 1 month traffic)
   - Fallback: **PropellerAds** or **Monetag**

2. **AdBanner.svelte** — lazy-loaded, SSR-safe
   ```svelte
   <script>
     import { onMount } from 'svelte';
     export let slot: string;  // ad slot ID
     export let size: '728x90' | '320x50' | '300x250';
     let mounted = false;
     onMount(() => { mounted = true; });  // prevent SSR hydration mismatch
   </script>
   {#if mounted}
     <div class="ad-container ad-{size}">
       <!-- ad script injected here -->
     </div>
   {/if}
   ```

3. **Ad placement layout** (`+layout.svelte`)
   - Top of page: `<AdBanner slot="header" size="728x90" />`
   - Mobile: `<AdBanner slot="mobile-top" size="320x50" />`
   - After format selection: `<AdBanner slot="mid" size="300x250" />`

4. **Interstitial between steps**
   - After user clicks "Extract" → show 3s countdown + ad before showing format picker
   - Skip button after 3s (UX balance)
   - Implement as Svelte modal with countdown store

5. **Google Analytics 4** (`src/lib/analytics.ts`)
   ```typescript
   export const trackEvent = (name: string, params?: Record<string, string>) => {
     if (typeof gtag !== 'undefined') gtag('event', name, params);
   };
   // Usage: trackEvent('download_started', { platform: 'youtube', quality: '1080p' })
   ```
   - Events: `url_submitted`, `extract_success`, `extract_error`, `download_started`, `format_selected`

6. **GA4 script** (`+layout.svelte` head)
   - Inject via `<svelte:head>` using env var `PUBLIC_GA_MEASUREMENT_ID`
   - Lazy load: `async` attribute, no blocking

7. **Ad policy compliance**
   - `/ads.txt` file at domain root (required by all networks)
   - Privacy policy page (required for AdSense)
   - Cookie consent banner (GDPR for EU traffic) — use `svelte-cookie-consent`

8. **Revenue optimization**
   - A/B test ad positions (track via GA4 custom dimensions)
   - Monitor RPM weekly; swap low-RPM slots
   - Blacklist: no ads on `/api/*` routes

## Todo
- [ ] Register AdsTerra account + get ad codes
- [ ] AdBanner.svelte component (SSR-safe, lazy)
- [ ] Header + mid-page ad placements
- [ ] 3s interstitial countdown between extract→format steps
- [ ] GA4 setup + event tracking
- [ ] ads.txt file
- [ ] Privacy policy page
- [ ] Cookie consent banner (GDPR)
- [ ] Test ad rendering on deployed Cloudflare Pages domain

## Success Criteria
- Ads load without blocking page render (async)
- RPM ≥ $2 within first month
- No CLS (layout shift) from ad loading → reserve space with fixed-size containers
- AdSense/AdsTerra approval within 2 weeks of launch

## Risk Assessment
| Risk | Mitigation |
|---|---|
| AdSense rejection (ToS — downloader content) | Use AdsTerra/PropellerAds as primary |
| Ad blockers reduce revenue | Acceptable; don't fight adblockers (bad UX) |
| GDPR fines | Proper consent banner; don't load ads without consent |
| Invalid clicks / ad fraud | Let ad network handle; don't artificially click own ads |

## Security
- Ad scripts from CDN only (no self-hosted ad JS)
- CSP: add ad network domains to `script-src` allowlist
- Never inject user-controlled content near ad slots (XSS risk)
