# RadioList: caller-owned controls

`src/radio_list/` composes Field with a named radio group. It supports horizontal and vertical layouts, label help, description and status associations, and required semantics inherited from FormLayout without imposing native validation on the controls. Callers may supply same-name native radios or use [RadioListItem](radio-list-item-port.md) with a group-controlled value and shared form name.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `RadioList/RadioList.tsx` (`929627caeee7921dca6c57e03adb9bd9a42b51c1`), its spec (`3e8c21b2246a669932098036edf71e5f70c49440`), and the notification and horizontal layout arrangements. Disabled reasons and the remaining catalog arrangements are open.

The initial port passed all five standard Rust checks and four Chrome suites. SSR verified name and description references and required and invalid semantics. Chrome verified accessibility-tree naming, layout, and native arrow-key navigation. Published visual parity and spoken output were not verified.

## Group disabled semantics follow-up

Verified the local source and spec blobs against the pinned upstream tree. Following FR5/FR8 and the pinned implementation, `is_disabled` forces `aria-disabled="true"` on the radiogroup. Otherwise, a caller's `aria-disabled` (boolean or text) is forwarded; with neither input the attribute is omitted. Caller-supplied ARIA alone does not disable native radios. `RadioListItem`, form names, and native selection and keyboard behavior are unchanged; caller-owned controls remain caller-owned.

Focused SSR coverage checks attribute precedence, omission, and the native disabled state. This follow-up has no new browser verification. Disabled reasons and the focusable-disabled option contract remain open, as do deterministic empty-group Tab entry and full catalog/visual verification. The checklist remains open.

## Controlled form reset

FR2 and FR3 make the caller's value the source of truth. A native form reset restores each radio's `defaultChecked` without firing `change`, so it could leave the native selection and submitted value out of sync with the controlled row. RadioList now observes reset on its containing form and reuses deferred checked-state reconciliation. To clear the controlled selection on reset, update the group value in the form's reset handler. The group does not cancel the native reset, move focus, emit a selection request, or reconcile unmarked caller radios and nested radiogroups.

`node tests/radio_list_sync.mjs` verifies scheduling, ownership, and cleanup at the controller boundary. The integrator can run `SHOWCASE_URL=… node tests/radio_list.browser.mjs` to verify native reset, subsequent form data, retained focus, and no synthetic change on the Size example. That browser check was not run locally. This does not address FR4 empty-group entry or FR5: focusable disabled radio behavior and reason associations require changes to the row's native disabled, activation, and form contracts.
