# Command Palette: local search checkpoint

`src/command_palette/component.rs` composes Dialog and the existing search, results, item, and empty-state components. The caller controls visibility and handles the selected value. The input retains focus while arrow keys move the active option; Enter selects it. Local filtering is case-insensitive. The adapted showcase retains the five original commands and adds a modal example; selection is displayed locally without executing commands.

Pinned sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `CommandPalette/CommandPalette.tsx` (`1bd245e6f9a8504a822ef5f1f3a6666f34d39b39`), its spec (`8fc4d2ee36eec08b5cb97b99ac4e554164951563`), and its showcase (`bb61dcb0ccaf25d37295e1d816699ccde0f0b97d`).

The stylesheet bundle check, all five Rust checks, and four Chrome suites passed. Chrome verified inline options, modal input focus, filtering, active-descendant navigation, empty results, selection, and focus return. Async search, grouped results, scrolling the active option into view, shortcut integration, and visual parity remain open. The checklist remains open.
