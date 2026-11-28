# Typeahead: labeled local-search checkpoint

`src/typeahead/component.rs` composes Field and BaseTypeahead. The native label targets the combobox, and the combobox references the field description and any status message. An error status marks the input invalid. Search and selection remain owned by BaseTypeahead and the caller, respectively.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Typeahead/Typeahead.tsx` (`fcd1096d62c45b3082d3d07d0d77ce53a342bffd`), its spec (`8e21374d538fddf6afa9ee418096cc7e2ca47b35`), and the Fruit showcase (`a2bf08fa6395f3e4f9ae619444af1cd7c718e069`). The adapted example keeps the label, placeholder, width, and eight fruits, and uses local filtering without the upstream bootstrap source.

The original checkpoint passed the stylesheet bundle check, all five Rust checks, and four Chrome suites. The focused suite verified the native label and description association, filtering, selection, and retained input focus.

## Selected-value clearing

This slice ports the clear-button path of upstream FR8. Enabled Typeahead fields with a selected value expose Field’s existing clear button. Activation requests `on_change(None)`; the caller’s `value` still determines the selection. The Fruit showcase displays the request count and lets the caller decline a clear. The SSR test covers selected, empty, disabled, and standalone BaseTypeahead rendering. `tests/typeahead.browser.mjs` adds pointer and keyboard activation, draft reset, the request count, and focus checks; the browser suite was not run for this slice.

The web adapter returns focus to the same combobox synchronously. For pointer activation, it briefly focuses the clear button so the existing BaseTypeahead blur handler can discard the draft and close the popup; when focus returns, the existing `has_entries_on_focus` behavior still applies. BaseTypeahead, Tokenizer, and the shared clear button are unchanged. A focus-preserving draft reset without this intermediate focus transfer would require a reset interface from shared BaseTypeahead. Focus handling for non-web renderers is not implemented here.

The upstream `hasClear` opt-out remains open: the default is ported without adding a field to the publicly constructible `TypeaheadProps`. Supporting that opt-out needs an approved API design. Selected-token presentation and editing (including FR12 keyboard clearing and announcement), localized clear text, async and bootstrap sources, touch behavior, status integration testing, and visual parity remain open. The checklist remains open.
