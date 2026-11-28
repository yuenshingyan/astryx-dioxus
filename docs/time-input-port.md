# Time Input: native picker checkpoint

## Native range commit follow-up

The supplied TimeInput source and spec blob IDs below were verified against
upstream's tree at the pinned commit. TimeInput passes `min` and `max` to
`NativeTimeSegment`, which checks the range before committing (FR6). The Rust
native control now also withholds `on_change` for nonempty out-of-range edits
instead of relying on native form validation to stop a callback. Bounds are
inclusive, omitted seconds compare as zero, and an empty edit still reports
`None`. Unlike the upstream linear range check, the port preserves HTML's
overnight range when `min` is later than `max`.

Focused Rust tests exercise the callback decision for ordinary and one-sided
bounds, mixed precision, clearing, and overnight ranges. Native picker and
form validation remain browser-owned; this follow-up does not implement
FR13–FR15 rejection messages, announcements, or blur restoration, and did not
rerun browser checks. The checklist remains open.

## Earlier checkpoints

`src/time_input/` pairs Field with a native time input. Caller-supplied status can be attached or detached and is linked alongside the field description. An error sets `aria-invalid` without changing native time constraints. The browser owns editing and constraint validation. Values and bounds are local times without a date or time zone. By default the field accepts `HH:MM`; enabling seconds also accepts `HH:MM:SS` and reports changes with second precision. Clearing the field reports `None` on native change.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `TimeInput/TimeInput.tsx` (`9af114b38e0f90812530bcfe23134235a063471d`), its spec (`ccbb5e61220861ca01a54d3a0f0d016ce7e8e1a1`), and the Time showcase (`8334d58b3b06f7808919c4a57eaa57f3a6d4b140`). The adapted example keeps the label and empty initial value but uses the native picker presentation and adds bounds.

All five Rust checks and three Chrome suites passed. Chrome verified label and description association, bounds and step validation, minute and second precision in form data, disabled form exclusion, and the callback using dispatched change events. The status follow-up passed the CSS bundle check, all five Rust checks, and the Time Input, Date Input, Date Time Input, Field, and showcase-navigation Chrome suites. The added Review time example uses a fixed lunch-break error; the component does not enforce that scheduling rule. Chrome verified detached placement, the accessible description, and unchanged native validity. The clear-action follow-up reuses Field’s InputClearButton. It appears only for a populated, enabled field when `has_clear` is set and requests `None`. It deliberately does not focus the native time control, matching upstream’s precaution against reopening the iOS picker. Chrome verified keyboard clearing, removal of the clear button, and no input, change, or focus event on the time control. The CSS bundle check, all five Rust checks, and five Chrome suites passed. The loading follow-up adds caller-controlled `is_loading`, exposing `aria-busy` and a decorative spinner before Clear without locking the native input. This follows the busy-state behavior of the pinned `DateTimeInput/NativeTimeSegment.tsx` (`648db20f8f8def045a768b540b651b86f7cba516`). Chrome verified retained focus, change handling and form data while busy, and clearing without refocusing the input. The CSS bundle check, all five Rust checks, and the TimeInput, DateInput, DateTimeInput, InputClearButton, and showcase-navigation Chrome suites passed. The system picker, Spinner motion, and spoken feedback were not tested. Custom presentation, hour-format controls, tooltip status, visual parity, and remaining catalog arrangements stay open.
