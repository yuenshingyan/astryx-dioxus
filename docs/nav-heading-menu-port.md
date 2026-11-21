# NavHeadingMenu inline checkpoint

`src/nav_menu/nav_heading_menu.rs` renders a named menu with size defaults and a caller-supplied minimum width. The menu passes its size to NavHeadingMenuItem. Its keyboard handler wraps Arrow Up/Down, supports Home and End and first-character navigation, and activates enabled rows with Enter or Space. Disabled rows remain reachable but do not activate. The host owns menu entry and dismissal.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `NavMenu/NavHeadingMenu.tsx` (`ecd02fa5d4f201937c1daa59a586b337f26eb00b`), its spec (`9af61fbe539990fb8fa78fa1dfc1abde0651175c`), and the product-switcher template. The showcase now uses the actual menu container inline rather than opening a heading popover. Popover integration, Escape dismissal and focus return, pointer slide selection, and touch overflow policy remain pending.

All five standard Rust checks passed. SSR verified role ownership, the width override, and inherited size. Chrome verified size and padding, keyboard movement, first-character navigation, and disabled activation suppression. NavHeadingMenuItem and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified. The migration checklist remains open.
