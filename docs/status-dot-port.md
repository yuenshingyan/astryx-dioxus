# StatusDot implementation and verification

## Authority and approved decisions

Pinned source: Meta Astryx [`476a3153417dbf75c565f7df05b0784e94ddd59e`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e), `packages/core/src/StatusDot/{StatusDot.tsx,StatusDot.doc.mjs,StatusDot.test.tsx,index.ts}`, the authoritative [StatusDot specification](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/docs/specs/components/StatusDot/StatusDot.spec.md), all four original templates, shared core/Neutral tokens and native Stack/Text owners. Tooltip/useTooltip, its specification and shared hit-area owners were inspected before implementing FR5/AR2.

Two paint decisions were explicitly approved for this port:

1. Otherwise-unpainted custom variants have the **neutral plate/ink baseline**, rather than the pinned transparent gap. Their real variant name remains reflected for theme overrides; no default variant is invented.
2. **All variants** receive the source status variants' 1px CanvasText outline, offset -1px, in forced colors. This resolves the original accent/neutral disappearance question, including custom variants.

The fixture keeps the unmodified source evidence; these improvements are tested separately, not presented as source-equivalent behavior. Normal built-in colors and the original pulse are unchanged. The authoritative spec's required standalone tooltip toggle/host delegation are implemented as spec-led corrections to the pinned implementation's documented FR5/AR2 gaps. They were not observed as shipped catalog functionality.

## Ownership and public contract

- `src/status_dot/` owns rendering, validated open tone names, icon content, root role/name decisions, native placement/host-name policy and the renderer activation adapter.
- `src/tooltip/` owns the shared surface, hover/focus preview, imperative show/hide, pin lifetime, dismissal and one-tip arbitration. It accepts a component-owned policy capability; it does not import StatusDot or own its role decisions.
- `src/utils/hit_area.{rs,js}` and `touch_hit_area.{rs,js}` retain separate original measurement-watch and ref-counted-consumer owners. No combined control context is introduced.
- Pin announcements consume the existing `hooks/use_announce/` polite channel, including native-modal routing/withdrawal. No duplicate live-region implementation moves under Tooltip or StatusDot.
- Shared tokens/styles remain in `assets/astryx.css`; examples remain showcase-owned. There are no new dependencies, suppressions, existing public API breaks, or ownership moves from upstream. Supporting Tooltip/Theme/announcement/hit-area work does **not** complete their independent public migrations.

`StatusDot` requires `variant: StatusDotVariant` and nonempty `label: String`. The five built-in constants are SUCCESS, WARNING, ERROR, ACCENT and NEUTRAL; there is no Default implementation. `new(name)` validates an extensible ASCII identifier without rewriting it, returning InvalidStatusDotVariant for invalid input. Missing required props fail compilation; whitespace-only labels return recoverable StatusDotError::EmptyLabel rather than mounting an unnamed mark. This is explicit Rust boundary validation, not identical invalid-input output to unchecked React props.

The root is one fixed 8×8 CSS-pixel span, inline-flex (blockified as a flex item), centered, circular and non-shrinking, with only the canonical `.astryx-status-dot` target and public data-variant axis. No `astryx-statusdot` alias, size enum, built-in glyph, progress role, live region, or status subscription is added. Native role/aria-label/aria-labelledby/tabindex and other DOM/data/ARIA/event inputs pass through; caller class/style merge after component styles. Private identity/pulse/placement/hit-area metadata cannot be forged through raw attributes. Caller semantic overrides remain caller-owned and can change the default role/name.

`StatusDotIcon::{Empty,Text,Rich}` adapts ReactNode: booleans, absent options and empty scalar strings create no wrapper; zero and exact integer text remain renderable. Opaque components/fragments stay opaque even if their children render nothing, as with a React element. Other renderable values can be supplied as Dioxus elements. The 8px flex wrapper is aria-hidden with line-height zero; SVG/component artwork, sizing and currentColor use remain caller-owned. No default icon is chosen per tone.

An accessible name alone does not make color distinguishable to sighted color-blind users. Applications must add visible text, distinct per-status shapes or another non-color carrier; using the same icon for every status does not solve color-only meaning.

## Plate/ink and pulse

Success/warning/error use the **status icon roles**, not strong fill roles, with background-surface as ink. Neutral uses icon-secondary with surface ink; accent uses accent/on-accent. The Neutral scope rebinds semantic icon aliases where its text-role overrides live; inherited root custom-property aliases would otherwise retain core colors. Core and Neutral values remain distinct and source-derived.

`is_pulsing` defaults false. When true, opacity is 1 → .5 → 1 over 2 seconds, ease-in-out, indefinitely; direction normal, delay zero, fill none. CSS applies easing to each half-cycle; WAAPI reports effect easing linear and per-keyframe ease-in-out. Reduced motion removes the animation and rests at opacity 1. Forced colors does not itself stop the pulse. The hidden hit band changes no paint geometry.

## Tooltip and native placement

Absent/empty/whitespace-only tooltip values attach no browser controller, observer, listener or timer. The internal Dioxus mount lifecycle is still remembered so an optional tooltip can be added later without replacing the span. Tooltip text is an explanation, never a replacement for the required status name.

With an explanation, native mount resolves actual ancestors, including native buttons/links and ARIA button/link/menuitem/option hosts:

- **Standalone:** the same span becomes a named role-button with default tabindex 0 and aria-expanded. Hover/focus previews; click/tap/Enter/Space pins or unpins. Escape, outside press and focus departure dismiss through the shared manager. A pin survives pointer leave and competing hover. Caller event builders run first; prevent-default cancels the pin. Keyboard repeat/composition are guarded, Space does not scroll, and a standalone pin press cannot activate a clickable Dioxus ancestor.
- **Inside a control:** the span remains an image with no automatic tab stop; the tooltip previews on hover only, not host focus, tap or press. Explanation text contributes once to the host's content-derived accessible name through an offscreen span outside the image. Equal contributions share one ref-counted node; existing names and hidden aria-labelledby references dedupe correctly. Host aria-label/aria-labelledby remain untouched; those callers must include the status themselves.

Actual ancestry, role/href changes, explanation/option updates and removal are reconciled. Component-owned generated attributes restore without overwriting explicit caller props or later native caller mutations. Host-name contributions, surfaces, observers, frames and hit holders release on drop. Component policy capabilities cross renderer eval scopes explicitly, so an earlier Button-created shared manager does not capture missing StatusDot utilities.

Surfaces are created lazily outside the interactive host (body, or the enclosing native modal dialog); no pre-rendered div is placed inside a button/link, and showing a tooltip cannot duplicate the host's accessible name. Theme paint/font and color-scheme are copied when shown. A held surface can update its text without losing the pin; pin announcements use rendered surface text through the existing polite owner. Full public Tooltip/theme/provider behavior remains independent.

On coarse pointers, the independent shared watch grows an invisible band toward 44px without changing the visible 8px dot. Bands clamp to neighbor midpoints and stop growth for already overlapping native boxes. Geometry is rechecked before accepting a band click, never redirected into another action. Measurements batch reads before writes, observe meaningful layout changes rather than pointermove, account for scale and caller borders, and ignore only their own generated style writes. Root transforms/style changes still invalidate bounds. Plain/nested/disabled/inert dots have no active expansion; holders restore prior values when released.

**SSR adaptation:** tooltip-bearing dots start as named images without generated tabstop/expanded state or a surface until actual native placement is known. First-client rendering uses the same snapshot. This avoids nested controls before discovering an arbitrary native ancestor, but a JavaScript-disabled server document does not gain the interactive tooltip. Separate SSR and browser behavior are tested; a complete hydration application is not claimed.

## Four original patterns and actual evidence

`examples/showcase/status_dot.rs` includes all 19 source controls in original order:

| Pattern | Count | Source content |
| --- | --- | --- |
| StatusDotShowcase | 5 | success/warning/error/accent/neutral; Positive/Warning/Negative/Info/Neutral |
| StatusDotPulsing | 5 | Live/Processing/Error/Processing/Error; repeated labels preserved by identity |
| StatusDotStatusIndicators | 4 | Online/Away/Offline/Unknown beside original body Text |
| StatusDotVariants | 5 | all five plain tones |

Native horizontal frames are 72×8 with 8px gaps. The labeled column is 75.046875×104 at 16px root, with 20px rows, 8px vertical gaps, centered dots and Figtree body text. The actual muted Showcase backdrop is preserved. Native layout framing does not claim a Layout/Text migration or replicate the entire documentation-site shell.

`tests/fixtures/status-dot-neutral-reference.json` records **38 actual catalog observations**, Neutral light/dark, with 23 source blobs verified against the pin, exact root/plate/ink/frame/text-fragment geometry and animation/media evidence. All four originals have no icon or tooltip; additional examples test those separately. Natural animation clocks were observed before pausing existing effects at time zero for deterministic comparisons. The catalog advertises v0.7.16, not a verified identical build of the pin.

`tests/status_dot_metrics.mjs` uses the same self-contained DOM measurements and compact decoders offline. Tests compare selected semantic attributes, every control's native geometry/plate/ink, true compositing backdrop, native frame/text rectangles, delay/duration/keyframes and actual easing samples. Geometry tolerance is 1/64px. Private data markers, generated animation names, raw style/reset spelling and unused zero-width border/outline properties are not golden public APIs. Figtree source URL/hash/declarations remain provenance; duplicated font bytes are omitted because tests use the already-bundled local showcase asset. No live service is required by regression tests.

## Verification

Run all five standard checks and:

```sh
node tests/status_dot.browser.mjs
node tests/status_dot_tooltip.browser.mjs
node tests/button.browser.mjs
node tests/button_group.browser.mjs
node tests/icon_button.browser.mjs
node tests/link.browser.mjs
node tests/spinner.browser.mjs
node tests/badge.browser.mjs
node tests/divider.browser.mjs
node tests/kbd.browser.mjs
node tests/skeleton.browser.mjs
node tests/spinner_rotor.browser.mjs
node tests/announcement.browser.mjs
node tests/status_dot.contrast.mjs
```

Use the [existing Chrome/CDP setup](button-port.md#browser-checks). CDP_URL/SHOWCASE_URL override defaults. Rebuild the watch-disabled showcase after modifications. The contrast command intentionally exits 1 for the preserved findings below.

Rust tests cover mandatory props/name validation, open variant validation/reflection, the 30 tone×pulse×icon matrix, booleans/empty/zero/opaque fragments, native props/ref/event forwarding, caller semantic overrides/private metadata protection, stable updates and pending SSR shape. Two compile-fail doc tests enforce required tone/name.

Chrome checks cover all 38 originals and native frames, true pulse easing samples, static versus animated state, caller SVG field/ink/AX exclusion, fallback/theme override, source core roles, native callbacks and semantics, standalone keyboard/mouse/touch preview/pin/dismiss/cancellation, renderer propagation, nested native/ARIA hosts and AX names, lazy external surfaces, actual placement/explanation/option/drop changes, universal forced-color outlines, reduced motion, fixed size under enlarged root text, RTL, coarse-pointer expansion/midpoints, caller border compensation and transform/overlap invalidation. Native owner probes additionally cover reparenting, duplicate host-name ref counts/hidden references, caller attribute restoration, isolated imperative hide, held-text refresh, disabled/inert state and complete shared-watch cleanup. Existing nine component and renderer-boundary suites remain regressions.

## Contrast and verification limits

`node tests/status_dot.contrast.mjs` (optional --json) models source core/Neutral, light/dark, five tones on body/card/surface, with opacity 1 and the original pulse minimum .5. For group opacity, plate and glyph pixels each composite directly over the parent background before comparison; antialiasing is not modeled.

- **Resting:** 0/60 conditional essential-plate and 0/60 optional essential-glyph cases below 3:1.
- **Pulse minimum:** **33/60 plate and 33/60 glyph cases below 3:1**. The original colors and .5 pulse are retained, not silently changed or certified compliant. Adjacent visible text can make the mark decorative; color-only rows still require callsite interpretation. Custom theme paint/system palettes and perceptual recognition of tiny glyphs remain outside this arithmetic.

Actual screen-reader delivery, physical devices, real Windows High Contrast, Safari/Firefox, full SSR hydration, and a distinct StatusDot compositor trace remain unrun. Chrome AX/media/touch emulation do not replace those checks. Tooltip/theming behaviors beyond the required internal subset, including a public controlled Tooltip or full provider/tempo normalization, remain separate migrations. No unresolved design decision blocks this component after the two approvals; supporting parents remain unchecked.
