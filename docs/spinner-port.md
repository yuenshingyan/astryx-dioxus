# Spinner implementation and verification

## Baseline and scope

Pinned reference: Meta Astryx [`476a3153417dbf75c565f7df05b0784e94ddd59e`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e), `packages/core/src/Spinner/{Spinner.tsx,spinnerRotor.ts,SpinnerOwnerContext.ts,Spinner.test.tsx}`, `docs/specs/components/Spinner/Spinner.spec.md`, the motion Chromium tests, English docs, and all four named templates.

Spinner now completes its standalone migration rather than relying on Button's integration audit. Required shared functionality stays with its source owners:

- `src/spinner/spinner.rs`: sizes, shades, indicator, visible labels, root/native props.
- `src/spinner/spinner_rotor.{rs,js}`: one shared frame batch, timeline-origin phase, scoped cleanup. The former Button rotor adapter is removed; Button uses the same Spinner lifecycle as any standalone consumer.
- `src/hooks/use_announce/`: the internal polite shared writer and original native-dialog fallback, not a new combined modal context.
- `src/hooks/use_announce_rendered_text/`: rendered text, appearance dedupe, page-load/hydration policy and scoped lifecycle.
- Text retains typography ownership. No complete Text, generic useAnnounce, ModalAnnounceProvider, modal route/exit holds, or internationalization provider migration is claimed. Those checklist parents stay independent.

No new dependencies, diagnostic suppressions, or breaking public prop changes were needed.

## Public API and Rust adapters

Four sizes draw diameters/strokes of 10/2, 14/3, 18/3, 28/4 CSS pixels; boxes are 14, 20, 24, 36px. Shades are Default, Subtle, OnMedia and Inherit. The rotating arc covers 0.375 of the circumference with round caps, beginning at twelve o'clock over a stationary track.

Existing `label: Option<String>`, `accessible_label`, `is_decorative`, class and style props remain compatible. New support:

- `label_content: Option<Element>` supplies a rich label instead of `label`. The two props are mutually exclusive and return recoverable `SpinnerError::ConflictingLabels` if both are supplied. Internally labels are modeled as text versus rich variants.
- A string label renders as bold body Text, not the old secondary label approximation. A rich node is rendered without an extra typography wrapper.
- `loading_label` supplies the localized fallback (default `Loading`). Applications provide translated text through this prop, as required by the project text policy; there is no pretend translation catalog. An actually needed empty fallback returns `SpinnerError::EmptyLoadingLabel`.
- Native aria-label overrides accessible_label; otherwise a string label names through aria-labelledby, never duplicated as aria-label. Native aria-labelledby is forwarded to the actual indicator even in the labelled form, fixing the source's documented lost-name gap. Empty/whitespace explicit names fall through to a meaningful visible/localized name, rather than reproducing the source's empty-name gap.
- `onmounted` is the root-ref adapter. Dioxus global attributes, IDs/test IDs, native ARIA and additional native event attributes reach the root. Additional events can be supplied through `attributes: vec![dioxus::events::onclick(...)]` at the application boundary; no library business operations are added.
- Without a label, the inline-grid span is the root/theme target. With a label, a column div carries the target, styling inputs and native attributes, followed by the indicator and label inside it.
- Root class/style/private identity and size/shade metadata are protected against raw attribute conflicts. Normal caller width/height cannot replace the ring box, while unrelated style and labelled-wrapper sizing remain caller-owned.
- `is_decorative` remains the prior Rust owner-silencing adapter: it hides the whole component and omits the unexposed role/name. Native aria-hidden retains the named progressbar but removes it from accessibility exposure. A caller non-progressbar role preserves upstream's legacy shared-channel silence. Spinner creates no Tab stop or determinate value.

## Geometry, theming and motion

The five public variables are declared only on the actual theme target:

`--spinner-diameter`, `--spinner-stroke-width`, `--spinner-arc-fraction`, `--spinner-color`, `--spinner-track-color`.

Per-size/shade defaults are in `@layer astryx-base`; `@layer astryx-theme` overrides work for labelled and unlabelled forms. Only these Spinner defaults enter the layer; other shared CSS is not reorganized. A labelled inner ring inherits the wrapper's values instead of redeclaring defaults. Private registered length properties make a bare zero stroke a valid length; a transparent track and a zero-width stroke remain distinct operations. CSS resolves var(), currentColor and color-mix() colors, including changes after mount. The arc fraction changes with the theme, and frame/rotor/SVG/circle geometry move together.

The box cannot shrink even with a narrow flex host and min-width:0. Default SVG dimensions/radius/stroke/dash survive a missing stylesheet. CSS property registration is stylesheet-owned, not repeated on component mount. No JS ring measurements, polling, per-ring listeners, or repeated style registration are introduced.

Rotors share a document-scoped pending Set. One requested frame reads all animations before writing every startTime=0, then empties the Set. Unmounted/disconnected rings are excluded; last-pending removal cancels the frame. No Web Animations support means no phase work. Closed eval channels terminate and clean up.

One iteration holds 100 turns at the pre-tempo slow-min token (525ms per turn in Neutral), linear and clockwise. Reduced motion slows to 3s per turn rather than freezing. Transform animation remains compositor-driven; it is not a dash-offset animation that repaints each frame.

## Announcements and cleanup

The indicator is a named indeterminate progressbar, not a live status role. A standalone appearance after page load announces rendered text through a persistent initially-empty polite region. Rich labels include rendered nested/block/flex text rather than inspecting virtual children; empty visual content uses referenced/fallback names. Unchanged renders do not repeat. Hidden/inert roots re-arm and read again on the next owner render; there is no observer watching every text mutation.

Content arriving with page load or retained Dioxus SSR hydration markers stays silent initially. The real Dioxus hydrated application is not exercised by these tests; marker behavior is checked at the browser boundary. Existing busy owners stay silent, as do explicit hidden and legacy-role cases and content inside an ancestor live container.

Same-frame messages join in order, duplicates keep independent owner shares, and pending shares withdraw on drop. Exposed text clears after 2s using a document-owned timer that retains no component callback. Native modal routing writes inside the nearest active modal and never behind an open modal; eligibility is checked again before exposure. Provider presentation ordering, exit holds and handoff remain with the future modal-owner migration. This is an internal polite subset, not a placeholder public announcement API.

## Examples and verification

`examples/showcase/spinner.rs` ports all four named patterns: Spinner (large), On Media Shade (default plus the original #1a1a2e media panel), Sizes (all four), and With Label (string and exact rich multiline typography). The same section adds all 16 size/shade cases, narrow hosts, themed geometry, localization/reference names, silent adapters, late mounts, wrapper transitions, text changes and removals.

Run all five standard checks plus:

```sh
node tests/spinner.browser.mjs
node tests/spinner_rotor.browser.mjs
node tests/announcement.browser.mjs
node tests/button.browser.mjs
node tests/button_group.browser.mjs
node tests/icon_button.browser.mjs
node tests/link.browser.mjs
```

Use the existing loopback Chrome/CDP setup from [Button browser testing](button-port.md#browser-checks). CDP_URL and SHOWCASE_URL can override defaults. Rebuild the watch-disabled showcase before testing and wait for its initial build to finish.

- Rust tests cover the complete 96 size × shade × content × decorative matrix, root/native attributes, name precedence and empty fallbacks, rich labels, unique label references, hidden/legacy adapters and recoverable validation.
- `spinner-neutral-reference.json` freezes all nine actual catalog controls in light/dark. Comparisons include native tags, type/shade/text, display/gap, root/ring dimensions, arc/track paint, radius/stroke/dash/caps, rotation period and both label typographies. Width permits 1/32 CSS pixel, SVG opacity permits 0.0001 for StyleX's rounded 0.302 versus exact 77/255 (the same 8-bit alpha); other compared fields match exactly. First font family is Figtree; differing unused system fallback stacks are not asserted identical. The catalog release is not asserted identical to the pinned commit. Tests use frozen evidence, not live services.
- **Pinned AR2 coverage follow-up:** The port already matches the reference: the track SVG and arc's rotor each carry `aria-hidden="true"`. `decorative_track_and_rotor_are_hidden_on_exposed_indicators` in `tests/spinner.rs` checks each separately for both unlabelled and string-labelled Spinners, without hiding the root or progressbar. This is SSR markup evidence, not a browser accessibility-tree check for the decorative subtrees. No production change was needed; the existing naming and owner-silencing adaptations are unchanged.
- Integrated browser checks cover all five theme seams including labelled inheritance/zero stroke/dynamic paint, actual AX names, non-shrinking sizing, announcements and drop, wrapper transitions, late-origin equality, eight exact clockwise phase samples, 3s reduced motion, forced colors and no-CSS SVG geometry.
- Idle tracing finds no Paint/Layout and at most two style recalculations over 1.2s of isolated motion. Screencast pixels from at least three frames during a 700ms JavaScript block show the arc turning by more than 120 degrees, rather than merely finding an animation object.
- The rotor boundary test controls only RAF/Web Animations APIs to prove one batch, read-before-write order, pending/unmounted cancellation, unsupported API fallback and null-channel cleanup.
- The announcement boundary suite checks load/hydration markers, rich/block/hidden text, equal-argument renders, references, appearance dedupe, pending/exposed cleanup, joined duplicate owners, actual native dialogs, covered/outside-modal silence and open/close-before-write races.
- Existing Button, ButtonGroup, IconButton and Link browser suites remain required regressions.

## Source-color findings and limits

`node tests/spinner.contrast.mjs` (or `--json`) audits intended core and showcase Neutral surfaces in light/dark: Default/Subtle/Inherit on page/surface; OnMedia on its original dark panel. All sizes share paint; Inherit assumes the default primary parent.

Results: **0/28 essential arc/backdrop failures at 3:1; 6/28 arc/track failures at 3:1; 2/28 optional string-label failures at 4.5:1**. A low-contrast track alone is not treated as essential. The two label failures are primary dark text on a media panel in light mode; use caller-owned light rich content or an appropriate theme when composing labels on dark media. The default named on-media example has no label.

The audit intentionally exits 1. Original colors are retained under the existing source-fidelity decision, not called contrast compliant. Arbitrary custom colors/backgrounds/rich content remain application-owned. Safari, Firefox, physical-device, actual screen-reader delivery and a real SSR hydration application remain unverified; Chrome's AX tree is not a screen-reader certificate.
