# SelectableCard port and verification

## Scope and authority

Authority: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`,
`SelectableCard/{SelectableCard.tsx,selectableCardInput.ts}`, its specification and
checkbox/radio tests, `hooks/useClickableContainer.ts`, shared press/focus/overlay
owners, and the three Card-directory CLI templates. This implementation is **not
marked fully migrated** while the integration/policy items below remain unresolved.
No staged next-minor behavior is enabled.

SelectableCard composes the existing Card; it does not duplicate its paint,
geometry, padding publication, sizing validation or elevation. The private native
input context remains in `src/selectable_card/selectable_card_input.rs`.
Lab's independent SelectableCardGroup is not implemented, merged into that context,
or advertised as complete. No public radio/checkbox mode prop was added.

## API and behavior

Exports: `SelectableCard`, `SelectableCardProps`, `SelectableCardError`.
`label`, `is_selected`, and `onchange: EventHandler<bool>` are required. A blank
label returns a render error. Card-owned `variant`, `elevation`, optional `padding`,
`width`, `height`, `max_width`, classes/styles and native root attributes compose
without changing the existing Card API. Following the existing Button convention, `onclick`, `onmouseup`, `onpointerdown`,
`onmounted`, and `onclick_capture` are reserved: supply them through named typed
props, not raw attribute builders. Raw builders for these owned names are ignored.
Non-reserved native event builders (including keydown) retain their original
renderer payloads, including while disabled. No private renderer converter or
invalid erased downcast is fabricated.

The first child is a visually hidden native checkbox; the root adds no role or tab
stop. Disabled inputs remain keyboard-reachable with `aria-disabled`, while the
owning theme target reflects `data-disabled`. The input's **checked DOM property**
is repaired after a rejected native change request; keeping an SSR attribute alone
would not implement a controlled input. The private radio channel selects only,
uses the source name/value/form-owner rules, and has no public mode prop.

Ordinary click and mouseup stay in Dioxus's delegated path, preserving child → root
metadata and callback ordering. Surface requests precede caller click; input/Space
caller click precedes native change; Enter requests without a click. Caller click
still runs for nested controls or selected text, but those do not request selection.
A root cancellation cannot revoke a nested Button action already admitted by its
own handler. Existing Button invokes its asynchronous action closure in a spawned
task, so action-body execution itself occurs after root bubbling; this component
does not refactor that independent owner.

`onclick_capture` is the explicit Rust spelling for the root capture callback.
Only the released open-layer compatibility forwarding uses synchronous native
capture and actual-event-backed MouseData. It calls the ordinary `onclick` before
layer controls, ignores its propagation cancellation, and defers its default
cancellation until the same original event reaches document. The source-prescribed
transient listener removes itself, is replaced safely on the next capture, and is
disposed on unmount. It is not a timer, measurement, synthetic click or interpreter
transport. The staged removal of layer forwarding is **not authorized or built**.

Ordinary callbacks retain Dioxus's original event metadata and native downcast.
The approved private adapter scopes `currentTarget` to the Card owner on the
**same original native event**, using a standard property descriptor only during
named owner callbacks. JS try/finally restores the exact previous own descriptor
or its absence, including nested scopes and thrown callbacks. Identity, target,
trust, coordinates and modifiers are preserved; no native event is replaced and
ordinary Dioxus cancellation metadata is never recreated.

Actual body-portaled/native-modal Tooltip capture consumes Layer-owned weak
surface → anchor metadata and the original private layer-root marker. Selectable
owns eligibility, disabled/callback order and cancellation; the shared neutral
owner-document listener owns lifetime and parent-before-child routing. DOM-contained
events use ordinary root capture only, preventing duplicate delivery. Hidden/dropped
surfaces lose their logical owner. Arbitrary dialog/popover/role attributes are
not promoted to an Astryx layer.

Selection uses Card's `--_card-ring` slot, a 2px inset shadow, plus the default
variant's border color. It composes with all elevations and does not change layout.
A local unselected slot reset enforces the representative-state contract: an
unselected nested SelectableCard cannot inherit its selected parent's ring. This
is an explicit normative correction of an inherited private-variable source leak,
not a claim of identical nested-runtime paint or an unrelated Card refactor.
Built-in palette variants read shared original `--color-border-*` roles; these
missing shared token declarations are added without changing their source values.
A theme-added variant reads `--selectable-card-ring-color` and keeps its border.
Only the first input's focus-visible state delegates the shared outline to the
root; nested controls draw their own indicator. Press lifetimes/cancellation,
registration and resource disposal remain with the shared hook/controller.

The governing `docs/architecture/press-feedback.md` INV5/INV7 requires recipe 2
(row-wide target): **every mouse/pen press goes through the engine immediately**,
with the shared release fade, and nested controls never paint the card. The older
component runtime and CardPress test still use/expect desktop CSS `:active`; the
architecture explicitly lists that as an implementation gap. The port follows the
governing architecture, not that stale marker-null/desktop-fast assertion. Rest,
hover and held tint/palette/geometry stay source-derived; mouse timing is a
spec-led correction, not claimed identical to the observed published runtime.
Reduced-motion colors apply instantly, while a touch release remains bounded to
200ms even beside a fine primary pointer (design:reduced-motion DR2–DR3).

## Original patterns and evidence

`tests/fixtures/selectable-card-neutral-reference.json` stores actual published
catalog observations with provenance for **67 independently Git-blob-SHA1-verified
source files**. Published v0.7.16 equivalence to the pin is not assumed. Tests make
no catalog/font request and use the existing bundled Figtree font.

All three source patterns are present: SelectableCardShowcase (3 plan checkboxes),
SelectableCardElevated (2 low-elevation plan checkboxes), and SelectableCardMulti
(6 colored multi-select tags). The first two preserve the source caller's
single-choice state recipe but explicitly do **not** claim radio semantics.
Heading/Stack/Grid/Theme framing is demonstration-only, not independent migration.
Application selection and demo feedback remain caller-owned; no network/storage
operation is fabricated.

Desktop native frames: **564×96, 420×84, 400×96**. At the actual catalog's 390px
coarse viewport: **564×104, 420×96, 400×104**. Those native catalog frames scroll
within the showcase presentation rather than inventing narrower Card presets.
Neutral's mobile semantic typography is scoped to this example, not silently
applied to unrelated components. Both light/dark desktop/mobile observations are
compared: **44 Cards** across the three patterns, including native content/text
rectangles, paint, ring and elevation. Geometry tolerance is 1/64px; equivalent
flex normal/stretch is normalized. Hidden input typography is excluded from paint
comparison because it renders no text; its native geometry/semantics are asserted
separately. Full original records/backdrops remain in the fixture.

## Contrast and forced colors

`node tests/selectable_card.contrast.mjs` reports **218/468 conditional essential
selection-ring cases below 3:1**, minimum **1.000**, against the adjacent inside
paint over body/card/surface, core/Neutral, light/dark, rest/hover/press. Some Neutral
light rings equal their tint. This intentionally exits 1. Colors are retained;
this is not a passing contrast or blanket accessibility claim. Card's independent
text audit and its documented source-palette findings still apply. Custom theme
contrast is caller-owned.

Native forced colors removes the inset shadow, and borderless variants have no
remaining component-owned persistent visual selection mark. Observed default
selected/unselected borders also both resolve to the same system ink. The checked native
semantics remain, but are not a visible indicator. There is no authoritative
source treatment. No forced-color redesign or acceptance of this limitation has
been invented; human policy is required before a completion claim.

## Verification and remaining integration items

Integration checkpoint (not a final completion claim), locally run with
`CARGO_INCREMENTAL=0`:

- Renderer-neutral locked library check, locked tests (**187 Rust + 11 doc
  tests**, including **7 new public SSR + 4 private input-channel tests**), fmt and
  locked wasm showcase check passed at the earlier checkpoint. Latest focused
  public SSR **7/7** and wasm showcase compile pass. Final standard-suite rerun is
  pending the shared cleanup below; no final all-checks pass is claimed.
- Host all-target Clippy passed before the latest support patch introduced a
  Clickable-only unused `cancel_disabled_click` helper in this isolated branch.
  Latest Clippy fails that shared dead-code diagnostic; wasm Clippy additionally
  reports unconsumed early native transport/proof helpers. No suppression or
  public API widening is added. Joint production-use/cleanup is required.
- `tests/selectable_card.browser.mjs` passed **44 original Card observations** plus
  native callback/property/Space/Enter/IME/selection, layer deferral, disabled,
  raw-field reservation/non-reserved builders, canonical nested roles, AX, focus,
  dynamic props and removal. The former broad-popover guard failure is corrected.
- `tests/selectable_card_events.browser.mjs` passed same-object native downcasts,
  trust/coordinates/modifiers, owner scopes, exact prior descriptor restoration,
  nested owners, before-layer cancellation, unmount-in-handler and retained
  detached-DOM listener cleanup.
- `tests/selectable_card_press.browser.mjs` passed governing every-pointer engine
  onset/release and actual held-tint pixels, nested mouse/pen/touch veto, custom
  ring roles, RTL, touch image/exit clock/easing, slide-off/cancel/disable/drop,
  reduced coarse and fine-primary-touch treatment, and source forced-color evidence.
- `tests/selectable_card_layer.browser.mjs` passed actual body/native-modal Tooltip
  capture, caller isolation, disabled capture, hidden/removal cleanup and nested
  parent-before-child logical order, explicit capture-stop propagation, and
  unrelated recursive native dispatch through the corrected Rc document adapter.
  All four browser suites passed together at this checkpoint; the shared clock
  update still requires a final rerun.
- CI, joint cross-card regressions, physical devices, other engines and real
  screen readers are unrun. AX assertions are not screen-reader verification.

Open integration/policy items (not silently redesigned):

1. Finish shared motion clock/profile corrections and cleanup of truly unused
   adapter helpers, then rerun final checks. Published-runtime desktop CSS timing
   is not the desired baseline; governing INV5/INV7 and shared source timing apply.
2. Consume/test the corrected depth-ordered, reentrant document route and retained
   declarative event-slot adapter. Final joint cross-card regressions remain
   integration-owned; neither a fake layer nor a weakened callback contract waives them.
3. The private radio channel passed an approved one-off native probe: **9 radios /
   four independent groups**, source surface/input/Enter/Space/arrows ordering and
   select-only behavior, rejected peer checked-property repair, disabled mutation
   and Tab reachability, detached form owners, names/values and AX state. Exact
   harness/probe/results are archived in the task research cache; all temporary
   visibility/reexports and the temporary example were removed. No public test
   feature, radio mode/context API or Lab migration is shipped.
4. Forced-color persistent-selection policy above remains unresolved. The
   component parent checklist remains unchecked.
