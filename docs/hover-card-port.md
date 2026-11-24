# Hover Card: desktop preview checkpoint

`src/hover_card/` uses the shared Popover controller for anchored presentation and dismissal. A focusable child opens the preview on keyboard focus; mouse entry and exit use configurable delays. Moving from the trigger into the card keeps it open. An optional accessible label makes the surface a named nonmodal dialog; without a label it is a group.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `HoverCard/HoverCard.tsx` (`3de4d6926ee14368dcc884b1c066676df56f96d8`), its spec (`353e1dff404c27973361361e995ffdd93529009b`), and the Jane Doe showcase (`08f264bac45b024db4568eb214253f87cc354f9d`). The adapted showcase retains the above-trigger profile preview and avatar fallback.

All five Rust checks and three Chrome suites passed. The focused suite verified keyboard focus, Escape, mouse opening, hover transfer into the card, and dismissal on pointer exit. Touch and pen behavior, controlled state, trigger ARIA integration, interactive-content focus behavior, and visual parity remain open. The checklist remains open.
