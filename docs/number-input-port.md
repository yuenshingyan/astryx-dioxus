# Number Input: native numeric field

`src/number_input/` composes FieldLabel with a native number input. The browser owns editing, step controls, and constraint validation. The callback reports the numeric value on native `change`, or `None` when the browser exposes an empty value. It does not replace browser form validation, or distinguish an empty field from invalid text the browser exposes as empty. Disabled fields are excluded from form data; read-only fields are included.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `NumberInput/NumberInput.tsx` (`7a9446d58c3eb13a12d60eaa4d1f692e138a59ca`), its spec (`839d670cac51b86244fa724bf569b56636ea7f46`), and the Quantity showcase (`3646b3cdbb3636676b623a35e270da0be53c0f05`). The local example keeps the label, width, initial value, and placeholder, but uses native stepping and adds explicit bounds instead of formatting the value as “items”.

All five Rust checks and three Chrome suites passed. Chrome verified the accessible name, description association, keyboard stepping, range overflow and step mismatch, the empty change callback, and read-only versus disabled form behavior. Formatted values, custom steppers and entry-error policies, wheel behavior parity, status integration, remaining catalog examples, and visual parity remain open. The checklist remains open.
