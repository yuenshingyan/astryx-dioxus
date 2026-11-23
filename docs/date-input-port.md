# Date Input: native picker checkpoint

`src/date_input/` pairs FieldLabel with a native date input. It accepts Gregorian `YYYY-MM-DD` values and bounds, rejects impossible dates and reversed bounds, and reports an empty value as `None`. The browser owns editing, picker presentation, and form constraint validation.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `DateInput/DateInput.tsx` (`610a6711752257da2c2f62c996485c79b2209056`), its spec (`dc4b270853c1bbd2d79a621b1313f7b15bd43e32`), and the Start date showcase (`94d9e8f66476c0ad6c1ecbebb0854ada6282fbb1`). The adapted showcase keeps the label and empty initial value, sets explicit bounds, and uses the native input without an Astryx clear button.

All five Rust checks and four Chrome suites passed. Rust tests cover Gregorian leap-year rules and malformed values. Chrome verified label and description association, required and range validity, leap-day form data, callback behavior with dispatched change events, and disabled form exclusion. The system picker UI was not automated. The custom and adaptive calendar presentations, formatted entry, clear-button and status integration, and visual parity remain open. The checklist remains open.
