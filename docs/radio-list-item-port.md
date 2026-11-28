# RadioListItem checkpoint

RadioListItem composes Item with a native radio and decorative RadioIndicator. RadioList supplies the controlled value, shared form name, control size, and disabled and required settings. Caller-owned native radios remain supported. The input references the row's visible label and description and owns keyboard focus.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `RadioList/RadioListItem.tsx` (`5a012a65cc6e39e8106a2086488058c84c9226cc`) and its spec (`7ae4ab8b2ebda5bcc3eb0a4fe0b495ff3fa4d9ad`). The two shipping examples follow the pinned `RadioListItemBasic` (`13fcaaccf`) and `RadioListItemShowcase` (`4fe80160`) templates.

The row publishes size, selection, and disabled state on the painted Item. Only an enabled row supplies the indicator hover scope. Disabled label, description, and end content use the same opacity as the pinned Item (`33a18f0c`); this does not disable caller-owned controls. A non-interactive row click focuses and activates the same radio as a direct click. No extra tab stop is added. The existing single focus cue follows `input:focus-visible` at the circle, not focus on a nested link or button. Slot order follows the pinned implementation (circle before `startContent`), not its contradictory prop prose.

### Verification boundaries

The initial Chrome run verified controlled arrow-key selection, retained focus, native disabled activation blocking, and a visible rich label and description in AX. This follow-up adds SSR checks and an integrator-only suite (`SHOWCASE_URL=… node tests/radio_list_item.browser.mjs`) for native focus and event routing, disabled transitions, Chrome AX, and computed hover, opacity and forced-colors paint. No new browser run, raster comparison, or spoken-output verification was performed locally.

- **Disabled focusability (FR6):** Native `disabled` is retained. Keeping disabled radios in native same-name navigation allows the browser to select them from an enabled peer; splitting the native group changes Tab order and form semantics. Closing this needs a RadioList-owned focus/navigation policy and disabled-reason context (including per-item descriptions), not an item-only `aria-disabled` substitution.
- **Focus and callback decisions (OQ1/OQ2):** Preserve the existing circle-local cue and radio-owned callback without choosing a new shared focus policy or changing callback ownership. A disabled focus cue awaits the focusable disabled contract.
- **Touch and press (FR8/FR13):** The input has a 24px coarse minimum, not a verified 44px row band. The shared hit-area scanner sees native inputs, not adjacent presentational Item rows; it needs row rectangles for the halfway-gap rule. Press overlay/long-hold and theme indicator replacement remain open; this does not introduce a parallel press controller or registry.
