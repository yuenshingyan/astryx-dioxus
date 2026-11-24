# Dropdown Menu: single-level popup checkpoint

`src/dropdown_menu/component.rs` composes the existing Button and menu rows with a native auto popover. The trigger names the menu and reflects its expanded state. The controller handles initial focus, arrow navigation, Home/End, single-character typeahead, activation, and Tab exit. The browser supplies native Escape and light dismissal. The Actions showcase displays the selected action without performing it. Duplicate is disabled to demonstrate a row that can receive focus without activating.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `DropdownMenu/DropdownMenu.tsx` (`9607edd074b5ab4bcc89d1b9f7a3fffd21e21bb6`), its spec (`ab1b071fb336a13a836c541e40af7041da9c37c1`), and the Actions showcase (`1d9ce50ba3917787ba5fa59a6b128a5ad6199d90`).

The CSS bundle check, all five Rust checks, and the Dropdown Menu, Dropdown Menu Item, Context Menu, and showcase-navigation Chrome suites passed. The focused suite verified below-trigger placement, accessible naming and expanded state, keyboard opening and selection, disabled-row focus without activation, Escape focus return, and Tab exit. Checkbox and radio rows were not tested inside this popup. Controlled open state, custom triggers, submenus, links, touch-sheet presentation, positioning fallbacks, and visual parity remain open. The checklist remains open.
