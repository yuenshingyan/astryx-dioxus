# Heading port checkpoint

**Not marked complete:** Heading-specific implementation and evidence are present,
including the integrated shared Tooltip lifecycle and private demand-loaded
presentation subset. See [presentation verification](tooltip-presentation.md).
No full public Tooltip/Layer port, independently executed upstream appearance
comparison, or full accessibility conformance is claimed.

## Authority and ownership

Source pin: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`.
Cached files were independently checked using Git blob SHA1 against the pinned
`tree.json`, not assumed equivalent to the published catalog.

| Source | Verified blob |
| --- | --- |
| `Heading/Heading.tsx` | `de84c5ad97c8c38ea6377dffed8569d76a5be6f3` |
| `Heading/Heading.test.tsx` | `6f7e32bec2e69afe85ad6bf8a0e4e235b71f8ef4` |
| `Text/Heading.doc.mjs` | `df602a522d83e580004a0ef9778105cd3dd43e47` |
| `Text/text.stylex.ts` | `392fa2ada66c179564cbea02878676b981d138d0` |
| `theme/tokens.stylex.ts` | `ec6ecd40da08501602ad730274d028d79640a1c9` |
| `docs/specs/components/Heading/Heading.spec.md` | `db61bd93786a6554c34db04e4d67444e27e71441` |
| `HeadingShowcase.tsx` | `74398a7fc87acf962528968fad86c0479e2a69ad` |
| `HeadingPageLayout.tsx` | `efcb765c37485e8806f1e9bad9d9360bfddba5c5` |
| `HeadingCardGrid.tsx` | `2038cdf1a2979c30ce6ab39f17c5ada59c46cf93` |
| `HeadingTruncation.tsx` | `1c49489730507e9bee333511eb5e703bd9e8017b` |
| `TextHeadingLevels.tsx` | `cf785c212168099d4ba5fdc1cbf7a43a22cfe714` |

Also verified the pinned neutral theme, reset, theme generator/types, Heading
storybook and theming/public-API architecture records. Heading owns its native
root, level/type, target metadata and composition. It consumes the **exact** Text
measurement/color implementation from `3daab94fa35d9f042b5aac3eeee3bbdb7ed0bfda`
(ancestor `cb9472a272eb9d319a353f3860357a0a307d7704`), existing shared typography
classes and Tooltip bridge. No copied measurement, Tooltip, shared observer,
color policy or new dependency. The file is `src/heading/heading.rs`; `component`
is the Rust module alias already used by other component owners to avoid module
inception, not a moved ownership boundary.

## Complete prop inventory / adapters

| Source | Rust / default |
| --- | --- |
| required `level` | `HeadingLevel::{One,Two,Three,Four,Five,Six}`; `TryFrom<u8>` rejects outside 1–6 |
| required `children` | rich Dioxus `Element`, caller-owned |
| `type` | `heading_type: Option<HeadingType>` unset; `DISPLAY_1/2/3`, validated open `new` |
| `weight` | `Option<TextWeight>` unset; explicit normal/medium/semibold/bold beats display/level |
| `accessibilityLevel` | `accessibility_level: Option<HeadingLevel>` unset; differing value only adds `aria-level` |
| `color` | `TextColor::PRIMARY`; custom names keep primary baseline |
| `display` | `TextDisplay::Block` |
| `maxLines` | `max_lines: u32 = 0`; no JS negative/fractional/NaN values |
| `hasTruncateTooltip` | existing shared convention `has_truncate_tooltip: bool = true`, plus `truncate_tooltip_placement: Option<LayerPlacement>` unset/above; false wins |
| `wordBreak` | `word_break: Option<WordBreak>`; break-all for one line, break-word for multiline; ignored without truncation |
| `textWrap` | `text_wrap: Option<TextWrap>` unset; wrap/nowrap/balance/pretty |
| `justify` | `TextJustify::Start`; no alignment declaration for start, logical center/end otherwise |
| `hasCapsize` | `has_capsize: bool = false`; shared cap-height trim, block display wins over multiline clamp |
| `hasStrikethrough` | `has_strikethrough: bool = false` |
| `className` | `class: String` empty; appended |
| `style`, `xstyle` | native inline `style: String` empty and consumer CSS/classes instead of StyleX values; dynamic clamp appended last |
| `ref` | chained `onmounted: EventHandler<MountedEvent>` reaches same native root, not React attach/detach callback |
| HTML/ARIA/data props | `GlobalAttributes` spread; caller `aria-level` wins over derived value without duplicate SSR attributes |
| DOM events | explicit Dioxus mouse/pointer/capture/focus/key/touch, clipboard/composition/drag/form/select/scroll/wheel, animation/transition, toggle/load/error/media callbacks |

Dioxus's generated native event registry does not expose React capture-suffixed
convenience props, beforeinput, animationcancel or transition-start/run/cancel.
Exact React API parity is not claimed. Reserved generated class/style and
Heading theme/private measurement metadata cannot be replaced through the raw
attribute vector; use their typed props. Arbitrary custom names follow the shared
validated identifier policy rather than introducing a Heading-specific policy.

## Layout, tokens and target

`astryx-heading` reflects `data-level`, `data-color`, optional `data-type` and
`data-weight`. Any visual type preserves h1–h6 semantics; a custom type preserves
the level baseline until consumer CSS paints it. Explicit weight wins. New
Heading-only CSS fills missing level 1/2/5/6 tokens and all six baseline rules,
before existing Text display/weight classes. No shared CSS reorganization or
replacement of Text tokens. Root margin normalization mirrors upstream reset
locally, without adding a global reset. Heading family uses the heading token.

At 16px root the default scales are 24/20/17/14/12/10px with leading
1.3333/1.4/1.4118/1.4286/1.6667/1.6 and semibold weight. Existing neutral theme
h3/h4 bold overrides remain intact. Display scales retain shared 42/35/29px normal
weight defaults. Rem/token overrides and root-font scaling remain responsive;
there is no invented breakpoint, full theme compiler, or iOS Dynamic Type claim.

Unclamped tokens wrap with `break-word` (the existing private container override
can supply `anywhere`). Caller flex children still need `min-width:0`; ordinary
`overflow-wrap:anywhere` alone does not pass through. One-line truncation is block
with ellipsis; multiline uses the clamp box unless capsize forces block. Full rich
content remains in DOM. User style is preserved except the dynamic clamp wins.

## Shared Tooltip boundary and source gaps

Hooks run unconditionally; disabled tooltips pass zero lines to exact shared
measurement, install no measurement controller, and still clip by CSS. Mounted
chaining runs measurement, shared Tooltip registration, then caller. Source-owned
LayerPlacement supplies above/below/start/end. No native title, tab stop or eager
Tooltip layer/description is added by Heading.

The approved shared follow-up adds the description on first interest, keeps the
closed tooltip mounted, gates focus on the actual trigger, and uses completed
touch/pen click to toggle. Heading's browser suite verifies that dismissal closes
the tooltip without removing its description target, and checks logical placement
of the open tooltip in both directions. Heading does not duplicate this behavior.
See [presentation verification](tooltip-presentation.md) for separate module-loading,
native placement/fallback, body typography and exit/reduced-motion evidence and limits.

The source itself records a keyboard reachability gap: a plain truncated Heading
is not focusable, so sighted keyboard users cannot reveal the clipped text. Full
text remains accessible in the DOM. No extra tab stop is silently invented.
Capsize can remove multiline clipping by design. Caller outline overrides can
produce an illogical document hierarchy; callers own content/outline choices.

## Original patterns and offline evidence

`examples/showcase/heading.rs` implements all four Heading templates plus Text's
HeadingLevels pattern (18 principal heading roots), shared by the showcase and
isolated `heading_patterns` preview. Layout/Stack are native div adapters with the
original spacing, not public Layout/Stack port claims. CardGrid uses existing Card
with its previously approved padding correction, not the source's known padding
overshoot. No made-up replacement patterns. Additional probes cover display types,
custom themes, explicit weight, outline, capsizing, wrap/decoration, placement,
disabled measurement, native events, reactive clamp and uncontrolled descendants.

`tests/fixtures/heading-source-reference.html` independently reconstructs the five
pinned patterns from StyleX/default tokens. The browser compares all 18 heading
roots in light/dark for text, native tag, root width/height, size/leading/weight,
family/color/display/wrap/ellipsis/clamp. Figtree is bundled locally and explicitly
set as the preview heading font. This is a **source-derived fixture**, not an
independently executed React/StyleX build, screenshot-perfect upstream proof, or
proof the live catalog equals the pin.

## Verification

Run from project root with `CARGO_INCREMENTAL=0`; shared Cargo target is practical:

```sh
cargo fmt --check
cargo check --locked --lib --no-default-features
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo check --locked --example showcase --target wasm32-unknown-unknown
cargo check --locked --example heading_patterns --target wasm32-unknown-unknown
```

Serve only your own preview on 8086 (`dx serve --example heading_patterns --web
--port 8086 --open false`) and serialize isolated CDP targets:

```sh
python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/heading.browser.mjs
node tests/heading.contrast.mjs
```

Focused SSR: levels/validation, built-in/custom type/color baselines, weight,
outline overrides across all 36 native/outline level pairs, raw caller precedence,
clamp/inline style precedence, shared layout treatments, all placements, rich
content and native passthrough. The typed and caller-provided `aria-level` now
share one Dioxus attribute slot: removing the caller value restores the typed
value on the same native root instead of deleting it after the update. The
mutation regression covers switching in both directions and clearing the
attribute when the typed level matches the native level. Neither SSR nor these
mutations verify assistive-technology speech.

Prior Chromium evidence: source-derived appearance above; neutral/custom theme
paint, AX native heading role/levels, no eager layer/title/tabstop, native mounted/
click/wheel/copy/animation/transition/drag handlers, stable native root and uncontrolled input/focus
across clamp changes, one/multiline overflow, first hover full text, logical end
placement LTR/RTL, disabled zero width reads/no controller, 320px ordinary reflow
and root-font scaling.

Contrast audit intentionally exits 1: **28/180 conditional active-text failures**,
all using retained disabled ink. Heading's color alone never makes it inactive;
no inactive exemption is claimed. Core default levels/display types on page and
surface in light/dark are modeled. Custom/inherit paint, arbitrary overrides and
image backdrops are unmodeled. Source palette is preserved, not certified.

Unrun: upstream React/StyleX execution; independent pixel/screenshot comparison;
manual screen readers; real-device touch/pen; Firefox/WebKit; Windows High
Contrast; iOS Dynamic Type; full existing browser regression suite; combined
Heading/Code/final shared Tooltip integration. Parent owns final combined checks.
The Heading port remains unchecked while shared fidelity work remains. The
`accessibilityLevel` checklist row covers only the verified outline attribute
contract.
