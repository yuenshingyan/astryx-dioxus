# InputGroup row checkpoint

InputGroup composes Field, InputGroupText, and caller-owned controls into a named row. The row links to its description and detached validation message, inherits and provides control size, and propagates disabled state to compatible descendants such as Button and nested InputGroup. Direct Button addons join the row shape and rise above their neighbors on keyboard focus. Native input semantics and behavior remain caller-owned.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `InputGroup/InputGroup.tsx` (`4b5c840d460900acadcc4388abb7dbccb05d736f`), its spec (`25c0a3e5e0e75fe6d0b7e2d5f7a9c9681bc5220a`), `groupStyles.ts`, and both catalog templates. The examples use native inputs in place of TextInput. Per-input label/description context, production input integrations, and label help remain pending. The row retains the pinned implementation's fixed height. Published visual parity and manual assistive technology were not verified; the checklist remains open.

### Description follow-up

Both reference files were compared byte-for-byte against the pinned commit. The pinned component generates a status message ID and lists it after the helper description ID (FR1, ORD2). The Rust row now generates that fallback when its optional custom status ID is empty or contains ASCII whitespace, and still preserves valid explicit IDs. Focused SSR coverage verifies that the ordered references resolve to the helper text and detached message. Child controls and shared Field behavior are unchanged.

### Earlier checkpoint verification

All five standard Rust checks passed. SSR covered naming, nested disabled propagation, size inheritance, and explicit message IDs. Chrome verified row geometry, descriptions in the accessibility tree, and direct addon focus stacking. InputGroupText (including the LTR URL within an RTL layout), Field, and showcase navigation regressions passed. These browser and broader Rust checks were not rerun for the description follow-up.
