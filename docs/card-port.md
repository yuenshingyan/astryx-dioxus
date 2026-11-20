# Card port and verification

## Scope and authority

This task ports **Card only**, per the requested one-by-one sequence. ClickableCard and SelectableCard retain separate upstream modules, APIs and pending checklist rows. Their examples are not passed off as static Card examples. Supporting native Heading/Stack/Grid/Layout recipes and the application's demo feedback do not complete those public migrations or Toast.

Authority: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, especially `docs/specs/components/Card/Card.spec.md`, `docs/architecture/container-padding.md`, `packages/core/src/Card/`, Layout's container/padding/corner/block-bleed utilities, and the six Card-owned CLI templates. The recorded source-padding bug is corrected according to authoritative FR3; the palette is not redesigned.

## Public API and ownership

Exports: `Card`, `CardVariant`, `InvalidCardVariant`; shared `SizeValue`, `SizeValueError`, `SpacingStep`, and `Elevation` remain owned by `src/utils/types.rs`. Spacing-step lowering remains under `src/layout/padding.rs`; Card owns its border subtraction, surface and shadow composition. Shared container/corner/block-bleed CSS stays in the existing stylesheet with its Layout protocol ownership identified. No catch-all component module, dependency, diagnostic suppression or existing API removal is introduced.

- Thirteen built-ins: default, transparent, muted, blue, cyan, gray, green, orange, pink, purple, red, teal, yellow. Validated open ASCII identifiers retain their spelling; unmatched custom variants are transparent/borderless base surfaces, **not** default or accent fallbacks.
- Elevation is the closed none/low/med/high scale. Ring and elevation compose in one shadow list, preserving Neutral's dark inset bezel. There is no selection or click behavior.
- Padding is `Option<SpacingStep>`: omission reads the theme's Card padding, with spacing-four fallback; explicit steps are 0, .5, 1, 1.5, 2, 3, 4, 5, 6, 8, 10. Neutral defaults to spacing-three.
- `width`, `height`, `max_width`, `min_height` accept `Option<SizeValue>`. For example, `width: SizeValue::from(320)` or `max_width: SizeValue::from("100%")`; `None` preserves omission. Numeric values must be finite and nonnegative. CSS remains open-ended but cannot terminate or consume adjacent declarations. Strings are browser-validated, not restricted to an invented dimension grammar.
- CSS-wide keywords operate on actual properties through private classes, including escaped/commented spellings; storing `inherit` only as a custom property would inherit the wrong value. Private sizing values preserve caller-class precedence; explicit caller `style` wins last.
- One `div`, arbitrary children, canonical `astryx-card` target, reflected variant/elevation, native props/classes/styles/events/ref callback. Raw and explicit mounted callbacks compose. Preferred width is not a minimum; caller minimum CSS can win.
- Card adds no role, accessible name, tabindex, focus behavior, state, hook, effect, timer, measurement, announcement or network operation. Native caller semantics and child controls remain independent.

Card clips rather than becoming a scroll container by default. A provided non-auto height selects overflow:auto; recognized CSS `auto` includes surrounding comments/case/identifier escapes. Unresolved variables and CSS-wide height inputs follow the explicit-height classification without measuring their computed result. A genuinely overflowing fixed-height Card may become a browser-provided unnamed scroll tab stop, as the upstream spec records; callers can supply appropriate native naming where applicable.

## Applied padding correction and geometry

Source default Card subtracts global `--border-width` from its applied padding, but publishes the **unsubtracted** token. Bleed descendants therefore cross the padding edge and can cause phantom scroll extents. FR3 and container-padding INV11 require published padding to equal applied padding.

The port derives each logical edge independently as `max(0px, requested-padding - default-border-inset)` and both applies and publishes that value. Borderless variants subtract zero. Explicit step overrides theme chains; directional theme tokens override broader theme values; nested Cards reset stale own-Layout/size/step variables and publish their own geometry. At padding zero, default still contributes its border but publishes zero.

Corner and block-edge bleed variables reset to zero on every publisher. Only a direct first/last `data-container-corner` reader receives the relevant radius/block inset; any deeper reader suppresses that publication. A plain wrapper cannot accidentally grant Table-style block bleed. All four logical values remain inherited for existing edge-specific consumers such as Divider. These internal variables are not new theme APIs. Arbitrary caller CSS that changes actual padding without matching publication is outside the architecture's alignment guarantee.

A global border token override changes subtraction; a component-specific caller border override is not silently measured. This preserves the source's global-token contract and keeps Card static.

## Original examples and appearance

`tests/fixtures/card-neutral-reference.json` preserves actual published-catalog observations and provenance for **70 verified pinned Git blobs**. Published release equality with the pin is not asserted. Tests use the bundled Figtree font and offline fixture; no live catalog/font request is required.

Six static patterns: Showcase (1), Callout (2), Elevations (4), Variants (6), WithInnerLayout (1), WithSimpleContent (1): **15 Cards × light/dark = 30 observations**, with full native frame/content/text-rectangle/backdrop evidence. Equivalent flex `normal`/`stretch` and renderer-specific adjacent text-node segmentation are normalized; geometry tolerance is 1/64px. Source shadows, colored secondary-text scope and font weights are retained. Every core variant is also checked against pinned paint declarations under both modes.

Native frames are 320×112, 556×96, 520×152, 868×148, source 380×170, and 360×124. Variants' native wrapping uses the catalog's 878px padded presentation host, not a new Card width preset; its desktop frame can scroll inside this showcase's narrower application column. Catalog decoration is excluded from native content comparisons.

**WithInnerLayout is a separately asserted FR3 correction**, not a fabricated identical source observation: Card is 380×166; the bleeding Layout starts at (1,1), spans 378×164, and remains inside the border. Client and scroll extents agree; exact content and ordinary Card paint/padding are checked separately. The original fixture retains the erroneous 380×170 outer-edge Layout and 12px publication unchanged. No broad tolerance waives corrected geometry.

The inline three-payout phone example is source-derived, **not** an observed catalog fixture: those IDs were absent from the actual catalog DOM. At 390px the local grid's Cards yield to their tracks and clip rather than widen the grid. Existing Text supplies one-line truncation and full-ID touch tooltips. Required Neutral coarse-preview typography (19px headings, 16px body, 14px supporting) is scoped to this demonstration so incomplete global Theme typography is not silently changed for unrelated components.

Layout's Cancel/Save callbacks provide the exact source demo message through an application-owned status output outside the native frame, not an unfinished Toast API. No real storage operation is invented.

## Contrast and accessibility limits

`node tests/card.contrast.mjs` models both text roles across 14 variants (including custom baseline), core/Neutral, light/dark and body/card/surface backdrops: **4/336 cases below 4.5:1**, all unchanged core light secondary text (gray on three backdrops, red over body), minimum approximately **3.824**. It deliberately exits **1** to preserve those findings. Neutral's source colored-Card secondary ink override is retained; no palette replacement is implied.

Card edges, category fills and resting shadows group content decoratively and receive no invented essential-graphic 3:1 threshold. Forced colors preserve native system paint: shadows/category distinctions disappear, default retains its border, and borderless variants get no invented outline. This is not a guarantee that category color alone conveys accessible meaning or that caller content/custom themes meet contrast.

Actual screen readers, physical touch devices, non-Chrome engines, native Windows High Contrast and complete hydration applications remain unverified. Browser AX assertions are not a screen-reader test.

## Verification

Passed locally:

- `cargo fmt --check`
- `cargo check --locked --lib --no-default-features`
- `cargo test --locked`: **176 Rust tests and 11 doc tests**
- `cargo clippy --locked --all-targets -- -D warnings`
- `cargo check --locked --example showcase --target wasm32-unknown-unknown`
- All fifteen existing Chrome suites plus `node tests/card.browser.mjs`: **16 suites**.
- New-file syntax/whitespace checks and the contrast model above.

Rust builds used `CARGO_INCREMENTAL=0` to limit disk-cache growth. Card browser coverage includes the six source patterns, all surfaces/elevations/steps, core paint, class/style precedence and CSS-wide values, preferred-width shrinkage, fixed-height overflow, theme/asymmetric/zero/nested padding, RTL full-bleed Divider, direct/deeper corner readers, native refs/events/AX, stable updates/removal, phone ID recovery, reduced motion and forced colors. Card itself has no native resources to cancel; child Text/Button resources stay with those owners.

CI is unrun for this branch. No commit, push or merge is implied.
