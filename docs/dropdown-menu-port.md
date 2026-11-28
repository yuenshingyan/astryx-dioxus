# Dropdown Menu: single-level popup checkpoint

`src/dropdown_menu/component.rs` composes the existing Button and menu rows with a native auto popover. The trigger names the menu and reflects its expanded state. The controller handles initial focus, arrow navigation, Home/End, single-character typeahead, activation, and Tab exit. The browser supplies native Escape and light dismissal. The Actions showcase displays the selected action without performing it. Duplicate is disabled to demonstrate a row that can receive focus without activating.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `DropdownMenu/DropdownMenu.tsx` (`9607edd074b5ab4bcc89d1b9f7a3fffd21e21bb6`), its spec (`ab1b071fb336a13a836c541e40af7041da9c37c1`), and the Actions template (`e8c8b3f16d4cee84e9465524a3c345ce68c4eae9`).

The CSS bundle check, all five Rust checks, and the Dropdown Menu, Dropdown Menu Item, Context Menu, and showcase-navigation Chrome suites passed. The focused suite verified below-trigger placement, accessible naming and expanded state, keyboard opening and selection, disabled-row focus without activation, Escape focus return, and Tab exit. Checkbox and radio rows were not tested inside this popup. Controlled open state, custom triggers, submenus, links, touch-sheet presentation, positioning fallbacks, and visual parity remain open. The checklist remains open.

## Compound divider follow-up

The Actions showcase now uses a compound `DropdownMenuDivider` before Delete. It doubles as the [divider host-input fixture](dropdown-menu-divider-port.md): `node tests/dropdown_menu.browser.mjs` checks the native mount target, caller spacing override, click delivery without selecting a row or closing the menu, and arrow navigation in both directions across the separator. The script passed syntax checking; the browser assertions await the integrator. The separate MenuSeparator fixture still covers default spacing. This is not a full port of the pinned data-mode Actions template.

Data/compound parity still needs approval for a public typed pointer-data input and a shared renderer that routes divider entries to `DropdownMenuDivider` in DropdownMenu, ContextMenu, and BreadcrumbItem. No API or CSS changed.
