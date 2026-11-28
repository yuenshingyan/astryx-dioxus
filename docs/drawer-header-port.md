# DrawerHeader checkpoint

`src/drawer/drawer_header.rs` composes LayoutHeader, a native level-two Heading, optional subtitle and action slots, and an icon-only ghost Button when a close callback is provided. The button requests `false`; the surrounding surface owns its name, dismissal, and focus. End-slot compensation uses logical edges and defaults to both axes when a close button is present.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Drawer/DrawerHeader.tsx` (`2486f5db193f3aa07ad739625c086d456a519413`), its spec (`b4d7b0faff26116d608c7a8967c298a6f52491d3`), and three catalog templates. The showcase adapts them with inline Layout panels. Full Drawer example parity remains open.

The close button uses the scoped `@astryx.dialog.close` message for both its
name and tooltip. A nonblank `close_label` remains authoritative, including an
explicit `"Close"`; a missing or blank translation falls back to `"Close"`.
DrawerHeader still does not name the Drawer, move focus, or own open state.

The pinned compensation rule is reflected in the existing stylesheet:
`--spacing-2` is subtracted on the logical inline-end edge, both block edges,
or all three edges, as selected by the end slot's attribute. The header row
adds no padding of its own; the inset and divider come from LayoutHeader.
This is source and authored CSS evidence, not measured inset geometry.
DrawerHeader's inherited divider default is verified; the checklist's contrast
with DialogHeader remains open because the current DialogHeader port also
forwards the omitted setting to LayoutHeader.

The earlier checkpoint passed the five standard Rust checks and recorded Chrome
AX evidence for the heading and close button, keyboard close, divider reflection,
and RTL compensation. Those checks predate the localized-name change. The focused
follow-up covers SSR name and tooltip markup, slot order and empty slots, divider
inheritance and compensation attributes, and separate end-slot and close buttons.
SSR is markup evidence, not a browser DOM or AX audit. No new computed CSS,
reference raster comparison, or spoken assistive-technology evidence was
collected. The migration checklist remains open.
