# CheckboxList: caller-owned rows

`src/checkbox_list/` composes Field and List into a named checkbox group with optional description and detached status. It combines caller-supplied label and description references with its own references. Selection and row behavior stay with the caller; the showcase uses [CheckboxListItem](checkbox-list-item-port.md) in standalone mode.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `CheckboxList/CheckboxList.tsx` (`a8560fcddba59072a6b7ac3cb441f837e364c9a6`), its spec (`daa2f53f63db77defb42cad4e64093f9ddc16e5f`), and the notification-preferences arrangement.

## Group description and optionality

FR1, FR4, and FR15 require the group's description to reference the detached status after caller and field descriptions. An empty or whitespace-containing `message_id` could produce a visible status with no usable reference. The group now falls back to its generated status ID in that case; a valid caller ID still reaches both the message and the group. No message means no detached status or status ID reference. This changes neither the group tab order nor how callers control checked state.

All four focused SSR tests in `tests/checkbox_list.rs` passed. They cover reference order, hidden-label and caller naming references, the detached message target, and native checked markup without a group tab stop. It also covers the FR16 render cases: a required FormLayout default does not mark the checkbox required, and an optional default suppresses a redundant Optional marker.

## Remaining verification and implementation gates

- FR6–FR7 (collection and pending state) require a group-to-item contract: the current rows have no collection identity, read only caller-supplied checked state, and do not consume pending state. FR9–FR10 and FR14 likewise need the item/control path for inherited disabled reasons, read-only state, and per-checkbox invalid state. Simulating these on the group wrapper would not implement the focused checkbox behavior.
- FR17 requires a group-owned validation mechanism that tracks the first eligible native checkbox as rows change, without requiring every option or interfering with caller ownership and focus. The current `CheckboxList` exposes no required-group API. FR15 style slots and the remaining named arrangements also stay open.
- At the preceding checkpoint, Rust and Chrome checks covered group naming and description, native list structure, and keyboard selection with retained focus. The integrator can rerun `tests/checkbox_list.browser.mjs` against the showcase for native interaction and AX-tree evidence. Live optionality changes with focus retained, manual screen-reader output, and matched upstream rendering remain unverified. The checklist stays open.
