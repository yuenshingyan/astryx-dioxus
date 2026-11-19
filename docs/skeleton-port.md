# Skeleton implementation and verification

## Baseline and ownership

Pinned authority: Meta Astryx [`476a3153417dbf75c565f7df05b0784e94ddd59e`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e), `packages/core/src/Skeleton/{Skeleton.tsx,Skeleton.doc.mjs,Skeleton.test.tsx,index.ts}`, `docs/specs/components/Skeleton/Skeleton.spec.md`, all four original templates, the original core/Neutral tokens and native Card/Stack owners. Motion duration/tempo owners were inspected before consuming their existing variable contract.

`src/skeleton/` owns the shape, dimensions, radius, paint, pulse and default accessibility exclusion. Shared tokens/styles remain in `assets/astryx.css`. Native template frames stay in showcase ownership, not a new partial Card/Layout module. No dependency, suppression, existing public API break, new context, or ownership move is introduced. Card, Layout, Theme and their shared normalization machinery are not independently marked migrated.

## Public contract and Rust adapters

`Skeleton` renders one childless div with the sole `.astryx-skeleton` target. Width and height default to 100%; a known containing block must provide meaningful percentage height. An indefinite-height parent can produce zero height, as in the source. The application owns the surrounding layout, busy/loading announcement, fetching, and replacement with real content. Skeleton is not a Spinner, progressbar or live region.

- `SkeletonLength` is Pixels(f64) or an open CSS string. Numeric props accept u8/u16/u32/i8/i16/i32/f32/f64 conversions; owned/borrowed string inputs convert to CSS values. Finite nonnegative pixels, including zero and fractions, render in px. CSS calc/var/percentages/keywords remain CSS, not a closed size enum.
- Public boundary validation rejects negative/nonfinite numeric dimensions, empty CSS values, top-level declaration terminators and unfinished strings/comments/delimiter blocks with recoverable `SkeletonError::InvalidWidth`/`InvalidHeight`. Complete comments, quoted/escaped tokens and balanced function fallback blocks remain allowed. This protects adjacent serialized geometry declarations; it does **not** implement a CSS grammar validator. Unrecognized self-contained CSS values are still interpreted/ignored by the browser. These deliberate Rust invalid-input errors replace the source's possible invalid browser declarations; no valid-pixel-output difference is claimed.
- CSS-wide keywords must apply to the dimension, not inherit a private variable. Internal keyword classes handle inherit/initial/unset/revert/revert-layer (including CSS identifier escapes and surrounding comments). They are not public theme targets or axes. Normal dimension values and radius/delay use private variables so caller classes can still override actual properties; caller inline style is applied last.
- `SkeletonRadius::{None,Zero,One,Two,Three,Four,Rounded}` models the seven source choices. None is literal zero; Zero reads radius-none; One/Two read inner/element; Three (default) and Four both read container; Rounded reads full. Radii are callsite geometry, **not** data-radius/themeProps axes. Index/size are not emitted as public axes either.
- `index: u32` enforces the source's nonnegative-integer concept. Delay is exactly 1000 + 100 × index ms, calculated in u64, including u32::MAX without overflow. Negative/fractional indices are outside this Rust typed API, rather than being passed to the source's permissive numeric formula.
- Root native data/ARIA/DOM attributes, class/style, onmounted ref callback and additional event builders pass through. Native aria-hidden can override default true, including false to expose a caller-named shape. No library role, tab stop, label, busy state or live region is added. Only caller-supplied event handlers are attached; the default component creates no task, eval, timer, observer or listener.

Example:

```rust
use astryx_dioxus::{Skeleton, SkeletonRadius};
use dioxus::prelude::*;

#[component]
fn ProfilePreview() -> Element {
    rsx! {
        div { "aria-busy": "true", "aria-label": "Profile preview",
            Skeleton { width: 40, height: 40, radius: SkeletonRadius::Rounded }
            Skeleton { width: "90%", height: 16, index: 1 }
        }
    }
}
```

The example's loading state/text are application-owned; reusable Skeleton itself accepts no user-facing text.

## Paint and exact motion

Core skeleton paint is `light-dark(#CCD3DB, #5A5E66)`; source Neutral is `light-dark(#d4d4d4,#525252)`. Resting opacity is 0.25. Increased contrast mixes the paint 30% toward primary text. Forced colors wins over that mix, uses system GrayText and resting opacity 1. Motion still pulses under forced colors unless reduced motion is active. GrayText is never replaced with the capture host's particular RGB.

The original “shimmer” is an **opacity pulse**, not a traveling gradient. CSS keyframes hold 0.25, 0.325, 0.4, 0.475, 0.55, 0.625, 0.7, 0.775, 0.85 and 0.925. Each level occupies one tenth; 0.01%-duration linear ramps connect them. Direction alternates forever; there is no steps() timing function or per-iteration delay. Peak opacity is 0.925, not 1. No fill mode is added; before its initial delay a block shows base opacity, including full resting opacity in forced colors.

Each climb/fall takes `var(--_astryx-pre-tempo-duration-medium-max, var(--duration-medium-max))`: core 550ms, Neutral 400ms. A provided pre-tempo copy wins over the scaled public duration. The component consumes this original contract; it does not implement the entire unported shared theme normalization/tempo API. Reduced motion sets animation-name none and leaves the static block. Native CSS animation cancellation handles removal; no JS lifecycle machinery is necessary.

## Four original patterns

`examples/showcase/skeleton.rs` ports all **33 original blocks**:

| Pattern | Blocks | Native source geometry at 16px root |
| --- | --- | --- |
| SkeletonShowcase | 6 | 300 × 144; 64px avatar, header/subtitle and three percentage lines |
| SkeletonCardSkeleton | 6 | 320 × 142; 40px avatar/header and three content lines |
| SkeletonStaggeredList | 5 | 320 × 112; widths 300/280/320/260/290 |
| SkeletonTableRowSkeleton | 16 | 458 × 88; four rows of widths 50/180/100/80 |

Native Card uses the original 12px total inset: 11px CSS padding plus 1px border, not 12px plus border. Stack gaps, centering, default radius, relative block/frame offsets and fractional 90%-of-296px line width match source. The first example retains the real muted presentation background; the others retain source default Card/page paint. Local horizontal scroll stages preserve known geometry on narrow viewports instead of compressing table columns. They do not claim the full documentation-site shell or reusable Card/Layout APIs. Table Rows remains div-based placeholder layout, not a semantic data table or a Table port.

Extra examples separately exercise all seven radii, defaults/CSS-wide sizing, class/style overrides, native ref/events, dynamic geometry/stagger, late mounts/removal, caller-owned busy replacement and pre-tempo duration. They are not added to the original-template count.

## Actual appearance evidence

`tests/fixtures/skeleton-neutral-reference.json` captures the actual catalog in light/dark: **66 block records**, relevant inherited tokens, all native frames, animation keyframes/timing and separate media observations. A fresh isolated Chrome target verified 28 pinned source blobs before capture. The catalog advertises v0.7.16; its release is **not** asserted identical to the pin. Published documentation stage ratios differ from pinned metadata (16:9 versus 1 for Showcase, measured scroll stages versus 4:3 for the others); native examples and component contracts match. No documentation metadata ratio becomes a Skeleton dimension.

Animations were first observed running, then existing effects were paused at currentTime 0, before each positive delay, for deterministic geometry/base-opacity records. Natural clocks, generated keyframe names, raw StyleX inline-variable names and whole-site coordinates are provenance, not public equality targets. Tests compare the same self-contained DOM metrics, selected root style, every keyframe/delay/timing, native frame anatomy/geometry/paint and actual compositing backdrop. Numeric geometry tolerance is one native layout unit (1/64px); selected strings are exact. Two transparent, zero-sized source Card shadows normalize to none because they paint identically. Native frame target classes and equivalent normal/stretch reset spellings are not treated as component migrations. Tests load only the checked-in fixture and local font assets; no live catalog request is needed.

## Verification

Run all five standard project checks and:

```sh
node tests/skeleton.browser.mjs
node tests/button.browser.mjs
node tests/button_group.browser.mjs
node tests/icon_button.browser.mjs
node tests/link.browser.mjs
node tests/spinner.browser.mjs
node tests/badge.browser.mjs
node tests/divider.browser.mjs
node tests/kbd.browser.mjs
node tests/spinner_rotor.browser.mjs
node tests/announcement.browser.mjs
```

Use the [existing Chrome/CDP setup](button-port.md#browser-checks). CDP_URL and SHOWCASE_URL override defaults; rebuild the watch-disabled showcase after changes.

Rust checks cover childless hidden defaults, no invented status/focus/axes, pixel/zero/fractional/open CSS geometry, every radius mapping, maximum-index delay, native semantic overrides/ref/events/styles, stable prop updates, recoverable errors and declaration isolation. Private logic tests cover complete CSS quoting/escapes/comments/blocks and CSS-wide identifier handling without merging comment-separated tokens.

Chrome checks cover all 66 actual source block comparisons and all native frames/backdrops; both alternating pulse legs across all ten held values and the real linear ramp; initial stagger; all radius states; core versus Neutral tokens; pre-tempo and caller styling precedence; AX default exclusion/caller exposure; native callbacks; stable root updates; dropped CSS animation; application-owned busy replacement; increased contrast, forced-color precedence/resting and animated opacity, and reduced motion. RTL, enlarged root-font tokens and narrow layouts are checked separately from source template capture.

After isolating a single visible Skeleton, a 1200ms idle trace records no Paint/Layout events and no per-frame style loop. Independent screencast pixels show changing opacity during a 700ms blocked JavaScript task, not merely an advancing animation clock. This proves the observed Chrome compositor path, not Safari performance.

The existing Link regression assertion exposed a pre-existing race after dark-to-light switching: separate CDP calls sampled label/root colors in different transition frames. Its assertion now samples both within one browser task; no Link implementation or paint changed and no retry masks failures.

## Accessibility/verification boundaries

Skeleton's placeholder is decorative and textless, with no interactive boundary/focus target. No invented 3:1 essential-edge or 4.5:1 text threshold is used to call its deliberately low-opacity paint a failure/pass. Native forced-color visibility, paint, hidden AX state and caller semantic overrides are actually checked. A product that makes a placeholder carry essential meaning must assess its own context; this is not a blanket contrast or WCAG certificate.

Physical devices, actual Windows High Contrast, Safari/Firefox, real screen-reader delivery and a complete SSR hydration application remain unrun. Forced-color/media emulation and Chrome AX do not replace those checks. No full shared Theme tempo/provider or application loading lifecycle is claimed. No remaining source implementation/design question blocks this component; independent supporting-component migrations remain unchecked.
