# Button port status

**In progress — keep the Button checklist item unchecked.**

## Reference

- [Component catalog](https://astryx.atmeta.com/components/Button) identifies `@astryxdesign/core` v0.7.16.
- This implementation references upstream commit [`476a3153417dbf75c565f7df05b0784e94ddd59e`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e), not a verified v0.7.16 release snapshot.
- [Button source](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Button/Button.tsx).
- [Button contract](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/docs/specs/components/Button/Button.spec.md).
- [Core tokens](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/theme/tokens.stylex.ts).
- [Catalog theme](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/apps/docsite/src/themes/astryxTheme.ts).
- [Upstream license attribution](../THIRD_PARTY_NOTICES.md).

## Implemented surface

- Required `label`; optional custom children, leading `icon`, and `end_content`.
- `ButtonVariant::{Primary, Secondary, Ghost, Destructive}`; Secondary is the default.
- `ButtonSize::{Small, Medium, Large}`; Medium is the default, with 28/32/36px heights.
- `ButtonElevation::{None, Low, Medium, High}`.
- `ButtonType::{Button, Submit, Reset}`, plus `name`, `value`, and `form`.
- `is_disabled`, `is_loading`, `is_interruptible`, and `is_icon_only`.
- Caller-controlled loading and optional caller-supplied `click_action` futures.
- Click callback runs before the async action; `prevent_default()` cancels the action.
- Same-tick deduplication for non-interruptible actions. Pending state lasts until all accepted action futures finish.
- Focusable disabled/busy controls with `aria-disabled`, activation guards, and no native `disabled` attribute. Blocked clicks do not reach ancestors; Enter/Space are prevented while other keys reach `onkeydown`.
- Safe server-rendered types for blocked submit/reset buttons until mount installs event guards.
- Busy links remain anchors; truly disabled links become inert buttons.
- Native link `href`, `target`, and `rel`; CSS `width`, `class`, `style`, and global attributes.
- URL boundary checks: relative URLs and HTTP(S), mailto, and tel schemes are accepted. Control characters and other schemes render an inert button.
- Busy content remains in layout, with a centered private loading indicator; explicit loading is immediate, action/interruptible loading is delayed by the upstream 310ms token.
- Caller-provided `loading_label` for localized announcements. Its default is empty, so pass it when loading needs a spoken status message. First-render loading populates the live region after mount and an animation frame.
- Core token names/values, optional `data-astryx-theme="astryx"` catalog styling, focus outline, hover/press paint, truncation, shadow tiers, forced-colors borders, and reduced-motion handling.

```rust
use astryx_dioxus::{Button, ButtonVariant};
use dioxus::prelude::*;

fn example() -> Element {
    rsx! {
        Button { label: "Save", variant: ButtonVariant::Primary }
        Button { label: "Saving", is_loading: true, loading_label: "Loading" }
    }
}
```

For async work, `ButtonAction` is a callback returning `Pin<Box<dyn Future<Output = ()>>>`. See `examples/showcase.rs` for a complete example. The button owns its Dioxus task; unmount cancels the awaiting task. Callers own error handling and any independently detached operations. Interruptible mode accepts overlapping actions; it does not itself cancel earlier actions.

## Starter API migration

The user approved breaking the provisional starter API/defaults:

- `Button { "Save" }` becomes `Button { label: "Save" }`.
- `disabled: true` becomes `is_disabled: true`.
- The default variant changes from Primary to Secondary; request Primary explicitly where needed.
- Use `button_type` for form behavior. Label text must provide a non-empty accessible name, including icon-only controls.
- Replace provisional `--astryx-*` token overrides with the original upstream semantic token names used in `assets/astryx.css`.

## Remaining parity and verification gaps

- Explicit Tooltip integration and automatic keyboard-focus tooltips for truncated labels are not implemented. Native `title` through global attributes is not an equivalent replacement.
- Coarse-pointer press tracking, touch-target expansion, and grouped press tracking are not implemented. Fine-pointer press feedback is implemented; do not claim touch parity.
- ButtonGroup, InputGroup, size-provider inheritance, router/link providers, and standalone Icon/Spinner integration await their corresponding components.
- The private CSS loading indicator is not yet a verified port of the standalone Spinner component.
- Icon slot dimensions have defaults but do not implement the upstream Icon size-provider contract.
- Loading hides all visible slot content, including content with its own colors. This intentionally avoids upstream's documented custom-colored end-content visibility gap.
- Reduced motion removes both loading delay and spinner animation; no claim is made that its static indicator exactly matches upstream Spinner behavior.
- The catalog theme uses Figtree when available; this project does not download or bundle the font. System fallback changes text metrics.
- No pixel-diff baseline against an upstream browser rendering has been approved. Source-derived tokens and geometry alone do not establish visual fidelity.
- Keyboard/browser checks and the Chromium accessibility tree do not replace screen-reader testing, Safari/Firefox testing, or touch-device testing.
- Browser checks are manual, not part of the existing GitHub Actions workflow.

## Browser checks

These tests use Node.js 22+'s built-in WebSocket and Chrome DevTools Protocol. No npm package or browser-driver dependency is required.

1. From the project root, serve the showcase (Rust, Dioxus CLI 0.7, and the WebAssembly target are required):

   ```sh
   dx serve --example showcase --web --addr 127.0.0.1 --port 8080 --open false
   ```

2. Start a separate Chrome instance with a dedicated temporary profile and a loopback-only debugging endpoint. On macOS:

   ```sh
   "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
     --headless=new --remote-debugging-port=9222 \
     --user-data-dir="$(mktemp -d)" --no-first-run about:blank
   ```

   Use the corresponding Chrome/Chromium executable on other platforms. Do not expose the debugging port publicly or reuse a personal browser profile.

3. Run:

   ```sh
   node tests/button.browser.mjs
   ```

   Optional overrides: `CDP_URL`, `SHOWCASE_URL`, and `SCREENSHOT_PATH` (for an inspection screenshot, not an approved visual baseline).

4. Stop the showcase and dedicated browser processes after testing.

The script checks native Enter/Space activation, click handling, disabled/busy guards and ancestor propagation, link behavior, blocked/native form submission, stable loading geometry and focus, live-region updates, async deduplication, interruptible actions, `prevent_default`, sizes, icon-only geometry, ellipsis, accessible names/disabled exposure, reduced motion, and forced-colors borders.

## Verification record

Local Rust checks and browser checks are recorded in the implementation task's completion report. A passing check does not mark this migration complete while the gaps above remain.
