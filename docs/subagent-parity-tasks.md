# Open tasks from the component sub-agent reports

This list tracks findings and their resolution. Verification tasks are separate from observed gaps.

## Observed gaps

- [x] **Text: make truncated plain text reachable by keyboard.** Enabled, overflowing plain Text gets a tab stop unless the caller sets `tabindex` or the Text is inside an interactive region. Chrome verified Tab access to the full text, visible focus, Escape dismissal without moving focus, and no extra tab stops for fitting, disabled, rich, or nested Text. See [Text verification](text-port.md).
- [x] **Text and Heading: define the custom tooltip exit fallback.** Keep immediate dismissal where the native popover cannot paint its exit (verified in Firefox and WebKit), rather than imitating the top layer. Chrome retains the anchor only for the actual native display transition, not for unrelated animations. Reduced motion makes even consumer-authored exits immediate. This does not add animated exits to Firefox or WebKit. See [tooltip presentation](tooltip-presentation.md).
- [x] **Stack: isolate the last screenshot differences in Alignment and Fill.** A fresh matched Chrome capture reproduced the archived Rust PNGs exactly. The remaining one-level grayscale differences are confined to Button and Avatar paint regions, not Stack layout. No rendering change was justified; a read-only regression check verifies those residuals and the pixel-identical Directions control. The published font binary was not retained, so the precise cause is unproven. See [Stack verification](stack-port.md).

## Verification (not confirmed bugs)

- [x] **Run focused tooltip lifecycle checks in installed Safari.** Safari 26.6.2 passed positioning, native Tab and Escape, focus retention, and cleanup when Text no longer overflows. See [tooltip presentation](tooltip-presentation.md) for the WebDriver command and scope.
- [x] **Verify truncated Text speech.** The user confirmed that VoiceOver read the expected full sentence after clicking **Start keyboard checks** and pressing Tab once. The browser version and manual Escape behavior were not recorded. See [tooltip presentation](tooltip-presentation.md).
- [ ] **Retest Stack Toast speech and focus return.** The user reported that VoiceOver did not speak the demo message after pressing **View**. The viewport now prepares the empty polite live region before the first action, and the Chrome regression test verifies the same region receives the message. This does not confirm spoken output. With VoiceOver running, activate **View** and listen for “Demo: this profile would open here” once without focus moving. Press F6 to enter the toast and Enter to dismiss it; check that focus returns to **View**. See [Stack Toast verification](stack-toast-port.md).
