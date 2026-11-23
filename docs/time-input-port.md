# Time Input: native picker checkpoint

`src/time_input/` pairs FieldLabel with a native time input. The browser owns editing and constraint validation. Values and bounds are local times without a date or time zone. By default the field accepts `HH:MM`; enabling seconds also accepts `HH:MM:SS` and reports changes with second precision. Clearing the field reports `None` on native change.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `TimeInput/TimeInput.tsx` (`9af114b38e0f90812530bcfe23134235a063471d`), its spec (`ccbb5e61220861ca01a54d3a0f0d016ce7e8e1a1`), and the Time showcase (`8334d58b3b06f7808919c4a57eaa57f3a6d4b140`). The adapted example keeps the label and empty initial value but uses the native picker presentation and adds bounds.

All five Rust checks and three Chrome suites passed. Chrome verified label and description association, bounds and step validation, minute and second precision in form data, disabled form exclusion, and the callback using dispatched change events. The system picker UI was not automated. Custom presentation, hour-format controls, visual parity, status integration, and remaining catalog arrangements stay open.
