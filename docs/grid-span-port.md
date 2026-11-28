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

The original port recorded five passing Rust checks and Chrome evidence for numeric/full/row spans, RTL, and the native wrapper. The closure audit adds a focused SSR/VDOM suite for independent axis precedence, listener routing, and both named compositions. It also adds a showcase case with a third implicit track, a long focusable child, and a mount measurement; the browser suite checks that `Full` covers only the two explicit tracks, both named layouts without forcing row heights, LTR/RTL, shrinkage, full height, caller CSS and inline style precedence, the native mount measurement, and child Tab order.

The single slotted Rust run stopped at compile errors in the new verification code (the integer shortcut for `Grid::columns`, direct event props on GridSpan, and a missing import). Those sites now use the existing `GridColumns`, attribute-vector form and explicit import. The corrected test target and the wasm showcase build have not been run. The browser suite was syntax-checked only; no browser or preview was started. For the integrator, against the current showcase:

```sh
SHOWCASE_URL=http://127.0.0.1:8113 node tests/grid_span.browser.mjs
```

- **Wrapper/styling item:** waiting for a live mount result on the native wrapper and computed-style evidence for caller class and ordinary inline placement. Raw CSS `!important` follows the CSS cascade rather than React style-object replacement; exact parity for this Rust-only input is not established.
- **Columns and Showcase items:** pinned source compositions match, but Columns’ two-of-three placement and Showcase’s masonry at its original auto row heights still need the browser regression and appearance comparison, not just a count of children.
- **Parent:** in addition to those rows, compare the two pinned originals with the Dioxus output (including the shared Grid/Card/Text appearance); the standalone Grid and Text migrations remain open. Check native child order/reachability in a screen reader and confirm layout outside Chrome. There is no current visual or manual AT result.
