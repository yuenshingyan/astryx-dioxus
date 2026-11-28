# CheckboxIndicator port

## Pinned source

Verified the supplied `Indicator/CheckboxIndicator.tsx` (`ca8594e71c04c6a3a80c2e6a06e1b0ff9fe8546c`) and `CheckboxIndicator.spec.md` (`4ee206165fdfe84a0005be03f16516e6b454e1ab`) against the complete Git tree at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`. Also verified the blobs for the owning `Indicator.doc.mjs`, `Indicator.stories.tsx`, `Indicator.test.tsx`, types and marker, `CheckIndicator`, `RadioIndicator`, `CheckboxInput`, `Spinner`, `isRenderable`, `mergeProps`, `themeProps`, theme tokens, `defineTheme`, registry, `useIndicator` and `useIndicatorFocusRing`. Also verified the CheckboxInput tests and indeterminate template and the theme target tests. The three named examples are inline in `Indicator.doc.mjs`; the pin contains no Indicator CLI templates.

## Contract and evidence

- The decorative span persists in all three states. The check SVG path and dash geometry match the source; CSS selects the visible mark. `sm` and `md` map to 20px and 24px; `md` and enabled are defaults. SSR covers all states and sizes, the canonical box/check/dash targets and owned data attributes. Prior Chrome geometry evidence for the unchanged base styles covers the empty box, 14px medium check and 10×2px small dash. The small mixed showcase renders the TSX example.
- The existing Rust slot maps rejected shallow React inputs to `None` at the call site (for instance, `is_loading.then(|| rsx! { Spinner { ... } })`). The component does not inspect a supplied `Element` or its descendants. SSR covers missing content, text, empty output and an empty wrapper. The showcase adds a decorative small Spinner using the inherited foreground.
- SSR confirms owned `aria-hidden`, no inherent role or tab stop, preserved `id`, `dir`, class and style, and a separate native checkbox with an associated label. Native attributes and `onmounted` are the existing Dioxus passthrough; there is no separate StyleX `xstyle` or React ref API. Replacement content must be decorative and non-focusable.
- The scoped restyle uses the canonical CSS target, the full-radius token, a 2px border and checked/disabled token paint. The BrandCheckbox demo uses direct composition, token paint, a single hidden root and children-first precedence. The single-selection demo contrasts checked/unchecked check and radio visuals; it does not change `CheckboxIndicator` states or substitute a theme registry.

## What still blocks closure

- **Owner interaction and accessibility:** pinned CheckboxInput puts `indicatorScope` on the enabled row and owns press paint; the pinned checkbox visual itself has no active-fill rule. Rust `CheckboxInput` does not attach `data-astryx-indicator-scope` or implement the upstream host press treatment, and its focus selector targets only the built-in indicator class. It cannot prove a ring following an arbitrary replacement root's radius. Host changes are outside this component. The browser regression for native keyboard activation/focus and the manual screen-reader check (one named native checkbox; no announcement of the decorative marks/Spinner) were not run.
- **Theme switching:** Rust `Theme` has no `indicators.checkbox` registry or `useIndicator` consumer wiring. This blocks the full theme customization item and the theme-installed BrandCheckbox example. The radio example additionally needs the `indicators.check` override consumed by a rich Selector option list; the native Selector does not provide that path. The local demonstrations are not substitutes for those integrations.
- **Visual evidence:** the scoped restyle's computed border, radius and disabled paint checks were added but not run. The previous Chrome run covered base geometry, scoped hover, reduced motion and distinct forced-color marks; it does not cover the new showcase, published/source side-by-side visual comparison, other browser engines or manual AT. Compare the state matrix and restyle in the integrator's preview, and verify keyboard focus, reduced-motion and forced-color paint there before closing.

Local checks: all four focused SSR tests passed via `cargo-slot`; `cargo fmt --check`, `python3 scripts/bundle_styles.py --check`, and `node --check tests/checkbox_indicator.browser.mjs` passed. No preview or browser was started; the full Rust/Clippy/Wasm suite was not rerun.

```sh
SHOWCASE_URL=http://127.0.0.1:8122 node tests/checkbox_indicator.browser.mjs
```
