# Badge implementation and verification

## Source and ownership

Baseline: Meta Astryx [`476a3153417dbf75c565f7df05b0784e94ddd59e`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e), `packages/core/src/Badge/{Badge.tsx,Badge.doc.mjs,Badge.test.tsx,index.ts}`, the Badge specification/layout/larger-text tests, and all four named templates. The Neutral theme's Badge overrides and actual catalog examples were also inspected.

- `src/badge/` owns the component, theme-extensible variant name and label alternatives.
- `src/utils/text_direction.rs` owns the original plain-text direction adapter.
- `src/i18n/first_strong_direction.rs` owns only the required strong-RTL predicate, not a placeholder first-strong API or a complete i18n provider.
- Shared tokens/styles remain in `assets/astryx.css`. New Badge styles use the existing base/theme layer order; no global CSS/layout reorganization, dependency, suppression, or breaking public API was introduced.

## Contract and Rust adapters

`Badge` requires a `label`. `BadgeLabel` represents Text, Number, Rich, or Empty content. Strings and numeric literals convert at the prop builder; integers retain exact decimal precision instead of being cast through f64. Floating numbers use the source's scalar display conventions, including zero, non-finite values and scientific-notation boundaries. Rich nodes are opaque and are never flattened to guess text, visibility, direction, title or semantics. Explicit Empty/boolean conversions represent non-rendering scalar slots; an empty rich fragment remains shallowly renderable, as upstream specifies.

`BadgeVariant` is an open validated name, not a closed enum pretending that themes cannot extend it. Its 14 built-in constants are Neutral (default), Info, Success, Warning, Error, Blue, Cyan, Green, Orange, Pink, Purple, Red, Teal and Yellow. `BadgeVariant::new` accepts nonempty ASCII identifiers containing letters/digits/hyphens/underscores and returns `InvalidBadgeVariant` otherwise, without trimming or rewriting them. Custom values reflect their original name and receive geometry but no built-in fill/color until a theme paints them. Neutral's original gray extension is supported as a theme value; it is not added as a fifteenth built-in.

The root is a single read-only span, inline-flex, centered, content-width, capped to its container and allowed to shrink. Its label span is isolated, clipped and ellipsized without wrapping. Fixed height follows spacing-5 (20px); a renderable label adds a one-line minimum so larger supporting text grows the pill. Empty scalar/color-only badges keep their original height. Root box-sizing locally reproduces the original site's border-box environment without changing the application's global reset.

A nonempty string or any number supplies the complete native title, clipped or not. Empty/rich labels supply no derived title; caller title wins. Full label content remains in the DOM and accessibility tree. Caller classes/styles/DOM/ARIA/data/native event attributes reach the root; variant metadata is protected from raw conflicts. `onmounted` adapts the root ref and adds no mount handler unless supplied. Additional native event builders can be passed through attributes. Caller role/tabIndex also pass through; Badge itself adds neither.

The optional icon renders directly before the label. Badge does **not** impose icon size, add icon semantics, or hide/name it: these remain the supplied node's responsibility. An Icon may use the surrounding Icon size context or an explicit size. The badge owns no loading/size enum, action, business operation, live region, tooltip, press controller, effect, observer, measurement or timer. It is server-safe; no browser bridge is added.

## Direction

Plain string/number labels without any source strong-RTL character get dir=ltr, keeping weak text such as `https://`, `.com`, `+3`, and `@alice` in order under RTL. A label containing any matching RTL script character omits dir and inherits the surrounding paragraph direction, even if Latin letters precede it. This is deliberately not first-strong or dir=auto. Rich labels receive no inferred direction; callers use dir/bdi on their own nodes.

Rust std has no Unicode Script-property query. The shared predicate uses a compact range table derived from the exact pinned JavaScript expression's **eight** named Script properties, both direction marks and both supplementary intervals. Unlike the separate first-strong algorithm, this predicate does not skip script-specific digits/marks. Common/Inherited holes stay excluded.

Provenance: Chrome 157.0.8103.79 / V8 15.7.112.43, revision `6dc9bc3bdaea5c35ed6d41fceab2f7a3a2227c4f`. An exhaustive scan of all 1,112,064 Unicode scalar values yielded 36 ranges / 5,739 matches and FNV-1a-32 fingerprint 0x99FC0102. Colocated Rust tests reproduce count/fingerprint and exercise scripts, marks/digits, mixed text and every boundary/gap. This pins engine property semantics, not an independently asserted Unicode data version; future engine data updates require explicit regeneration/review. The source hash and the expression's constituents are recorded beside the table. No Unicode package dependency was added.

## Examples and appearance

`examples/showcase/badge.rs` ports all four named patterns:

- **Variants**: semantic row plus the original five-color row.
- **Colors**: Teams and Priority category groups, all nine category colors.
- **Counts**: Messages/Alerts/Completed/Pending with original values and supporting captions.
- **Status**: System status and Workflow rows.

Additional examples cover all 14 tones, a custom variant, numbers/zero/rich/empty labels, title overrides, icon ownership, native ref/events, static updates, grid/stack intrinsic sizing, crowded/narrow rows, large text and LTR/RTL content. Shared core color tokens and the implemented Neutral palette are preserved. Neutral specifically paints its neutral/gray badge from gray tokens rather than the general translucent neutral token; this matches the pinned theme override.

`tests/fixtures/badge-neutral-reference.json` freezes actual catalog evidence for all 28 named controls, light/dark. Tests compare text/title/direction, variant, display, width/height, fill/text paint, typography, gap/padding/radius, line-height minimum, width cap, wrapping, ellipsis and isolation. Width permits 1/32 CSS pixel; other compared fields are exact. First font family is Figtree; unused system fallback stacks are not asserted identical. Larger-text containment permits the browser's 1/64px line-box quantization. The catalog release is not asserted identical to the pinned commit, and tests do not contact live services.

## Verification

Run all five standard checks and:

```sh
node tests/badge.browser.mjs
node tests/button.browser.mjs
node tests/button_group.browser.mjs
node tests/icon_button.browser.mjs
node tests/link.browser.mjs
node tests/spinner.browser.mjs
node tests/spinner_rotor.browser.mjs
node tests/announcement.browser.mjs
node tests/badge.contrast.mjs
```

Use the existing [Chrome/CDP setup](button-port.md#browser-checks); CDP_URL and SHOWCASE_URL can override defaults. Rebuild the watch-disabled showcase before browser checks.

Rust tests cover the 56 built-in tone/content combinations, scalar formatting, exact large integers, empty/rich slot rules, escaped content, required-label compile failure, default/open/invalid variants, native title/ref/event/ARIA/state forwarding, direct icon order/semantics/sizing and plain-versus-rich direction. Shared classifier tests cover the complete scalar fingerprint and boundary corpus.

Browser assertions include all 28 original controls in both modes, all 14 tones, actual AX full text despite clipping, no invented role/tab/live/press machinery, caller icons, native ref/event callbacks, grid/stack/content width, constrained row/cell bounds, larger text versus empty slots, custom layered theme geometry, Neutral's gray extension, actual rendered character positions in both directions, silent static updates, mobile layout, and forced-color status outlines. The five existing component suites and two renderer-boundary suites remain regression checks.

## Contrast and source limitations

`node tests/badge.contrast.mjs` (optional --json) models all 14 standalone tones on core and showcase Neutral page/surface backgrounds, in light/dark: **0/112 text failures below 4.5:1 and 0/112 inherited-glyph failures below 3:1**. Static Badge has no hover/pressed paint to invent or audit. A boundary is not treated as essential when the label carries the meaning. Arbitrary custom themes, rich content, icon overrides and surrounding backgrounds remain caller-owned.

This standalone result does **not** replace the earlier [Badge-inside-Button composite audit](button-port.md#core-palette-contrast-conflict): its 267/672 findings involve different parent/control backgrounds and interaction states. Existing source colors and that accepted fidelity exception remain unchanged. Existing Button showcase counters are caller-supplied styled content; the canonical standalone Badge examples are the four patterns above.

The original source leaves clipped-title recovery pointer-only (Badge.spec.md AR4/OQ1; upstream #5585): keyboard/touch users cannot reveal the clipped tail through Badge itself, although the full text remains available to assistive technology. This port preserves the static API rather than silently resolving that open design decision by adding a focus stop or measured tooltip. Applications requiring visual recovery must expose the full value elsewhere or provide an explicit accessible disclosure. This is not a claim of complete accessibility compliance. Non-Chrome, physical-device and actual screen-reader verification remain unrun; Chrome AX checks are not a screen-reader certificate.
