# LayoutPanel checkpoint

`LayoutPanel` renders one non-shrinking `div` that owns its padding and overflow. It scrolls by default, and disabling scrolling clips overflow. An overflowing panel without a caller role acquires a named group through the shared scroll-region hook. A numeric width of zero clips content and disables automatic scroll naming; it is not a resize-controller collapse and does not make children inert. The numeric-zero path also omits the panel divider: the pinned source leaves the divider on, but a 1px border floors the border-box width at 1px, defeating the zero-width geometry. CSS string widths, including `"0px"`, still follow ordinary CSS sizing.

The panel uses the Layout slot context to determine which edges face adjacent regions. When placed in a side slot, the divider faces the content. Without a divider or explicit padding, the panel shares the adjacent gap through a negative logical margin. The private slot context resets at a layer boundary. The showcase uses [Layout](layout-port.md) to supply the start-slot context.

The port follows Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, including `Layout/LayoutPanel.tsx` (Git blob `c1a4de83f4878f45f0b6dfc743a64c2d88f51808`) and the LayoutPanel spec (Git blob `f80991f1749fd668384ee83d9ceba228d6ec8156`). The Navigation and Showcase arrangements use Layout and its public regions with demo-only list framing. The `resizable` integration, slide transitions, and focus handoff on collapse remain pending the ResizeHandle controller. The migration checklist stays open.

The original checkpoint passed the five standard Rust checks. Its Chrome run verified DOM overflow naming and scrolling, numeric zero-width clipping **without** a divider, and the two adapted arrangements, including computed RTL start-slot inset and border widths. That was not an AX, raster, or spoken-output check.

The expanded SSR suite covers all 11 padding steps, both side slots at numeric zero versus CSS `"0px"`, and native content, attributes, and style order. The expanded Chrome script needs integrator verification: it checks LTR/RTL box geometry under flex pressure, shared gap and child bleed, computed divider width/style/color, and the numeric-zero case with a requested divider. It checks AX names separately from geometry. Neither the computed paint checks nor AX names establish published raster parity or spoken output.

`scroll_label` is caller-localizable but defaults to the literal `"Content"`. Automatic catalog fallback needs a compatible way to distinguish an omitted value from explicitly supplied `"Content"`; replacing both would change the public prop's behavior. Manual keyboard and screen-reader checks, other engines, and published raster comparison remain open. The existing ResizeHandle is a caller-controlled keyboard splitter; it does not supply `useResizable` region/props, pane lending and edge-reveal motion, or post-focus inertness. These remain with the Resizable owner, not an independent controller in LayoutPanel.

```sh
SHOWCASE_URL=http://127.0.0.1:8127 node tests/layout_panel.browser.mjs
```
