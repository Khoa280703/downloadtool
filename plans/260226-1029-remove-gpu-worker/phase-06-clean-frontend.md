# Phase 06 — Clean Frontend

## Overview
- **Priority:** P3
- **Status:** pending
- **ETA:** 10m
- **Depends on:** none (independent)

## File to Modify

`/home/khoa2807/working-sources/downloadtool/frontend/src/components/FormatPicker.svelte`

---

## Changes Required

### Remove "Add branding (GPU)" toggle

The toggle is a disabled `<label>` block at the bottom of the template (lines 215–220):

```svelte
<label class="toggle-option">
    <input type="checkbox" disabled />
    <span class="toggle-slider"></span>
    <span class="toggle-label">Add branding (GPU)</span>
    <span class="coming-soon">Soon</span>
</label>
```

Remove this entire `<label>` block.

### Remove orphaned CSS rules

The following style rules are only used by the toggle — remove them all from `<style>`:

```css
.toggle-option { ... }
.toggle-option input { display: none; }
.toggle-slider { ... }
.toggle-slider::after { ... }
.toggle-option input:checked + .toggle-slider { ... }
.toggle-option input:checked + .toggle-slider::after { ... }
.toggle-label { ... }
.coming-soon { ... }
```

That's 8 CSS rule blocks. Removing them reduces the file by ~55 lines.

---

## Scan for Other GPU/Transcode References

Run before implementing:

```bash
grep -r 'transcode\|gpu\|GPU\|branding' frontend/src/
```

Expected: only the FormatPicker hit. No transcode API calls exist in the frontend (confirmed — no `/api/transcode` fetch calls found in codebase).

---

## Implementation Steps

1. Open `frontend/src/components/FormatPicker.svelte`
2. Delete the `<label class="toggle-option">` block in the template section
3. Delete all 8 orphaned CSS rule blocks from `<style>`
4. Run `pnpm --filter frontend check` (or `svelte-check`) to confirm no type errors

## Success Criteria
- No reference to `GPU`, `branding`, `toggle-option`, `toggle-slider`, `toggle-label`, `coming-soon` remains in `FormatPicker.svelte`
- `svelte-check` passes with zero errors/warnings introduced by this change
