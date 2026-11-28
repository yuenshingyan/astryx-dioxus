# LayoutContent port

`LayoutContent` renders one native `div` that owns the scrollport and holds caller children directly. It scrolls by default and clips overflow when scrolling is disabled. An overflowing scrollport without a caller role gains `role="group"` and a name from the caller label or `scroll_label` (default: “Content”). It adds no tab stop, and a caller role takes precedence. StackItem and LayoutContent share the internal overflow measurement hook.

Padding defaults to the Layout edge inset where a slot is absent and the corresponding inner inset where a slot is present. An explicit spacing step overrides those base insets; zero also disables alignment gutters. The scrollport keeps its width while inline padding aligns its children to the parent Layout's width constraint. The base inset remains published separately from the alignment gutter. Block bleed is limited to eligible direct edge children. The slot context resets at a layer boundary.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Layout/LayoutContent.tsx` (Git blob `414d95797e783e4573daf8af17976d4c1a5bf853`) and `LayoutContent.spec.md` (`d28129d95b30098e6b88770fb05b34477569582e`). Basic and Showcase use real Layout slots in bounded frames and named regions rather than nested `main` landmarks. Layout owns the middle row's alignment query box; only the direct content-slot scroller adds its gutter, not descendant regions that inherit the same CSS variables.

At the original checkpoint Chrome verified DOM naming and removal, programmatic scrolling, and computed inset values. The current browser suite checks those contracts in real Layout composition and adds AX names, scroll ownership against stationary bands, no/one/two-panel geometry, RTL, all four published insets, clipping, explicit overrides, and nested Layout reset within `LayerContentBoundary`. The revised browser suite requires integrator execution; the context fixture does not exercise a real modal's CSS reset. Geometry is not a paint or raster comparison, and an AX name is not a recording of spoken output.

Browser keyboard scrolling, root mount/event delivery, other engines, published raster comparisons, and manual assistive technology remain open. `scroll_label` is caller-localizable; the default is literal English, not a shared catalog lookup. The Rust root-ref adapter is the native `onmounted` attribute, not a React ref.

```sh
SHOWCASE_URL=http://127.0.0.1:8126 node tests/layout_content.browser.mjs
```
