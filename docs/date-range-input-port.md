# Date Range Input: native endpoint checkpoint

`src/date_range_input/` composes two existing Date Inputs in a named fieldset. Callers own the committed range. Either endpoint can be entered first. Both dates must be present, ordered, and within the inclusive bounds before the component emits a range. Clearing an endpoint of a committed range requests clearing the range. A reversed or out-of-bounds draft shows a status message without emitting a new value.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `DateRangeInput/DateRangeInput.tsx` (`2f219186e165500e35270b867a73639a7825b613`), its spec (`309c1cb7a679e3d973e9821bd907ebc80d0d0f56`), and its showcase (`2fe10b58138fea1e6f410c6ee99e40b33da5896b`). The local showcase uses separate native controls and fixed bounds instead of the upstream calendar and relative presets.

All five Rust checks and four Chrome suites passed. The focused suite used dispatched change events to verify endpoint labels, end-first entry, rejection of reversed and out-of-bounds drafts, same-day ranges, and clearing. The system picker UI was not automated. Shared calendar selection, presets, custom constraints, maximum span, and visual parity remain open. The checklist remains open.
