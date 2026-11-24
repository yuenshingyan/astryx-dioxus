# Top Nav Menu: desktop activation checkpoint

`src/top_nav/top_nav_menu.rs` composes the shared Popover controller and NavHeadingMenu rows. A named button opens a menu of link or action rows. The menu moves focus to the first enabled row on open, uses the shared arrow, Home/End, and typeahead behavior, and closes on Escape or when focus leaves via Tab. Activation closes by default; rows can opt out or cancel the event.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `TopNav/TopNavMenu.tsx` (`5ce201186443583543e08e55f6f7916d4b41b6af`), the spec (`fe7f68b67c2b5d11212f25dfbedad5fe1dde98fe`), and the Tools showcase (`093205d7ab62c86da07c0efd0ad82dbe52b505f4`). The adapted showcase retains the link titles and descriptions, uses local fragment targets, and omits the decorative icons.

All five Rust checks and five Chrome suites passed. The focused suite checked the menu name, initial focus, arrow movement, Escape focus return, Tab exit, and keyboard link activation. Hover timing, pointer-specific dismissal (upstream keeps the menu open for mouse picks), mobile drawer integration, and visual parity remain open. The checklist remains open.
