# CheckboxIndicator port

## Pinned source

Verified the supplied `Indicator/CheckboxIndicator.tsx` (`ca8594e71c04c6a3a80c2e6a06e1b0ff9fe8546c`) and `CheckboxIndicator.spec.md` (`4ee206165fdfe84a0005be03f16516e6b454e1ab`) against the complete Git tree at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`. Also verified the blobs for the owning `Indicator.doc.mjs`, `Indicator.stories.tsx`, `Indicator.test.tsx`, types and marker, `CheckIndicator`, `RadioIndicator`, `CheckboxInput`, `Spinner`, `isRenderable`, `mergeProps`, `themeProps`, theme tokens, `defineTheme`, registry, `useIndicator` and `useIndicatorFocusRing`. Also verified the CheckboxInput tests and indeterminate template and the theme target tests. The three named examples are inline in `Indicator.doc.mjs`; the pin contains no Indicator CLI templates.

## Contract and evidence

- The decorative span persists in all three states. The check SVG path and dash geometry match the source; CSS selects the visible mark. `sm` and `md` map to 20px and 24px; `md` and enabled are defaults. SSR covers all states and sizes, the canonical box/check/dash targets and owned data attributes. Prior Chrome geometry evidence for the unchanged base styles covers the empty box, 14px medium check and 10×2px small dash. The small mixed showcase renders the TSX example.
- The existing Rust slot maps rejected shallow React inputs to `None` at the call site (for instance, `is_loading.then(|| rsx! { Spinner { ... } })`). The component does not inspect a supplied `Element` or its descendants. SSR covers missing content, text, empty output and an empty wrapper. The showcase adds a decorative small Spinner using the inherited foreground.
- SSR confirms owned `aria-hidden`, no inherent role or tab stop, preserved `id`, `dir`, class and style, and a separate native checkbox with an associated label. Native attributes and `onmounted` are the existing Dioxus passthrough; there is no separate StyleX `xstyle` or React ref API. Replacement content must be decorative and non-focusable.
- The scoped restyle uses the canonical CSS target, the full-radius token, a 2px border and checked/disabled token paint. The BrandCheckbox demo uses direct composition, token paint, a single hidden root and children-first precedence. The single-selection demo contrasts checked/unchecked check and radio visuals; it does not change `CheckboxIndicator` states or substitute a theme registry.

## Integrated Chrome and offline source checks

PR #241 passed `tests/checkbox_indicator.browser.mjs` as one of eleven integrated Chrome suites, and passed full CI. The focused suite checked the state geometry, hidden attributes, Spinner precedence, and the scoped restyle's computed 2px border, rounded corners and distinct fills. It focused the native CheckboxInput programmatically, checked its associated label and the built-in indicator's solid outline, then used Space to check the input and update the indicator. It also verified that removing the demo's ancestor marker removes the hover fill, that reduced motion gives a computed `0s` transition, and that the check and dash computed colors differ from their box fills under Chrome forced-color emulation. These are DOM, interaction and computed-style checks, not speech, raster or upstream comparison results.

The offline source review rechecked the TSX, spec and family doc against the pinned Git blobs. The demo's box rules match the checkbox portion of the family restyle at the declaration level: full-radius token and 2px border, checked accent fill and border, and checked-disabled muted fill. The demo scopes those rules to one showcase row, not a theme override applied across hosts; the pinned family example also restyles the radio box and dot.

## What still blocks closure

- **Owner interaction:** pinned CheckboxInput puts `indicatorScope` on the enabled row and owns press paint. Rust `CheckboxInput` does not attach `data-astryx-indicator-scope` or implement upstream row press treatment. Its focus selector targets only the built-in indicator class, so it cannot verify a ring following an arbitrary replacement root's radius. The synthetic scoped-hover row and built-in focus check do not close those host gates.
- **Manual AT:** with a screen reader, verify that the native owner is exposed and announced once by name and state (including mixed), and that the decorative marks and replacement Spinner are not announced separately. The Chrome suite does not exercise speech or mixed/busy owner announcements.
- **Theme and Selector:** Rust `Theme` has no `indicators.checkbox` registry and hosts have no `useIndicator` equivalent to consume it. This blocks theme-installed BrandCheckbox and replacement-root focus verification. The radio example additionally requires the `indicators.check` override consumed by a rich Selector option list; native Selector has no such path. The scoped restyle and direct-composition demos do not prove the family-wide theme examples.
- **Rendered comparison:** no rendered upstream comparison for the state matrix and restyle was available in the pinned fixtures. Side-by-side rendering and raster evidence for focus and forced-color visibility remain open, as does verification outside the integrated Chrome run. Verified source declarations and computed CSS do not substitute for that evidence.

Local checks: all four focused SSR tests passed via the assigned offline `cargo-slot`; `cargo fmt --check`, `python3 scripts/bundle_styles.py --check`, and `node --check tests/checkbox_indicator.browser.mjs` passed. No preview or browser was started, and full CI was not rerun locally.

```sh
SHOWCASE_URL=http://127.0.0.1:8122 node tests/checkbox_indicator.browser.mjs
```
