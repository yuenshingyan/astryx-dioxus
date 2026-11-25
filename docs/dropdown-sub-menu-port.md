# Dropdown Menu Submenu: desktop flyout checkpoint

`src/dropdown_menu/dropdown_menu_sub_menu.rs` renders a named menu and an Item-based trigger. The shared Dropdown Menu controller scopes navigation to the current menu. The logical forward arrow opens the flyout; Escape and the reverse arrow close only that flyout. Selecting an action closes the root menu and returns focus to its trigger. The flyout uses CSS anchor positioning with logical inline placement.

Pinned Meta Astryx commit: `476a3153417dbf75c565f7df05b0784e94ddd59e`. Verified Git blobs: `DropdownMenuSubMenu.tsx` (`e303c2585051b5d1adda0e521203b3f48aa72efd`), its spec (`70d76c0c03e26042f4b43633026e6e38950e3623`), and the `DropdownMenuWithSubmenu.tsx` template (`421e3eaad80e3f45e3d674ce6747dd453f5c7988`). The showcase retains the Actions / Move to arrangement and reports the selected action; it omits the upstream decorative row icons.

The CSS bundle check, five Rust checks, and four Chrome suites passed. The focused suite verified the accessible flyout name, LTR/RTL arrow behavior, Escape focus return, and selection closing both menus. Hover timing and pointer traversal, contextual menus, stacked and touch presentation, multi-level regression coverage, and visual parity remain open. The checklist stays open.
