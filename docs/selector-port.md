# Selector: native single-choice checkpoint

`src/selector/component.rs` composes Field with a native `<select>`. The caller owns the selected value; browser changes request a new value and reconcile to the caller's state. Native form behavior handles required selection and submission. Empty option values are reserved for the placeholder, and the first option wins when values are duplicated.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Selector/Selector.tsx` (`be45c18a41598401e9e9dc1265837de9297c363f`), its spec (`215561fd0c596ed5a55ebabcb39e55779e0a37d3`), and the Fruit showcase (`976425f99e0cb78977c10b2ed49d5f691a1ee6dd`). The adapted showcase retains the five fruits and uses the native picker; it deliberately declines Mango to demonstrate controlled rollback.

The CSS bundle check, all five Rust checks, and four Chrome suites passed. The focused suite verified the native label, required validation, accepted and rejected changes, and form data using dispatched change events. The system picker UI was not automated. The custom popup, rich SelectorOption rows, clearing, read-only mode, and visual parity remain open. The checklist remains open.
