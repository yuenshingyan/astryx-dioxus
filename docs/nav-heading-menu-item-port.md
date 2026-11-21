# NavHeadingMenuItem standalone checkpoint

`src/nav_menu/nav_heading_menu_item.rs` ports the medium-sized link and action rows from Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `NavMenu/NavHeadingMenuItem.tsx` (`a4027996bb08af7afc4a653795c7038f1015818b`). Rows expose `menuitem` semantics without adding tab stops. Enabled links use the shared LinkProvider renderer. Disabled or rejected links retain the anchor shape without a destination or selection handler.

The showcase adapts the product-switcher rows from `NavHeadingMenuShowcase.tsx` (`d2fb516fe29edc70c818945321dbf6a3abd37374`) as a standalone preview. The parent menu, size inheritance, focus transfer across renderer changes, disabled pointer-focus handling, keyboard navigation, and dismissal remain open.

All five standard Rust checks passed. SSR verified link markup, owned semantics, and destination removal. Chrome verified link navigation, action selection, and disabled option semantics. SideNavHeading, TopNavHeading, and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified; the checklist remains open.
