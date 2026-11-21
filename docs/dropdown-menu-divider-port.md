# DropdownMenuDivider checkpoint

`src/dropdown_menu/dropdown_menu_divider.rs` delegates the horizontal separator to Divider, adds `--spacing-1` on both block edges, and preserves the menu-specific theme target. Caller classes, styles, root attributes and callbacks pass through; the separator owns its role and orientation and does not accept a tab stop. `ContextMenuDivider` and `BreadcrumbMenuDivider` are aliases of the same component and props.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `DropdownMenuDivider.tsx` (Git blob `b8dbdeedc3a156752423193ba08caf432146a3a8`) and the component spec (`b5e6c07884a1319ef08ba0d358a5472cbd26bbcd`). There is no standalone catalog template; the showcase demonstrates the separator between two labels. Data-mode integration and arrow-key traversal in a real menu remain pending the menu owners. The migration checklist remains open.

All five standard Rust checks passed. Chrome verified separator semantics in the accessibility tree, block spacing, and rule geometry. The Divider and showcase navigation suites also passed. Published screenshot parity and manual assistive technology were not verified.
