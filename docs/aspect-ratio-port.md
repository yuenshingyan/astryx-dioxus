# AspectRatio implementation and verification

## Authority and provenance

Source: Meta Astryx [`476a3153417dbf75c565f7df05b0784e94ddd59e`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e).
This pin is **not a verified catalog release tag**. The authoritative
`docs/specs/components/AspectRatio/AspectRatio.spec.md` records shipped behavior;
its verification gaps are not evidence of passing upstream browser checks.

`tests/fixtures/aspect_ratio_source/manifest.json` records Git blob hashes for
25 retained originals: component, exports, consumer documentation, authoritative
spec, unit/browser tests, all six templates and their metadata, Storybook,
BaseProps, mergeProps, themeProps, tokens, Center and Grid. Run
`node tests/aspect_ratio_source.mjs` to verify every blob offline. No original
published measurements were obtained. Browser comparisons use declarations
lowered directly from the retained TS style objects, not compiled StyleX,
a published screenshot, a release capture, or invented historical measurements.

## Complete component inventory

| Input | Default / behavior | Dioxus boundary |
| --- | --- | --- |
| `ratio` | Required open-ended number, width / height | Required `f64`, not a preset enum |
| `shape` | rectangle; ellipse clips both axes at 50% | `AspectRatioShape::{Rectangle,Ellipse}` |
| `fit` | unset leaves child unstyled; cover fills/crops; contain fills/letterboxes; center keeps natural size | `Option<AspectRatioFit>` |
| `children` | Required caller content; recommend one element | `Element`, never cloned or edited |
| `ref` | Root div | `onmounted`, native `MountedEvent` |
| BaseProps | Root DOM/ARIA/data/events, className/style/xstyle | `GlobalAttributes`, `class`, `style`; native event attributes through `attributes: vec![onclick(...)]` |

React/StyleX values are not new Rust dependencies: CSS classes (including media
and container rules) replace `xstyle`; caller inline styles win. As with existing
primitives, the Dioxus native global attribute builder is broader than upstream's
curated BaseProps whitelist. No JavaScript actions, storage, media loading,
measurements, subscriptions, cancellation, timers or component state are added.
Callers retain all operations and content semantics. The mounted callback reaches
the root, not the internal slot. Native child events bubble normally.

Public module ownership remains `src/aspect_ratio/{mod.rs,aspect_ratio.rs}`.
No context/provider is invented and no dependency is moved. Supporting public
Center, Grid, Layout and Text migrations remain independent. Native demo framing
reproduces only the used flex/grid dimensions; it does not consume unmerged
Typography work or claim those migrations. A later coordinated standalone Text
swap may replace the demo label spans; it is not required by AspectRatio.

### Anatomy, styling and precedence

- Root: `.astryx-aspect-ratio`, `data-shape=rectangle|ellipse`, relative,
  width 100%, overflow clip, min-height 0, flex-shrink 0.
- Class-level `aspect-ratio:var(--astryx-aspect-ratio)` reads a numeric custom
  property; no component-owned inline `aspect-ratio`. Unlayered consumer CSS,
  responsive rules, later layers and inline styles can override it.
- Slot: absolute top 0 / inline-start 0, width/height 100%. The compatibility
  `data-astryx-aspect-ratio-override` marker carries fit only when set. Fit is
  **not** reflected on the root theming target.
- Cover/contain style every **direct** child to 100% width/height, and only direct
  img/video receive object-fit. Center uses flex align/justify center on the slot.
  No-fit leaves the child's own natural size/styles unchanged. All fit rules
  stay in `astryx-base`, not the reset or an unlayered stylesheet. Child inline,
  unlayered and later-layer utilities win without editing child props.
- No component color, spacing, typography, animation or focus tokens are needed.
  Example spacing uses existing `--spacing-2` / `--spacing-4` (8/16px at 16px
  root); examples preserve `--radius-element` / `--radius-container`. Shared
  token source is retained and hash verified. Existing palette remains unchanged.

### Preserved limitations and invalid input

The authoritative spec says invalid numeric ratios are passed to CSS. Finite
zero and negative values remain unchanged; non-finite Rust values are explicitly
validated and emitted as an invalid CSS token, not a fabricated positive ratio.
CSS resolves invalid ratios naturally. No panic or diagnostic suppression.

A definite ancestor width is necessary. Shrink-to-fit parents can collapse to
zero. Height-only constraints clamp the box off ratio; pair height with
`width:auto` for height-driven sizing. Multiple fitted children are sized
individually and may be clipped; compose captions/overlays inside one wrapper.
These source limitations are documented and tested, not silently corrected.

No role/name/tabindex/busy state is added. The caller supplies image alt text,
video semantics, keyboard controls and busy announcements. Skeleton retains its
existing hidden/decorative contract; its caller owns replacement and aria-busy.
There is no intrinsic essential graphic or text contrast threshold for this
unpainted structural box. Replacement sample artwork is not a palette audit or
an accessibility certification for arbitrary caller content.

## Original patterns

`examples/showcase/aspect_ratio.rs` appears in the main showcase and the isolated
`aspect_ratio` example. All images are a self-contained 100×100 SVG replacement,
**not original catalog artwork**; no network asset is required.

| Original template | Preserved geometry/content contract |
| --- | --- |
| Showcase | height 120, width auto, ratios 1 / 4:3 / 16:9, cover, container radius, labels below with spacing 2; row gap 4 |
| Widescreen | definite width 600, 16:9, cover |
| Square Image | width 300, ratio 1, cover |
| Circle Image | width 300, ratio 1, ellipse, cover |
| Image Gallery | width 600, three columns, gap 4, six 4:3 images with caller element radius and original alt strings |
| With Skeleton | width 600, 16:9, unset fit, Skeleton 100% × 100% |

The Widescreen and WithSkeleton metadata preview `aspectRatio:4/3` is **not**
the component ratio; both original TSX templates actually use 16/9. Preview
metadata is retained unchanged in source fixtures. Scroll stages keep fixed-width
samples usable on narrow screens without claiming upstream responsive reflow.
Additional probes cover center/contain/unset, oval, video, media/container ratio
overrides, native events, refs, root attrs, busy replacement and inline overrides.

## Verification

From the project root:

```sh
node tests/aspect_ratio_source.mjs
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/yuenshingyanhindy/vscode/rust/astryx-dioxus/target cargo test --locked --test aspect_ratio
# Own isolated preview; do not use another worktree's server.
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR="$PWD/target" dx serve --example aspect_ratio --web --port 8092 --open false
python3 /tmp/astryx-interactive-cards-reference/browser_lock.py node tests/aspect_ratio.browser.mjs
```

Browser tests create and close only their own CDP target. After localhost startup,
the target is forced offline. Tests compare all six patterns against pinned-source
style lowering and assert explicit geometry, natural-size centering, cropping/
letterboxing declarations, circular/oval radii, no-fit child ownership, video,
clipping/no-shrink, logical anchoring, CSS override precedence, media/container
responsiveness, native mounted callback/event bubbling/focus/Enter, child AX name,
and caller-owned Skeleton replacement without layout shift.

Both documented Tailwind v3/v4 layer orders are exercised in isolated iframes
using the relevant self-contained preflight declarations (`img,video` display,
max-width, height:auto), not a claim of running the actual Tailwind package.
The existing source browser suite used compiled Storybook and Tailwind's full
preflight; that exact React toolchain/release suite is **not** run here. SSR tests
assert both slots, defaults, all fits/shapes, root attrs/inline precedence,
unchanged child props and invalid numeric behavior.

The upstream spec records missing direct checks for base declarations, clipping,
ellipse, flex sizing, center, anchoring, AR1 and PR1. Local tests add explicit
checks for these behaviors; PR1 additionally follows source review (no runtime
resources). This does not rewrite upstream's audit or imply that its baseline
images loaded remote media. Original asset appearance, compiled React/StyleX
output, catalog-release equality, every writing mode and OS/assistive technology
combination remain unverified. Full project check results and exact commits are
recorded in the assignment handoff.
