# Section port

Section preserves the upstream two-box structure: sizing, caller styles, and native attributes and events belong to the outer `div`; the inner `div` owns paint, the reflected variant, dividers, and padding. Neither box adds a landmark or tab stop. The three built-in variants use existing theme tokens. Valid custom variants still reflect their name without painting a default background.

Per-edge padding overrides axis and uniform props. With no prop, theme edge and axis values override the nearest ancestor Section's explicit uniform padding, then the theme shorthand and spacing step 4 supply defaults. The painted box publishes the applied padding for descendants; nested wrappers escape the parent's inline inset and escape its top or bottom inset only at the corresponding child boundary. Overlay roots reset inherited Section padding. Public Layout and AppShell roots are not part of this port. The upstream `minHeight`-only limitation is preserved: the outer box can grow without stretching the inner paint.

The port follows Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`; 15 Section and shared Layout source, spec, test, and template blobs were verified against the pinned Git tree. `Section.tsx` has blob `65d38e593153e4da761b5e3fe851371311bc9dfd`; `Section.spec.md` has blob `23b0528c530c753b156ea92da79d71a2ae35bd7e`.

The showcase includes Variants, With Dividers, and Wash Highlight. SSR and Chrome checks cover wrapper ownership, variants, nesting and overlay reset, explicit height, and RTL divider and bleed behavior. The browser comparison checks source-derived behavior, not published screenshots. Other browsers and manual assistive technology were not tested; the migration checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8118 node tests/section.browser.mjs
```
