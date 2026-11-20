# ProgressBar port and verification

## Authority and approved treatments

Authority is Meta Astryx commit `476a3153417dbf75c565f7df05b0784e94ddd59e`, especially `packages/core/src/ProgressBar/ProgressBar.spec.md`, implementation, tests and five CLI templates. The specification overrides implementation gaps: theme-timed updates under every pointer, own-region direction, and a sufficient standalone graphic take precedence over measured touch timing or ambiguous source paint.

Two treatments were explicitly approved:

- Retain original fill colors and add high-contrast inset boundaries. Both track and fill have a 1px text-primary inset: the whole-track endpoint remains visible against its backdrop, and the completed region remains distinct where source fills have insufficient track contrast. This changes paint, not geometry. Forced colors use inset CanvasText outlines instead of shadows.
- Otherwise-unpainted custom variants use accent/on-accent. Their identifier remains available for theme overrides.

The port is not pixel-identical where these corrections apply. It does not change the source disabled-marker palette or claim complete accessibility certification.

## Public contract

`ProgressBar` requires a string `label`. `ProgressBarState` separates determinate `{ value, max, marks }` from `Indeterminate`; unknown progress cannot accidentally acquire values or marks. Constructors `value`, `range` and `marked` cover common cases. Defaults are zero of 100, accent, visible label, no displayed value and not disabled.

Finite values clamp into the range. Nonfinite values and nonpositive/nonfinite maxima normalize to zero; invalid maxima expose a safe zero range rather than negative or nonfinite ARIA data. Nonfinite marks are omitted; finite marks clamp to endpoints and require nonblank names. Repeated positions render independently. Value/name-based identity retains focused marks across range changes and reordering, with occurrence keys disambiguating exact duplicates.

Default value text is rounded percentage. Applications can supply `percentage_template` with exactly one `{percent}` placeholder, or `format_value_label`, a synchronous callback receiving normalized `(value, max)` and returning a string. The same text appears in `aria-valuetext` and the optional counter. Indeterminate mode does not call formatters or validate an unused template. Labels, finite-mark names and formatter output must contain non-whitespace text; failures return `ProgressBarError` through Dioxus error boundaries.

Native attributes, classes, styles, event builders and mounted callbacks reach the container. Raw and explicit mounted callbacks compose with the internal bridge. Protected theme/motion metadata cannot be forged through the attribute list. The track retains its own required name/value semantics; caller container roles and supplementary descriptions remain independent.

## Rendering, ownership and interaction

- Canonical targets remain `astryx-progress-bar`, `astryx-progress-bar-track`, `astryx-progress-bar-fill` and `astryx-progress-bar-mark`. Private layout classes add no public label/value targets.
- The 8px rounded track clips a full-width translated determinate fill. The root has a 48px minimum width. Labels use source 14px/500 typography; counters use 14px/400 secondary text. Neutral rebinds the component-local track token to border-emphasized.
- Visible and clipped names remain referenced by the semantic track. The internal fixed-span VisuallyHidden dependency stays under its original owner. `is_disabled` changes source paint without inventing ARIA disabled or disabling explanatory marks.
- Marks are named, focusable images **outside** the progressbar subtree. Matching on-colors apply behind the fill; disabled marks retain secondary ink. Themes can override tick dimensions without clipping their overhang.
- `progress_bar_mark_tooltip` owns anchor registration, click preview and disposal. Shared Tooltip owns hover/focus surfaces and name deduplication; press feedback recognizes tabindex anchors. Actual keyboard focus, mouse hover and touch preview are tested. The anchor binding restores focus only after a same-batch keyed DOM remove/reinsert when focus fell to the body; deliberate blur, explicit focus transfer and removal are not overridden. Its child-list observer runs only while the anchor is focused. Removing a mark unregisters it, releases that observer/listeners and destroys its tooltip.
- `progress_bar_touch` owns native motion, while timing tables remain under `src/motion/` and shared resize/touch utilities under `src/utils/`. These dependencies do **not** complete public Motion, Tooltip, Theme or VisuallyHidden migrations.
- No network/storage operation, progress timer or completion announcement is owned by the component. Applications own progress, replacement content and any live-region policy.

## Motion

Determinate changes use `--duration-medium` and `--ease-standard` for every pointer: core 410ms, Neutral 300ms by default. Native WAAPI samples the current drawn position and starts a full-duration effect, including reversal; CSS reversal shortening and distance-proportional touch timing are not retained. Reduced motion lands immediately. Forced-color updates also land immediately, preserving the entire sized outline without per-frame layout animation.

Coarse positive progress has a 16px minimum, capped to the track; zero paints nothing. Minimum-covered marks update their fill/track placement using only the component's own track measurement. Mark-only replacement and resize are covered. Nearest rendered direction controls both determinate and traveling fills, including LTR regions inside RTL ancestors.

Indeterminate progress uses a 40% segment traveling for 1500ms ease-in-out. Reduced motion replaces travel with a full-width opacity pulse, 1 → .84 → 1, 2000ms **per leg**, using the source move curve. This loop does not scale with theme tempo. It remains CSS/compositor-owned during main-thread blocking.

Mode changes, dropped roots, channel closure and mark removal release owned native effects/listeners/observers/registrations. Detached marks are removed from placement bookkeeping. There is no per-frame Rust/JavaScript animation loop.

## Source appearance evidence

`tests/fixtures/progress-bar-neutral-reference.json` records actual catalog observations and 34 verified pinned Git blobs, not reconstructed port output. The published deployment is not asserted identical to the pin. The local Figtree asset is used offline; source font URL/hash and declarations remain provenance.

All five original patterns are represented: CustomFormat, Indeterminate, SemanticVariants, Showcase and WithValueLabel. Nine controls × light/dark produce **18 actual-source comparisons**, including native frames, text rectangles, source colors, track/fill geometry and compositing backdrops. Tests tolerate at most 1/64px geometry differences and normalize equivalent token spelling, generated identifiers and non-contract reset details. Approved shadows are asserted separately, not presented as original source paint. Native timing is checked behaviorally rather than requiring the original CSS transition implementation.

Native frames: CustomFormat 300×56, Indeterminate 300×32, Showcase 300×32 and WithValueLabel 300×32. The catalog's intrinsic-width SemanticVariants frame is **84.421875×224**, not an invented 300px width. The showcase reproduces its containing-block recipe; a definite-width application host may legitimately produce a wider column. Catalog titles/cards are presentation, not reusable component content.

## Contrast and limits

`node tests/progress_bar.contrast.mjs` reproduces source-token alpha composition for core/Neutral, both modes and body/card/surface backdrops. It includes custom accent fallback:

- Approved graphic boundary: **0/84** resting and **0/84** conservative .84-floor failures below 3:1; minimum ratios approximately 7.81 and 6.11.
- Whole-track boundary/backdrop: **0/12** failures; minimum approximately 12.64.
- Unchanged disabled-marker ink: **15/168** conditional mark cases below 3:1. The audit deliberately exits **1** to preserve these findings. Inactive-component and essential-mark applicability still need contextual review. Marks never render in indeterminate pulse mode.

These calculations do not certify arbitrary custom themes, subpixel rasterization, physical devices or color-only semantic distinctions. User-defined paint can invalidate the default contrast treatment. Actual screen readers, non-Chrome engines, physical touch devices and complete hydration applications remain unverified.

## Verification

Passed locally:

- `cargo fmt --check`
- `cargo check --locked --lib --no-default-features`
- `cargo test --locked`: **167 Rust tests and 10 doc tests**
- `cargo clippy --locked --all-targets -- -D warnings`
- `cargo check --locked --example showcase --target wasm32-unknown-unknown`
- All thirteen existing Chrome suites plus `tests/progress_bar.browser.mjs` and `tests/progress_bar_motion.browser.mjs`: **15 suites**.

The first full test attempt stopped with `No space left on device` while writing Rust's incremental query cache. The complete Rust test, Clippy and wasm runs then passed with `CARGO_INCREMENTAL=0`; no test retry policy or diagnostic suppression was added.

The native boundary suite checks theme timing/full reversal under both pointers, preference changes, minimum/cap/mark replacement/resize, direction, forced colors, no-WAAPI fallback and resource release. Mounted-component tests additionally check source frames, AX names/values, native refs/events, normalization, localized/custom values, tooltip input/removal, focused-mark range/reorder retention, custom paint, mode changes and remounts. Idle traces show no per-frame Paint/Layout for either indeterminate presentation; screencast pixels change during a 700ms main-thread block in both traveling and reduced-motion modes.

CI has not been run for this branch. No commit, push or merge is implied.
