# Typeahead: labeled local-search checkpoint

`src/typeahead/component.rs` composes Field and BaseTypeahead. The native label targets the combobox, and the combobox references the field description and any status message. An error status marks the input invalid. Search and selection remain owned by BaseTypeahead and the caller, respectively.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Typeahead/Typeahead.tsx` (`fcd1096d62c45b3082d3d07d0d77ce53a342bffd`), its spec (`8e21374d538fddf6afa9ee418096cc7e2ca47b35`), and the Fruit showcase (`a2bf08fa6395f3e4f9ae619444af1cd7c718e069`). The adapted example keeps the label, placeholder, width, and eight fruits, and uses local filtering without the upstream bootstrap source.

The stylesheet bundle check, all five Rust checks, and four Chrome suites passed. The focused suite verified the native label and description association, filtering, selection, and retained input focus. Selected-token editing, clearing, async and bootstrap sources, touch behavior, status integration testing, and visual parity remain open. The checklist remains open.
