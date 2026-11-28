# FieldLabel checkpoint

`src/field/field_label.rs` renders a native label for one control or a span that names a group. Descriptions stay outside the label, and hidden labels and descriptions remain accessible without taking a layout slot. The label reads FormLayout's optionality default to show only the relevant marker. The control still owns `aria-required`, native validation, and description associations.

Description clicks use the input-container hook to focus text inputs or activate controls such as checkboxes without forwarding clicks from nested links and controls. The shared hook is currently used only by FieldLabel.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Field/FieldLabel.tsx` (`4faf84745d61a68881b4c48abf50ecfbbf8cba60`), its spec, `useInputContainer.ts`, `useResolvedRequired.ts`, and both catalog templates. The showcase uses native inputs. The API key example now restores plain-text label help through [InfoTip](info-tip-port.md). The help button is a sibling of the native label and has its own accessible name. The hidden-label example reveals the help button on keyboard focus without otherwise reserving space. Chrome verified both behaviors; touch and spoken output were not verified. The [Field checkpoint](field-port.md) now composes FieldLabel with attached and detached status. Full input-family integration and published screenshot parity remain open.

All five standard Rust checks passed. Chrome verified label and description associations, hidden label semantics, optionality markers, and description activation without intercepting nested links. FormLayout, FieldStatus, and showcase navigation regressions passed. Touch, other browsers, and manual assistive technology remain unverified. The migration checklist stays open.

The pinned spec's FR4/ORD1 precedence already matches: when both flags are set, Optional wins with no default or a Required default; an Optional default suppresses Optional and leaves Required. `tests/field_label.rs` covers all three cases using caller-localized marker text. This is a regression test only; it does not change marker resolution, the port's caller-supplied translations, or the sibling help button and its existing “More information about {label}” fallback.

Upstream FR7/OQ1 still needs an API decision on group-mode element and ref typing before any parity change there. The checklist remains open; this SSR coverage does not establish browser, touch, assistive-technology, or visual parity.

```sh
SHOWCASE_URL=http://127.0.0.1:8146 node tests/field_label.browser.mjs
```
