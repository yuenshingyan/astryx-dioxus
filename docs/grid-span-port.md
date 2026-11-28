# GridSpan port

GridSpan is the static participation modifier for Grid. It renders one native `div` with the `astryx-grid-span` target and no added role, name, or tab stop. Its inner grid has one `minmax(0, 1fr)` track, zero minimum width, and full height. `GridSpanColumns::Count` and `rows` accept positive integer counts, not a size scale; `Full` covers only explicit columns (`1 / -1`). Unset and zero counts preserve caller placement. Negative and fractional spans are not represented in the Rust API. Caller children retain their DOM order and semantics.

## Pinned comparison

The supplied component and spec and the following additional sources were verified by hashing the files against the Git tree for [Meta Astryx `476a315`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e).

| Source (paths relative to their upstream directory) | Git blob |
| --- | --- |
| `Grid/GridSpan.tsx` | `7c109c52923e91c4e306fe07faf4a260703fc2db` |
| `GridSpan.spec.md` | `2e2be77a4e57c5e60372f26b4f968440abf0c08d` |
| `Grid/GridSpan.doc.mjs` | `1d653f6be4782f44356871b98efc35ab6e9c100e` |
| `Grid/Grid.test.tsx` | `6cb2f3a16f1e4df89bc964b8e6b8c98369113c81` |
| `GridSpanAudit.a11y.chromium.spec.ts` | `2d028d0137835fb00fe7a8a2e2a0d163b1fbf0cf` |
| `GridSpanColumns.tsx` / `.doc.mjs` | `1e41033a5e8a6d4af9b8c905dff1b6a2b44b1213` / `080a5c1297e079714a356847b38b188f0aed74b1` |
| `GridSpanShowcase.tsx` / `.doc.mjs` | `6a6b245e1e02a5411e75d7373d88b33742083313` / `999288eb790216ad1e25409b5bfc69f68c5f78d6` |
| `Grid/Grid.tsx` / `Grid.doc.mjs` | `b9ec203c718fbee3868a5d164574b694989e0785` / `fec52b5c4b2f362bc290a17ce1895245d8394c66` |
| `Card/Card.tsx` / `Text/Text.tsx` | `66960eeca06bbc0f618ecf3d1cec69be20c2475a` / `2ebc2776ec25710c70e5f958a41e5cc2b488bd27` |
| `theme/tokens.stylex.ts` | `ec6ecd40da08501602ad730274d028d79640a1c9` |

Both named patterns already match the source compositions: a 400px Grid with `--spacing-3` (0.75rem), default Cards (80px except the two-row card), and secondary body Text. Columns has two two-of-three spans on opposite sides; Showcase has a two-row span and three- and two-column spans. The Showcase label “Full-width row” is upstream text, not `Full`: that item spans the three columns alongside the tall card. Grid owns the tracks and the target mapping; GridSpan has no private paint, typography, or spacing tokens.

Upstream `BaseProps`, `mergeProps`, `themeProps` and `stableClassName` were also checked. The port substitutes caller CSS classes for React `className`/StyleX and native attributes for host props. Pass the mount callback as `attributes: vec![onmounted(handler)]` (from `dioxus::events`), not as a named component prop. This delivers the mounted element through Dioxus rather than a retained React ref. Span declarations are appended only on the specified axes, so they override normal inline placement, not raw CSS `!important`. No CSS cascade or public API change was made for that adaptation.

## Evidence and blockers

- **Wrapper and host styling verified for the existing Rust API.** The integrated SSR/VDOM run passed all six `tests/grid_span.rs` tests: native child semantics and order, per-axis precedence and placement passthrough, native listener routing, and both named compositions. [PR #241](https://github.com/yuenshingyan/astryx-dioxus/pull/241) passed full CI and all eleven integrated Chrome suites. The GridSpan suite matched a live `onmounted` measurement to the native wrapper, checked ordinary inline column precedence and preservation of the caller row and color, and measured the caller class overriding the base minimum width. Raw CSS `!important` still follows the CSS cascade rather than React style-object replacement; no change to that adaptation is needed for this evidence.
- **Both template layouts verified in Chrome.** `tests/grid_span.browser.mjs` checks Columns’ opposite-side two-of-three placement and Showcase’s masonry alignment with auto row heights in LTR and RTL. It also checks that `Full` excludes a forced implicit track, that a long focusable child shrinks to its track, that the wrapper fills an 80px row without default stretch, and that Tab moves from the native link to the button. This is Dioxus geometry and keyboard evidence, not raster, rendered upstream, or screen-reader evidence.
- **Named composition details verified in SSR.** The strengthened `named_showcase_patterns_render_the_pinned_compositions` test checks the rendered target order against the pinned templates, including each Card's label, span declaration (or no wrapper), default variant, height, and secondary body Text. It guards the unsized two-row Card and distinguishes the “Full-width row” label from the `Full` sentinel. All six GridSpan tests passed using `cargo-slot test --locked --offline --test grid_span`; formatting and bundle verification also passed.
- **Template appearance still open.** The cached GridSpan fixtures are source-only. The missing comparison input is a rendered upstream baseline for each pinned template (`GridSpanColumns` and `GridSpanShowcase`). The integrator needs paired upstream and Dioxus captures with matching theme, font, viewport, and device pixel ratio, including shared Grid/Card/Text output. Standalone Grid and Text remain open; resolve any discrepancy with its owner rather than compensating in GridSpan.
- **Parent still open.** Verify the layouts in a non-Chromium engine and manually verify reading order and reachability with a screen reader (including the long link’s full label, then the next button, without a new wrapper stop). No non-Chrome, visual or manual AT result was obtained.

This follow-up changed tests and notes only. No browser, preview, WASM build, or full test suite was run locally.
