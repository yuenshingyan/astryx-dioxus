# Astryx Dioxus

Reusable Astryx UI components for Dioxus 0.7.

## Status

Button implements the upstream variants, sizes, loading/actions, slots,
tooltips, touch behavior, link/form modes, and shared size/icon/spinner/group
integration. The showcase includes the documented Button patterns and a locally
bundled Figtree font. See [Button implementation and verification](docs/button-port.md)
for source references, tested behavior, API adapters, and verification boundaries.
The [component checklist](COMPONENT_CHECKLIST.md) records verification separately;
ButtonGroup now implements connected geometry, inherited sizing/disabled state,
shared elevation, roving focus, layer isolation, and touch/pen tracking. All three
named patterns and interactive regression cases are in the showcase; see
[ButtonGroup verification](docs/button-group-port.md). IconButton's standalone
audit now covers its thin Button API, five named examples, inherited theme aspect,
state/geometry matrix, and native interactions; see
[IconButton verification](docs/icon-button-port.md). Link adds native/action/disabled
forms, safe optional destinations, external announcements, typography/clamping,
provider routing, linkification, and all six named patterns; see
[Link verification](docs/link-port.md). `LinkRenderProps.href` is now `Option<String>`
(an approved API correction); native attribute forwarding still works, while
string-specific router logic must handle `None`. Spinner now covers standalone
labels, all four original examples, shared announcements, theme geometry, and
batched compositor motion; see [Spinner verification](docs/spinner-port.md).
Badge adds all 14 tones, extensible theme names, static scalar/rich labels,
truncation, bidirectional isolation and all four original patterns; see
[Badge verification](docs/badge-port.md). Divider adds named separators, both
axes/weights, logical full bleed and all four original patterns; see
[Divider verification](docs/divider-port.md). Kbd adds platform-aware keycaps,
spoken/localized shortcut names and all four original display-only patterns;
see [Kbd verification](docs/kbd-port.md). Skeleton adds CSS-only loading shapes,
all seven radii, staggered compositor-safe opacity pulses, contrast/motion
preferences and all four original patterns; see [Skeleton verification](docs/skeleton-port.md).
StatusDot adds five extensible tones, caller icons, all four original patterns,
and spec-led standalone/nested explanatory tooltips; its approved custom fallback,
forced-color treatment and preserved pulse-contrast findings are recorded in
[StatusDot verification](docs/status-dot-port.md). ProgressBar adds determinate/unknown
progress, localized values, explanatory marks, theme-timed retargeting and all five
original patterns. Approved graphic boundaries/custom fallback and retained disabled-marker
findings are recorded in [ProgressBar verification](docs/progress-bar-port.md).
Card adds the static surface, all thirteen tones/four elevations, shared sizing/spacing,
corrected applied-padding publication and six static patterns plus the phone-row example;
see [Card verification](docs/card-port.md). ClickableCard adds native action/navigation,
independent nested controls, disabled focusability and all three original patterns;
see [ClickableCard verification](docs/clickable-card-port.md) for callback adaptations,
spec-led press corrections and retained contrast findings. SelectableCard adds controlled
native input behavior, released layer capture and all three original patterns; shared
press/event integration, the private native-radio proof and the approved forced-color
selection cue are verified. See [SelectableCard verification](docs/selectable-card-port.md)
for callback adaptations, source-palette findings and verification boundaries.
Semantic inline Code now has closed color/inherited-size props, native root events,
shared typography tokens and an offline four-pattern example; see
[Code verification](docs/code-port.md). Shared Text/Heading frame integration and
unrun engine/device/screen-reader checks remain explicitly separate.
Text adds shared post-layout/content truncation measurement, placement and offline
source-derived Text-only examples. The approved shared follow-up adds retained
first-interest tooltips, owned descriptions, actual-trigger focus gating and
completed touch/pen toggling while preserving Button's undescribed cut-label path.
Private truncation presentation uses independently downloaded Dioxus assets,
native CSS anchors, source body typography and reduced-motion/exit semantics;
see [presentation verification](docs/tooltip-presentation.md) for the tested internal
subset and remaining full Tooltip/Layer limits. Text is not marked complete.
See [Text verification](docs/text-port.md).
Blockquote adds semantic quotes with shallow author/source attribution, logical
RTL rules, and all three original patterns including existing Card testimonials;
see [Blockquote verification](docs/blockquote-port.md) for source-derived offline
comparisons and evidence boundaries.
AspectRatio adds required numeric ratios, both clipping shapes, optional child fit,
responsive CSS overrides and all six original patterns using offline replacement
artwork; see [AspectRatio verification](docs/aspect-ratio-port.md) for pinned-source
comparisons and remaining release/asset boundaries.
Grid adds native fixed and responsive tracks, gap overrides, and five showcase patterns.
See [Grid verification](docs/grid-port.md) for tested behavior and demo adaptations.
[GridSpan](docs/grid-span-port.md) adds numeric, full-width, and row spans with both original showcase patterns.
[Center](docs/center-port.md) adds flex-axis centering and logical padding with all three original patterns.
[Section](docs/section-port.md) adds themeable surfaces, dividers, and nested padding with all three original patterns.
[LayoutHeader](docs/layout-header-port.md) and [LayoutFooter](docs/layout-footer-port.md) add header and footer bands with inherited padding, a content-width cap, and optional dividers.
[CheckIndicator](docs/check-indicator-port.md) adds a decorative selection mark and a same-size replacement slot.
[RadioIndicator](docs/radio-indicator-port.md) adds a decorative radio circle with a replaceable selection dot.
[CheckboxIndicator](docs/checkbox-indicator-port.md) adds a decorative checkbox with checked and mixed marks and a replacement slot.
[AvatarStatusDot](docs/avatar-status-dot-port.md) supports standalone dots; Avatar integration remains open.
Stack implements axis/alias resolution, shared token gaps, local logical padding,
sizing precedence, wrapping, scrolling and caller-rendered native roots. Its SSR and
offline source-derived comparisons plus actual published captures/isolated layout
replay are recorded in [Stack verification](docs/stack-port.md). Private upstream-owned
Token and Avatar slices and the public [StackItem](docs/stack-item-port.md) render the original patterns, including named Avatar
keyboard/hover tooltips and observable published action messages. Actual Rust screenshots,
intrinsic geometry and paint are compared against captured Neutral-dark originals;
a same-node raster control isolates the historical transform-induced differences.
Layout-positioned screenshots are identical for Directions and differ by at most one
channel level for Alignment/Fill. The [private Stack Toast slice](docs/stack-toast-port.md)
presents the published action messages; its open geometry, paint and selected interactions
are checked in Chrome. This does not complete public Toast, Token, Avatar,
Grid or Layout migrations.
Supporting layer-boundary
utilities do not complete the standalone Layer migration. Unrelated components
and unrun audits are not marked complete.
Heading now has native h1–h6, independent display types/weight/outline controls,
all four original Heading patterns plus TextHeadingLevels, and focused SSR/offline
Chromium evidence. It shares Text’s retained tooltip lifecycle and opt-in animation
callbacks and demand-loaded private presentation; remaining verification limits
are documented in [Heading verification](docs/heading-port.md).
Publishing remains disabled until package metadata and licensing are confirmed.
See [third-party notices](THIRD_PARTY_NOTICES.md) for the upstream MIT license.

## Run the showcase

Requires Rust (edition 2024), Dioxus CLI 0.7, and the WebAssembly target:

```sh
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --version '~0.7' --locked

dx serve --example showcase --web
```

For the isolated inline Code patterns (not CodeBlock), run
`dx serve --example code_patterns --web --port 8087`.

## Use the library locally

```toml
[dependencies]
astryx-dioxus = { path = "../astryx-dioxus" }
dioxus = { version = "0.7.3", features = ["web"] }
```

```rust
use astryx_dioxus::{Button, STYLES};
use dioxus::prelude::*;

#[component]
fn App() -> Element {
    rsx! {
        document::Style { "{STYLES}" }
        Button { label: "Save", onclick: move |_| println!("clicked") }
    }
}
```

The library does not select a renderer. The showcase uses the web renderer.
CSS styles target HTML/webview output, not a native renderer.

## Layout

- `src/aspect_ratio/`: CSS-only ratio box and fit slot; child semantics and operations stay caller-owned.
- `src/badge/`, `src/button/`, `src/button_group/`, `src/card/`, `src/clickable_card/`, `src/divider/`, `src/icon/`, `src/icon_button/`, `src/kbd/`, `src/progress_bar/`, `src/skeleton/`, `src/spinner/`, `src/status_dot/`: upstream-owned component modules, named using Rust conventions.
- `src/blockquote/`: semantic quote and bare attribution; no layout/typography dependency.
- `src/toggle_button/`: [ToggleButton](docs/toggle-button-port.md) and [ToggleButtonGroup](docs/toggle-button-group-port.md), with controlled single and multiple selection. Group touch tracking and asynchronous toggle actions remain pending.
- `src/selectable_card/`: controlled Card selection and its private native input context; not a completed Lab group migration.
- `src/stack/`: Stack and its axis/wrap types, plus StackItem.
- `src/h_stack/`: [HStack](docs/h-stack-port.md), the horizontal Stack wrapper with both original patterns.
- `src/v_stack/`: [VStack](docs/v-stack-port.md), the vertical Stack wrapper with both original patterns.
- `src/avatar/`: [Avatar](docs/avatar-port.md) and [AvatarStatusDot](docs/avatar-status-dot-port.md), with offline image examples and status-name composition.
- `src/avatar_group/`: [AvatarGroup](docs/avatar-group-port.md) and [AvatarGroupOverflow](docs/avatar-group-overflow-port.md), with shared member geometry and keyboard navigation.
- `src/side_nav/`: [SideNavHeading](docs/side-nav-heading-port.md) and [SideNavSection](docs/side-nav-section-port.md), with static and linked headings and named groups. [SideNavCollapseButton](docs/side-nav-collapse-button-port.md) supports caller-controlled collapse. Rail context and menu integration remain pending.
- `src/nav_icon/`: [NavIcon](docs/nav-icon-port.md), a circular accent mark with caller-owned artwork and semantics.
- `src/top_nav/`: [TopNavHeading](docs/top-nav-heading-port.md), [TopNavMegaMenuFeaturedCard](docs/top-nav-featured-card-port.md), and [TopNavMegaMenuItem](docs/top-nav-mega-menu-item-port.md). Parent menu and drawer integration remain pending.
- `src/input_group/`: [InputGroup](docs/input-group-port.md) and [InputGroupText](docs/input-group-text-port.md), with named rows and logical addon styling. Production input integration remains pending.
- `src/citation/`: [Citation](docs/citation-port.md), with inline source labels and superscript references.
- `src/command_palette/`: [CommandPaletteEmpty](docs/command-palette-empty-port.md), a standalone empty result surface. Parent palette integration remains pending.
- `src/breadcrumbs/`: [Breadcrumbs](docs/breadcrumbs-port.md) and [BreadcrumbItem](docs/breadcrumb-item-port.md), with named navigation trails and link, button, and current-page forms. Menu integration remains pending.
- `src/token/`: [Token](docs/token-port.md), with static, clickable, and link forms and optional removal.
- `src/metadata_list/`: [MetadataList](docs/metadata-list-port.md) and [MetadataListItem](docs/metadata-list-item-port.md), with native description-list pairs, column layouts, and disclosure.
- `src/empty_state/`: [EmptyState](docs/empty-state-port.md), with native headings, compact layout, and shared rendered-text announcements.
- `src/timer/`: [Timer](docs/timer-port.md), with elapsed and clock formats and browser-driven duration updates.
- `src/thumbnail/`: [Thumbnail](docs/thumbnail-port.md), with attachment preview states and separate Open and Remove actions.
- `src/form_layout/`: [FormLayout](docs/form-layout-port.md), with responsive field arrangement and layer-scoped defaults. FieldLabel consumes the optionality default; input integration remains pending.
- `src/field/`: [Field](docs/field-port.md) and [FieldLabel](docs/field-label-port.md), with native label and group-label forms and attached or detached status. InfoTip and tooltip-only status remain pending.
- `src/field_status/`: [FieldStatus](docs/field-status-port.md), with attached and detached validation messages and shared live announcements. Field control integration remains pending.
- `src/dropdown_menu/`: [DropdownMenuDivider](docs/dropdown-menu-divider-port.md) and [DropdownMenuGroup](docs/dropdown-menu-group-port.md). Popup and menu navigation integration remain pending.
- `src/scrollable_area/`: [ScrollableArea](docs/scrollable-area-port.md), with a native viewport, logical scroll axes, and overflow-dependent keyboard access.
- `src/layout/`: [Layout](docs/layout-port.md), [LayoutContent](docs/layout-content-port.md), LayoutHeader, LayoutFooter, [LayoutPanel](docs/layout-panel-port.md), and shared padding helpers. Panel resize integration remains pending.
- `src/layer/`: the LayerContentBoundary surface-context utility; not a completed Layer/Popover component.
- `src/size_context/`: Astryx's shared size context/provider.
- `src/link/`: Link, shared renderer/provider types, target/rel handling, and linkification.
- `src/text/`, `src/theme/`: Text typography and truncation support, upstream-owned type definitions, and [MediaTheme](docs/media-theme-port.md). The standalone Theme checklist remains independent.
- `src/hooks/`, `src/tooltip/`: shared press/tooltip renderer bridges under their source owners.
- `src/motion/`: shared internal timing tables, not a public Motion migration.
- `src/visually_hidden/`: [VisuallyHidden](docs/visually-hidden-port.md), with fixed clipping, custom roots, and the four original patterns.
- `src/interactive_role_context/`, `src/utils/`: inherited role, shared navigation policy, text-direction and platform-detection adapters; no combined control environment.
- `src/i18n/`: the required server-safe strong-RTL predicate, not a completed internationalization runtime.
- `src/input_group/`: the input-owned internal state contract used by existing controls; not a completed InputGroup component.
- `src/compatibility/`: adapters for provisional public APIs only. Core components resolve independent upstream-owned contexts instead of a combined Button environment.
- `src/lib.rs`: selected public re-exports; existing root imports remain available.
- `assets/astryx.css`: shared design tokens and component styles.
- `examples/showcase.rs`: interactive web showcase.
- `tests/`: server-rendered component tests.

## Checks

```sh
cargo fmt --check
cargo check --lib --no-default-features
cargo test
cargo clippy --all-targets -- -D warnings
cargo check --example showcase --target wasm32-unknown-unknown
```

AspectRatio's offline suite: `python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/aspect_ratio.browser.mjs`
against its own `aspect_ratio` example on port 8092; provenance-only check:
`node tests/aspect_ratio_source.mjs`. See [verification boundaries](docs/aspect-ratio-port.md).

For browser interaction checks, see [Button browser testing](docs/button-port.md#browser-checks),
[ButtonGroup testing](docs/button-group-port.md#verification),
[IconButton testing](docs/icon-button-port.md#verification), [Link testing](docs/link-port.md#verification),
[Spinner testing](docs/spinner-port.md#examples-and-verification), [Badge testing](docs/badge-port.md#verification),
[Divider testing](docs/divider-port.md#verification), [Kbd testing](docs/kbd-port.md#verification),
[Skeleton testing](docs/skeleton-port.md#verification), [StatusDot testing](docs/status-dot-port.md#verification),
[ProgressBar testing](docs/progress-bar-port.md#verification), [Card testing](docs/card-port.md#verification),
[ClickableCard testing](docs/clickable-card-port.md#verification),
and [SelectableCard testing](docs/selectable-card-port.md#verification-and-integration-boundary).
Blockquote's offline suite is `node tests/blockquote.browser.mjs` (preview port 8091),
with `node tests/blockquote_source.mjs` and `node tests/blockquote.contrast.mjs`;
see its verification notes. For Stack, serve `dx serve --example stack --web --port 8093` and run
`python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/stack.browser.mjs`;
`python3 tests/stack_source.py --check` verifies the offline source-derived fixture.
Run `python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/stack_published.browser.mjs`
for the older frozen published-pattern layout comparison on the same owned 8093 preview.
For actual rendered patterns, serve the focused `stack` example on owned port 8097, then run
`STACK_URL=http://127.0.0.1:8097 python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/stack_patterns.browser.mjs`,
`python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/stack_raster.browser.mjs`,
and `python3 tests/stack_raster.py`. These write additive corrected/control fixtures;
the historical actual/published captures remain intact. None needs catalog access.
`tests/stack_published.capture.mjs` is a separate opt-in network capture, also requiring the lock.
See its verification notes for the cache input, evidence limits and remaining gaps.
Run all fourteen existing component suites: `node tests/button.browser.mjs`, `node tests/button_group.browser.mjs`,
`node tests/icon_button.browser.mjs`, `node tests/link.browser.mjs`, `node tests/spinner.browser.mjs`,
`node tests/badge.browser.mjs`, `node tests/divider.browser.mjs`, `node tests/kbd.browser.mjs`,
`node tests/skeleton.browser.mjs`, `node tests/status_dot.browser.mjs`, `node tests/progress_bar.browser.mjs`,
`node tests/card.browser.mjs`, `node tests/clickable_card.browser.mjs`, and `node tests/selectable_card.browser.mjs`.
Also run `node tests/container_press.browser.mjs` for the shared card press profile.
The internal `tests/native_event_boundary.browser.mjs` proof requires serving the
`native_event_boundary` example separately. ClickableCard's contrast audit
(`node tests/clickable_card.contrast.mjs`) intentionally exits 1 for preserved
conditional active-text and focus findings; see its verification notes.
SelectableCard's additional suites are `node tests/selectable_card_events.browser.mjs`,
`node tests/selectable_card_press.browser.mjs` and `node tests/selectable_card_layer.browser.mjs`.
Serve the `interactive_card_boundaries` example on port 8083 for
`node tests/interactive_card_boundaries.browser.mjs` and
`node tests/selectable_card_forced_colors.browser.mjs` (or set `INTERACTIVE_CARD_BOUNDARIES_URL`).
Serialize CDP sessions using the shared browser lock. SelectableCard's contrast audit
(`node tests/selectable_card.contrast.mjs`) intentionally exits 1 for 218/468
conditional persistent-ring findings. The approved forced-color cue and its final
integrated proof are recorded separately in its verification notes.
Card's contrast audit (`node tests/card.contrast.mjs`) intentionally exits 1 for
4/336 inherited core secondary-text findings; decorative edges get no invented graphic threshold.
Also run `node tests/progress_bar_motion.browser.mjs` for native timing/resize/media/cleanup contracts.
ProgressBar's contrast audit (`node tests/progress_bar.contrast.mjs`) intentionally exits 1:
approved track/fill boundaries pass all modeled cases, but 15/168 conditional disabled-marker
findings retain the source ink palette.
Also run `node tests/status_dot_tooltip.browser.mjs` for native placement/pin/name/hit-area ownership.
StatusDot's contrast audit (`node tests/status_dot.contrast.mjs`) intentionally exits 1:
resting cases pass, but the preserved pulse minimum has 33/60 conditional essential-plate
and 33/60 optional-glyph findings below 3:1. Approved neutral custom fallback and
universal forced-color outlines resolve separate source gaps; they do not certify color-only status.
Skeleton is decorative and hidden by default; caller-owned regions supply busy semantics.
Its pulse and forced-color visibility are verified without inventing an essential-edge contrast requirement.
Kbd's audit (`node tests/kbd.contrast.mjs`) passes its 12 modeled keycap-text cases;
Kbd displays shortcuts but never registers them. Its localized text is caller-supplied.
Divider's audit (`node tests/divider.contrast.mjs`) intentionally exits 1 for
16/16 conditional essential-line findings; secondary labels pass all 16 cases.
Unmatched custom Divider variants retain the documented upstream paint gap.
Standalone Badge's audit (`node tests/badge.contrast.mjs`) passes its 112 modeled
page/surface cases; it does not replace the prior Badge-inside-Button findings.
Also run `node tests/spinner_rotor.browser.mjs` and `node tests/announcement.browser.mjs`
for deterministic renderer-boundary contracts. Spinner's contrast audit
(`node tests/spinner.contrast.mjs`) intentionally exits 1 for unchanged arc/track
and on-media string-label findings; see its verification notes.
The Link audit `node tests/link.contrast.mjs` reports 19/96 active text failures
and intentionally exits 1; merely using the disabled color does not disable a link.
The non-text audit `node tests/icon_button.contrast.mjs` reports conditional
boundary findings and intentionally exits 1; all modeled essential glyph/arc
and focus/backdrop cases meet 3:1.
The Group audit `node tests/button_group.contrast.mjs` also reports unchanged
source-palette failures and intentionally exits 1.
The separate [core-palette contrast audit](docs/button-port.md#core-palette-contrast-conflict)
reports inherited upstream failures under an accepted source-fidelity exception;
run `node tests/button.contrast.mjs` (it exits 1). Those failures are not a claim
of passing contrast or accessibility compliance.

## Migration workflow

1. Inventory the original components, dependencies, states, and design tokens.
2. Port shared tokens before matching component styles.
3. Add each component in its own module and export it from `src/lib.rs`.
4. Preserve semantic HTML, keyboard interaction, and accessible names.
5. Add rendering tests and showcase examples for its states.
6. Compare appearance and behavior with the original before marking it migrated.

Start with Button, then primitives such as inputs, labels, badges, and cards.
Leave complex focus management (dialogs, menus, popovers) until primitives are stable.
