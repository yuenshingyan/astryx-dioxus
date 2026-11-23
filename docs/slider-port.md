# Slider: single-thumb checkpoint

`src/slider/` uses a controlled native range input with FieldLabel, horizontal and vertical axes, and caller-supplied bounds and step. The browser owns keyboard interaction and value snapping. `on_change` reports native input events; `on_change_end` reports native change events. After a committed interaction, the input reconciles to the caller's value, including when the caller rejects a change. Disabled sliders are excluded from form submission.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Slider/Slider.tsx` (`6752e5f3bf03ca78119ca3b951cb5f1b8071a554`), its spec (`2704a19697fa477cc34babe28f08b6b56c39c956`), and the Volume showcase (`ec78578a0fe2528b3da84457ff216270aaad2335`). The local Volume example retains the initial value and width but uses the browser's native thumb and track instead of the custom track and value tooltip.

All five Rust checks and three Chrome suites passed. Chrome verified the accessible name, horizontal LTR and RTL keyboard behavior, vertical keyboard behavior, controlled rollback, and disabled form exclusion. Dual-thumb range, marks, value tooltip and formatting, status/help integration, pointer and touch parity, remaining catalog examples, and visual parity remain open. The checklist remains open.
