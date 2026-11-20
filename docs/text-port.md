# Text checkpoint and verification

**Not marked complete.** Typography, measurement, the approved shared truncation
interest lifecycle and private demand-loaded Tooltip/Layer presentation subset are
implemented. See [presentation verification](tooltip-presentation.md) for loading,
native anchor fallback, typography and exit/media evidence. This is not a full
public Tooltip/Layer, Table, Stack or Theme port.

## Authority and ownership

Authority: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, not the
published catalog. Relevant downloaded files were verified against `tree.json`
using Git blob SHA-1 (`sha1("blob " + byte_length + NUL + bytes)`). Examples:

| Pinned path | Git blob SHA-1 |
| --- | --- |
| `packages/core/src/Text/Text.tsx` | `2ebc2776ec25710c70e5f958a41e5cc2b488bd27` |
| `packages/core/src/Text/text.stylex.ts` | `392fa2ada66c179564cbea02878676b981d138d0` |
| `packages/core/src/Text/useTruncation.ts` | `b0c33dad2906c6dcf66ade0c48124faf53ba9673` |
| `packages/core/src/Text/TruncationTooltip.tsx` | `93a94b66d587718e4aa5a454a5233cfad765e678` |
| `packages/core/src/Text/Text.test.tsx` | `aaf1fc1aaed5595b1132bfd32dd65dc0a97b3cba` |
| `packages/core/src/Text/useTruncation.test.tsx` | `11dd75b5ee787ce555021e4dbb230558018f9096` |
| `docs/specs/components/Text/Text.spec.md` | `feb975df6bc18211455d6d9ad006d92e3be9f94b` |
| `packages/core/src/theme/tokens.stylex.ts` | `ec6ecd40da08501602ad730274d028d79640a1c9` |
| `packages/core/src/utils/sharedResizeObserver.ts` | `a86684be9898fff43b30832f7302aad1e10f26d5` |
| `packages/core/src/utils/sharedMutationObserver.ts` | `1dbe9809e42189ea4f885c4b48646b22875d000e` |
| `packages/core/src/Layer/useLayer.tsx` | `46a979075030e09b37bb6cf853d7194bef210470` |
| `packages/core/src/Tooltip/Tooltip.tsx` | `02158c36794e99c9eb973cd427df8628b72c571e` |
| `packages/core/src/Tooltip/useTooltip.tsx` | `bed87b68b5542c1f7e1177d8bc544b22ad2ee16d` |
| `packages/core/src/Tooltip/Tooltip.test.tsx` | `2c419549ec59d3430c2c409e15f426a0218c9513` |
| `packages/core/src/Tooltip/useTooltip.test.tsx` | `31c5a501594287a83989070b641810b64f5b2671` |
| `packages/core/src/Button/Button.truncationTooltip.test.tsx` | `75d682af985845b493140aada19488c330614f66` |
| `packages/core/src/Layer/useTouchTrigger.ts` | `60aeb0d767597b58786b65d57571734352eb8e26` |
| `packages/core/src/utils/triggerFocus.ts` | `3a637670be96ad02ffcbde8f4d72304885edd826` |
| `docs/architecture/public-component-api.md` | `6d3ff5868cbeb2c8af61714afc0a10c40749f6d8` |
| `docs/architecture/component-theming-surface.md` | `83deab857ec9587d647a2c03ebaa3b66c9ac5083` |

`src/text/` owns typography mapping, root semantics, measurement and the existing
clamp context. Heading is separate and uses the shared color resolver and hook.
`src/theme/types.rs` retains open validated TextType/TextColor names. Shared
observers remain under `src/utils/`; LayerPlacement belongs under `src/layer/`,
not Text. `text/truncation_tooltip.js` owns first-interest retention, measured
content and the private Button cut-text adapter, matching TruncationTooltip.tsx.
Tooltip owns presentation, descriptions, focus decisions and open/closed state.
`utils/trigger_focus.js` owns auto focus judgement; Layer's private gesture/touch
adapters own completed-click recognition, action judgement and outside dismissal.
The renderer bridge delegates native events through these owners instead of
introducing React's present-interest context or remounting children.

## Complete prop inventory / Rust adapters

| Source | Rust / default |
| --- | --- |
| children | `Element`, required; rich children stay mounted across clamp changes |
| type | `text_type: TextType`, body; built-ins body/large/label/supporting/code/display-1/2/3/inherit; custom names use body baseline |
| size | `Option<TextSize>`, none; all eleven 4xs through 4xl options, source `xsm` spelling retained; only size changes |
| color | `Option<TextColor>`, supporting→secondary, inherit→inherit, otherwise primary; custom colors use primary baseline |
| weight | `Option<TextWeight>`, none; normal/medium/semibold/bold |
| display | `TextDisplay`, inline; clamp/capsize precedence retained |
| maxLines | `max_lines: u32`, 0; typed nonnegative integer, not JS fractions/NaN |
| hasTruncateTooltip | Existing `has_truncate_tooltip: bool`, true; **not retyped** |
| placement string alternative | Additive `truncate_tooltip_placement: Option<LayerPlacement>`; above default; false still disables it; above/below/start/end, start/end follow direction |
| wordBreak | `Option<WordBreak>`, none; truncation-only break-all for one line, break-word otherwise |
| textWrap | `Option<TextWrap>`, none; wrap/nowrap/balance/pretty |
| justify | `TextJustify`, start; default sets no alignment, preserving inheritance |
| hasCapsize | `has_capsize: bool`, false; native text-box trim, block display removes multiline clamp only |
| hasStrikethrough | `has_strikethrough: bool`, false |
| hasTabularNumbers | `has_tabular_numbers: bool`, false |
| as | `tag: TextTag`, span; p/div/label/h1/h2/h3 supported |
| ref | `onmounted: EventHandler<MountedEvent>` reaches the native root; framework-native adapter, not a React ref type |
| xstyle/className/style | Existing `class` and `style`; no StyleX runtime; stylesheet/class cascade followed by inline CSS, then owned dynamic line clamp |
| DOM/ARIA/data | Existing GlobalAttributes extension and raw attributes vector; `html_for` associates labels; owned typography/controller metadata protected |
| events | Existing mouse/pointer/focus/keyboard/touch handlers plus explicit clipboard/composition/drag/form/selection/scroll/wheel/animation/transition/toggle/image/media handlers |

Dioxus's native event registry is not React's: capture-suffixed props,
`beforeinput`, `animationcancel`, and transition-start/run/cancel convenience
callbacks are not implemented. Existing raw-attribute escape hatches remain;
this is not a claim of exact React event API parity. No existing prop is removed.

`use_truncation(id: String, max_lines: u32, is_expanded: bool) -> Truncation`
returns `onmounted`, `is_truncated: ReadSignal<bool>`, and
`full_text: ReadSignal<String>`. Its root carries the matching private controller
id. It installs no observers for zero lines, waits for the shared resize
observer's first post-layout notification, measures already-observed roots in a
guarded microtask, and shares content observation. Single-line overflow compares
scrollWidth/offsetWidth; multiline uses a Range/offsetHeight, with scrollHeight
fallback. Expanded mode keeps the last clamped answer without measuring;
replacing that root discards the kept answer when clamped again. Shared removal
observation replaces ref-detach cleanup, and scope drop releases the channel.

Theme tokens/styles already present in `assets/astryx.css` match the pinned
baseline; this change does not reorganize or replace them. Text consumes rem
sizes, unitless leading, weights, colors and code family from the active scope.
CSS variable overrides remain responsive to root font size and theme conditions;
there is no invented Text-specific breakpoint. A complete upstream Theme compiler
or iOS Dynamic Type runtime is not claimed. `overflow-wrap` preserves the
container's private anywhere override and otherwise uses break-word; callers
still own flex `min-width:0`. Forced colors use native text behavior, not a new
palette. Optical text-box trim depends on browser support.

## Original template inventory / offline appearance evidence

Pinned `packages/cli/assets/templates/blocks/components/Text/`:

- TextShowcase (5 samples), TextTypes (7 treatments + supporting captions),
  TextColors (5), TextWeight (4), TextInline (nested code), TextTruncation (1/2/3
  lines), TextWrap (4), TextWordBreak (2): reproduced in
  `examples/showcase/text.rs` using native vertical frames instead of claiming
  a Stack port. 31 principal Text roots are compared in light/dark using
  `tests/fixtures/text-source-reference.html` and the bundled Figtree font.
- TextHeadingLevels: six headings, owned by Heading; not duplicated in Text.
- useTruncationHookUsage: original three file paths and one-/two-line hook use
  appear in a native table adapter. The original Tooltip/Table composition is
  **not** faithfully ported here because neither public component is complete.

The checked-in HTML fixture reconstructs the original Text-only templates and
StyleX/default-token declarations directly from the pin; it is **not an
independently executed upstream React/StyleX build**, a published-catalog capture,
or a screenshot certification. It checks content, computed paint/typography,
root/line geometry and native frames in the same engine offline. It uses no live
network fonts. Exact upstream compiled appearance and other theme outputs remain
unverified; source fixtures must not be represented as catalog equivalence.

## Approved lifecycle follow-up and remaining fidelity limits

The user explicitly approved shared truncation ownership work and Button/Link
regression scope. Implemented contracts:

- No initial layer or owned description. First hover mounts a closed layer at
  once, opens after 200ms, and leaving early cancels opening without unmounting.
  Escape/blur/dismissal retain that closed layer and its description. Losing
  overflow, disabling, or removal unmounts it; first interest survives a
  same-root disable/enable or overflow interval.
- Tooltip composes its owned `aria-describedby` token with caller tokens, even
  when its text repeats the name. Cleanup removes only its own token, preserving
  descriptions added after attach. Explicit caller tooltip precedence remains.
- First keyboard focus inside unfocusable Text mounts closed but does not open.
  Auto focus is judged on arrival against the actual trigger, not a focusable
  ancestor; counted focus still closes after its trigger leaves the tab order.
  Standalone Text's source-documented keyboard reachability gap is not fixed by
  inventing a tab stop.
- Touch/pen down and up alone open nothing. The completed click toggles; a
  keyboard/new press, movement, scroll or cancellation invalidates stale taps.
  A pen press cancels its pending hover. Outside release/cancel dismisses a
  tap-opened layer, not outside down. Text-entry taps do not open/close it or
  steal focus. Native caller preventDefault does not veto interest tracking.
- Button's private Text-owned cut-label adapter measures visible, non-field
  content only on keyboard focus. Hidden duplicates are excluded. It stays
  undescribed and unmounts on blur/Escape/press; explicit tooltips win. Existing
  explicit Tooltip, StatusDot/ProgressBar policy, native-modal/portal and
  announcements remain on their existing paths.

The presentation follow-up replaces the former bundled-code limit with two
separate Dioxus asset modules under Tooltip and Layer. Overflow preloads them
before interest; pending hover/focus/completed-tap state is held safely, cancelled
on dismissal, and discarded after disable/overflow loss/removal. Failed imports
recover on later interest through a fresh module URL. The existing shared
explicit Tooltip and Button cut-label paths remain unchanged.

Private truncation surfaces now use source body typography, native CSS anchor
placement/fallback, modal-local hosting and the source entrance/reduced-motion
rules. Default close is immediate (the source declares no default exit);
consumer discrete-display exits keep their anchor until finite animation
completion. Full public Layer/Tooltip APIs, software-keyboard clearance, theme
compiler equivalence and independently executed upstream React comparison are
not claimed. See [presentation verification](tooltip-presentation.md) for exact
scope, source hashes, offline network/browser evidence and regression results.

## Verification

Run from the project root with `CARGO_INCREMENTAL=0`; cargo checks used the
shared absolute target directory. All five standard checks passed:
`cargo fmt --check`, `cargo check --locked --lib --no-default-features`,
`cargo test --locked`, `cargo clippy --locked --all-targets -- -D warnings`, and
`cargo check --locked --example showcase --target wasm32-unknown-unknown`.
The isolated `text_patterns` wasm example check also passed.

```sh
# Own preview only, never stop another user's server:
CARGO_INCREMENTAL=0 dx serve --example text_patterns --web --port 8085 --open false --hot-reload false
python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/text.browser.mjs
python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/text_truncation.browser.mjs
python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/text_tooltip.browser.mjs
node tests/text.contrast.mjs
```

The Text browser suites passed in isolated Chrome targets. The added
`tests/text_tooltip.browser.mjs` tests production owner scripts offline for
initial/retained DOM, description-token composition and cleanup, first focus
mount/actual focus gate, roving blur, completed touch/pen click/repeated toggle,
stale/cancelled/scrolled taps, text-entry exclusion, outside release, overflow
and disable intervals, and Button's undescribed visible-label adapter. They cover shared
observer scheduling/content/resize/removal/expanded state/fallback/peer cleanup,
31 original-pattern fixture comparisons, logical placement in LTR/RTL, live
content and tooltip toggles, stable clamp root, native mounted/click/attributes,
nested Link clamp behavior, size/leading override, AX presence, no eager
layer/title/tabstop, narrow ordinary-flow wrapping and root-font scaling.
Touch/pen are synthetic browser sequences, **not** real-device evidence.

The source-palette audit intentionally exits 1: **26/160 conditional active-text
findings**, all using disabled ink. Text's disabled color does not actually
inactivate content; only genuinely inactive content has the normal contrast
exemption. Original colors are retained, not silently corrected. Inherit/custom
colors and arbitrary consumer backdrops/size overrides are excluded.

Unrun: independently built upstream React appearance, real-device touch/pen,
VoiceOver/NVDA/JAWS and manual keyboard/SR audit, Firefox/WebKit, real Windows High
Contrast, iOS Dynamic Type, separate `native_event_boundary` and
`interactive_card_boundaries` preview suites, and combined Heading/Code integration. Parent owns
combined integration checks. These passing focused checks do not imply WCAG
compliance or resolution of the framework/presentation limits above.

Follow-up regression runs used this worktree's showcase on **8085**, never the
parent's 8080, with every CDP command under `browser_lock.py`. Button, Link,
ButtonGroup, IconButton, StatusDot, StatusDot's internal Tooltip suite,
ProgressBar's dedicated motion suite, ClickableCard, SelectableCard, its events/
press/layer suites, container press, Badge, Card, Divider, Kbd, Spinner, its rotor
suite and announcements passed. A Link launch attempted during a dx rebuild timed out waiting for
registration; rerunning after the build completed passed (not an automatic retry).
ProgressBar's main browser suite failed its idle trace assertion
`UpdateLayoutTree <= 2`; earlier assertions reached that point. The trace is
browser-wide and the cause is not established; this is **not** labeled a
pre-existing failure or a passing suite. The threshold was not changed and the
suite was not retried to hide it. Skeleton likewise failed its idle trace
assertion `no per-frame style loop`; it was not retried, weakened or labeled
pre-existing. Spinner's independent idle trace did pass. The full browser suite
therefore does **not** have an all-green result.

The integrated Text/Heading/Code branch passed the five standard Rust checks
and the Button and Link browser suites. Skeleton and ProgressBar reproduced
those same idle-trace failures on the integrated branch, including in a fresh
Chrome profile. Both suites passed on the built, unchanged `de1d606` baseline
in the shared Chrome session. The cause was default no-op animation callbacks registering delegated listeners
on the page. Text and Heading now register their new animation callbacks only
when supplied. On the final combined preview, the unchanged Skeleton and
ProgressBar suites passed, along with Button, Link, and the new animation
registration and callback-delivery regression suite.
