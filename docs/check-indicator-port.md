# CheckIndicator port

## Pinned authority

Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `packages/core/src/Indicator/CheckIndicator.tsx` (`9903cf1c`), `docs/specs/components/Indicator/CheckIndicator.spec.md` (`26ef6d4e`), the owning `Indicator.doc.mjs` (`baeea1c0`), and `types.ts` (`17ae1ae6`). The supplied source and spec matched the Git blob IDs in the complete pinned tree. That tree contains no standalone CheckIndicator CLI template. It does contain `apps/storybook/stories/Indicator.stories.tsx` (`45630bdd`), with direct state, size, disabled and busy-slot examples; the three family recipes are inline in the family doc.

Also verified against the pinned tree: `isRenderable`, `mergeProps`, Icon rendering/sizing/registry and `defaultIcons.tsx` (`8bd4b70b`), theme tokens, Spinner, RadioIndicator, the indicator registry and hook, `defineTheme`, Selector, and the upstream indicator, registry and theming-target tests. The pinned comparison images are `.github/visual-baseline/shots/core-indicator--all-states__neutral-{light,dark}.png` (light `62d78fcb`, dark `280dbc13`). Both PNGs were hash-verified and inspected offline: the CheckIndicator row has a blank unchecked cell and a bare check in the checked cell, dark on light and light on dark. The fixtures do not show disabled, replacement, or theme-provided artwork. No matching port raster was available for comparison.

## Supported standalone contract

- The required enum has exactly two states. Unchecked without a replacement renders nothing. Checked uses the shared Icon root without a CheckIndicator wrapper or new theme target. The local default check now uses the pinned `defaultIcons.tsx` path and stroke. The global `check` icon entry can override that artwork, not the indicator component or its unchecked behavior.
- Both sizes resolve to the small Icon box (one rem); Medium and enabled are the defaults. The glyph and reserved replacement box use `--color-accent` / `--color-icon-disabled`. Disabled is visual only.
- The Rust shallow slot choice is `Option<Element>`: map nullish, boolean and empty-string slot inputs to `None`; pass `Some` for actual replacement elements, including empty-rendering elements or containers. The port deliberately does not inspect an `Element`'s descendants. The reserved one-rem span is used before testing selection, so a pending visual works in either state.
- Class, inline style and neutral native attributes/callbacks go to the rendered root, with `aria-hidden` enforced on both branches. The component introduces no role, name, tab stop or keyboard behavior. Keep focus props and interactive content out of the hidden subtree. The native button demo owns the accessible name, pressed/busy state, and activation; its small indicator is mounted in either selection state and can swap in a decorative, inherit-shade Spinner.

## Why the parent and five detailed items remain open

| Item | Remaining evidence or implementation |
| --- | --- |
| Named `indicators.check` override | Upstream Selector resolves `useIndicator('check')` and supplies both states to the result. The Rust Theme and global icon registry do not provide themed indicator **component** resolution. A host choosing `RadioIndicator` directly would not implement that contract. This needs a shared Indicator/Theme resolver and host adoption, outside this component's scope. |
| Restyle an indicator | The exact family recipe is for checkbox/radio theme targets and combined states, not a CheckIndicator target. Existing Icon/host classes are not the family `defineTheme({components: …})` recipe. |
| Replace an indicator with your own component | The family recipe registers a token-styled, children-first `BrandCheckbox` through `defineTheme({indicators})`. The CheckIndicator content slot is not component registration. |
| Use radio visuals for single selection | Needs the named resolver and a consuming selection host to show radio chrome in both states under a theme override. |
| Source examples | The small checked/unchecked expression and busy content are represented in the showcase; the second example (replace check with RadioIndicator through `defineTheme`) is still missing. |

Other parent-level gates:

- **Raster and cross-browser:** Coordinate with the integrator to capture the port's equivalent light/dark unchecked and checked cells and compare them against the pinned images. Disabled, busy and theme-provided artwork need corresponding upstream renders beyond these two fixtures. Run the standalone regression in a second browser engine.
- **Manual AT:** With a screen reader, focus the showcase toggle, change selection by keyboard, and enter/exit pending in both states. Verify that the host alone receives focus and has the accessible name, record what is announced for selection and busy changes, and check that neither the glyph nor the Spinner is exposed as a separate object.
- **Native mount and upstream API:** Source inspection shows `onmounted` is passed to the slot span or the shared Icon span. Actual browser callback delivery on those roots, and its absence in the empty branch, remain unverified. The existing native callback and class/style APIs are not React/StyleX `ref`/`xstyle` parity. FR4/OQ1 needs an upstream decision on the declared span ref versus the Icon-owned root, or explicit acceptance of the Rust adaptation for closure. FR3/OQ2 records that empty containers suppress the mark today; preserve the current shallow slot behavior unless that contract changes.

## Verification

[PR #241](https://github.com/yuenshingyan/astryx-dioxus/pull/241) passed full [CI](https://github.com/yuenshingyan/astryx-dioxus/actions/runs/46143279666), including the focused Rust tests, and the integrated WASM showcase build and all eleven Chrome suites passed. The focused Chrome suite checked unchecked DOM absence, same-root Icon and host classes, 16×16 CSS-pixel boxes, `aria-hidden`, computed Neutral light/dark foregrounds, and host focus and selection/pending DOM transitions with Enter. This closes the current build and Chrome regression gates, not raster comparison, accessible speech, or native mount delivery. The Rust tests cover the pinned fallback path and stroke, both small-box mappings, class/style and decorative precedence, host-owned busy markup, and the global icon artwork override (not theme component registration).

This follow-up did not change behavior. Formatting, CSS bundle verification, and browser-script syntax checks passed locally. The focused `cargo-slot test --locked --test check_indicator` run stopped during dependency linking with `No space left on device`; no local tests ran. No preview, browser, or manual AT was started.
