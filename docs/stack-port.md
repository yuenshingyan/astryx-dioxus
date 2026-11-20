# Stack implementation and verification

## Authority and boundaries

Source pin: Meta Astryx [`476a3153417dbf75c565f7df05b0784e94ddd59e`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e).
This is a **commit pin, not a claim about a released package or current catalog**.
The authoritative `docs/specs/components/Stack/Stack.spec.md` at that pin governs
Stack FR1–FR5, FR7, AR1 and static rendering. FR6 and AR2 concern **StackItem**, not
Stack: they do not authorize adding a minimum-size reset, automatic group label,
role or focus target to Stack.

`src/stack/{stack.rs,stack_style.rs}` retain the upstream Stack owner and styling
utility boundary. `src/layout/padding.rs` still owns spacing-step lowering;
`src/utils/types.rs` owns `SizeValue` and `SpacingStep`. Only numeric reflection
was added to the existing spacing type. HStack and VStack remain separate
upstream owners and are **not implemented** by this task. StackItem, Grid, Layout,
Token, Avatar, and the independent typography migration are not claimed complete.
No unmerged typography changes, new dependency, feature, palette change, suppression,
CSS relocation, or shared ownership consolidation is used.

## Complete Stack inventory

| Original input | Rust contract | Default / behavior |
| --- | --- | --- |
| `direction` | `StackDirection::{Horizontal, Vertical}` | Vertical; row follows inherited LTR/RTL, column flows top-to-bottom |
| `hAlign`, `vAlign` | `h_align`, `v_align`: `Option<StackAlignment>` | Unset. Union: Start, Center, End, Between, Around, Evenly, Stretch |
| `justify` | `Option<StackMainAlignment>` | Unset. Start, Center, End, Between, Around, Evenly |
| `align` | `Option<StackCrossAlignment>` | Unset. Start, Center, End, Stretch |
| `gap` | `Option<SpacingStep>` | Unset, not zero or two; equal row and column gaps |
| `padding` | `Option<SpacingStep>` | Unset, local only |
| `paddingInline`, `paddingBlock` | `padding_inline`, `padding_block` | Unset; override uniform padding per axis |
| `paddingInlineStart`, `paddingInlineEnd`, `paddingBlockStart`, `paddingBlockEnd` | corresponding snake-case props | Unset; override axis then uniform per edge, explicit zero wins |
| `wrap` | `Option<StackWrap>`: Nowrap, Wrap, WrapReverse | Unset: browser nowrap, **no data-wrap reflection** unless supplied |
| `isScrollable` | `is_scrollable: bool` | False; true sets overflow auto, no implicit name, tabindex or minimum reset |
| `width`, `height`, `maxWidth`, `minHeight` | `Option<SizeValue>` and snake-case names | Unset; numeric pixels or open CSS dimensions; explicit props follow caller style |
| `as` | `as_element: Option<StackElement>` | One div by default; caller-owned root renderer, below |
| `ref` | `onmounted`, also native `attributes` listeners | Reaches the actual root, no wrapper; composed with builder callbacks |
| `xstyle`, `className`, `style` | native `class`, `style` | Component CSS in existing layer, caller class CSS overrides it; caller inline style follows layout, explicit dimensions follow inline style |
| `children` | optional `Element` | Empty; otherwise untouched caller content |
| supported native DOM/ARIA/data/events | `GlobalAttributes` and `attributes: Vec<Attribute>` | Passed to root; `onclick` and `onmounted` have direct composed callback props |

Every spacing prop uses the existing eleven-step enum: 0, .5, 1, 1.5, 2, 3, 4,
5, 6, 8, 10. It lowers to original `--spacing-*` variables, not copied pixel
constants. Pinned defaults are **rem** (0, .125, .25, .375, .5, .75, 1, 1.25,
1.5, 2, 2.5rem), so spacing scales with root text size. There are no paint,
elevation, size, status, disabled, loading, selection or interaction variants.
Theme target `astryx-stack` reflects `data-direction`, supplied `data-gap` (including
`0` / `0.5`), and supplied `data-wrap`. Caller data passthrough follows generated
reflection, matching upstream's final spread.

Horizontal: resolved h → justify-content; resolved v → align-items. Vertical:
resolved v → justify-content; resolved h → align-items. Physical props win their
aliases **before validity filtering**. For example, horizontal h=Stretch ignores
justify entirely, and horizontal v=Between ignores align entirely. The invalid
physical value then emits no declaration. This preserves the authoritative
contract instead of silently substituting a valid alias. Unset alignment leaves
browser normal/stretch behavior, not an invented explicit default.

Numbers/CSS dimensions use the existing shared boundary validation: reject
nonfinite/negative pixels and declaration-breaking CSS; CSS grammar and arbitrary
units/functions/variables/keywords remain the browser's responsibility. Invalid
sizes return the existing Dioxus render error through `?`, not a panic. CSS-wide
keywords are applied directly to dimensions, never stored in custom properties.
Height alone does **not** select scrolling. Padding emits no `--container-padding-*`
or layout inset variables; inherited publisher context is not reset or republished.
Nested Stacks without padding/gap do not acquire the parent's local declarations.

## Native and polymorphic API adaptation

Like the existing Link renderer seam, `StackElement = Callback<StackRenderProps,
Element>` is the Dioxus equivalent of React `ElementType`. It supports native tags
and caller-owned components without a closed tag list, leaked dynamic templates,
extra wrapper or new shared abstraction:

```rust
use astryx_dioxus::{Stack, StackRenderProps};
use dioxus::prelude::*;
let _ = rsx! {
    Stack {
        as_element: |p: StackRenderProps| rsx! { nav { ..p.attributes, {p.children} } },
        "aria-label": "Projects",
        button { "Open project" }
    }
};
```

The renderer must forward attributes and children to **one** intended root;
this responsibility is caller-owned, as with upstream custom components. Native
`attributes` can contain every Dioxus event builder (`onkeydown`, `onfocus`, etc.)
and native properties needed by the chosen tag. Direct `onclick` and `onmounted`
compose with builder callbacks rather than replacing them. Ref callbacks use
Dioxus `MountedData`, not React ref objects/null-on-unmount notifications. Stack
owns no listeners, timers, subscriptions, state, tasks or focus management to clean
up. React StyleX objects are not a Rust public input: native class/CSS and inline
style are the existing repository adapter. GlobalAttributes permits more native
attributes than the upstream BaseProps TypeScript allowlist; it does not remove
supported native behavior or claim TypeScript-source compatibility.

## Original patterns and explicit remaining gaps

All originals and their `.doc.mjs` metadata are embedded with Git blob SHA1 in
`tests/fixtures/stack-source-reference.json`:

- **Stack — Alignment / StackAlignment**: 100% width, max 500px, vertical gap 3;
  three padding-4 Cards; each contains a gap-4 column with supporting secondary
  text and a gap-1 horizontal Start/Center/End row of small Cancel/Save Buttons.
  Uses the actual dependencies present on main. No new typography API is consumed.
- **Stack — Directions / StackDirections**: horizontal outer gap 10, centered;
  a horizontal gap-2, cross-centered triple and vertical gap-2 triple. Original
  HStack/VStack lower to explicit Stack directions. Original **Token**, not Badge,
  is unavailable: labeled native outlined spans supply demo-only content. No
  Token fidelity or completed HStack/VStack migration is asserted.
- **Stack — Fill Item / StackFillItem**: max 300px, 100% width, vertical gap 3;
  three horizontal gap-3 cross-centered user rows (Olivia Chen / Engineering Lead,
  Marcus Rivera / Product Designer, Aisha Patel / Marketing Manager), static
  Avatar, filling gap-0 name/role column, static small secondary View Button.
  Native static/fill wrappers and initial circles explicitly replace unavailable
  StackItem and Avatar **only in this demo**. No AR2 scrolling-item behavior or
  faithful Avatar geometry/paint is claimed. View remains caller-owned/inert.

The demo also covers defaults, every axis pair and spacing step, edge precedence,
RTL, wrap, scroll, explicit/invalid alias precedence, nested local padding,
CSS precedence, dynamic updates, native navigation root/events/ref and media.
`examples/stack.rs` serves the focused offline fixture; `examples/showcase.rs`
registers the same section. Existing older examples' native Stack framing is not
silently rewritten. Later Token/Avatar/StackItem/typography swaps should be explicit,
separately coordinated owner migrations. The overall checklist stays open until
these original-pattern dependencies and a true upstream-runtime visual comparison
are resolved; the implemented Stack contracts are checked individually.

## Verification and evidence limits

Regenerate/verify the fixture offline (stdlib only):

```sh
python3 tests/stack_source.py
python3 tests/stack_source.py --check
```

An optional final argument selects another cache root containing `tree.json`,
original files, and `stack/dependencies/`. It verifies **23 Git blobs** against the
full pinned tree, extracts original direction/main/cross/wrap/gap/padding declaration
maps and spacing defaults, and checks resolution/precedence invariants in Stack.tsx.
The fixture retains full source/spec/test/doc/template texts and copyright headers;
see [notices](../THIRD_PARTY_NOTICES.md). Regeneration fails on a hash mismatch.

Browser run (existing shared Chrome 9222, one owned target; no browser restart):

```sh
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/tmp/astryx-stack-worktree/target \
  dx serve --example stack --web --port 8093 --open false
python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/stack.browser.mjs
```

`STACK_URL` can select another local preview. The suite blocks HTTPS/catalog,
loads only the local compiled example, then puts its owned target offline. Reference
nodes lower the verified **original declarations**, independent of port CSS; actual
and reference geometry/styles compare in the same browser. It checks 48 axis pairs
× LTR/RTL × light/dark × 16/20px root sizes, eleven gap/padding steps, and the Stack
topologies of all three original patterns with the same supporting content. Further
checks cover edge mirroring, no publication, nested noninheritance, all four sizing
props winning over caller style, CSS-wide inheritance, class/inline layout overrides,
all wrap modes, overflow independent of height, exact native NAV ref identity,
composed callbacks, real mouse/Enter events, AX name/role, stable updates/removal and
reduced-motion/forced-color inertness. The suite closes only its own CDP target.

These are **source-derived fixtures**, not measurements from the published site,
not executed upstream React/StyleX, and not a full-image visual parity certificate.
Supporting Card/Text/Button output is held constant in pattern comparisons; its
independent fidelity/contrast findings remain governed by those owners. No catalog
network, external font/image or live service is needed. Stack supplies no ink or
essential graphic; it does not introduce a contrast audit exemption for its children.
The upstream scroll accessibility limitation remains: bare scrolling Stack gets no
invented focus/name behavior; callers choose semantic names and focusable content,
or a future ScrollableArea. Nothing here certifies overall accessibility compliance.

Seven SSR tests cover output, all 48 axis pairs, explicit/inapplicable alignment,
all steps and wrap reflection, per-edge zero precedence/no publication, sizing and
scroll independence, validation, polymorphic root and native attribute passthrough.
Actual local results: `cargo fmt --check`, `cargo check --locked --lib
--no-default-features`, `cargo test --locked`, `cargo clippy --locked --all-targets
-- -D warnings`, and `cargo check --locked --example showcase --target
wasm32-unknown-unknown` all passed. The focused `stack` example WebAssembly check,
seven focused SSR tests, source-fixture verification, and serialized offline browser
suite also passed. Ordinary Cargo checks used `CARGO_INCREMENTAL=0` and the assigned
shared absolute target; dx used only this worktree's target. Initial direct-mounted
prop compilation, module-inception lint and CDP Enter text failures were fixed, not
suppressed or retried. Existing unrelated browser/contrast suites and executed upstream
React/StyleX/published-site comparisons were **not run**; no pre-existing failure was
observed in the checks run. Exact commits are recorded in the task handoff; CI must
still run the full existing suite. This task's browser suite is additional to,
not a replacement for, the fourteen existing component/browser/contrast suites.
