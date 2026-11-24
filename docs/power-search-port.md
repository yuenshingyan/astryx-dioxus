# Power Search: typed filter checkpoint

`src/power_search/` builds caller-owned filter clauses from configured fields and operators. This checkpoint supports text and enumeration values, prevents duplicate additions, and renders committed clauses as removable Tokens. The component emits changes; it does not execute a search.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `PowerSearch/PowerSearch.tsx` (`580c801f6a8196f7f9ac95ca1eb5b61ad9ed62c7`), its spec (`bc581d6020060700658b26809450c927aff30a3c`), and the showcase (`7737940ccae19881bb3cd245100d17c0f7d06282`). The adapted showcase retains the Status and Title filters but uses native controls in place of the upstream tokenized editor.

The CSS bundle check, all five Rust checks, and four Chrome suites passed. The focused suite checked the group name, duplicate prevention, text and enumeration additions, and removal using dispatched events. The tokenized editor, additional value types, async suggestions, keyboard and touch parity, and visual parity remain open. The checklist remains open.
