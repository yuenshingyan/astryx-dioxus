# Popover: anchored nonmodal checkpoint

`src/popover/` implements the caller-rendered trigger path. The private `use_popover` hook owns presentation and open-state requests; callers can control the state or leave it local. A manual native popover hosts the named nonmodal dialog, with CSS anchor positioning. Escape from the trigger or content and outside clicks request close; closing returns focus to the trigger if focus was inside the surface.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Popover/Popover.tsx` (`c481bbedc4a67c8f1d926f4b5fd9ae5f590822be`), its spec (`a352c4806f39e2c8e00ec204956e8e221f3b17e0`), and the Settings showcase (`6f8c9bb70ef0b381451e8b16d2591c817055bb51`). The adapted showcase uses explicit trigger bindings and a caller-owned Done action.

All five Rust checks and three Chrome suites passed. Chrome verified below-anchor geometry, dialog naming, Escape, outside click, and focus return. Shared layer stacking and dismissal, modal and touch-sheet presentations, other trigger modes, browser fallbacks, and published visual parity remain open. The checklist remains open.
