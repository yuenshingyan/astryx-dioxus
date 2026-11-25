# Bottom Sheet Switcher: controlled modal checkpoint

`BottomSheetSwitcher` coordinates separately mounted `BottomSheet` children through a private context. Give each child a unique, nonblank `sheet_id`. The caller owns `active_sheet`; Escape requests `None` only from the active step. Standalone sheets continue to use `is_open` and `on_open_change`.

Pinned Meta Astryx commit `476a3153417dbf75c565f7df05b0784e94ddd59e`: `BottomSheetSwitcher.tsx` (`01527cabf59630c2a94b61360fdad0ca9c3130f9`), spec (`acd70786dd50c2094629d27a3f7eecf1ff7ab974`), and review flow (`c4a67ee8d4df1958e7adee4dd48876acd9078861`). The review/confirm demo adapts the maintained standalone-sheet template to exercise the compatibility wrapper. Save only updates the demo. Upstream deprecates the wrapper in favor of standalone sheets driven by one step value; prefer that pattern for new flows.

The bundle check, all five Rust checks, and Chrome suites for the switcher, standalone Bottom Sheet, Dialog, and showcase navigation passed. The focused suite verified one active native modal after each step change, focus inside it, and dismissal through Escape and Save. Animated handoff, final focus return across a multi-step flow, nonmodal presentation, stacking, and visual parity remain open. The checklist stays open.
