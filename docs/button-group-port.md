# ButtonGroup implementation and verification

## Baseline and ownership

ButtonGroup follows Meta Astryx revision [`476a3153417dbf75c565f7df05b0784e94ddd59e`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e):

- `packages/core/src/ButtonGroup/{ButtonGroup.tsx,ButtonGroupContext.ts,ButtonGroup.doc.mjs}`
- `docs/specs/components/ButtonGroup/ButtonGroup.spec.md`
- `packages/core/src/hooks/{useListFocus.ts,useGroupPressTracking.ts}`
- `packages/core/src/IconButton/IconButton.tsx`
- `packages/core/src/Layer/layerScopedContext.tsx`
- All three named examples under `packages/cli/assets/templates/blocks/components/ButtonGroup/`.

`src/button_group/` owns rendering, its context, and the scope-owned controller in `assets/button-group-controller.js`. Button continues to own each member's paint, activation, async work, tooltip, and press feedback. Group roving is no longer implemented in Button's controller.

The required IconButton is the same thin, explicit icon-only Button composition as upstream, under `src/icon_button/`. It requires an icon and label, forwards all applicable Button props, and excludes visible children/end content. `src/layer/layer_scoped_context.rs` provides the original LayerContentBoundary utility: a headless reset of the implemented surface contexts, not a new combined environment or a standalone Popover/DropdownMenu implementation. Its stable typed provider sequence is the Rust counterpart of React's dynamic reset registry. Shared LinkProvider state is not reset.

## Implemented contract

- Horizontal (default) and vertical orientations; small/medium/large inherited sizing; explicit member overrides.
- None/low/medium/high elevation on the shared surface; every member reports and paints no elevation.
- Required, non-empty accessible group label; invalid names return a recoverable render error. Root role, theming data, native attributes, class/style, keyboard/focus callbacks, and MountedData forwarding.
- Focusable but inert disabled Button/IconButton members; a group cannot re-enable a child's own disabled state. Pending work stays on its initiating child. Disabling a held group cancels press feedback and activation.
- Exactly one mounted Tab stop, initially on the first enabled member (or one disabled member when none are enabled). Links and native focusable members participate. Native `disabled` controls are not focusable members.
- Orientation-aware arrows, RTL visual direction, Home/End, wrapping, and skipping disabled/noninterruptible-busy members during navigation. A focused or existing holder stays discoverable when it becomes disabled/busy. Mutation repair handles insertion, removal, and anchor/button replacement without adding a second stop.
- Pointer/programmatic focus adopts the roving stop, so Tab/Shift+Tab leave the group instead of stopping again inside it. Root `onfocus` uses bubbling focusin; caller `onkeydown.prevent_default()` cancels group navigation.
- Member-owned popovers/dialogs form keyboard boundaries. A group inside a layer still owns its own keys. LayerContentBoundary resets inherited group/input/size/icon-slot defaults; new providers inside it work normally.
- Gap-free joined surfaces, outer-only logical corners, automatic leading-edge dividers, solid-member divider colors, and no press scale. Trailing popovers/templates/dialogs do not steal the final member's corners. Display-contents wrapped members join correctly; unrelated native children retain their own paint.
- Under coarse-primary-pointer presentation, finger and pen slides highlight the release member and activate it once. Release outside targets, cancellation, scrolling, unmount, or disablement do not activate. Mouse input remains native even on a coarse-primary-pointer device. Menu/listbox triggers keep their own opening rather than joining group tracking.
- The group's hit layer grows member targets toward 44px without changing control dimensions. Growth is clipped at half the gap to neighbouring controls, and empty space in a wide group is not an action. Layout/membership changes refresh measurements through scope-owned observers; no idle polling loop is used.
- Scope cleanup disconnects mutation/resize observers, restores managed tab indices, removes owned geometry/press markers and hit insets, and releases shared listeners when the last group is removed. Closed eval channels also terminate the controller.

Upstream's contract records gaps in wrapped-member geometry and roving-holder repair. This implementation follows the stated invariants rather than reproducing those defects. Existing provisional API spellings and adapters remain compatible; no public APIs were removed.

## Examples

`examples/showcase/button_group.rs` includes:

1. **ButtonGroup — Basic**: Copy, Cut, Paste.
2. **ButtonGroup — Floating**: Zoom out, Reset, Zoom in, with medium group elevation.
3. **ButtonGroup — Showcase**: clipboard glyph actions and the Save/Save options split surface, using the same Heroicons glyphs as upstream.

It also demonstrates sizes/elevations/orientations, member size overrides, RTL, native members, key cancellation, dynamic busy/disabled/removal/replacement, wrapped members, caller-owned native layer content, bounded targets, and controller cleanup. The native layer fixture is not a placeholder DropdownMenu port; it demonstrates the group boundary using application-owned content. Other component migration parents remain independent.

## Verification

Run the five standard project checks from README, then:

```sh
node tests/button.browser.mjs
node tests/button_group.browser.mjs
```

Browser prerequisites and loopback-only Chrome setup are documented in [Button browser checks](button-port.md#browser-checks). The new suite uses the same `CDP_URL`/`SHOWCASE_URL` overrides; `GROUP_SCREENSHOT_PATH` optionally saves a screenshot. Tests use real CDP mouse, keyboard, finger, and pen input. Scroll targeting waits for compositor motion to finish rather than retrying failed assertions.

Rust tests cover all 48 orientation × size × elevation × disabled combinations, root attributes/callback props, validation errors, size precedence, independent layer contexts, shared routing, IconButton forwarding, and existing compatibility contracts. The browser suite covers the interaction and lifecycle cases above and checks the native accessibility tree's group role/name. The existing Button regression suite must still pass.

### Source appearance comparison

`tests/fixtures/button-group-neutral-reference.json` contains measurements from the actual [Astryx ButtonGroup page](https://astryx.atmeta.com/components/ButtonGroup), in Neutral light and dark modes, for all named patterns. The catalog's displayed release is not asserted to equal the pinned source revision.

The suite compares member colors, font/weight, height, padding, gap, corners, dividers, and shadows; shared root display/alignment/orientation/radius/shadow/height; and widths. Non-width recorded properties must match exactly. Each member permits at most 1/32 CSS pixel (two Chromium layout subpixels), consistent with Button's reference comparison; the group's total width permits only the sum of those member tolerances. It also checks vertical corners/dividers/cross-axis stretch, all size heights, all elevation tiers, zero inter-member gaps, and a lone member's corners. This is not an automatically accepted screenshot snapshot.

### Source-palette exception

Run:

```sh
node tests/button_group.contrast.mjs
node tests/button_group.contrast.mjs --json
```

The unchanged core palette inherits Button's documented dark-primary hover/press issue: **4/48 label cases fall below 4.5:1**. Inherited essential-icon/spinner-arc paint passes 3:1 in the 48 modeled cases; focus versus page/surface backdrops passes in all 16 modeled cases. **24/48 dividers fall below the conditional 3:1 target**. If a divider is necessary to distinguish a control, applications must strengthen that theme; matching source paint is not a claim of compliance.

The audit intentionally exits 1 for those findings. Original colors are retained consistently with the [accepted source-fidelity decision](button-port.md#core-palette-contrast-conflict); known failures are not reported as passing contrast. The shared color calculation helper leaves the existing Button audit results unchanged. These are source-token calculations, not arbitrary custom-content/theme, antialiasing, adjacent-surface, or assistive-technology certification.

## Boundaries

- Chromium/CDP checks are not a completed Safari/Firefox, physical-device, or screen-reader matrix.
- Nested groups and overflow/wrapping are uncontracted upstream. Use separate groups with a gap; this component does not add wrapping/collapsing or selection state.
- Each member's application work stays caller-owned; the group does not cancel tasks or disable itself to represent pending work.
- A mounted browser controller owns roving; server markup alone does not implement keyboard navigation. Native raw children retain their own semantics and disabled handling; use Button/IconButton for inherited availability and connected paint.
