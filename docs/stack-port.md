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
  is not publicly ported. The private `src/token/pattern.rs` exact label-only
  md/default slice uses hash-verified Token declarations and stable theme targets,
  including the nested isolated label. Original usage is noninteractive: no invented
  button, tab stop or tooltip. No completed HStack/VStack migration is asserted.
- **Stack — Fill Item / StackFillItem**: max 300px, 100% width, vertical gap 3;
  three horizontal gap-3 cross-centered user rows (Olivia Chen / Engineering Lead,
  Marcus Rivera / Product Designer, Aisha Patel / Marketing Manager), named
  Avatar, filling gap-0 name/role column, small secondary View Button.
  Private `src/stack_item/pattern.rs` wrappers use exact static/fill/reset rules.
  `src/avatar/pattern.rs` supplies the name-only md structure: 36px circle,
  pseudo-element initials at .4 ratio, role=img/name, tab stop, focus-visible outline
  and above-name hover/focus tooltip. Shared Tooltip owns presentation/interest;
  `src/tooltip/pattern_tooltip.rs` is only its private renderer-channel adapter.
  No public Avatar/image-loading API or AR2 scrolling-item API is exposed.
  Published callbacks emit their exact messages via caller-owned polite status.

The demo also covers defaults, every axis pair and spacing step, edge precedence,
RTL, wrap, scroll, explicit/invalid alias precedence, nested local padding,
CSS precedence, dynamic updates, native navigation root/events/ref and media.
`examples/stack.rs` serves the focused offline fixture; `examples/showcase.rs`
registers the same section. Existing older examples' native Stack framing is not
silently rewritten. Supporting slices compile privately into the demo and SSR fixture
via module paths; `src/lib.rs` exports no incomplete supporting APIs. Standalone owner
migrations remain separate. Pinned templates have inert Buttons; later captured public
code adds Toast callbacks. The follow-up [private Toast slice](stack-toast-port.md)
now supplies the published demo overlay, timing, queue and announcement behavior;
this is not a public Toast migration. Its actual open-Toast evidence supersedes
older polite-status-only presentation limits. The actual-node follow-up below supersedes
the earlier static-framing/isolated-layout limits for these exact patterns and mode.

## Verification and evidence limits

Regenerate/verify the fixture offline (stdlib only):

```sh
python3 tests/stack_source.py
python3 tests/stack_source.py --check
```

An optional final argument selects another cache root containing `tree.json`,
original files, and `stack/dependencies/`. It verifies **25 Git blobs** (including Token.tsx and Avatar.tsx) against the
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

The source fixture and `stack.browser.mjs` are **source-derived**, not published
measurements or executed upstream React/StyleX. The separate captured published
fixture below supplies actual observations, with narrower isolation boundaries;
neither is a full-image visual parity certificate.
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
suppressed or retried. Existing unrelated browser/contrast suites and a pinned upstream React/StyleX
build were **not run**. Published capture and isolated replay were subsequently run
as recorded below; no full-image parity is claimed. Exact commits are recorded in the task handoff; CI must
still run the full existing suite. This task's browser suite is additional to,
not a replacement for, the fourteen existing component/browser/contrast suites.

## Published evidence follow-up (historical isolated-layout checkpoint)

Captured **2026-11-20T09:00:49.775Z** from
<https://astryx.atmeta.com/components/Stack>, title `Stack · Astryx`, displayed
`@astryxdesign/core v0.7.16`, Chrome 157.0.8103.80, 1200×1600 viewport, DPR 1,
16px root, LTR, observed Neutral dark examples. `tests/fixtures/stack-published.json`
retains original target-node boxes/computed styles, displayed Alignment/Fill code,
static computed DOM replay, resource URLs/ETags/SHA256 and capture environment.
`tests/fixtures/stack-published/{directions,alignment,fill}.png` are actual clipped
published screenshots, with digests in the JSON. The rendered DOM, screenshots and
displayed code establish pattern identity; **the deployment commit is unknown**.
Resource hashes are independent HTTP retrievals immediately after DOM capture,
not an assertion that a displayed version proves equivalence to the source pin.
Captured PNG backgrounds include their original composited catalog surface; a
transparent Stack is not credited with that background. Font readiness was awaited,
but font binaries are not captured; frozen used dimensions avoid relying on live fonts.

Exact observed CSS-pixel geometry (not rounded to whole pixels):

| Pattern | Observed geometry |
| --- | --- |
| Directions | 329.859375×88; horizontal group 232.234375×88; vertical group x=272.234375, 57.625×88; outer gap 40; inner gap 8 |
| Tokens | Horizontal 72.078125×24; Vertical 57.625×24; radius 6; padding-inline 8; supporting 12px, medium 500, line-height 20.0004px |
| Alignment | 158.234375×312; three 158.234375×96 Cards at y=0/108/216; padding 16; row 126.234375×28; Cancel 67.640625×28, Save 54.59375×28, gap 4 |
| Fill | 219.125×144; three 219.125×40 rows at y=0/52/104; gap 12; Avatar 36×36 at y=2 within row; text x=48, 103.3125×40; View x=163.3125, y=6, 55.8125×28 |

Observed paint: Stack itself transparent; primary white `rgb(255,255,255)`;
secondary `rgb(158,158,158)`; neutral Token/Avatar/secondary Button fill
`rgba(255,255,255,0.1)`; Card `rgb(27,27,27)`, radius 12; primary Save fill
`rgb(241,241,241)`, ink `rgb(17,17,17)`; Buttons radius 10. Avatar circle radius
9999px, fallback 14.4px medium Figtree (line-height 20.5718px). Body 14px Figtree,
line-height 20.0004px. Full per-node observations remain in the fixture.

**Deployment difference:** published Directions reflects `data-wrap="wrap"`;
the pinned StackDirections.tsx has no wrap prop. The test applies that observed
value only to the published replay, not to the pinned showcase or public defaults.
No consequential API/dependency change is required or silently made. The local
focused example uses the existing Astryx theme (e.g. radius-inner **4px**, not
Neutral's 6px); source-derived framing consumes the existing tokens instead of
copying this deployment's palette. A public Token/Avatar migration, Avatar tooltip,
StackItem scroll-region implementation, or theme policy change remains separate.

Reproduce capture (network opt-in) and offline comparison, **always under lock**:

```sh
python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/stack_published.capture.mjs
python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/stack_published.browser.mjs
python3 tests/stack_published.py
```

The stdlib integrity check verifies screenshot digests, identity and explicit observed
geometry/paint metadata without network. Source-fixture generation also extracts the
flat Token/Avatar/item declaration subsets directly from hash-verified originals.
The browser independently lowers those rules in the inherited local token environment
and compares demo framing geometry, typography, sizing, flex behavior and paint.

The offline test uses only the owned 8093 preview and blocks HTTPS before loading,
then goes offline. First it checks that the frozen original reproduces observed
visible target-node geometry and background/ink/radius. Then it transplants the
**actual rendered Dioxus Stack classes/local variables** into those nodes, removes
original Stack layout declarations, and checks the same measurements again within
1/64px. Supporting used dimensions and computed paint stay fixed, intentionally
isolating layout from unavailable public owners. CSSOM six-significant-digit used
size serialization is corrected to Chrome's 1/64px grid with a documented sub-grid
bias; no failing geometry tolerance was widened. This is not an intrinsic sizing,
full-image, pseudo-element, interactive Avatar/Token/Toast, light-mode, mobile,
other-browser, or pinned React-build parity proof. The original screenshot is not
presented as a screenshot of the Rust port. Frozen paint equality controls paint;
it does **not** independently validate the Rust Card/Text/Button implementations.

Follow-up checks passed: formatting, focused seven Stack SSR tests, source hashes
(25 blobs), both locked offline browser suites and showcase/focused wasm checks.
The full Rust suite/Clippy were passed in the implementation checkpoint above,
not rerun for that historical demo/evidence-only follow-up. Capture initially
failed on serializing an element-valued wait predicate (fixed to boolean); replay
exposed CSSOM used-size truncation and the Astryx-vs-Neutral radius distinction,
both resolved explicitly rather than suppressed or hidden with retries.

## Actual supporting-pattern follow-up

Base `838b371`, branch `fix/stack-original-patterns`; compiling implementation
checkpoint `3d1f298`. No public Stack API, dependency, suppression or palette changes.
Private slices remain under Token, Avatar, StackItem and Tooltip owners; none is
inside production Stack or a catch-all support module. The example alone compiles
them; they do not add unrelated standalone public component migrations.

`tests/stack_patterns.browser.mjs` measures **actual rendered Dioxus nodes** (no
reference clone, transplant or frozen supporting width/paint). The measurement stage
matches the captured catalog's intrinsic inline-flex host, existing Neutral dark theme,
LTR, root16px, Figtree and observed antialiased font smoothing. The original root's
fractional pixel phase is reproduced for screenshots only; child dimensions and paint
are never overridden. The owned target blocks HTTPS before local load, explicitly
loads the already bundled OFL Figtree before going offline, and closes only itself.

All **59 owner nodes** (9 Directions, 19 Alignment, 31 Fill) match captured relative
boxes within 1/64px, with exact background/ink/radius and font size/weight equality.
Intrinsic pattern sizes match: 329.859375×88, 158.234375×312, 219.125×144. This check
caught and fixed an actual defect: Save had inherited the Rust Button's default
secondary variant instead of the original explicit primary variant. An initial
font-loading order failure was fixed by loading the local font before offline mode,
not by freezing widths or increasing tolerance.

Actual Rust screenshots and node measurements are in `tests/fixtures/stack-actual/`,
separate from original captures. `tests/stack_patterns.py` decodes both PNGs with
stdlib and records the **nonzero** raster residuals in `pixel-comparison.json`:
Directions mean absolute channel error .082/255; Alignment .954/255; Fill 1.104/255.
Different-pixel fractions are 3.13%, 6.54%, 4.38%; maximum channel errors are 6, 203,
204 respectively. Geometry/paint equality is not pixel identity. These residuals
include raster/edge/text differences; their complete cause has not been established.
No full-image zero-difference certificate or light/mobile/other-browser original
comparison is claimed. The approved Astryx and Neutral palettes were not redesigned.

`stack_patterns.capture.mjs` separately observed the live original (2026-11-20,
Chrome157) and saved `stack-published-interactions.json`: named img/tabstop, default
no-describedby, keyboard focus/name tooltip, Escape preserving focus, published View
message, antialiased smoothing and **zero profile images**. Displayed deployment
commit remains unknown. Exact pinned Fill also has no src/fallbackSrc: initials are
the actual source content, not image substitutes. No profile asset was copied,
no unverified image license is assumed, and offline tests require no remote images,
font services or catalog. Existing bundled Figtree remains covered by its OFL notice;
original Meta screenshots/source remain covered by the upstream notice.

Actual native-browser checks cover each Avatar's Tab/focus-visible/default tooltip,
no duplicate description, Escape, AX image/name, real pointer hover, hoverable
persistence and dismissal, Enter/Space/real pointer activation of demo messages,
and disposal while a tooltip is visible. Pattern Tokens correctly have no actions
or tab stops. SSR checks cover stable targets, named image/tabstop semantics, tooltip
siblings, no duplicate aria-describedby and absence of remote image elements.
The current shared tooltip controller is reused, not forked. Requirements and exact
adapter ownership are coordinated in the cache `stack/patterns-handoff.md`; tooltip
presentation refinements remain the independent Tooltip follow-up. The original's
host-derived focus color and Toast overlay/timing are not certified by these tests.

Reproduce on this worktree's owned preview (never user8080):

```sh
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/tmp/astryx-stack-patterns-worktree/target \
  dx serve --example stack --web --port 8096 --open false
python3 tests/stack_source.py --check
python3 tests/stack_published.py
python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/stack_patterns.browser.mjs
python3 tests/stack_patterns.py
STACK_URL=http://127.0.0.1:8096 python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/stack.browser.mjs
STACK_URL=http://127.0.0.1:8096 python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/stack_published.browser.mjs
```

Follow-up results: all five standard checks passed with `CARGO_INCREMENTAL=0` and
isolated absolute target `/tmp/astryx-stack-patterns-worktree/target`; focused Stack
wasm check, source hash verification (25 blobs), original screenshot integrity,
actual raster comparison, actual/source-derived/historical frozen browser suites
and SSR tests passed. No pre-existing check failure found. Unrelated component
browser/contrast suites, pinned upstream React build, Toast presentation parity and
new original light/mobile captures were **not run**. CI must still run the full suite.

## Raster residual isolation (static visual follow-up)

The historical `stack-actual/` and `stack-published/` captures above are **unchanged**.
The corrected browser suite writes additive `tests/fixtures/stack-raster/` evidence.
`tests/stack_raster.browser.mjs` also retains same-node controls in
`tests/fixtures/stack-raster-controls/`. No component CSS, static supporting source,
action/Toast implementation, palette, public API, ownership or dependency changed.
`python3 tests/stack_source.py --check` again verified all 25 Git blobs against
pin `476a3153417dbf75c565f7df05b0784e94ddd59e` in the existing offline reference tree.
No reproducible source-paint discrepancy was found that would justify a CSS change.

**Causal finding:** the previous suite matched the root's final fractional rectangle
using a CSS transform on its host. Child paint had already been pixel-snapped before
that transform. Equal final rectangles therefore did not mean equal raster origins.
The new control uses the **same actual Rust nodes**, in one owned offline target,
with identical rectangles for every descendant: first the historical fractional
transform, then relative layout offsets instead, then an unchanged repeat capture.
The transform arm reproduces all three historical actual PNGs **pixel-for-pixel**;
the layout arm matches the corrected original-coordinate captures **pixel-for-pixel**;
the unchanged repeat is byte-identical. Those are strict zero-error assertions, not
increased tolerances or a recapture of the published baseline. They identify the
large errors as capture-induced rasterization, not source paint differences.

The primary suite now positions the hosts at the captured original coordinates
through layout without transforms. Its isolated stage has explicit stacking order so
unrelated demo content cannot paint above the fixture. It still checks all 59 owner
nodes' intrinsic boxes within 1/64px, exact paint/visible borders/font size/weight,
and additionally exact line-height. Child sizing and paint are never overridden.
Chrome **157.0.8103.80**, viewport **1200×1600**, DPR **1**, Neutral dark, LTR,
root16px, antialiased bundled Figtree match the recorded environment. Local font SHA256:
`8330490a01c60c196eae00b823de8102275aaa5862e7b76a7af21b8745338928`.
The original font binary was not retained, so binary font identity is not asserted.

| Pattern | New mean absolute channel error /255 | Max channel error | Different pixels |
| --- | ---: | ---: | ---: |
| Directions | 0 | 0 | 0% |
| Alignment | .0042599805 | 1 | .425998% |
| Fill | .0093226788 | 1 | .932268% |

The historical comparator formula/PNG decoder is unchanged; it remains runnable on
`stack-actual` and reports the original nonzero values. The new controls separately
require zero differences for the causal comparisons and at most one channel level
against published screenshots. The remaining one-level differences are retained,
**unclassified quantization-scale residuals**; their precise cause is not proven.
They are not silently called source defects or certified environment noise. No
palette/alpha adjustment is justified from these measurements. The proven
transform-induced noise is distinct from this small unresolved remainder.

Reproduction from the project root (own preview **8097**, never user8080):

```sh
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 dx serve --example stack --web \
  --addr 127.0.0.1 --port 8097 --open false --interactive false --watch false --locked
STACK_URL=http://127.0.0.1:8097 python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/stack_patterns.browser.mjs
python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/stack_raster.browser.mjs
python3 tests/stack_raster.py
```

This follow-up actually passed: isolated `dx` wasm build, formatting, seven Stack SSR
tests, one static-pattern SSR test, 25 source hashes, original PNG integrity,
actual-pattern browser (including its existing native interaction cases), same-node
raster controls, source-derived Stack browser matrix and frozen published-layout
browser suite. Browser sessions were serialized and closed only their owned targets;
HTTPS was blocked and tests went offline after local font loading. No network capture
or changes to a shared server were used. An initial new-stage shrink-to-fit failure
was fixed by giving the host stage the viewport's available width, without freezing
pattern sizes, weakening assertions or retrying failures.

The full Cargo test suite, library check, Clippy, showcase wasm check, unrelated
browser/contrast suites, pinned React build, original font binary comparison and new
original light/mobile/other-browser captures were **not run in this follow-up**.
Toast/action implementation and presentation remain outside this static visual task.
Directions alone is pixel-identical in this captured environment; this is not a
full catalog, public supporting-owner or cross-environment parity certificate.
