# ContextMenuGroup checkpoint

`ContextMenuGroup` and `ContextMenuGroupProps` re-export the DropdownMenuGroup APIs, matching `ContextMenu/index.ts` (`48f49e2331524e92021b43271f977819e9262527`) at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`. The alias adds no wrapper, style, or controller.

The standalone preview combines titled and untitled groups with existing ContextMenuItem and ContextMenuDivider aliases. All five Rust checks and four Chrome suites passed. The focused browser test verified the accessible group name, menu rows, and separator. ContextMenu popup integration and keyboard navigation remain open.
