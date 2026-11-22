# CheckboxListItem: standalone mode

`src/checkbox_list/checkbox_list_item.rs` composes ListItem with a single native CheckboxInput. The caller supplies the checked state and handles change requests. The checkbox uses the visible label and description for its accessible name and description, with an optional name override. The row adds no tab stop. Checked, enabled, writable rows use the accent-muted fill.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `CheckboxList/CheckboxListItem.tsx` (`df860469b06050b9a8d96e16bd00dbf543d9466b`) and its spec (`c41a8d60d4ef60f4f84acc31a1799c290b9aed27`). The notification-preferences preview uses the real row with caller-owned selection. Collection mode, click delegation from the row, inherited checkbox size and availability, loading, and full catalog parity remain open.

All five standard Rust checks and four Chrome suites passed. Chrome verified Space activation with retained focus, a rich label and description in the accessibility tree, native mixed state, and blocked disabled activation. Published visual parity and spoken output were not verified. The checklist remains open.
