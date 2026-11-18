# IconButton implementation and verification

## Baseline and ownership

IconButton follows Meta Astryx revision [`476a3153417dbf75c565f7df05b0784e94ddd59e`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e):

- `packages/core/src/IconButton/{IconButton.tsx,IconButton.doc.mjs,IconButton.test.tsx}`
- `docs/specs/components/IconButton/IconButton.spec.md`
- All five templates under `packages/cli/assets/templates/blocks/components/IconButton/`
- The Button, Icon, shared size/link providers, Spinner, tooltip/press, and theme contracts those sources use.

`src/icon_button/icon_button.rs` remains the upstream thin projection: exactly one Button, fixed icon-only mode, required icon/label, and no children/end-content/content-mode props. It adds no element, state, effect, listener, timer, task, theme target, or separate activation implementation. Button owns every rendered behavior. The exported API and existing root imports are unchanged.

The standalone audit found one shared CSS defect: declaring the default `--button-icon-only-aspect` locally prevented ancestor theme values from cascading. Button CSS now uses the square fallback in `aspect-ratio: var(--button-icon-only-aspect, 1 / 1)`. A theme can supply another aspect; explicit width still wins. This changes no default geometry or ownership boundary.

## Contract and Rust adapters

- All four variants, three sizes, and four elevations retain Button defaults. Default squares are 28/32/36px, zero-padding, with centered 16/16/20px contextual Icons. Explicit Icon size wins; arbitrary caller artwork is accepted without inspection.
- Only the icon is visible. `label` supplies the accessible name; callers must provide meaningful non-empty names. No automatic tooltip is made from that label. Explicit tooltips work on hover/keyboard focus and describe without repeating identical names. Disabled touch hints remain available without activation.
- Disabled, loading, and interruptible behavior is exactly Button's. Controls remain focusable with aria-disabled rather than native disabled. Loading announces caller-supplied `loading_label` (default `Loading`) and hides even self-colored artwork under the spinner.
- `click_action` owns automatic busy state, deduplication, and the awaiting scope task. Interruptible mode permits overlapping invocations; it is not a task-cancellation guarantee. Applications own error handling and detached business operations. Unmount cancels the owned awaiting future.
- Links use native anchors, shared LinkProvider, or explicit `render_link`; disabled/unsafe destinations become inert buttons. Root replacement preserves focus. Renderers must forward LinkRenderProps to one focusable native link root, as described in [Button adapters](button-port.md#rust-integration).
- `onmounted` forwards actual-root MountedData instead of React ref. Click, focus, keyboard, mouse/pointer/touch callbacks and ordinary attributes pass through to Button. Owned mode/theme/availability attributes remain protected.
- `button_type` uses ButtonType; form/name/value and native submit/reset remain supported, including pre-mount blocked-submit guards. Width and form values are Rust strings, consistent with Button: use `"96px"` rather than a JavaScript numeric width, and serialize application values before passing them.
- Square aspect is the default, not a restriction on explicit widths or theme aspects. A caller/theme can make a target too small; the default smallest square exceeds 24×24px. Coarse-pointer feedback/hit expansion remains Button-owned.
- Reduced-motion presses do not scale and fine-pointer loading appears immediately. Forced-color borders use system colors. Button's fixes for the upstream reduced-motion, link-focus, and self-colored-content gaps apply unchanged.

## All five named examples

`examples/showcase/icon_button.rs` contains:

1. **IconButton** (`IconButtonShowcase`): Settings with the Neutral wrench glyph.
2. **IconButton — Action Bar**: Search, Copy, Info, Menu, Close; ghost, spacing-1.
3. **IconButton — Floating**: all four variants, high elevation, Heroicons Plus; spacing-3, with supporting text and spacing-4.
4. **IconButton — Loading State**: Copy/Search/Close, caller-controlled loading for 1500ms. The active operation is an enum; replacing it cancels the previous application-owned delay instead of letting an old timeout clear a newer selection.
5. **IconButton — With Tooltips**: Search items, Copy to clipboard, and same-name More options.

Additional fixtures cover sizes/elevations, inherited/explicit sizing, theme aspect/width precedence, disabled/loading/interruptible state combinations, callbacks, automatic async work/cancellation, native/custom links, forms, and custom-colored artwork. Delay resources and their JavaScript timers are canceled on replacement/unmount; they belong to the showcase, not IconButton.

Neutral's built-in icon aliases use Lucide artwork, while the original floating template directly imports Heroicons Plus. The showcase copies those exact SVG shapes and preserves their respective stroke widths, with licenses in third-party notices. This is not an Icon registry/alias API port and adds no package dependency. It does not mark standalone Icon, Link, Spinner, or other component checklists complete.

## Verification

Run the five standard project checks in README, including the full Rust suite and wasm showcase check, then all three browser suites:

```sh
node tests/icon_button.browser.mjs
node tests/button.browser.mjs
node tests/button_group.browser.mjs
```

The browser setup is the same loopback-only Chrome CDP setup in [Button browser checks](button-port.md#browser-checks). Override `CDP_URL` and `SHOWCASE_URL` as needed. `ICON_SCREENSHOT_PATH` optionally saves a screenshot. Rebuild the watch-disabled showcase after source changes.

- **Rust rendering:** six new tests include the full 384 variant × size × elevation × disabled × loading × interruptible combinations, anatomy/name preservation, tooltip deduplication, sizing precedence, raw-attribute protection, links/providers/unsafe destinations, and form guards. Existing IconButton dependency tests remain in place.
- **Compile-time API:** five compile-fail doc tests reject missing icon/label, visible children, end content, and an icon-only override.
- **Browser:** all default sizes are measured for square shape, zero padding, centered correctly sized glyphs, explicit Icon size, theme aspect and width precedence, and four distinct elevations. Actual keyboard/mouse/touch actions, mounted/focus/key callbacks, aria-disabled/busy policies, tooltip hover/focus/Escape/touch behavior, native accessibility name/description, automatic loading/deduplication/interruption/unmount cancellation, native/custom navigation and focus retention, submit/reset, reduced motion, self-colored loading, and forced colors are asserted. The existing Button and ButtonGroup suites must also pass.
- **Independent review:** no missing forwarding fields or extra wrapper-owned resources were found. The theme-aspect finding was fixed with a measured regression; missing standalone verification was filled rather than reported as missing functionality.

### Actual upstream appearance

`tests/fixtures/icon-button-neutral-reference.json` records all **16 controls across all five named examples** from the actual [Astryx IconButton page](https://astryx.atmeta.com/components/IconButton), in Neutral light and dark. The catalog's displayed release is not asserted identical to the pinned commit.

The suite checks dimensions, padding, radius, background/foreground, shadow, font size/weight, SVG dimensions/centering, and every SVG child's geometry/fill/stroke/width/cap/join. Recorded non-width properties match exactly; width permits only 1/32 CSS pixel, consistent with Button's reference checks. Classes, DOM wrapper implementation, and whole-site layout are not screenshot-equality claims. Tests use frozen measurements and do not depend on the live site.

### Contrast results and limits

```sh
node tests/icon_button.contrast.mjs
node tests/icon_button.contrast.mjs --json
```

The unchanged core source-token model has **0/48 essential glyph/spinner-arc failures at 3:1** and **0/16 focus/backdrop failures at 3:1**, including destructive focus. **8/16 resting surfaces are below the conditional boundary target**. A recognizable glyph need not depend on a visible edge, but if the application needs an edge to identify the control, it must strengthen that theme.

The audit exits 1 to expose those conditional findings, not to conceal them as passing checks. Original colors remain under the existing [source-fidelity decision](button-port.md#core-palette-contrast-conflict). This non-text audit does not impose a visible-label 4.5:1 requirement on an icon-only control. It does not certify caller-colored artwork, arbitrary themes, antialiasing, adjacent UI, physical devices, or assistive technology. Chromium/CDP is not a completed Safari/Firefox or screen-reader verification matrix.
