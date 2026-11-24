# Base Typeahead: local search checkpoint

`src/typeahead/base_typeahead.rs` provides a single-selection combobox with a caller-owned selection and a local query. The results use the existing TypeaheadItem presentation. Arrow keys move the active option without moving DOM focus out of the input. Enter commits the active result; Escape dismisses the list. On blur, uncommitted text gives way to the selected label.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Typeahead/BaseTypeahead.tsx` (`7e49d504d09187ec5cc9f519de745b9fa5aaaa06`), its spec (`a106e433d826d0e25ea6012e48c285f670bd76cc`), and the frameworks showcase (`6a5efcbd8fa86fb7a95126acce02072e62fe293b`). The adapted showcase retains the three frameworks and their categories, using built-in result rows instead of custom rendering.

The stylesheet bundle check, all five Rust checks, and three Chrome suites passed. The focused suite checked the combobox name, focus opening, case-insensitive filtering, keyboard selection, empty results, Escape, and restoring the selected label on blur. Async sources, custom result rendering, external anchoring and viewport collision handling, touch behavior, and visual parity remain open. The checklist remains open.
