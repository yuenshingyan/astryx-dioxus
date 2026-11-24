# Tab Menu: navigation checkpoint

`src/tab_list/tab_menu.rs` adds a menu button to a navigation TabList. It uses the surrounding list's controlled value and change callback. When a menu option is selected, its label appears on the trigger. The popup uses radio menu items and shares the existing Popover and menu keyboard controllers.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `TabList/TabMenu.tsx` (`f83985b30aef1811bd8344aa54e2601644d67c5e`), its spec (`4bc04d75d6d4025c7be8fac4f6973c1f3c2f8617`), and the showcase (`c8824c30fa56c76878db267059ea48078e4c61f6`). The showcase retains Overview, Activity, and the three menu options, and adds a following button to verify Tab exit.

All five Rust checks and five Chrome suites passed. The focused suite checked the selected trigger label, accessible menu name, radio state, keyboard selection, Escape focus return, and Tab exit. Panel-mode integration, touch and press-drag selection, and visual parity remain open. The checklist remains open.
