# Blockquote: pinned-source migration and verification

## Authority and provenance

Baseline: Meta Astryx [`476a3153417dbf75c565f7df05b0784e94ddd59e`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e), not a verified release tag or a claim about the live catalog's displayed version. The current authoritative record is `docs/specs/components/Blockquote/Blockquote.spec.md` (FR1–FR4, AR1–AR2, PR1). Consumer documentation, upstream tests, Storybook stories, and all three CLI template sources/metadata were inspected together.

`tests/fixtures/blockquote-upstream/manifest.json` records **25 exact Git blob hashes** checked against the supplied pinned recursive tree. It includes the component/index/docs/tests/spec/stories; all six template/source-metadata files; shared `BaseProps`, `isRenderable`, `mergeProps`, `themeProps`, naming, tokens and Neutral theme/palette references; and source owners for Card, Grid/GridSpan and Stack styles. `node tests/blockquote_source.mjs` verifies every cached byte offline and extracts the component declarations, token values and seven template quotes. Original notices are retained; see `THIRD_PARTY_NOTICES.md`.

**Evidence distinction:** no live published browser measurements were obtained for this task. Browser comparisons use independently created native DOM/CSS fixtures derived from the verified source, not an executed React/StyleX bundle or a published screenshot. All geometry and text-range measurements reported here are observations of those source-derived fixtures and the Dioxus port in local Chrome. The fixture is intentionally limited to these patterns, not a general TSX/StyleX interpreter.

## Complete component inventory

| Input | Pinned contract | Rust/Dioxus boundary |
| --- | --- | --- |
| `children` (required ReactNode) | Arbitrary quoted content, rendered first; caller owns quotation marks | Required `Element`; content/paragraphs/links remain untouched |
| `cite` (optional ReactNode) | Shallow `isRenderable`; present content follows children in bare `cite` | `BlockquoteCite`, default `Empty`, `Text`, `Number`, `Rich(Element)`; string/numeric/boolean/Element conversions |
| `ref` | Root HTMLQuoteElement | `onmounted` receives the native root MountedEvent; native event builders can also be supplied in `attributes` |
| `xstyle`, `className`, `style` | Base → xstyle → caller class/style; no wrapper | Existing project adapter: layered base CSS, caller `class`, then inline `style`; StyleX objects are not a Rust runtime/API |
| remaining BaseProps | Supported DOM, ARIA, data and event props reach root | `GlobalAttributes` plus `attributes: Vec<Attribute>` for native event builders, including `dioxus::events::onclick` |

The `cite` prop is **attribution content**, not the native HTML `cite` URL attribute. It never creates a source link automatically. As upstream, a caller can use rich attribution to supply a native link; the caller owns its destination and operations. Native attributes supplied via the attribute vector are forwarded without inventing validation/sanitization policies or changing event behavior. Strings are escaped by Dioxus.

Absent/default/explicit `Empty`, booleans and empty strings omit the attribution. Whitespace, numeric zero, and nonempty strings render. Rich elements/fragments remain opaque, including empty fragments: the wrapper still renders even if descendants produce no visible DOM. Optional/null-like content is represented by `Empty`, not descendant inspection. Integer conversions retain exact decimal text; floating conversions preserve the JavaScript zero, scientific-notation and non-finite display boundaries. No closed size/variant axis exists. There are no disabled/loading/selection/hover/press states, generated quote marks, hooks, effects, listeners, timers, resource ownership or application operations in this static component.

### Anatomy and styling

- Semantic `blockquote` with the stable `astryx-blockquote` theme target; no invented role, tabindex, accessible name, busy state or landmark.
- Solid logical inline-start border: `--spacing-0-5` (0.125rem / 2px at a 16px root), `--color-border-emphasized`; inline-start padding `--spacing-4` (1rem / 16px). RTL moves both to the right, and logical properties also retain vertical-writing semantics.
- Root `--color-text-secondary`, `overflow-wrap: break-word`, zero block/inline margins. Quote font, line height, paragraph spacing and content semantics remain inherited/caller-owned.
- Bare final `cite`, never a `footer`: block display, `--spacing-2` (0.5rem / 8px) block-start margin, `--text-supporting-size` (`--font-size-sm`, 0.75rem / 12px), `--text-supporting-leading` (1.6667), normal font style. It inherits color and weight rather than introducing new tokens.
- `astryx-blockquote__cite` is an internal BEM style hook adapting the upstream private StyleX rule, not a new public theming part. The root remains the only documented theme target. Source palette and theme customization are unchanged; no forced-color recoloring or motion was added.

## All original templates

| Template | Original composition | Port / comparison |
| --- | --- | --- |
| **Blockquote — Showcase** | Vertical Stack, gap 4, width 500/max 100%; design quote with no cite, people quote attributed to Steve Jobs | Native demo flex-column framing, identical strings, widths and token gap |
| **Blockquote — With Attribution** | VStack gap 4/max 500; same two quotes | Separate named example with the original max-width behavior |
| **Blockquote — Testimonials** | Grid columns 2/gap 3/max 600; three default Cards padding 4, last in GridSpan columns 2 | Existing public `Card`/`SpacingStep::Four`; demo-only native Grid/GridSpan framing; all three original author names and quotations |

The testimonial fixture independently models the original default unelevated Card's fill/border/radius, `min-width:0`, clipping, shadow and **border-subtracted padding**. The port uses the already-merged Card, including its documented corrected container-padding publication; this quote-only pattern has no full-bleed child and no publication discrepancy. Narrow tracks stay two columns as upstream; no invented responsive stacking. Comparisons include the frame, span, all Cards, quotes/cites, text content, geometry, text-range wrapping and computed styles.

Neither Heading nor Stack is imported. Native demo framing does not complete Stack/Grid/GridSpan/Heading/Section migrations. Later independent layout work can explicitly replace the demo flex/grid wrappers with their upstream-owned APIs once integrated, preserving these props/geometry. No public dependency boundary is merged or moved under Blockquote. No reusable component depends on showcase code.

Additional demo cases cover rich multi-paragraph content/link attribution, absent/boolean/empty/space/zero/rich-empty citations, long unbroken content/long source, caller class and inline overrides, root mount/events, dynamic attribution and removal/remount.

## Verification

Run from the repository root. Ordinary Cargo commands use `CARGO_INCREMENTAL=0` and `CARGO_TARGET_DIR=/Users/yuenshingyanhindy/vscode/rust/astryx-dioxus/target`.

- All five standard checks passed: `cargo fmt --check`, `cargo check --locked --lib --no-default-features`, `cargo test --locked`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo check --locked --example showcase --target wasm32-unknown-unknown`.
- Six dedicated SSR tests cover native/static defaults, content/attribution order and escaping, shallow renderability (including opaque empty nodes), numeric values, attributes/classes/styles/native event builders/mount bridge and existing Card composition.
- `node tests/blockquote_source.mjs`: 25 Git blobs and pinned source extraction passed.
- `python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/blockquote.browser.mjs`: passed **48 pattern comparisons** (3 patterns × 2 themes × 2 color schemes × 2 directions × 2 widths). Local preview defaults to port **8091**; `BLOCKQUOTE_URL` can override it. Each run creates/closes only its own CDP target on the shared Chrome endpoint 9222. HTTPS resources are blocked at navigation, fonts/assets are bundled locally, and the comparison phase forces network offline.
- Additional browser assertions passed: native AX blockquote role/caller name and no contentinfo landmarks; native pointer click and keyboard activation of caller's source link; caller focus/attributes; attribution-only updates retain the root; removal/remount delivers a new mount; long-word/source wrapping; class/inline precedence; logical RTL edges; forced-color solid-rule visibility; no animations with reduced motion.
- `node tests/blockquote.contrast.mjs`: **12/12** modeled secondary quote/cite text cases meet 4.5:1 across core/Neutral, light/dark and body/surface/default Card backgrounds. The preserved supporting rule measures 1.312–2.220:1 in these cases. It is decorative supporting quotation framing, not an essential control boundary: no invented 3:1 pass/compliance claim. Caller markup/custom themes/other Card variants remain caller-owned and unaudited.

Verification interruptions were not hidden: one later full test invocation could not execute an existing `tests/icon.rs` binary because the shared target file disappeared (no test assertion ran for that binary); the subsequent full invocation passed. A watch-mode preview rebuild also temporarily served no app and failed the browser readiness check. Restarting only this worktree's preview with `--watch false` restored the final passing browser run. Neither condition was handled by automatic test retries; logs are retained in the task handoff cache.

### Authoritative upstream gaps and remaining boundaries

The pinned spec records gaps in original tests for detailed style/merge-order assertions, attribution order/type, rendered RTL, AX role/no-landmark proof and lack of hooks. The port's SSR/browser tests add direct coverage of styles/order/RTL/AX and observe no animation, while the component source is visibly static. These additions do **not** retroactively close upstream audits, prove no future hooks can be added, or claim a React runtime/exhaustive StyleX merge test. The source-extracted fixtures cannot detect upstream compiler defects or substitute for a published visual baseline. Actual assistive-technology announcements, vertical-writing browser coverage, and unrelated existing component browser suites were not run. Existing palette exceptions and independent supporting migrations remain unchanged. There are no known implementation blockers for this pinned-source Blockquote scope.
