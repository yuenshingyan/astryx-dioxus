# Hover Card: desktop preview checkpoint

`src/hover_card/` uses the shared Popover controller for anchored presentation and dismissal. A focusable child opens the preview on keyboard focus; mouse entry and exit use configurable delays. Moving from the trigger into the card keeps it open. An optional accessible label makes the surface a named nonmodal dialog; without a label it is a group.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `HoverCard/HoverCard.tsx` (`3de4d6926ee14368dcc884b1c066676df56f96d8`), its spec (`353e1dff404c27973361361e995ffdd93529009b`), and the Jane Doe showcase (`08f264bac45b024db4568eb214253f87cc354f9d`). The adapted showcase retains the above-trigger profile preview and avatar fallback.

## Caller-controlled visibility (FR7 slice)

`ControlledHoverCard` takes `is_open: bool`. Use `HoverCard` for the unchanged uncontrolled mode. A separate entry point preserves existing `HoverCardProps` struct literals; putting the upstream optional field on `HoverCardProps` would break those callers.

The caller can hold the preview open or closed regardless of hover and focus. Escape from the trigger or card and outside clicks use the existing dismissal controller and request `on_open_change(false)` while open. Ignoring a request leaves the card open; prop changes do not emit requests. Closing with focus inside uses the last focused trigger descendant or, in controlled mode, the wrapper's first element child if the trigger was not previously focused.

The SSR test (`cargo test --locked --test hover_card`) checks both controlled values, the original props shape and closed default, and popup semantics. The showcase demonstrates a caller that logs and declines dismissal requests and closes the card from its own button. `tests/hover_card.browser.mjs` retains the uncontrolled focus, Escape, and pointer-transfer coverage and adds held-open/held-closed coverage, an ignored Escape request, and focus return after caller closure. The browser suite was not run for this slice; the earlier desktop checkpoint passed its browser checks.

## Remaining gaps

- **Shared-layer blocker (FR6/FR7):** Popover only handles Escape from the trigger or card. Upstream's topmost, document-scoped Escape routing, including when focus is elsewhere and when the caller declines dismissal, requires approval to change the shared layer dismissal runtime. Popover is unchanged.
- `is_default_open`, `is_enabled`, configurable focus triggers and alignment, touch and pen, trigger ARIA integration, broader interactive-content focus and press handling, inline-safe layer hosting, closed-content unmounting, text-trigger affordances, and visual parity remain open. The checklist is unchanged.
