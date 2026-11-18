# Button implementation and verification

**Status: source-faithful Button implementation complete**, with the explicit upstream-palette exception below. This does not claim contrast compliance or completion of other components.

## Implemented Button surface

- Primary, Secondary (default), Ghost, and Destructive variants; Small/Medium/Large sizes; None/Low/Medium/High elevation; Button/Submit/Reset form types.
- Visible label, custom children, leading icon, icon-only mode, trailing content, explicit width, truncation, and forwarded root attributes/callbacks.
- Controlled loading and caller-owned async actions; same-tick deduplication; interruptible overlapping actions; click callback before action; `prevent_default()` cancels the action.
- Busy and disabled controls remain focusable and cancel activation without native `disabled`. Blocked clicks do not activate ancestors. Blocked submit/reset server markup is safe before mount.
- Native links, explicit router renderers, provider renderers, target/rel forwarding, disabled-link fallback, safe destinations, and focus preservation across repeated anchor/button replacement.
- Explicit tooltips: hover/focus triggers, hoverable content, Escape/blur dismissal, viewport-constrained placement, existing description-ID composition, repeated-name suppression, and disabled-control hints.
- Automatic truncated-label tooltips: measured only on keyboard focus; full label shown without a duplicate description; no hover/tap activation; explicit/wrapping tooltips take precedence.
- Shared SVG Spinner with original 135-degree rounded arc, small 14px frame, inherited foreground/track alpha, phase-aligned rotation, and reduced-motion timing. Loading preserves control geometry and announces through a persistent live region.
- Shared Icon sizing: unsized Button icons use Small (16px) in small/medium buttons and Medium (20px) in large buttons; explicit Icon sizes win.
- Reactive size and button-environment providers; connected-group disabled state, own elevation suppression, corners/dividers, roving focus, orientation/RTL navigation, and touch release-target tracking. Headless slot environments provide the integration boundary for input-owned button slots without pretending the standalone InputGroup has been ported.
- Touch feedback, scroll-intent delay/cancellation, momentum-stop guard, leave/re-enter tracking, quick-tap flash/release fade, expanded touch targets bounded by neighbouring controls, and no popup-anchor press scale.
- Scope-owned async tasks, timers, frames, layers, controllers, and shared document listeners are cleaned up on unmount. Old release timers cannot clear a newer held press; touch suppression cannot swallow keyboard activation.

## Reference

- [Astryx Button page](https://astryx.atmeta.com/components/Button), displayed package version `@astryxdesign/core` v0.7.16.
- Implementation reference: [`meta/astryx` revision `476a3153417dbf75c565f7df05b0784e94ddd59e`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e).
- [Button source](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Button/Button.tsx) and [contract](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/docs/specs/components/Button/Button.spec.md).
- Upstream Spinner, icon-size, Tooltip, press-gesture, token, and theme sources are listed in [THIRD_PARTY_NOTICES.md](../THIRD_PARTY_NOTICES.md).

The showcase contains all six named upstream Button template patterns (Variants showcase, Variants states, Sizes, Floating, Icon, End Slot), plus Actions in narrow rows and the interaction/integration cases above. It includes the original Neutral preview palette and the Astryx catalog-brand palette. Figtree is bundled locally for the showcase; the library Button inherits its application's font.

## Rust integration

```rust
use astryx_dioxus::{Button, ButtonVariant};
use dioxus::prelude::*;

fn example() -> Element {
    rsx! {
        Button { label: "Save", variant: ButtonVariant::Primary, tooltip: "Save your changes" }
        Button { label: "Saving", is_loading: true, loading_label: "Loading" }
    }
}
```

- `ButtonSize` uses Astryx's shared `ElementSize` scale. `SizeContext`, `SizeProvider`, and `use_size` belong to `src/size_context/`. Explicit size wins over the nearest provider, then the medium fallback.
- Button reads separate ButtonGroup and InputGroup-internal contexts from their upstream owners. Neither requires setup for standalone Button usage. Group disabled state and input-owned disabled state are independent; either can block activation.
- `LinkProvider { component: ... }` supplies a reactive `LinkComponentType`; Button's `render_link` overrides it. Renderers receive `LinkRenderProps` and must forward attributes, click/key/mount handlers, native link properties, and content to one focusable link root.
- Compatibility only: `ControlSize` aliases `ElementSize`, `ButtonSizeContext` aliases `SizeContext`, and `ButtonLinkProps`/`ButtonLinkRenderer` alias the shared link types. `ButtonLinkProvider { render: ... }` forwards to `LinkProvider`.
- The provisional `ButtonEnvironment`, context, provider, and hook remain in `src/compatibility/` solely to preserve existing public APIs. The adapter translates old settings into independent contexts; Button does not read the combined environment. ButtonGroup publishes a legacy snapshot for direct consumers of the old public context. These compatibility APIs are not upstream primitives or the recommended basis for new component implementations.
- Use `onmounted` to obtain the actual root's `MountedData` instead of React refs. Root focus, blur, keyboard, mouse, pointer, and touch callbacks are public props.
- `ButtonAction` returns `Pin<Box<dyn Future<Output = ()>>>`. The button owns the awaiting Dioxus task. Callers own error handling and independently detached operations. Interruptible mode accepts overlapping actions; it does not cancel earlier work itself.
- `label` must provide a non-empty accessible name. Loading text defaults to English `Loading`; override `loading_label` for the application's locale.
- Width and form values use Rust strings: express pixel lengths as CSS strings and serialize application values before passing them.
- Tooltip descriptions for opaque caller-rendered slots are initially conservative in server markup. The mounted controller resolves actual rendered names and removes repeated-name descriptions, including decorative Icons. `aria-labelledby` remains authoritative.
- All slot content is visually hidden while loading, including custom-colored content. This avoids the upstream contract's documented colored-end-content visibility bug.
- The supporting Icon, Spinner, ButtonGroup, provider, and input-slot contracts satisfy Button's dependencies; they do not automatically complete those separate standalone migration checklist entries.

## Verification

`tests/button.rs` and `tests/button_complete.rs` cover Button rendering and interaction contracts. Component/provider tests live in `tests/button_group.rs`, `tests/icon.rs`, `tests/spinner.rs`, `tests/size_context.rs`, and `tests/link_provider.rs`; `tests/compatibility.rs` protects the provisional public APIs. Run the normal project checks from [README.md](../README.md).

### Browser checks

Requires Node.js 22+, Dioxus CLI 0.7, the WebAssembly target, and a dedicated Chrome/Chromium instance. No npm package is required.

1. Serve the showcase from the project root:

   ```sh
   dx serve --example showcase --web --addr 127.0.0.1 --port 8080 --open false
   ```

2. Start a separate browser instance with a temporary profile and a loopback debugging endpoint. On macOS:

   ```sh
   "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
     --headless=new --remote-debugging-port=9222 \
     --user-data-dir="$(mktemp -d)" --no-first-run about:blank
   ```

3. Run:

   ```sh
   node tests/button.browser.mjs
   ```

   Optional overrides: `CDP_URL`, `SHOWCASE_URL`, `SCREENSHOT_PATH`.

4. Stop the dedicated processes after testing; never expose the debugging port publicly or reuse a personal browser profile.

The browser suite exercises native mouse/keyboard/touch activation; disabled/busy/form guards; propagation; loading geometry/focus/live regions; async deduplication/interruption/unmount cancellation; explicit and truncated-label tooltips; hoverability/dismissal/description deduplication; size/icon inheritance; group navigation and sliding touch selection; reactive router adapters; repeated focus-preserving root swaps; scroll cancellation; repeated-press timing; reduced motion; and forced colors.

### Upstream appearance comparison

`tests/fixtures/button-neutral-reference.json` records measurements taken from the actual Astryx Button page's Neutral examples in light and dark modes. The browser suite compares all four variants' computed fill/text colors, radius, font size/weight/line height, height, padding, gap, and width against those values. Width permits at most 1/32 CSS pixel (two Chromium layout subpixels); other recorded values must match exactly. These are explicit property assertions, not automatically accepted screenshot updates.

## Core-palette contrast conflict

Run the reproducible source-color audit:

```sh
node tests/button.contrast.mjs
node tests/button.contrast.mjs --json
```

It uses the pinned upstream core tokens and Button/Badge fill/foreground mappings. It composites 14 Badge colors × 4 Button variants × 3 states × 2 backdrops × 2 color modes: **267 of 672 Badge combinations fall below 4.5:1**. It also checks 48 Button-label combinations; four fail: dark Primary on each backdrop reaches **4.3639:1 on hover** and **4.0267:1 when pressed**. The upstream Button contract already documents the dark-accent hover/press issue.

This is a source-color calculation, not a claim that the standalone Badge renderer has been implemented or that every application theme was browser-audited. The script exits 1 while these conflicts remain; those failures must not be reported as a passing check.

### Decision: retain the original palette

The user delegated this choice. Retain the upstream colors rather than silently redesigning Button or caller-owned Badge content. The completed migration accepts the reported source-palette defects at the pinned revision as a **fidelity exception**, not a successful contrast result. Functional/appearance verification and the completed contrast investigation are recorded separately in the checklist.

This exception is limited to the unchanged upstream palette analyzed here. It does not authorize new contrast regressions, waive checks on custom themes/content, or claim that the Badge component or a WCAG compliance program is complete. Any later palette correction must be an explicit, separately reviewed visual change.

## Verification boundaries

- Chromium browser checks and source comparison are not a claim of screen-reader certification or a completed Safari/Firefox/device matrix.
- The upstream contract documents contrast limitations for the default core dark accent under hover/press and custom-colored end content. Matching upstream paint does not prove arbitrary caller-supplied content or themes meet a contrast target. The source-color matrix above records failures rather than silently treating Button's example slot markup as a passing Badge audit.
- The component checklist remains the verification ledger. Check only items actually verified; implementing the behavior does not justify marking a failing contrast audit passed or claiming other component migrations complete.
