# CheckIndicator port

## Pinned authority

Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `packages/core/src/Indicator/CheckIndicator.tsx` (`9903cf1c`), `docs/specs/components/Indicator/CheckIndicator.spec.md` (`26ef6d4e`), the owning `Indicator.doc.mjs` (`baeea1c0`), and `types.ts` (`17ae1ae6`). The supplied source and spec matched the Git blob IDs in the complete pinned tree. That tree contains no standalone CheckIndicator CLI template. It does contain `apps/storybook/stories/Indicator.stories.tsx` (`45630bdd`), with direct state, size, disabled and busy-slot examples; the three family recipes are inline in the family doc.

Also verified against the pinned tree: `isRenderable`, `mergeProps`, Icon rendering/sizing/registry and `defaultIcons.tsx` (`8bd4b70b`), theme tokens, Spinner, RadioIndicator, the indicator registry and hook, `defineTheme`, Selector, and the upstream indicator, registry and theming-target tests. The pinned comparison images are `.github/visual-baseline/shots/core-indicator--all-states__neutral-{light,dark}.png` (light `62d78fcb`, dark `280dbc13`). They were located and hash-verified, not compared to a current preview.

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

Parent-level evidence still needed: build the current WASM showcase; run the current browser regression and compare the glyph, geometry and light/dark paint against the pinned/published rendering (including theme-provided artwork where applicable). Check another browser and use manual AT on the host to confirm it alone is named, focused and announced during selection and pending changes. For full API parity, upstream spec FR4/OQ1 leaves the checked React ref contract undecided; Rust offers native `onmounted` passthrough, not a React/StyleX `ref`/`xstyle` API. Root mount delivery was not browser-tested here.

## Verification

Earlier port evidence: all five standard Rust checks passed, and Chrome verified absent unchecked markup, same-element Icon/host classes, replacement geometry, and Neutral light/dark colors. Those results predate this follow-up. New SSR cases cover the pinned fallback, both small-box mappings, native styling and decorative precedence, busy content with host-owned semantics, and the existing artwork registry seam. The browser regression now also exercises host keyboard activation, focus retention, and pending replacement in both states. The focused `cargo-slot test --locked --test check_indicator` invocation timed out after 240 seconds while compiling dependencies; it produced no test results. Formatting, style bundle verification, and browser-script syntax checks passed. No preview, browser or manual AT was run for this follow-up.
