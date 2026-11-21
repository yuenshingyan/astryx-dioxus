# FieldLabel checkpoint

`src/field/field_label.rs` renders a native label for one control or a span that names a group. Descriptions stay outside the label, and hidden labels and descriptions remain accessible without taking a layout slot. The label reads FormLayout's optionality default to show only the relevant marker. The control still owns `aria-required`, native validation, and description associations.

Description clicks use the input-container hook to focus text inputs or activate controls such as checkboxes without forwarding clicks from nested links and controls. The shared hook is currently used only by FieldLabel.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Field/FieldLabel.tsx` (`4faf84745d61a68881b4c48abf50ecfbbf8cba60`), its spec, `useInputContainer.ts`, `useResolvedRequired.ts`, and both catalog templates. The showcase uses native inputs. InfoTip-based label help is not implemented; the API key example shows only the leading icon. The [Field checkpoint](field-port.md) now composes FieldLabel with attached and detached status. Full input-family integration and published screenshot parity remain open.

All five standard Rust checks passed. Chrome verified label and description associations, hidden label semantics, optionality markers, and description activation without intercepting nested links. FormLayout, FieldStatus, and showcase navigation regressions passed. Touch, other browsers, and manual assistive technology remain unverified. The migration checklist stays open.

```sh
SHOWCASE_URL=http://127.0.0.1:8146 node tests/field_label.browser.mjs
```
