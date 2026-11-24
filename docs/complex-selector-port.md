# Complex Selector: desktop popup checkpoint

`src/complex_selector/` composes Field with the shared Popover controller. The caller owns the typed value and supplies popup content. Content receives the current value and separate change and close callbacks. ArrowDown opens the popup; an owned `data-autofocus` target receives focus, or the popup itself does if no target is supplied. Escape and explicit dismissal return focus to the trigger when focus was inside the popup.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `ComplexSelector/ComplexSelector.tsx` (`20dd988dd6a07b49c7734ba5f1e9f4d5736875da`), its spec (`a786d91770378b592dde6541cc71e24af179722e`), and the Fruit blend showcase (`3fac01acbf9d3b7beccf3d62c2566314328d85ce`). The adapted showcase retains the fruit and ripeness choices and the initial Apple · Juicy value. It uses native table buttons rather than the catalog's arrow-navigable grid.

The CSS bundle check, all five Rust checks, and five Chrome suites passed. The focused suite verified the dialog name, ArrowDown opening, initial focus, caller-owned value updates, and focus return after selection or Escape. Custom triggers, async changes, status integration, touch behavior, grid navigation in the showcase, and visual parity remain open. The checklist remains open.
