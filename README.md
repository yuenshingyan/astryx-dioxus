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
source-derived Text-only examples. Its existing shared Tooltip bridge still has
pin-fidelity blockers; Text is not marked complete. See [Text checkpoint](docs/text-port.md).
Supporting layer-boundary
utilities do not complete the standalone Layer migration. Unrelated components
and unrun audits are not marked complete.
Heading now has native h1–h6, independent display types/weight/outline controls,
all four original Heading patterns plus TextHeadingLevels, and focused SSR/offline
Chromium evidence. It consumes the exact shared Text APIs; shared Tooltip fidelity
work still prevents completion. See [Heading checkpoint](docs/heading-port.md).
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

- `src/badge/`, `src/button/`, `src/button_group/`, `src/card/`, `src/clickable_card/`, `src/divider/`, `src/icon/`, `src/icon_button/`, `src/kbd/`, `src/progress_bar/`, `src/skeleton/`, `src/spinner/`, `src/status_dot/`: upstream-owned component modules, named using Rust conventions.
- `src/selectable_card/`: controlled Card selection and its private native input context; not a completed Lab group migration.
- `src/layout/`: shared spacing-step lowering for container publishers, not a completed Layout migration.
- `src/layer/`: the LayerContentBoundary surface-context utility; not a completed Layer/Popover component.
- `src/size_context/`: Astryx's shared size context/provider.
- `src/link/`: Link, shared renderer/provider types, target/rel handling, and linkification.
- `src/text/`, `src/theme/`: Text typography/clamp support and upstream-owned type definitions; standalone Text/Theme checklists remain independent.
- `src/hooks/`, `src/tooltip/`: shared press/tooltip renderer bridges under their source owners.
- `src/motion/`, `src/visually_hidden/`: required internal timing tables and fixed-span accessibility support; not completed public Motion/VisuallyHidden migrations.
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

For browser interaction checks, see [Button browser testing](docs/button-port.md#browser-checks),
[ButtonGroup testing](docs/button-group-port.md#verification),
[IconButton testing](docs/icon-button-port.md#verification), [Link testing](docs/link-port.md#verification),
[Spinner testing](docs/spinner-port.md#examples-and-verification), [Badge testing](docs/badge-port.md#verification),
[Divider testing](docs/divider-port.md#verification), [Kbd testing](docs/kbd-port.md#verification),
[Skeleton testing](docs/skeleton-port.md#verification), [StatusDot testing](docs/status-dot-port.md#verification),
[ProgressBar testing](docs/progress-bar-port.md#verification), [Card testing](docs/card-port.md#verification),
[ClickableCard testing](docs/clickable-card-port.md#verification),
and [SelectableCard testing](docs/selectable-card-port.md#verification-and-integration-boundary).
Run all fourteen component suites: `node tests/button.browser.mjs`, `node tests/button_group.browser.mjs`,
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
