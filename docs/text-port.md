# Text checkpoint and verification

**Not marked complete.** Static typography and the Text-owned truncation hook are
implemented and tested, but the existing shared Tooltip bridge differs from the
pin. No public Tooltip, Table, Stack, Theme, Heading or Code port is claimed here.

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
| `docs/architecture/public-component-api.md` | `6d3ff5868cbeb2c8af61714afc0a10c40749f6d8` |
| `docs/architecture/component-theming-surface.md` | `83deab857ec9587d647a2c03ebaa3b66c9ac5083` |

`src/text/` owns typography mapping, root semantics, measurement and the existing
clamp context. Heading is separate and uses the shared color resolver and hook.
`src/theme/types.rs` retains open validated TextType/TextColor names. Shared
observers remain under `src/utils/`; LayerPlacement belongs under `src/layer/`,
not Text. Tooltip owns presentation, interest timing, descriptions and dismissal.
Button's existing cut-text tooltip integration is not moved into Text in this
checkpoint: faithfully relocating that source-owned internal hook needs a
separate approved shared-owner follow-up, not a claim that Text owns Tooltip.

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

## Open pin-fidelity blockers — no completion checkbox

The existing shared automatic-tooltip bridge differs from Text's pin:

1. It deliberately omits automatic tooltip descriptions and removes the layer
   when hidden. Pinned Text tests expect Tooltip-owned `aria-describedby` after
   first interest and a retained mounted (possibly closed) layer thereafter.
2. Focus inside an unfocusable Text opens the current bridge's tooltip; source
   tests expect the first focus to mount a closed Tooltip, whose own trigger
   gate opens nothing. Standalone Text is unfocusable in both: the source spec
   explicitly records that keyboard-reachability gap. No new tab stop is added.
3. The current bridge completes touch/pen taps on pointerup, rather than the
   source recognizer's completed click, and repeated taps do not implement the
   same toggle. Drag/scroll/cancel and text-entry exclusion are tested, but this
   does not settle full gesture semantics.
4. Tooltip JavaScript is part of the current shared renderer bridge, not an
   independently lazy-loaded public Tooltip module. Tooltip layers themselves
   remain absent until interest.

A parent decision is required on faithful shared Tooltip/interest ownership work
versus an explicit accepted exception. This checkpoint does not silently decide
that policy, claim full Text completion, or claim public Tooltip migration.

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
node tests/text.contrast.mjs
```

Both Text browser suites passed in isolated Chrome targets. They cover shared
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
Contrast, iOS Dynamic Type, and the full pre-existing browser suite. Parent owns
combined integration checks. These passing focused checks do not imply WCAG
compliance or resolution of the blockers above.
