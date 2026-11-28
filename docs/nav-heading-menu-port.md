# NavHeadingMenu inline checkpoint

`src/nav_menu/nav_heading_menu.rs` renders a named menu with size defaults and a caller-supplied minimum width. The menu passes its size to NavHeadingMenuItem. Its keyboard handler wraps Arrow Up/Down, supports Home and End and first-character navigation, and activates enabled rows with Enter or Space. Disabled rows remain reachable but do not activate. The host owns menu entry and dismissal.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `NavMenu/NavHeadingMenu.tsx` (`ecd02fa5d4f201937c1daa59a586b337f26eb00b`), its spec (`9af61fbe539990fb8fa78fa1dfc1abde0651175c`), and the product-switcher template. The showcase now uses the actual menu container inline rather than opening a heading popover. Popover integration, Escape dismissal and focus return, pointer slide selection, and touch overflow policy remain pending.

At the inline checkpoint, all five standard Rust checks passed. SSR verified role ownership, the width override, and inherited size. Chrome verified size and padding, keyboard movement, first-character navigation, and disabled activation suppression. NavHeadingMenuItem and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified. The migration checklist remains open.

## Keyboard focus follow-up

The pinned spec calls out that focus was indicated by fill alone. Rows inside
NavHeadingMenu now use the shared focus outline tokens on `:focus-visible`,
including disabled rows reached by keyboard. The outline is inset, like
DropdownMenu's, to fit between closely spaced rows. The existing fill and
activation behavior are unchanged. This addresses the spec's documented gap;
it is not a pixel-for-pixel copy of the pinned styling.

The showcase browser test now checks that the outline follows arrow-key focus
from an enabled row to a disabled row. That regression was added but not run in
this pass; browser rendering, contrast, and assistive technology remain
unverified for this change. Heading integration (including menu semantics and
naming in a popup), Escape and focus return, pinned caller key-handler
composition, and pointer/touch behavior remain open.
