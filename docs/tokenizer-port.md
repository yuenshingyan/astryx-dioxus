# Tokenizer: local search checkpoint

`src/tokenizer/` composes Field, BaseTypeahead, and removable Tokens. The caller owns selected items and receives the next list with typed add/remove metadata. Already-selected IDs are excluded from search. Named fields submit repeated token IDs in selection order; disabled fields do not submit values. Removing a token moves focus back to the search input.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Tokenizer/Tokenizer.tsx` (`0eca0a20fb1210286a610c3cf49bbec7adce7786`), its spec (`307f94144b0da0802a97a7eb83c48bb57f476306`), and the Tags showcase (`304a096679bf269a97128c281e74b78867f223ae`). The adapted showcase retains the six tags and two initial selections, using a separate search input below the tokens.

The CSS bundle check, all five Rust checks, and four Chrome suites passed. The focused suite verified native labeling, filtered choices, keyboard add and remove, repeated form values, and focus return. Async sources, token creation, token-to-token keyboard navigation, touch-sheet presentation, and visual parity remain open. The checklist remains open.
