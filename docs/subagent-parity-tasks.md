# Open tasks from the component sub-agent reports

This list tracks findings and their resolution. Verification tasks are separate from observed gaps.

## Observed gaps

- [x] **Text: make truncated plain text reachable by keyboard.** Enabled, overflowing plain Text gets a tab stop unless the caller sets `tabindex` or the Text is inside an interactive region. Chrome verified Tab access to the full text, visible focus, Escape dismissal without moving focus, and no extra tab stops for fitting, disabled, rich, or nested Text. See [Text verification](text-port.md).
- [ ] **Text and Heading: resolve the custom tooltip exit-animation gap in Firefox and WebKit.** These engines hid the tooltip immediately in the native popover probe. Determine whether a fallback can retain the tooltip and anchor for a finite consumer exit. Keep normal dismissal immediate (upstream has no default exit animation) and respect reduced motion. Close this task with browser evidence or an explicit decision to keep immediate dismissal on affected engines. See [tooltip presentation](tooltip-presentation.md).
- [ ] **Stack: isolate the last screenshot differences in Alignment and Fill.** The corrected captures differ from the published screenshots by at most one color-channel level. Reproduce with matched browser, font, viewport, device scale, and capture coordinates. Fix any confirmed rendering mismatch; otherwise document the measured residual and the evidence for accepting it. Keep the original captures and the pixel-identical Directions case as controls. See [Stack verification](stack-port.md).

## Verification still open (not confirmed bugs)

- [ ] **Run the tooltip lifecycle checks in installed Safari.** Enable Allow Remote Automation only with approval. Record the Safari version and check positioning, keyboard focus, dismissal, and cleanup. Playwright WebKit results do not substitute for this run.
- [ ] **Test real screen-reader behavior.** Use an interactive screen-reader session to verify full truncated text, descriptions without duplicate announcements, and focus after dismissal. Test the Stack Toast's polite announcement and focus return. Record the browser, screen reader, steps, and spoken results; DOM and accessibility-tree tests do not verify speech. See [tooltip presentation](tooltip-presentation.md) and [Stack Toast verification](stack-toast-port.md).
