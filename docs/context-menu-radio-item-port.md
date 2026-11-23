# ContextMenuRadioItem

`ContextMenuRadioItem` and `ContextMenuRadioItemProps` re-export DropdownMenuRadioItem, matching Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `ContextMenu/index.ts` (blob `48f49e2331524e92021b43271f977819e9262527`). The alias shares the existing selection context, markup, and styles.

The inline ContextMenuRadioGroup preview uses the alias for enabled and disabled rows. The browser regression checks exclusive selection and verifies that a disabled `menuitemradio` cannot change it. Popup integration, keyboard navigation, and visual parity remain open.
