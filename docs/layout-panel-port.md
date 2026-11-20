# LayoutPanel checkpoint

`LayoutPanel` renders one non-shrinking `div` that owns its padding and overflow. It scrolls by default, and disabling scrolling clips overflow. An overflowing panel without a caller role acquires a named group through the shared scroll-region hook. A numeric width of zero clips content and disables automatic scroll naming; it is not a resize-controller collapse and does not make children inert.

The panel uses the Layout slot context to determine which edges face adjacent regions. When placed in a side slot, the divider faces the content. Without a divider or explicit padding, the panel shares the adjacent gap through a negative logical margin. The private slot context resets at a layer boundary. The showcase uses [Layout](layout-port.md) to supply the start-slot context.

The port follows Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, including `Layout/LayoutPanel.tsx` (Git blob `c1a4de83f4878f45f0b6dfc743a64c2d88f51808`) and the LayoutPanel spec (Git blob `f80991f1749fd668384ee83d9ceba228d6ec8156`). The Navigation and Showcase arrangements use Layout and its public regions with demo-only list framing. The `resizable` integration, slide transitions, and focus handoff on collapse remain pending the ResizeHandle controller. The migration checklist stays open.

All five standard Rust checks passed. Chrome verified scrolling, overflow naming, numeric zero-width clipping, and the two showcase arrangements, including the start-slot divider and inset in RTL. LayoutContent, LayoutHeader, LayoutFooter, StackItem, and showcase navigation regression checks passed. Published screenshot parity, other browsers, and manual assistive technology were not tested.

```sh
SHOWCASE_URL=http://127.0.0.1:8127 node tests/layout_panel.browser.mjs
```
