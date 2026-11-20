# Open tasks from the component sub-agent reports

This list tracks findings and their resolution. Verification tasks are separate from observed gaps.

## Observed gaps

- [x] **Text: make truncated plain text reachable by keyboard.** Enabled, overflowing plain Text gets a tab stop unless the caller sets `tabindex` or the Text is inside an interactive region. Chrome verified Tab access to the full text, visible focus, Escape dismissal without moving focus, and no extra tab stops for fitting, disabled, rich, or nested Text. See [Text verification](text-port.md).
- [x] **Text and Heading: define the custom tooltip exit fallback.** Keep immediate dismissal where the native popover cannot paint its exit (verified in Firefox and WebKit), rather than imitating the top layer. Chrome retains the anchor only for the actual native display transition, not for unrelated animations. Reduced motion makes even consumer-authored exits immediate. This does not add animated exits to Firefox or WebKit. See [tooltip presentation](tooltip-presentation.md).
- [x] **Stack: isolate the last screenshot differences in Alignment and Fill.** A fresh matched Chrome capture reproduced the archived Rust PNGs exactly. The remaining one-level grayscale differences are confined to Button and Avatar paint regions, not Stack layout. No rendering change was justified; a read-only regression check verifies those residuals and the pixel-identical Directions control. The published font binary was not retained, so the precise cause is unproven. See [Stack verification](stack-port.md).

## Verification still open (not confirmed bugs)

- [ ] **Run the tooltip lifecycle checks in installed Safari.** Enable Allow Remote Automation only with approval. Record the Safari version and check positioning, keyboard focus, dismissal, and cleanup. Playwright WebKit results do not substitute for this run.
- [ ] **Test real screen-reader behavior.** Use an interactive screen-reader session to verify full truncated text, descriptions without duplicate announcements, and focus after dismissal. Test the Stack Toast's polite announcement and focus return. Record the browser, screen reader, steps, and spoken results; DOM and accessibility-tree tests do not verify speech. See [tooltip presentation](tooltip-presentation.md) and [Stack Toast verification](stack-toast-port.md).
