# ContextMenuItem checkpoint

`ContextMenuItem` and `ContextMenuItemProps` re-export the corresponding DropdownMenuItem APIs. The alias adds no wrapper, state, or styles. The inline Basic example preserves descriptions and the destructive action without pretending to implement a context-menu popup.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `ContextMenu/index.ts` (`48f49e2331524e92021b43271f977819e9262527`), `ContextMenuItem.spec.md` (`9066e4a78c10e968d0579c75c17ea19558f699ff`), and `ContextMenuItemBasic.tsx` (`19e1946484c50f124bdbdddc2b960a3d47f3e4f2`).

All five Rust checks and three Chrome suites passed. The focused test verified menu-item markup, descriptions, and action dispatch. Popup ownership, keyboard navigation, link rows, and the icon showcase remain open.
