# ContextMenuCheckboxItem

`ContextMenuCheckboxItem` and `ContextMenuCheckboxItemProps` re-export the existing DropdownMenuCheckboxItem APIs, as in pinned Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `ContextMenu/index.ts` (blob `48f49e2331524e92021b43271f977819e9262527`). The alias adds no separate state, markup, or styles.

The inline ContextMenuGroup preview includes a caller-controlled “Show details” row. All five Rust checks and four Chrome suites passed. The focused test verified `menuitemcheckbox` semantics, the accessible name and checked state, and toggling in both directions. Popup integration, menu keyboard navigation, and visual parity were not verified.
