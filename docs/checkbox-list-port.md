# CheckboxList: caller-owned rows

`src/checkbox_list/` composes Field and List into a named checkbox group with optional description and detached status. It combines caller-supplied label and description references with its own references. Selection and row behavior stay with the caller; the showcase uses [CheckboxListItem](checkbox-list-item-port.md) in standalone mode.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `CheckboxList/CheckboxList.tsx` (`a8560fcddba59072a6b7ac3cb441f837e364c9a6`), its spec (`daa2f53f63db77defb42cad4e64093f9ddc16e5f`), and the notification-preferences arrangement. Collection mode, group-required validation, inherited disabled and read-only state, and the remaining arrangements are open.

All five standard Rust checks and four Chrome suites passed. SSR verified label and description references; Chrome verified the group name and description, native list structure, and keyboard selection with retained focus. Published visual parity and spoken output were not verified. The checklist remains open.
