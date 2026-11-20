# ClickableCard port and verification

## Authority and scope

Authority: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`,
`ClickableCard/{ClickableCard.tsx,ClickableCard.test.tsx,ClickableCard.doc.mjs}`,
`docs/specs/components/ClickableCard/ClickableCard.spec.md`, the three Card-owned
ClickableCard templates, and `docs/architecture/press-feedback.md` (INV4/5/7).
Pinned source blobs were verified against the upstream Git tree before implementation.

This is the interactive Card action/navigation component, not SelectableCard,
a public headless-hook release, or a completed Tooltip/Layer/Theme migration.
Card remains the surface, padding, dimensions, palette, radius and elevation owner;
its already documented FR3 applied-padding correction is reused unchanged.

## API and ownership

Exports: `ClickableCard`, `ClickableCardError`. Existing shared `CardVariant`,
`Elevation`, `SpacingStep` and `SizeValue` types remain with their owners.

- Required nonblank `label` names the first, visually hidden native action.
  Invalid names return `ClickableCardError::EmptyLabel` to an error boundary.
- `href: Option<String>` selects navigation, even when `onclick` is also supplied.
  `target` is an open optional native target. Every explicit navigation sink uses
  the shared safe-URL decision before selecting native/provider/new-tab behavior.
- Without `href`, a real `button type="button"` owns keyboard activation.
  The released focusable no-op when both destination and callback are absent is
  preserved; upstream OQ1 is **not** decided by this port.
- `is_disabled` keeps the action focusable and announced but inert. Links retain
  role/name/tab stop without `href` or `target`, and bypass the provider. Owning
  root state is reflected by `data-disabled`; opacity is .5, without hover/press paint.
- Card's thirteen built-ins plus validated open custom names, four elevations,
  optional padding/width/height/max-width, caller class/style and children compose
  through `Card`, rather than a second surface implementation. Unmatched custom
  variants keep the transparent/borderless base; they get no invented palette.
- Root native attributes and non-reserved event builders are supported.
  **`onclick`, `onmouseup`, `onpointerdown`, `onmounted` are reserved**: use their
  named typed props, matching Button's established protected-callback convention.
  Raw reserved builders are not semantic callback aliases. Named pointerdown and
  non-reserved events remain available while disabled; own click/mouseup do not.
- `onmounted` receives the real root ref after internal ref/controller setup calls.
  Resources belong to shared hooks; application operations remain caller-owned.

The root is an unlabelled `div` with no added role or tab stop. Its hidden native
control supplies the accessible name and keyboard behavior. Surface primary
activation calls `onclick` once with the root owner, then navigates unless
cancelled. Hidden button activation calls it once with the button owner; hidden
link activation navigates without the surface callback. Middle-click opens an
accepted destination before caller mouseup, without calling `onclick`.
Nested controls, read-only ancestor rules, selected text and registered/private
layer boundaries use the shared hook's source-owned ancestry policy.

### Dioxus event adaptation

Ordinary callbacks use the **original Dioxus event and metadata**: child callbacks
run first, their stop/prevention remains effective, and native data stays
original/downcastable, including trusted pointer coordinates and modifiers.

An approved internal adapter scopes `currentTarget` to the owning root/action on
that same native event during the callback. Standard JavaScript `try/finally`
restores the exact prior own descriptor or its absence, including nested scopes
and exceptions. Invocation errors propagate the same `JsValue`; they are not
silently treated as successful no-ops. There is no synthetic callback transport,
renderer-private converter or event reconstruction.

A provider must forward the existing `LinkRenderProps` callbacks, attributes,
ref and children. Only the internally delegated hidden link stops its own virtual
bubbling to avoid Dioxus `FnMut` reentry; ordinary caller content is unaffected.
Source-required no-callback native action dispatch is activation, not transport.
Focused provider/native link replacement restores focus without scrolling **only**
on disabled flips; ordinary rerenders do not request focus.

The internal `use_retained_event_attributes` adapter remembers event names for the
root lifetime, preserves active payloads and leaves inert declarative slots after
builder removal. Stable trailing event-name positions prevent Dioxus's positional
spread diff from removing retained event ownership. Old caller closures are
released, not cached. No private renderer marker is read or written.

## Press and source-versus-spec differences

The shared press owner/controller and interaction-overlay recipe supply all
pointer paint; no component-local timer, listener, measurement or tint formula
was added. Mouse/pen press is immediate; nested targets do not paint ancestors.
Cards use 10px pan hysteresis even outside scroll containers, without changing
Button's independent tracking profile. No scale animation is applied.

Release uses the original motion-owned native CSS timing reader for finite,
nonnegative time values, `calc()`/`var()` and browser-supported easing. Invalid
press clocks fall back to the source 175ms default. CSS owns live theme/tempo
release paint, and the controller waits for finite CSS release animations after
the clock, excluding nested press owners/infinite animations. Repeated presses
cancel the previous fade. Reduced motion bounds release to `min(exit,200ms)`,
including touch on a fine-primary device.

**Normative corrections, not identical released behavior:** architecture INV5/7
requires every pointer through the engine; old CardPress runtime tests expecting
fine-pointer marker absence/CSS `:active` describe a documented source gap.
The port follows that architecture, not the stale desktop runtime timing.
ClickableCard's documented focus/disabled gaps are also corrected: only focus on
its own hidden action outlines the root, and disabled state reaches the painted
target. Ordinary palette and Card geometry are not redesigned.

Actual Tooltip registration/weak logical anchor metadata remain Layer-owned;
private marker publication remains under `private_attributes`. This enables the
shared guards and Selectable's exceptional portal-capture policy without moving
component selection policy into a shared context or claiming public Layer work.

## Originals and contrast

`tests/fixtures/clickable-card-neutral-reference.json` retains actual published
catalog observations, native frame/text/backdrop evidence and provenance.
Published equality with the source pin is not assumed. The offline suite compares
all three exact originals under light and dark: **ClickableCardShowcase**,
**ClickableCardElevated**, **ClickableCardWithNestedButton** (six observations),
using bundled Figtree, 1/64px geometry tolerance, and equivalent flex
normal/stretch normalization. Button's empty non-painted live-region placement is
excluded from these Card frame comparisons and remains Button-test-owned.

The source demo's Add to cart message is delivered by an application-owned status
output, not a placeholder Toast migration or invented storage operation.

`node tests/clickable_card.contrast.mjs` intentionally exits **1**:

- **58/1008** active-text cases below 4.5:1, including hover/pressed overlay ink.
- **321/336** disabled-text advisory cases below 4.5:1 (disabled text exemption).
- **84/336** focus/backdrop cases below 3:1; focus remains essential when disabled.

These are conditional core/Neutral, light/dark, body/card/surface and variant
models using the preserved source palette, **not** a blanket accessibility pass.
Decorative fills/borders/shadows get no invented essential-graphic threshold.
Custom-baseline results do not certify arbitrary application themes. Forced-color
checks preserve native system treatment/shadow removal, not category-color meaning.

## Verification

Passed locally with `CARGO_INCREMENTAL=0` for build commands:

- `cargo fmt --check`
- `cargo check --locked --lib --no-default-features`
- `cargo test --locked`: **186 Rust tests + 10 doc tests**, including nine
  ClickableCard SSR contract tests.
- `cargo clippy --locked --all-targets -- -D warnings` (host).
- `cargo check --locked --example showcase --target wasm32-unknown-unknown`
- `cargo check --locked --example native_event_boundary --target wasm32-unknown-unknown`
- Serialized Chrome suites: `tests/clickable_card.browser.mjs`,
  `tests/container_press.browser.mjs`, `tests/native_event_boundary.browser.mjs`;
  affected regressions: Button, StatusDot tooltip, ProgressBar native motion, Card.
- New-file JavaScript syntax and `git diff --check`.

Browser coverage includes original appearance, original event identity/trust and
owner restoration, child cancellation/order, action/native/provider/modified/middle
and blocked sinks, reserved/non-reserved callbacks, dynamic builder removal/readd,
provider disabled focus/no-scroll, Enter/Space/Tab, disabled button/link AX names,
RTL link keyboard activation, nested/selection guards, hover border, all-pointer
clocks/pan/repeat/cancel, reduced fine-touch timing, forced colors and drop/remount.
Native-boundary proof additionally covers unrelated recursive document events,
interleaved registration ordering, fresh callbacks, exception restoration and
shared last-listener disposal. The source-palette contrast findings above remain.

Standalone WASM library Clippy was run and fails on **13 Selectable-dependent
unused private native capabilities** in this isolated Clickable-only branch.
No suppressions or visibility widening were added; obsolete pointer wrappers and
old native root-click transport were removed. Parent integration must verify the
combined WASM library is clean. Full combined CI/browser regressions are parent-owned,
not claimed here. Physical screen readers/devices, other engines, native Windows
High Contrast and complete hydration applications remain unverified.
