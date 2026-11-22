# DropdownMenuCheckboxItem checkpoint

The row composes Item with a decorative CheckboxIndicator. It reflects the caller's boolean value and requests its inverse before calling the consumer click handler. Disabled rows remain exposed as unavailable but do not activate. The row adds no sequential tab stop; navigation belongs to the enclosing menu.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `DropdownMenu/DropdownMenuCheckboxItem.tsx` (`74832db7bfb4cbc9a83f746f1d49f3ef87fb74d7`) and its spec (`6c8c13f2ca160ca4d953723abc853a85545837e9`). The inline pointer preview is a local example. Popup navigation, hover focus, inherited menu size, configurable close-on-select, and shared press behavior remain open.

All five standard Rust checks and three Chrome suites passed. SSR verified checked and disabled markup and the tab-stop policy. Chrome verified independent toggles, disabled activation, accessibility-tree semantics, and radio-row and showcase regressions. Touch behavior, published visual parity, and manual assistive technology were not verified. The checklist remains open.
