# FieldStatus checkpoint

`FieldStatus` renders a visible validation message. The attached box overlaps the control without intercepting pointer input; the detached box has a decorative status glyph. The control still owns `aria-invalid` and any description association. Custom variants and the deprecated `tooltip` identifier render a plain box, not a tooltip.

The FieldStatus-owned announcement hook writes on mount and message or severity changes, skipping empty messages. It uses the shared persistent writer: errors use its assertive channel, and warnings and successes use its polite channel. Under FieldStatus AR1 and [AST-086 FR4](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/docs/specs/AST-086-status-announcements/spec.md), an enclosing live region owns the visible message instead. FieldStatus checks that ownership both when queuing and before writing so it does not send a duplicate to the shared channel. The visible box does not acquire a live-region role. Native-dialog routing uses the existing fallback; the upstream modal announcement provider is not ported.

The implementation and both catalog arrangements follow Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`. Verified source includes `FieldStatus.tsx` (`868510a060a3c2cbda4019b75e5b5f712e53db76`), `useFieldStatusAnnouncement.ts` (`ff612e4daa0872afe35e60e7e9162372aa281949`), the Core and Neutral tokens and icons, and the two catalog templates. The attached showcase control is demo-only. Post-first-paint entry animation, field-control composition, published screenshot parity, and manual screen-reader verification remain open.

At the initial checkpoint, all five standard Rust checks passed. Chrome verified both examples and the attached control, including separate live-region writes, type-only and text changes, overlap, decorative icons, and the forced-colors outline. Shared announcement, StatusDot tooltip, Stack Toast, and showcase navigation regressions passed. DOM writes do not establish spoken output.

The ownership follow-up has a focused Node test of the FieldStatus bridge with a simulated DOM and shared writer. The expanded browser regression awaits integrator verification of conditional mount and severity changes inside an already-mounted `role="log"`, and of shared routing when leaving that context. Field integration, form-owned repeated-submit behavior, and the entry animation remain outside this change.

```sh
node --test tests/field_status_announcement.mjs
SHOWCASE_URL=http://127.0.0.1:8129 node tests/field_status.browser.mjs
```
