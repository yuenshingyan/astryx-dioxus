# Context Menu: desktop popup checkpoint

`src/context_menu/` opens a cursor-positioned native popover from a focusable trigger region. Shift+F10 and the Context Menu key position it at the focused element instead. The popup uses the existing menu keyboard controller and accepts the existing ContextMenu row aliases as compound content. The showcase reports the selected Cut, Copy, or Paste action without accessing the clipboard.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `ContextMenu/ContextMenu.tsx` (`ae68da4fc5f497cfd73b07c5336e7d49b47ea825`), its spec (`af08cfd78df09fb0a471821a221636c01d2bf57f`), and the showcase (`4170a560298396a938b00e8849bfb65fb2c3ebb1`). The adapted example uses compound rows instead of the data-driven API.

At the original checkpoint, five Rust checks and five Chrome suites passed. The focused suite checked the accessible menu name, right-click positioning, Shift+F10, keyboard activation, Escape, and focus return.

The ContextMenu controller now prevents repeated Enter and Space keydowns on
its own rows from activating them (spec FR8), without changing shared row or
menu behavior. `node tests/context_menu.browser.mjs` covers both held keys
using the stay-open Paste row; this new browser coverage is pending. FR8 paging
and label-only typeahead remain open, as do touch and bottom-sheet presentation,
submenu and press-drag behavior, the data-driven API, cross-browser testing, and
visual and spoken parity. The checklist remains open.
