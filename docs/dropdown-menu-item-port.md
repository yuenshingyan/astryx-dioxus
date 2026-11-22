# DropdownMenuItem action-row checkpoint

`src/dropdown_menu/dropdown_menu_item.rs` composes Item for action rows. The row owns `menuitem` semantics and stays out of sequential tab order. Disabled actions do not invoke the caller callback. The destructive variant sets the label and description foreground without changing the shared focus paint.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `DropdownMenu/DropdownMenuItem.tsx` (`b071ec07156946795330d201ddd971afb293da8b`), its spec (`87db28318fcbff91a99129957047de91dd41600d`), and the action rows in the Storybook project menu. The inline preview is pointer-only. Link rows, the popup keyboard and dismissal owner, inherited menu size, hover focus, and shared press behavior remain open.

All five standard Rust checks and four Chrome suites passed. SSR verified owned role and tab order; Chrome verified action and disabled behavior, destructive foreground, accessibility-tree semantics, and checkbox, radio, and showcase regressions. Published visual parity and manual assistive technology were not verified. The checklist remains open.
