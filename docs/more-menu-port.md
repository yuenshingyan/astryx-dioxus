# More Menu: icon-trigger checkpoint

`src/more_menu/mod.rs` composes DropdownMenu with an icon-only ghost trigger, a default three-dot icon, and an accessible label. DropdownMenu owns placement, keyboard navigation, dismissal, and focus return. Trigger tooltip presentation waits until native popup dismissal finishes so focus restoration cannot start a second popover operation inside the first.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `MoreMenu/MoreMenu.tsx` (`96846c26a256b9d05581a59f395aff5de1929b33`), its spec (`23aac0a0ee7ca45e5d67b29162d89ccd59f0e0b5`), and the showcase (`80c289935b3bb3e426915e33e4600d6ee27acad7`). The showcase retains the three actions with caller-composed rows and adds a disabled trigger. Selecting a row only displays its label.

The CSS bundle check, all five Rust checks, and four Chrome suites passed. Chrome verified icon and trigger naming, disabled behavior, keyboard selection, and focus return without a popover exception. Controlled visibility, the data-driven item API, alignment overrides, touch presentation, and visual parity remain open. The checklist remains open.
