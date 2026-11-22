# InputClearButton checkpoint

The Field-owned clear button is used by `TextInput { has_clear: true }` when the value is nonempty and editable. It uses the shared ghost Button with a contextual accessible name and a short “Clear” tooltip. TextInput owns the controlled value and focus: clearing requests an empty string through `on_change` and focuses the input. It does not synthesize a DOM `input` event. The opt-in search field hides the browser's duplicate clear affordance where supported.

Sources at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Field/InputClearButton.tsx` (`750b7a288f6ba6e0aa494bf17888ba30dc3fd703`) and `TextInputSearch.tsx` (`47eeb2647803915f6df5c82fd55c6d8fdaccfed6`). The Search showcase adapts the empty and prefilled arrangements.

All five Rust checks and four Chrome suites passed. Chrome verified the button's accessible name, its omission for empty, disabled and read-only fields, keyboard and mouse activation, and focus retention or return. The internal layer-trigger variant, touch hit-area tuning, iOS software-keyboard behavior, and published visual parity remain open.
