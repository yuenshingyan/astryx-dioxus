# DropdownMenuRadioGroup checkpoint

`src/dropdown_menu/dropdown_menu_radio_group.rs` renders a named group with menu-row spacing and provides the controlled value, change callback, and close-on-select policy to descendants. It adds no keyboard handling or tab stop. [DropdownMenuRadioItem](dropdown-menu-radio-item-port.md) requests selection; the enclosing menu will own dismissal.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `DropdownMenu/DropdownMenuRadioGroup.tsx` (`3697fb2ecbf770b2d4f5f034bcf867a2c66d1c92`), its spec (`7a190cb4030d86038fb3ca1b65acbe41c914ee81`), and the Storybook sort arrangement. The original inline preview uses native radios. A separate pointer preview now uses DropdownMenuRadioItem, without popup dismissal or keyboard navigation.

All five standard Rust checks and three Chrome suites passed. The Rust test verifies nearest-group context and close defaults. Chrome verifies name precedence, spacing, no group tab stop, and native preview selection. Production menu integration, published visual parity, and manual assistive technology checks remain open.
