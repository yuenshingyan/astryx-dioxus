# Divider implementation and verification

## Baseline and owners

Pinned authority: Meta Astryx [`476a3153417dbf75c565f7df05b0784e94ddd59e`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e), `packages/core/src/Divider/{Divider.tsx,Divider.doc.mjs,Divider.test.tsx,index.ts}`, `docs/specs/components/Divider/Divider.spec.md`, and all four Divider template implementations. Card, Section, Stack/StackItem and the shared container style owners were inspected before reproducing example framing.

`src/divider/` owns Divider, orientation, open variant names and label adaptation. Divider consumes the original inherited container-padding protocol; it does not publish container insets, relocate shared layout infrastructure, or invent a combined context. Shared tokens/styles remain in `assets/astryx.css`. No dependencies, diagnostic suppressions or breaking existing Rust APIs were added.

## Public contract

- `DividerOrientation`: Horizontal (default) or Vertical. The root reflects the actual visual axis in data-orientation.
- `DividerVariant`: an open validated name, default Subtle, with Strong as the other built-in. Subtle uses color-border; Strong uses color-border-emphasized, at the same border-width. Custom names are reflected unchanged. The constructor rejects empty/non-ASCII-identifier content without silently rewriting it.
- `DividerLabel`: optional Empty (default), Text, Number or Rich content. Plain strings/numbers/rich elements convert at the builder. Booleans and empty strings omit the label; numeric zero survives; whitespace and opaque rich containers are shallowly renderable. Rich descendants are never flattened or inspected. Integers retain exact decimal precision without f64 conversion; floating scalar presentation preserves zero/non-finite/exponent boundaries.
- Without a label there is one rule; with a label there are two rules around one wrapper. The label uses supporting text in secondary color, with spacing-3 padding on the separator axis. Vertical labels get block padding, not inline padding.
- The root defaults to separator role and matching aria-orientation. A visible string or rich label names it through a stable aria-labelledby relationship. Explicit aria-labelledby wins, then explicit aria-label, then the generated relationship. An unlabelled separator is unnamed; explicit empty names remain caller-owned native input.
- Native attributes, class/style, root ref through onmounted, and additional native event builders reach the root. Caller role and aria-orientation override semantic defaults; data-orientation/data-variant remain component-owned visual theme state.
- IsFullBleed is a Rust `is_full_bleed` bool. Horizontal consumes logical inline-start/end inset; vertical consumes block-start/end. Negative margins and added width/height use the same two per-edge values. Missing variables resolve to zero. Asymmetric values, RTL and post-mount CSS changes require no component measurement.
- Rule segments add no role/name/focus target. Divider creates no Tab stop, interaction, live region, browser bridge, listener, observer, request, effect or timer. Its sole ID hook matches the original stable label-ID responsibility; caller-supplied native listeners remain caller-owned.

The aggregate `.astryx-divider` target reflects orientation/variant. Internal line/label classes implement anatomy; they are not newly invented public theme targets or a stable part API. Shared base/theme CSS layering supports existing token overrides without reorganizing other component CSS.

## Authoritative contract versus source gaps

The pinned spec explicitly records two current implementation gaps:

1. **Caller semantic overrides (FR2/FR6):** the authoritative contract says explicit role/aria-orientation must reach the root, even when contradictory to the visual orientation. The React implementation overwrites them. This new Rust port follows the authoritative rule, with dedicated SSR/browser assertions, rather than reproducing the implementation bug. Defaults and all original examples remain identical.
2. **Unmatched custom variant paint (FR3/OQ2):** the requested custom name is reflected, but unmatched inner rules have no built-in paint. The spec leaves the fallback/painting-part decision open to human API review because a subtle fallback may cover an existing themed root background. This port preserves and tests that source behavior instead of closing the axis, choosing a fallback, adding a new public paint variable or adding public Rule/Label targets (OQ1). Use built-ins for reliable component-owned paint. Resolving custom-rule paint remains separate approved design work, not an implemented fallback or a passing visibility claim.

## Original examples and native framing

`examples/showcase/divider.rs` ports all four named patterns:

- **Divider — Variants / DividerShowcase:** 500px maximum-width column, subtle, strong and `or`.
- **Divider — Full Bleed:** exact order summary content, two edge-to-edge rules.
- **Divider — Variants / DividerVariants:** the separate 400px sign-in comparison Card, labelled alternative and strong rule.
- **Divider — Vertical:** Revenue/Users/Conversion values and supporting text, two stretched auto-height separators.

Native showcase-only article/section/stack markup reproduces required geometry. It is not a placeholder reusable Card/Section/Stack port and does not complete those migration checklists. Neutral publishes 12px container inset; a default Card draws a 1px border inside it, leaving **11px CSS padding + 1px border**, while full bleed still consumes **12px**, reaching the Card's outer border edges. A transparent Section adds 12px outside the Card. The Vertical template is intrinsic (398.4375px Card at the captured viewport), not forcibly 400px; StackItem fill uses grow with auto basis rather than equal zero-basis columns.

Additional cases exercise zero/boolean/empty/rich labels, explicit names/references/semantic overrides, root ref/events, dynamic label removal/restoration, custom names, both labelled axes/weights, asymmetric horizontal/vertical bleed in LTR/RTL, absent inset and live CSS inset changes.

## Appearance evidence

`tests/fixtures/divider-neutral-reference.json` selects contract-relevant measurements from an actual catalog capture: all nine original controls, light/dark, at 1200×1600/DPR1, Chrome 157.0.8103.79. The catalog reports Core v0.7.16; it is not asserted identical to the pinned commit. The unrelated site-footer divider is excluded. No replacement controls were injected into the catalog.

Tests use the same DOM metric function for source and local frames, comparing the default attributes/naming relationship, root/rule/label geometry and paint, flex alignment, logical margins/padding, typography and first font family, native frame sizes/insets/backgrounds, and relative offsets from layout/Card/Section content and borders. Generated IDs, page coordinates, documentation chrome, unused font fallback stacks and no-op global reset styles are not golden public contracts. Numeric geometry permits one native layout unit (1/64px); other selected values match exactly. Rule-width asymmetry of one layout unit is native flex rounding. The tests use frozen evidence, not live services.

## Verification

Run the five standard project checks plus:

```sh
node tests/divider.browser.mjs
node tests/button.browser.mjs
node tests/button_group.browser.mjs
node tests/icon_button.browser.mjs
node tests/link.browser.mjs
node tests/spinner.browser.mjs
node tests/badge.browser.mjs
node tests/spinner_rotor.browser.mjs
node tests/announcement.browser.mjs
```

Use the existing [Chrome/CDP setup](button-port.md#browser-checks). CDP_URL and SHOWCASE_URL can override defaults; rebuild the watch-disabled showcase before running browser tests.

SSR/unit tests cover all 48 built-in axis × weight × bleed × label cases, scalar skip/zero/rich/large-integer behavior, floating presentation, stable/unique reference IDs, explicit naming and semantic precedence, native ref/event/style inputs, protected target metadata, invalid/open variant names, dynamic label updates and None-valued reference inputs. Browser checks add actual AX names/orientation, all nine original light/dark controls and native frame geometry, no extra rule semantics/press/focus/live machinery, root callbacks, both labelled axes, logical/asymmetric/changed/absent inset, narrow/mobile Card edges, theme thickness/paint, custom source gap and forced-color semantic exposure. Existing six component suites and two renderer-boundary suites remain regression checks.

## Contrast and remaining verification boundaries

`node tests/divider.contrast.mjs` (optional --json) audits 16 core/Neutral × light/dark × page/surface × subtle/strong contexts. **16/16 rules are below the conditional 3:1 line target; 0/16 secondary labels are below 4.5:1.** Orientation and full bleed do not change paint. The line threshold is required when the line is necessary to understand grouping; a purely decorative separator is not automatically an essential boundary. If a product relies on these lines, strengthen border tokens in its theme.

The audit intentionally exits 1 and retains the original palette under the existing source-fidelity decision, not a blanket accessibility-compliance claim. Custom/unmatched paint, arbitrary rich labels and caller backgrounds remain caller-owned. Forced-color testing establishes continued separator semantics/name, not certification of essential line visibility in every system palette. Actual screen-reader delivery, physical devices, Safari and Firefox remain unrun. Custom paint/part OQ1/OQ2 remain upstream human design questions; no remediation policy is invented here.
