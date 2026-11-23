# Date and Time Input: native picker checkpoint

`src/date_time_input/` renders separate native date and time segments and reports one local ISO datetime, without a time zone. A time picked before a date is retained as a draft. Picking the date first uses the browser's current local time as a default. When named, the form submits one combined value rather than separate segments. The renderer does not read the clock until a date-first change needs a default.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `DateTimeInput/DateTimeInput.tsx` (`5c290d11c7c666ee828bc1403fcdeba00bfc882c`), its spec (`01567226dfc0172f9f8385030b4e72c1510198d6`), and the Meeting time showcase (`68102e9d323e001f71a53bb5c6771d9956df0db4`). The adapted showcase starts empty and omits the custom clear button.

All five Rust checks and five Chrome suites passed. The focused test dispatched segment changes and checked labels, time-first entry, combined form data, clearing, and a date-first default. The system picker UI, datetime bounds, controlled rejection and reset behavior, custom and adaptive pickers, clear-button and status integration, and visual parity remain open. The checklist remains open.
