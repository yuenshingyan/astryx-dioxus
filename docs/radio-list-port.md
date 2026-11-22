# RadioList: caller-owned controls

`src/radio_list/` composes Field with a named radio group. It supports horizontal and vertical layouts, label help, description and status associations, and required semantics inherited from FormLayout without imposing native validation on the controls. In this checkpoint, callers supply same-name native radios and own their values and availability.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `RadioList/RadioList.tsx` (`929627caeee7921dca6c57e03adb9bd9a42b51c1`), its spec (`3e8c21b2246a669932098036edf71e5f70c49440`), and the notification and horizontal layout arrangements. RadioListItem, controlled group value and name, disabled reasons, and the remaining catalog arrangements are open.

All five standard Rust checks and four Chrome suites passed. SSR verified name and description references and required and invalid semantics. Chrome verified accessibility-tree naming, layout, and native arrow-key navigation. Published visual parity and spoken output were not verified. The checklist remains open.
