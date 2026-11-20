# Astryx Component Migration Checklist

Source: [Astryx component catalog](https://astryx.atmeta.com/components), including its sidebar. Detailed options and named examples use [upstream revision `476a3153417dbf75c565f7df05b0784e94ddd59e`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e).

This is the single working checklist: **276 component/API tasks with 2,573 detailed tasks nested directly underneath them**. There are 189 component, subcomponent, provider, and theme entries plus 87 supporting hooks/utilities. Category order is not implementation priority.

- Tick a detailed task only after implementing and verifying it. Tick its parent only when every applicable detailed task is complete, including shared dependencies, accessibility, tests, showcase examples, and comparison against the original. Do not mark partial ports complete.
- Options and defaults are upstream contracts, not seeded playground values. Arbitrary text, data, dimensions, URLs, and theme extensions are not finite style variants. Boolean options cover both true and false unless their documented contract says otherwise.
- Named examples may overlap variant/state tasks. They do not imply a separate example for every Cartesian product. All 699 associated template metadata files are represented.
- The source snapshot is not a verified release tag for the catalog's displayed version. This covers the linked documentation, not every undocumented package export.
- Preserve upstream project structure and module ownership. Adapt React-specific mechanics idiomatically to Rust/Dioxus without inventing combined contexts or moving dependencies under their first consumer. Follow [AGENTS.md](AGENTS.md) and the [README migration workflow](README.md#migration-workflow).
- [AspectRatio](docs/aspect-ratio-port.md), Button, [ButtonGroup](docs/button-group-port.md), [IconButton](docs/icon-button-port.md), [Link](docs/link-port.md), [Spinner](docs/spinner-port.md), [Badge](docs/badge-port.md), [Divider](docs/divider-port.md), [Kbd](docs/kbd-port.md), [Skeleton](docs/skeleton-port.md), [StatusDot](docs/status-dot-port.md), [ProgressBar](docs/progress-bar-port.md), [Card](docs/card-port.md), and [SelectableCard](docs/selectable-card-port.md) are complete for the source-faithful migration baseline. Their source-color audits retain the explicit [upstream-palette exception](docs/button-port.md#core-palette-contrast-conflict), not a claim that contrast targets pass. Other component parents remain unchecked, including dependencies that have not completed their own standalone migration audits. Bottom Sheet Switcher is deprecated upstream; confirm inclusion before porting.

## Action

- [x] **[Button](https://astryx.atmeta.com/components/Button)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Button/Button.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Button).

  - **Finite choices**
    - [x] `variant`: `primary`, `secondary` (default), `ghost`, `destructive`.
    - [x] `size`: `sm`, `md` (default), `lg`; implicit leading Icon size sm for sm/md buttons, md for lg.
    - [x] `elevation`: `none` (default), `low`, `med`, `high`; ignored inside ButtonGroup, which owns elevation.
    - [x] HTML `type`: `button` (default), `submit`, `reset`; optional form/name/value participation.
  - **States, content, and activation**
    - [x] `isDisabled=false` default; disabled uses aria-disabled and stays focusable.
    - [x] `isLoading=false` default; spinner replaces icon, aria-busy/aria-disabled, blocks repeat activation without dropping focus, live-announces Loading.
    - [x] `clickAction` synchronous or Promise-returning, automatic pending spinner/deduplication; regular onClick alternative. `isInterruptible=false` default; true retains busy/spinner but permits fresh clicks without dedupe while pending.
    - [x] `isIconOnly=false` default; square icon-only requires icon, label becomes aria-label; trailing content ignored. Dedicated IconButton is a separate component/import.
    - [x] Required label, optional visible children override, leading icon, trailing Icon or Badge with inherited color, optional tooltip.
    - [x] Content-width default or arbitrary width/full-width; constrained labels ellipsize with full accessible name and focus tooltip; icon-only stays square.
    - [x] Native action button versus href anchor/custom `as` link; target/rel for navigation.
    - [x] Rest, hover, pointer-down, keyboard-focus, disabled, loading; touch press paints temporary ::before layer, so style child rather than overriding that pseudo-element.
    - [x] Complete the source-color contrast audit and resolve the fidelity/contrast conflict. Targets: label/badge 4.5:1, essential icon/spinner and focus 3:1; necessary boundary 3:1, disabled exceptions. Audit all 14 Badge colors on page/surface in rest/hover/down. **Decision: retain original paint under the [accepted upstream-palette exception](docs/button-port.md#core-palette-contrast-conflict). The audit found 267/672 Badge and 4/48 Button-label failures; these are known failures, not passing contrast results. Custom content remains the caller's responsibility.**
  - **Inline example and templates**
    - [x] Inline **Actions in narrow rows**: long review/invitation labels truncate; square icon controls and static short Cancel remain usable.
    - [x] **Button — Floating** (`ButtonFloating.doc.mjs`): elevation med FAB.
    - [x] **Button — Variants** (`ButtonShowcase.doc.mjs`): four styles.
    - [x] **Button — Sizes** (`ButtonSizeVariants.doc.mjs`): sm/md/lg.
    - [x] **Button — Variants** (`ButtonVariants.doc.mjs`): all four in default/disabled/loading.
    - [x] **Button — End Slot** (`ButtonWithEndSlot.doc.mjs`): trailing count/status badge.
    - [x] **Button — Icon** (`ButtonWithIcon.doc.mjs`): leading icon and label.

- [x] **[Button Group](https://astryx.atmeta.com/components/ButtonGroup)**

  - Sources: [English doc, nested component API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/ButtonGroup/ButtonGroup.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ButtonGroup).

  - **Choices and behavior**
    - [x] `orientation`: `horizontal` (default), `vertical`.
    - [x] Group default `size`: `sm`, `md` (default), `lg`; individual child size may override.
    - [x] `elevation`: `none` (default), `low`, `med`, `high`; connected children share one lifted surface, dividers, outer-edge radii.
    - [x] `isDisabled=false` default versus whole-group disabled; pending work belongs on initiating child rather than disabling group.
    - [x] Required accessible group label; Button/IconButton children, normally matching variants; automatic internal dividers.
    - [x] One Tab stop; orientation-aware arrows, Home/End to edges; label announced with members.
    - [x] Touch slides highlight across buttons and activates release target, outside release does nothing; native mouse press, menu trigger retains own opening. Invisible touch targets grow toward 44px bounded by neighbor gap.
    - [x] Complete the source-color contrast audit: text 4.5:1; essential icon/spinner, necessary divider/boundary, focus 3:1; disabled appearance exempt. **Retain original colors under the existing source-fidelity decision. The [Group audit](docs/button-group-port.md#source-palette-exception) reports 4/48 label failures and 24/48 dividers below conditional 3:1; these are known findings, not passing contrast or compliance claims.**
  - **Templates**
    - [x] **ButtonGroup — Basic** (`ButtonGroupBasic.doc.mjs`): three named connected actions.
    - [x] **ButtonGroup — Floating** (`ButtonGroupFloating.doc.mjs`): med elevation shared action bar.
    - [x] **ButtonGroup** (`ButtonGroupShowcase.doc.mjs`): named showcase, metadata gives no further description.

- [ ] **[Dropdown Menu](https://astryx.atmeta.com/components/DropdownMenu)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/DropdownMenu/DropdownMenu.doc.mjs). Templates: [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components), directory `DropdownMenu/`.

  - **Presentation and trigger**
    - [ ] `presentation`: `popover`, `bottom-sheet`, `adaptive`; omission is anchored but differs from explicit `popover` for touch press timing. Compound children support popover only.
    - [ ] `placement`: `above`, `below` (default), `start`, `end`; `alignment`: `start` (default), `center`, `end`. Logical start/end mirror in RTL; ignored in sheet mode.
    - [ ] Built-in `button` (default `{label: 'Menu'}`) versus mutually exclusive `renderTrigger`: IconButton, chip, avatar, or list-row trigger; spread press, keyboard, ID, and disclosure props.
    - [ ] `hasChevron`: true (default) / false, including icon-only triggers.
    - [ ] Controlled `isMenuOpen` with `onOpenChange` versus internally managed visibility; mounting open does not steal focus, later opening focuses the first item.
    - [ ] Width matches trigger by default, capped to viewport; custom length/intrinsic/CSS-wide widths; default 300px maximum height can be lifted with `menuMaxHeight`, still viewport-bounded.

  - **Entries, slots, and interaction**
    - [ ] Data `items` versus compound `children` (mutually exclusive): action rows, `{type: 'divider'}`, `{type: 'section', title?, items}`, nested items, checkbox/radio rows in compound mode.
    - [ ] Action label, icon, description, trailing `endContent` (shortcut/badge), stable ID; `variant`: `default`, `destructive`; disabled/enabled and close-on-select/stay-open actions.
    - [ ] Destination rows use real links, including target/rel and browser modified/middle-click behavior, rather than treating every row as a callback.
    - [ ] Optional `header` and `footer` outside menu rows; with either, popover becomes a modal dialog containing controls and menu: Tab/Shift+Tab wrap; Escape closes.
    - [ ] Mouse opens on press-down and can drag to select; highlight follows held pointer; release selects row under pointer; ignored trailing click after light dismissal.
    - [ ] Omitted presentation: finger/pen opens as pressed, with 150ms scroll-view delay and selection after 10px movement; explicit `popover` retains pointer behavior including 500ms hold.
    - [ ] Adaptive anchored/sheet surface; nested flyouts versus phone drill-in with Back row; trigger exposes `aria-haspopup`, `aria-expanded`, `aria-controls`, menu named by trigger.

  - **Named templates**
    - [ ] `DropdownMenuActions` — **DropdownMenu — Actions**: dividers separate safe and destructive row actions.
    - [ ] `DropdownMenuBottomSheet` — **DropdownMenu — Adaptive presentation**: product media-query policy selects sheet versus anchor.
    - [ ] `DropdownMenuNoChevron` — **DropdownMenu — Icon Trigger**: icon-only trigger without chevron or visible label.
    - [ ] `DropdownMenuShowcase` — **DropdownMenu**: action menu initially open for preview, not an API default.
    - [ ] `DropdownMenuWithDisabledItems` — **DropdownMenu — Disabled**: permission-based disabled rows.
    - [ ] `DropdownMenuWithSections` — **DropdownMenu — Sections**: titled action categories.
    - [ ] `DropdownMenuWithSubmenu` — **DropdownMenu — Submenu**: hover/Right opens; Left/Escape returns.

- [x] **[Icon Button](https://astryx.atmeta.com/components/IconButton)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/IconButton/IconButton.doc.mjs). Templates in `IconButton/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Finite choices and state**
    - [x] `variant`: `primary`, `secondary` (default), `ghost`, `destructive`; `size`: `sm`, `md` (default), `lg`.
    - [x] `elevation`: `none` (default), `low`, `med`, `high`; supports floating action button.
    - [x] `type`: `button` (default), `submit`, `reset`; optional form/name/value integration.
    - [x] `isDisabled`, `isLoading`, `isInterruptible`: each false by default / true.
    - [x] Disabled/loading remain focusable with aria-disabled; loading adds aria-busy and blocks reactivation unless interruptible action policy permits it.
    - [x] `clickAction` provides automatic async loading; interruptible pending action permits re-click/new invocation rather than deduping, not a guaranteed task-cancellation API.

  - **Content, links, and access**
    - [x] Required accessible label (not visible) and icon; contextual icon size sm for sm/md buttons, md for lg.
    - [x] Optional tooltip on hover/focus describes unless identical to accessible name; default square aspect versus explicit arbitrary width.
    - [x] `href` renders anchor/custom `as` link; disabled IconButton still renders button. Target/rel support normal link integration.
    - [x] Audit rest/hover/pressed essential glyph and loading arc contrast 3:1; focus indicator 3:1; boundary 3:1 only when needed; disabled contrast exception. **[Results](docs/icon-button-port.md#contrast-results-and-limits): 0/48 essential glyph/arc and 0/16 focus/backdrop failures; 8/16 resting surfaces below the conditional boundary target. Original source paint is retained; strengthen the theme if an edge is needed to identify the control. This is not a blanket compliance claim.**

  - **Named templates**
    - [x] `IconButtonActionBar` — **IconButton — Action Bar**: ghost toolbar row.
    - [x] `IconButtonFloating` — **IconButton — Floating**: high-elevation FAB.
    - [x] `IconButtonLoadingToggle` — **IconButton — Loading State**: spinner during async click.
    - [x] `IconButtonShowcase` — **IconButton**: wrench icon button.
    - [x] `IconButtonTooltipIconButton` — **IconButton — With Tooltips**: hover action explanations.

- [x] **[Link](https://astryx.atmeta.com/components/Link)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Link/Link.doc.mjs). Templates in `Link/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Typography and modes**
    - [x] `type`: `body` (default), `large`, `label`, `supporting`, `code`, `display-1`, `display-2`, `display-3`, `inherit`; custom theme text types remain open-ended.
    - [x] `size`: `4xs`, `3xs`, `2xs`, `xsm`, `sm`, `base`, `lg`, `xl`, `2xl`, `3xl`, `4xl`.
    - [x] `weight`: `normal`, `medium`, `semibold`, `bold`; `color`: `primary`, `secondary`, `disabled`, `placeholder`, `accent` (default), `inherit`.
    - [x] `display`: `inline` (default), `block`; maxLines default 0, arbitrary positive clamp.
    - [x] `hasUnderline`, `isDisabled`, `isExternalLink`, `isStandalone`: each false by default / true.
    - [x] Inline anchor versus standalone base font sizing; custom `as` versus provider/native anchor; onClick without href may render button.

  - **Navigation, content, and access**
    - [x] Required children supply name for text links; label only for non-self-descriptive/icon-only content; optional tooltip.
    - [x] External link opens new tab with icon and safe rel; newTabLabel defaults `(opens in new tab)`; target=_blank merges noopener noreferrer.
    - [x] Download false/true or suggested filename; referrer policy and arbitrary target/rel remain browser attributes, not closed visual variants.
    - [x] Disabled link retains Tab stop and announced state but cannot navigate/onClick; rejected URL inert and never calls router.
    - [x] Shared URL rule strips controls/trims/case-inspects schemes: rejects javascript:, vbscript:, data:text/html; accepts ordinary relative/fragments/http(s)/mailto/tel/safe custom schemes/downloads. Resources have separate policies.
    - [x] Outer Text maxLines can clamp flowing text Link; link-owned box/display/maxLines, button rendering, or block children break outer clamp. Audit text contrast 4.5:1 including pressed overlay. **[Source-color results](docs/link-port.md#contrast-and-verification-boundaries): 19/96 active cases below 4.5:1, including disabled-color and accent pressed paint; 0/24 focus failures at 3:1. Original colors are retained under the existing fidelity decision, not reported as passing contrast.**

  - **Named templates**
    - [x] `LinkExternalLinks` — **Link — External Links**: new-tab external icon list.
    - [x] `LinkInlineLink` — **Link — Inline Text**: body paragraph link.
    - [x] `LinkShowcase` — **Link**: standalone link.
    - [x] `LinksWithTooltips` — **Link — With Tooltips**: standalone horizontal row.
    - [x] `useLinkComponentHookUsage` — **useLinkComponent — Article Cards**: as override, nearest provider, native fallback with router location.
    - [x] `useLinkifyHookUsage` — **useLinkify — Links in a Comment Thread**: URL/email/issue patterns and unchanged plain comment.

- [ ] **[More Menu](https://astryx.atmeta.com/components/MoreMenu)**

  - Sources: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/MoreMenu/MoreMenu.doc.mjs), [Button type choices](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Button/Button.doc.mjs). Templates in `MoreMenu/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Trigger, placement, and visibility**
    - [ ] Trigger `variant`: `primary`, `secondary`, `ghost` (default), `destructive`; ButtonSize `sm`, `md`, `lg`, surrounding SizeProvider/ButtonGroup/Toolbar wins when omitted, otherwise md.
    - [ ] `isDisabled`: false (default) / true; default three-dot icon versus custom ReactNode icon.
    - [ ] Label/tooltip default `More options`; required data items use DropdownMenu actions/dividers/sections and nested entries.
    - [ ] `placement`: `above`, `below` (default), `start`, `end`; `alignment`: `start` (default), `center`, `end`; logical RTL.
    - [ ] `presentation`: `popover`, `bottom-sheet`, `adaptive`; omission anchored with touch press behavior, explicit popover includes hold. Adaptive compact threshold: coarse primary pointer and width below theme md (default 768px) or height below 500px.
    - [ ] Controlled isMenuOpen/onOpenChange versus internal open state; same menu press timing (150ms scroll delay/10px pick movement) as DropdownMenu.
    - [ ] Host data attributes reach trigger, name/ID/disclosure remain owned; secondary overflow actions rather than hidden primary actions.

  - **Named templates**
    - [ ] `MoreMenuBottomSheet` — **MoreMenu — Bottom Sheet**: visible trigger, short sheet action set.
    - [ ] `MoreMenuDefaultMoreMenu` — **MoreMenu — Default**: text-only actions.
    - [ ] `MoreMenuShowcase` — **MoreMenu**: basic overflow action list.
    - [ ] `MoreMenuWithDividers` — **MoreMenu — With Dividers**: separate safe/destructive actions.
    - [ ] `MoreMenuWithSections` — **MoreMenu — With Sections**: titled groups.

- [ ] **[Segmented Control](https://astryx.atmeta.com/components/SegmentedControl)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/SegmentedControl/SegmentedControl.doc.mjs). Templates in `SegmentedControl/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Selection and layout modes**
    - [ ] Required controlled value/onChange and accessible radio-group label; exactly one mutually exclusive mode, not route navigation or independent toggles.
    - [ ] size `sm`, `md` (default), `lg`; layout `hug` (default content width capped to container, truncating labels), `fill` (equal stretched segments).
    - [ ] isDisabled false (default) / true; selected segment retains Tab stop/aria-disabled and cannot select another; disabledMessage describes each segment on hover/focus for group-disabled state.
    - [ ] Required SegmentedControlItem children; icon+label, text-only, hidden-label icon-only content patterns.

  - **Pointer, keyboard, and accessibility behavior**
    - [ ] Coarse-pointer tap/key/programmatic selection slides; reduced motion crossfades without travel; selected press lifts, drag carries selection/color to release position, even beyond control bounds.
    - [ ] Return to selected segment or Escape cancels change; unselected press uses shared overlay without dimming; mouse/pen clicks and dragging also slide on themed duration/easing.
    - [ ] Named radiogroup/segments, arrow-key selection; labels retain full accessible names despite truncation.
    - [ ] Text 4.5:1 across rest/hover/pressed/selected and translucent-track surfaces; icon-only glyph/focus 3:1; selected indicator/boundary 3:1 when relied on; disabled exception.

  - **Named inline example**
    - [ ] **In a narrow card**: hug control in VStack remains content-sized then caps at narrow Card, labels ellipsize instead of overflow, full name retained.

  - **Named templates**
    - [ ] `SegmentedControlDisabledItem` — **SegmentedControl — Disabled Item**: individually unavailable segment.
    - [ ] `SegmentedControlFillLayout` — **SegmentedControl — Fill Layout**: equal width across container.
    - [ ] `SegmentedControlIconOnly` — **SegmentedControl — Icon Only**: hidden visible labels/accessibly named icons.
    - [ ] `SegmentedControlShowcase` — **SegmentedControl**: standard group.
    - [ ] `SegmentedControlWithIcons` — **SegmentedControl — With Icons**: icon/label view switcher.

- [ ] **[Toggle Button](https://astryx.atmeta.com/components/ToggleButton)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/ToggleButton/ToggleButton.doc.mjs).

  - **Choices and state ownership**
    - [ ] `size`: `sm`, `md` (default outside group), `lg`, inherits group unless explicit; `elevation`: `none` (default), `low`, `med`, `high`, retained inside group too.
    - [ ] Standalone controlled isPressed false/true; synchronous onPressedChange can prevent pressedChangeAction. Action may run without callback, optimistic pressed state/spinner while pending, re-click interruptible; callback-only has no Action-pending feedback.
    - [ ] Value-bound group member ignores own isPressed/onPressedChange/pressedChangeAction; group onChange owns state, no member transition.
    - [ ] `isDisabled`, `isLoading`, `isIconOnly`: false (default), true. No separately documented variant/color prop; colored pressed icons are caller content.

  - **Content and accessibility**
    - [ ] Stable required label; text children, icon with label, icon-only automatically when icon/no children, or explicit isIconOnly; accessible name/automatic tooltip from label, optional explicit tooltip.
    - [ ] Default icon/pressedIcon swap (pressed fallback to icon), label weight emphasis, pending spinner.
    - [ ] Label contrast 4.5:1; essential icon/spinner/focus outline 3:1; selected background/boundary 3:1 only if relied on to convey state/control; disabled excepted.

  - **Named templates**
    - [ ] [`ToggleButtonShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ToggleButton/ToggleButtonShowcase.doc.mjs) — “ToggleButton” / “Toggle Button”.
    - [ ] [`ToggleButtonColor`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ToggleButton/ToggleButtonColor.doc.mjs) — “ToggleButton — Color”: accent formatting and caller-colored star/heart/bookmark reactions.
    - [ ] [`ToggleButtonGroup`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ToggleButton/ToggleButtonGroup.doc.mjs) — “ToggleButton — Group”: single view switcher/multiple formatting controls (metadata stored with ToggleButton).
    - [ ] [`ToggleButtonIconSwap`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ToggleButton/ToggleButtonIconSwap.doc.mjs) — “ToggleButton — Icon Swap”: outline/solid favorite/bookmark/mute.
    - [ ] [`ToggleButtonLabel`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ToggleButton/ToggleButtonLabel.doc.mjs) — “ToggleButton — Label”: text weight shift.
    - [ ] [`ToggleButtonStates`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ToggleButton/ToggleButtonStates.doc.mjs) — “ToggleButton — States”: default, pressed, disabled, loading.

- [ ] **[Toggle Button Group](https://astryx.atmeta.com/components/ToggleButtonGroup)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/ToggleButton/ToggleButtonGroup.doc.mjs).

  - **Modes and behavior**
    - [ ] `type`: `single` (default; string/null), `multiple` (string array); controlled value/onChange, no defaultValue API documented. Single permits no pressed member.
    - [ ] `orientation`: `horizontal` (default), `vertical`; `size`: `sm`, `md` (default), `lg`; `isDisabled`: false (default), true.
    - [ ] Required accessible group label, value-bearing ToggleButton children. Group owns their pressed state, ignores member state/callback/Action props, activates through group onChange; child size may override inherited default.
    - [ ] Touch press-slide tracks highlight; only release over member toggles, release outside does nothing. Finger targets grow toward 44px, limited by half neighbor gap without growing visual buttons.

  - **Named templates**
    - [ ] [`ToggleButtonGroupShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ToggleButtonGroup/ToggleButtonGroupShowcase.doc.mjs) — “ToggleButtonGroup” / “Toggle Button Group”: view/filter selection.
    - [ ] [`ToggleButtonGroupVertical`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ToggleButtonGroup/ToggleButtonGroupVertical.doc.mjs) — “ToggleButtonGroup — Vertical”: vertical single and multiple.
    - [ ] Related family metadata [`ToggleButtonGroup`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ToggleButton/ToggleButtonGroup.doc.mjs) — “ToggleButton — Group”: horizontal single/multiple (listed under ToggleButton association above).

- [ ] **[Toolbar](https://astryx.atmeta.com/components/Toolbar)**

  - Sources: [English family API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Toolbar/Toolbar.doc.mjs), [SectionVariant](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Section/Section.doc.mjs).

  - **Choices and slots**
    - [ ] `size`: `sm`, `md` (default), `lg`, inherited by Button/TextInput/TabList/Selector; `gap`: `0`, `0.5`, `1` (default), `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`.
    - [ ] `variant`: `section`, `transparent` (default), `muted`; divider edge array `top`, `bottom`, `start`, `end`.
    - [ ] `orientation`: `horizontal` (default), `vertical` for arrow-key navigation, not a promise of vertical chrome layout.
    - [ ] Named toolbar aria-label; Section-owned surface/padding/divider chrome. Start/end slots; optional center creates three lanes centered while space allows, wrapping when lanes do not fit.
    - [ ] Contextual bulk actions/card header/table filter/tabbed section patterns; unlike TopNav it is not app-wide navigation. Group child sizing at toolbar rather than each child.

  - **Named templates**
    - [ ] [`ToolbarBulkActions`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Toolbar/ToolbarBulkActions.doc.mjs) — “Toolbar — Bulk Actions”: small muted batch actions.
    - [ ] [`ToolbarCardHeader`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Toolbar/ToolbarCardHeader.doc.mjs) — “Toolbar — Card Header”: title and icon controls.
    - [ ] [`ToolbarSizes`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Toolbar/ToolbarSizes.doc.mjs) — “Toolbar — Sizes”: automatic child size cascade.
    - [ ] [`ToolbarTableFilter`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Toolbar/ToolbarTableFilter.doc.mjs) — “Toolbar — Table Filter”: search, field-as-filter-chip triggers, end-folding clause count, live result count, clear all, column picker.
    - [ ] [`ToolbarThreeSlot`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Toolbar/ToolbarThreeSlot.doc.mjs) — “Toolbar — Three Slot”: centered heading/navigation/actions (showcase).
    - [ ] [`ToolbarWithTabs`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Toolbar/ToolbarWithTabs.doc.mjs) — “Toolbar — Tab Navigation”: tabs plus primary action.

## Chat

- [ ] **[Chat Composer](https://astryx.atmeta.com/components/ChatComposer)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Chat/ChatComposer.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ChatComposer).

  - **Choices and composition**
    - [ ] `density`: `compact`, `balanced` (default), `spacious`.
    - [ ] `elevation`: `none` (bordered flat), `low` (default, med on hover/focus); shared focus ring only for keyboard editor focus.
    - [ ] Controlled `value` / `onChange` or internal uncontrolled draft; `onSubmit(value)`, optional `onStop`.
    - [ ] `isDisabled=false`, `isStopShown=false` defaults; send action versus stop while generating; default token-capable input and send/stop controls.
    - [ ] `status.type`: `error`, `warning`, optional message; `statusPosition`: `top`, `bottom` (default).
    - [ ] Slots: drawer, headerActions, headerContext, input, footerActions, sendActions, sendButton; default placeholder `Type a message...`.
    - [ ] Custom inputs/buttons join `useChatComposerContext()` value/change/submit/canSend/disabled/stop contract; register inputControlRef for body-click-to-focus.
  - **Templates**
    - [ ] **ChatComposer — Attachments** (`ChatComposerAttachments.doc.mjs`): removable file-token drawer.
    - [ ] **ChatComposer — Flat** (`ChatComposerFlat.doc.mjs`): elevation none and text-input treatment.
    - [ ] **ChatComposer — Footer Actions** (`ChatComposerFooterActions.doc.mjs`): model/settings menus and mic.
    - [ ] **ChatComposer — Full Featured** (`ChatComposerFullFeatured.doc.mjs`): all slots, attachment drawer, progress, menus, mic.
    - [ ] **ChatComposer** (`ChatComposerShowcase.doc.mjs`): named showcase; no further metadata description.
    - [ ] **ChatComposer — Simple** (`ChatComposerSimple.doc.mjs`): placeholder and submit.
    - [ ] **ChatComposer — Streaming** (`ChatComposerStreaming.doc.mjs`): stop generation.
    - [ ] **ChatComposer — Validation** (`ChatComposerValidation.doc.mjs`): error/warning above or below.

- [ ] **[Chat Layout](https://astryx.atmeta.com/components/ChatLayout)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Chat/ChatLayout.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ChatLayout).

  - **Layout choices and slots**
    - [ ] `density`: `compact`, `balanced` (default), `spacious`; dock padding, message width/inset, blur layer height.
    - [ ] Normal-flow children/message list, bottom-docked composer with frosted blur, vertically centered emptyState when no children.
    - [ ] scrollButton defaults to integrated ChatLayoutScrollButton; custom element overrides, null hides.
    - [ ] Internal root scroll context versus external scrollRef for parent/document scrolling; auto-scroll and scroll-to-bottom share target. One layout owner per scroll context.
    - [ ] Full-page container uses 100dvh so mobile toolbar does not obscure composer.
  - **Templates**
    - [ ] **ChatLayout — Panel View** (`ChatLayoutPanelChat.doc.mjs`): narrow sidebar/embed compact density.
    - [ ] **ChatLayout** (`ChatLayoutShowcase.doc.mjs`): named showcase; metadata has no further description.

- [ ] **[Chat Message](https://astryx.atmeta.com/components/ChatMessage)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Chat/ChatMessage.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ChatMessage).

  - **Sender, density, and slots**
    - [ ] `sender`: `user`, `assistant`, `system` determines alignment/layout.
    - [ ] `density`: `compact`, `balanced`, `spacious`; doc default balanced, inherits list context when unset.
    - [ ] Optional avatar/name/metadata with free-form children: bubbles, assets, tool calls, images; raw children flush to message edge, ghost bubble aligns custom content to text column.
    - [ ] Name on wrapper for first raw child, on first bubble otherwise; metadata on wrapper for last raw child, on last bubble otherwise.
    - [ ] Null/booleans/empty string omit slots; other non-rendering content collapses them. Missing name uses translated sender label.
  - **Templates**
    - [ ] **ChatMessage — Avatar & Name** (`ChatMessageAvatarName.doc.mjs`): slot ownership for bubbles/raw content.
    - [ ] **ChatMessage — Ghost** (`ChatMessageGhost.doc.mjs`): transparent bubble child, not a ChatMessage variant prop.
    - [ ] **ChatMessage — Multi-Bubble** (`ChatMessageMultiBubble.doc.mjs`): first/middle/last bubble grouping.
    - [ ] **ChatMessage** (`ChatMessageShowcase.doc.mjs`): grouped user/delivery, ghost assistant/avatar/name/time/model.

- [ ] **[Chat Message Metadata](https://astryx.atmeta.com/components/ChatMessageMetadata)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Chat/ChatMessageMetadata.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ChatMessageMetadata).

  - **Status and content**
    - [ ] Optional delivery `status`: `sending`, `sent`, `delivered`, `read`, `error`, shown as icon plus label; no default status documented.
    - [ ] Optional timestamp string/Timestamp and footer model/reaction/copy/retry controls; below message or last-bubble metadata slot.
    - [ ] Non-rendering scalars create no separator, zero remains; composite React content and accessible action names/behavior are caller-owned.
  - **Templates**
    - [ ] **ChatMessageMetadata — Footer Actions** (`ChatMessageMetadataFooter.doc.mjs`): copy/retry/thumbs/model label.
    - [ ] **ChatMessageMetadata** (`ChatMessageMetadataShowcase.doc.mjs`): error retry, delivery, full footer.
    - [ ] **ChatMessageMetadata — Status** (`ChatMessageMetadataStatus.doc.mjs`): all five statuses with time.
    - [ ] **ChatMessageMetadata — Timestamps** (`ChatMessageMetadataTimestamp.doc.mjs`): timestamp-only absolute/relative formats.

- [ ] **[Chat System Message](https://astryx.atmeta.com/components/ChatSystemMessage)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Chat/ChatSystemMessage.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ChatSystemMessage).

  - **Choices and semantics**
    - [ ] `variant`: `default` (default, centered wrapping notice), `divider` (labelled separator with side lines).
    - [ ] Text/ReactNode children; optional leading icon only rendered in default branch, not divider.
    - [ ] No live role of its own; enclosing ChatMessageList log announces new notices.
  - **Templates**
    - [ ] **ChatSystemMessage** (`ChatSystemMessageShowcase.doc.mjs`): realistic date/status/information flow.
    - [ ] **ChatSystemMessage — Status Updates** (`ChatSystemMessageStatusUpdates.doc.mjs`): membership/time/resolution notices.
    - [ ] **ChatSystemMessage — Variants** (`ChatSystemMessageVariants.doc.mjs`): default/divider.
    - [ ] **ChatSystemMessage — Icon** (`ChatSystemMessageWithIcon.doc.mjs`).

- [ ] **[Chat Tool Calls](https://astryx.atmeta.com/components/ChatToolCalls)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Chat/ChatToolCalls.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ChatToolCalls).

  - **States, grouping, and data slots**
    - [ ] Call `status`: `pending`, `running`, `complete` (default), `error`; explicitly set during pending/failure.
    - [ ] Single call inline versus multiple-call collapsible summary with latest visible at surface.
    - [ ] Controlled isExpanded/onExpandedChange or uncontrolled defaultIsExpanded=false; optional expanded summary label, default translated call count.
    - [ ] Call content: name, target, duration, node, additions/deletions/stats, errorMessage, resultDetail, key/data; stable key for streaming additions without remounting completed rows.
    - [ ] Result details may expand to diffs/command output; arbitrary call names/stats are data, not type/style enums.
  - **Templates**
    - [ ] **ChatToolCalls — Expandable** (`ChatToolCallsInteractiveToolCalls.doc.mjs`): click-to-reveal code/diff details.
    - [ ] **ChatToolCalls** (`ChatToolCallsShowcase.doc.mjs`): named showcase without further metadata description.
    - [ ] **ChatToolCalls — Statuses** (`ChatToolCallsStatuses.doc.mjs`): four states together.
    - [ ] **ChatToolCalls — Simple** (`ChatToolCallsToolCallsWithNodes.doc.mjs`): single inline plus grouped diff stats.

## Container

- [x] **[Card](https://astryx.atmeta.com/components/Card)** — static Card contract complete; related interactive templates below remain pending under their separate component owners.

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Card/Card.doc.mjs), [templates, including related interactive-card patterns](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Card).

  - **Choices and composition**
    - [x] `variant`: `default` (default), `transparent`, `muted`, `blue`, `cyan`, `gray`, `green`, `orange`, `pink`, `purple`, `red`, `teal`, `yellow`; colors categorize, not status. Validated theme extensions retain a transparent/borderless base, never another variant.
    - [x] `elevation`: `none` (default), `low`, `med`, `high`; closed shared Elevation, source shadows/bezel and ring composition.
    - [x] `padding`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`; omission uses theme card padding, spacing 4 without theme. Shared spacing types/Layout-owned lowering; FR3 corrects published inset to actual border-reduced padding, with asymmetric/zero/nested/RTL bleed and direct-reader corner evidence.
    - [x] Free-form children, simple heading/body or source-derived native Layout header/content/footer recipe; validated shared SizeValue supports open dimensions, CSS-wide keywords and native ref/events/semantics. No public Heading/Layout/Toast completion implied.
    - [x] Preferred width shrinks in flex/grid; caller minimum can win, ordinary overflow clips and explicit non-auto height scrolls. Text owns ID truncation/recovery; wide content owns its scroll region. **[Verification](docs/card-port.md):** 30 source observations/six static patterns, explicit corrected Layout geometry, core/Neutral paint and 16 browser suites. Source text audit retains 4/336 core secondary-text findings.
  - **Card-owned examples**
    - [x] Inline **Cards sharing a row on a phone, with long IDs**: three shrinking payout cards, one-line IDs/full tooltip.
    - [x] **Card — Callout** (`CardCallout.doc.mjs`): muted supplementary information.
    - [x] **Card — Elevations** (`CardElevations.doc.mjs`): four depths.
    - [x] **Card** (`CardShowcase.doc.mjs`): default heading/body.
    - [x] **Card — Variants** (`CardVariants.doc.mjs`): default/muted/category colors adapting to light/dark tokens.
    - [x] **Card — Layout** (`CardWithInnerLayout.doc.mjs`): header/body/footer dividers and actions.
    - [x] **Card — Simple** (`CardWithSimpleContent.doc.mjs`).

  - **Related interactive templates — verified separately from static Card completion**
    - [x] **ClickableCard — Elevated** (`ClickableCardElevated.doc.mjs`): med shadow on action surface.
    - [x] **ClickableCard** (`ClickableCardShowcase.doc.mjs`): navigation and independent nested controls.
    - [x] **ClickableCardWithNestedButton** (`ClickableCardWithNestedButton.doc.mjs`): product navigation plus Add to cart.
    - [x] **SelectableCard — Elevated** (`SelectableCardElevated.doc.mjs`): low shadow plus inset selection ring; observed desktop/mobile light/dark frames.
    - [x] **SelectableCardMulti** (`SelectableCardMulti.doc.mjs`): multi-select tags, matching source color-axis rings.
    - [x] **SelectableCard** (`SelectableCardShowcase.doc.mjs`): preserved caller single-choice plan recipe; native semantics remain checkboxes, not radios.

- [ ] **[Carousel](https://astryx.atmeta.com/components/Carousel)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Carousel/Carousel.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Carousel).

  - **Choices and controls**
    - [ ] Horizontal children; `gap`: `0`, `0.5`, `1` (default), `1.5`, `2`, `3`, `4`.
    - [ ] `padding`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`; matching scroll-padding aligns stops.
    - [ ] `hasButtons=true`, `hasEdgeFade=true`, `hasLoop=false` defaults; navigation/fades only relevant when overflowing. Loop wraps end/start and retains edge buttons.
    - [ ] `hasSnap=false` default; true uses native child-start CSS snap; `useSnapPaging()` value adds touch/trackpad paging, `pages` alternatives view and `item`; paging code loads only for hook value.
    - [ ] Handle: `scrollNext()`, `scrollPrev()`, `scrollTo(index)`, `canScrollNext()`, `canScrollPrev()`.
    - [ ] APG carousel region, label default Carousel, aria-roledescription carousel; each item named Slide N of M. Overflowing scroller is arrow-key-reachable Tab stop; fitting scroller is not.
    - [ ] Trackpad swipe and Shift+wheel; no automatic rotation; critical content must not depend on discovering horizontal scroll.
  - **Templates**
    - [ ] **Carousel — Cards** (`CarouselCards.doc.mjs`): snapping cards with automatic fade/buttons.
    - [ ] **Carousel** (`CarouselShowcase.doc.mjs`): card browsing.
    - [ ] **Carousel — Snap** (`CarouselSnap.doc.mjs`): team cards land at start.
    - [ ] **useSnapPaging — Onboarding Pages** (`CarouselSnapPaging.doc.mjs`): switch one view/one card per flick.

- [x] **[Clickable Card](https://astryx.atmeta.com/components/ClickableCard)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/ClickableCard/ClickableCard.doc.mjs), [associated Card templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Card). No own template directory supplied; related patterns live in Card metadata.

  - **Choices and activation**
    - [x] Card-owned thirteen variants and validated open custom names; custom base remains transparent/borderless, not a fallback palette.
    - [x] Shared four-value Elevation, eleven spacing steps and optional dimensions. Padding omission reads the Card theme (Neutral spacing 3; unthemed fallback spacing 4); no duplicate surface implementation.
    - [x] Surface action versus href navigation (href wins), optional open target, native/provider/new-tab/Cmd/Ctrl/middle sinks share destination acceptance; hidden button/link retain distinct callback ownership. Released focusable no-op retained without deciding OQ1.
    - [x] Disabled removes destination/click/mouseup, keeps named pointerdown/nonreserved native events, Tab stop/name/disabled AX state, half opacity and no hover/press paint. Provider disabled flips retain focus without scrolling.
    - [x] Required nonblank accessible label, independent nested controls/selected text/layers, original ordinary child-before-root event metadata and scoped owner currentTarget. Named owned callbacks reserve raw onclick/onmouseup/onpointerdown/onmounted; nonreserved builders survive removal/readd without starving retained callbacks. Headless adapter remains internal, not a separately completed public hook.
    - [x] Spec-led single own-action focus outline, owning-root disabled reflection, shared every-pointer press/nested veto/10px pan, theme exit clock and reduced fine-touch bound. **[Verification and limits](docs/clickable-card-port.md):** six original observations, nine SSR tests, native/browser proofs; preserved conditional contrast findings (58/1008 active text, 84/336 focus; disabled text advisory), no blanket AA/physical-screen-reader claim. Combined WASM Clippy/CI remain parent-integration checks.
  - **Associated named templates**
    - [x] **ClickableCard — Elevated** (`Card/ClickableCardElevated.doc.mjs`): elevation med.
    - [x] **ClickableCard** (`Card/ClickableCardShowcase.doc.mjs`): navigation with independent nested control.
    - [x] **ClickableCardWithNestedButton** (`Card/ClickableCardWithNestedButton.doc.mjs`): product link plus Add to cart action.

- [ ] **[Collapsible](https://astryx.atmeta.com/components/Collapsible)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Collapsible/Collapsible.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Collapsible).

  - **Disclosure modes and content**
    - [ ] Standalone controlled isOpen/onOpenChange or uncontrolled defaultIsOpen=true; group-bound `value` delegates state/notification to CollapsibleGroup and ignores standalone state props.
    - [ ] `isDisabled=false` default; disabled trigger dimmed/aria-disabled/out of Tab order, does not close already-open content.
    - [ ] `chevronPosition`: `start`, `end` (default or group inherited). End points down collapsed/up open; start points inward collapsed/down open, mirrored RTL.
    - [ ] Always-visible trigger and revealed children; optional animatedLayout handle glides following layout/content/chevron together; omitted means following layout moves immediately.
  - **Templates**
    - [ ] **Collapsible — Controlled** (`CollapsibleControlledAccordion.doc.mjs`): external state and Previous/Next navigation.
    - [ ] **Collapsible — FAQ** (`CollapsibleDividedAccordion.doc.mjs`): built-in dividers/density, body-semibold question typography.
    - [ ] **useCollapsible — Custom Disclosure** (`CollapsibleHookUsage.doc.mjs`): headless state.
    - [ ] **Collapsible — Multiple Mode** (`CollapsibleMultipleAccordion.doc.mjs`): multiple open, one Card per section.
    - [ ] **Collapsible** (`CollapsibleShowcase.doc.mjs`): three-section single accordion.
    - [ ] **Collapsible — Single Mode** (`CollapsibleSingleAccordion.doc.mjs`): one open, defaultValue, Section heading.
    - [ ] **Collapsible — With Dividers** (`CollapsibleWithoutCard.doc.mjs`): manually divided flat detail/sidebar rows, metadata under answers.

- [x] **[Selectable Card](https://astryx.atmeta.com/components/SelectableCard)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/SelectableCard/SelectableCard.doc.mjs), authoritative spec, and the three Card-directory templates listed above. [Verification](docs/selectable-card-port.md), including the approved forced-color cue and source-palette findings; standalone Lab group/Layer migrations remain independent.

  - **Surfaces and selection**
    - [x] variant `default` (default), `transparent`, `muted`, `blue`, `cyan`, `gray`, `green`, `orange`, `pink`, `purple`, `red`, `teal`, `yellow`; source border-role tokens and custom ring lever.
    - [x] elevation `none` (default), `low`, `med`, `high`; selected 2px inset ring composes with resting shadow without layout jitter.
    - [x] Required controlled isSelected true/false and onChange; focusable native isDisabled false (default) / true; actual checked-property repair.
    - [x] Card-owned padding `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4` (default), `5`, `6`, `8`, `10`; validated width/height/maxWidth and rich children delegate without duplicating Card geometry.

  - **Actual interaction and recipes**
    - [x] Required accessible label; native checkbox semantics, one Tab stop per card; single Space/Enter request and input-only delegated focus indicator.
    - [x] Multi-select choices/add-ons/option grids with caller Set of IDs; source single-choice recipes explicitly retain checkbox semantics; not navigation (ClickableCard owns navigation).
    - [x] Shared logical layer capture, reentrant owner-document routing, retained dynamic native event slots and all-pointer theme release clocks/10px pan. All four final isolated browser suites passed; parent joint cross-card harness passed before the final cue/source follow-up.
    - [x] Private native-radio channel: approved actual-browser proof of 9 radios/four groups, ordering/select-only/controlled peer repair/form ownership/disabled/AX; temporary exports/example removed. No public radio/context or Lab group API.
    - [x] Explicitly approved forced-colors-only stationary 2px CanvasText selection border: parent dedicated browser proof passed default/borderless/custom/disabled, independent focus, state-toggle geometry, scrolling and unchanged normal-mode paint. Exact CSS mirrored; all four isolated suites rerun successfully. Normal source ring audit remains 218/468 conditional essential-selection findings below 3:1; no passing-contrast claim.

## Content

- [ ] **[Avatar](https://astryx.atmeta.com/components/Avatar)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Avatar/Avatar.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Avatar).

  - **Choices and modes**
    - [ ] `size`: `xsm` (20px), `sm` (24px), `md` (36px, default), `lg` (48px), `xl` (128px); numeric alternatives `16`, `20`, `24`, `32`, `36`, `40`, `48`, `60`, `64`, `72`, `96`, `128`, `144`, `180`. Group size overrides child size.
    - [ ] `shape`: `circle` (default), `rounded`, `square`; status follows circle's 4-o'clock edge or rectangular corner.
    - [ ] Fallback chain: primary `src`, `fallbackSrc`, initials from name, default person icon. Initials use first/last word letters/digits/emoji, skipping punctuation; no valid initials gives icon.
    - [ ] `tooltip`: omitted/`true` (default) shows name on hover/focus, string overrides, `false` suppresses; absent name means no automatic tooltip. Disable explicitly under consumer Tooltip/HoverCard.
    - [ ] Static image, `href` link/custom `as` link (plus target/rel), or `onClick` button without href.
    - [ ] `name` and optional `alt` override name the avatar; interactive avatar without either warns. Status slot accepts AvatarStatusDot and composes its label into accessible name, even through wrappers.
    - [ ] Interactive avatars in group use one Tab stop and arrow navigation; initials are part of picture, not separately announced letters.
  - **Templates**
    - [ ] **Avatar — Fallback Chain** (`AvatarFallbackChain.doc.mjs`).
    - [ ] **Avatar — Group** (`AvatarGroup.doc.mjs`): overlapping participant facepile.
    - [ ] **Avatar — Initials** (`AvatarInitialsFallback.doc.mjs`).
    - [ ] **Avatar — Interactive** (`AvatarInteractive.doc.mjs`): link and button, focus-visible ring.
    - [ ] **Avatar** (`AvatarShowcase.doc.mjs`): sizes, photo, initials, status.
    - [ ] **Avatar — Tooltip** (`AvatarTooltip.doc.mjs`): automatic, custom, disabled tooltip.
    - [ ] **Avatar — User Card** (`AvatarUserCard.doc.mjs`): avatar, name, role row.
    - [ ] **Avatar — Photo** (`AvatarWithImage.doc.mjs`): photo sizes and failure fallback.
    - [ ] **Avatar — Status Dot** (`AvatarWithStatus.doc.mjs`): availability.

- [ ] **[Avatar Group](https://astryx.atmeta.com/components/AvatarGroup)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/AvatarGroup/AvatarGroup.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/AvatarGroup).

  - **Choices and composition**
    - [ ] Group `size`: `xsm` (20px), `sm` (24px), `md` (36px, default), `lg` (48px), `xl` (128px), or `16`, `20`, `24`, `32`, `36`, `40`, `48`, `60`, `64`, `72`, `96`, `128`, `144`, `180`; always overrides child size, even when omitted.
    - [ ] Group `shape`: `circle` (default), `rounded`, `square`; overrides avatars and overflow.
    - [ ] Avatar children with their own status, callbacks, tooltips, optional final AvatarGroupOverflow; caller slices visible avatars and computes hidden count, group never slices.
    - [ ] Wrapped Tooltip/HoverCard avatars still overlap; disable their built-in tooltip explicitly.
    - [ ] Interactive group: roving tabindex, single Tab stop, arrow navigation through avatars and overflow button, announced keyboard hint.
  - **Templates**
    - [ ] **AvatarGroup — Interactive** (`AvatarGroupInteractive.doc.mjs`): profile links, overflow opens full member list.
    - [ ] **AvatarGroup** (`AvatarGroupShowcase.doc.mjs`): sliced facepile and server-side overflow count.

- [ ] **[Blockquote](https://astryx.atmeta.com/components/Blockquote)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Blockquote/Blockquote.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Blockquote).

  - **Content and behavior**
    - [ ] No finite visual variant/size enum documented; quote children with or without `cite` attribution rendered in a cite element.
    - [ ] Secondary text and logical inline-start rule/padding, mirrored to right in RTL.
  - **Templates**
    - [ ] **Blockquote — Showcase** (`BlockquoteShowcase.doc.mjs`): with/without citation.
    - [ ] **Blockquote — Testimonials** (`BlockquoteTestimonials.doc.mjs`): quotes in Card/Grid social-proof layout.
    - [ ] **Blockquote — With Attribution** (`BlockquoteWithCite.doc.mjs`): plain quote and credited quote.

- [ ] **[Citation](https://astryx.atmeta.com/components/Citation)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Citation/Citation.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Citation).

  - **Choices and source content**
    - [ ] `variant`: `label` (default, title chip), `number` (compact reference badge).
    - [ ] CitationSource title/url, optional image src/icon node; display number is data, not a size/style.
    - [ ] Linked versus visible non-navigating reference when destination is rejected by shared Link policy; image resources have separate handling. Templates also cover without icons/links.
  - **Templates**
    - [ ] **Citation — Inline Text** (`CitationInlineText.doc.mjs`): paragraph flow.
    - [ ] **Citation — Showcase** (`CitationShowcase.doc.mjs`): label/number, with/without icons/links.
    - [ ] **Citation — Source List** (`CitationSourceList.doc.mjs`): attributed sources footer/list with icons.

- [ ] **[Code](https://astryx.atmeta.com/components/Code)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Code/Code.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Code).

  - **Choices and semantics**
    - [ ] `color`: `primary` (default), `secondary`, `inherit`.
    - [ ] `size`: omitted code type scale versus explicit `inherit` for surrounding font size/line height; no sm/md/lg enum.
    - [ ] Semantic inline code children/ref; no interactive copy/navigation behavior by itself. Use appropriate Button/Link for actions and CodeBlock for multi-line content.
  - **Templates**
    - [ ] **Code — Text Sizes** (`CodeAcrossTextSizes.doc.mjs`): heading/body/supporting/label contexts; metadata describes matching parent sizing.
    - [ ] **Code — Inline** (`CodeInlineInParagraph.doc.mjs`): prose/API terms.
    - [ ] **Code** (`CodeShowcase.doc.mjs`): sentence snippets.
    - [ ] **Code — Content Types** (`CodeVariousContent.doc.mjs`): variables/commands/CSS/paths/shortcuts (content examples, not style enum).

- [ ] **[Code Block](https://astryx.atmeta.com/components/CodeBlock)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/CodeBlock/CodeBlock.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/CodeBlock).

  - **Display and language choices**
    - [ ] `size`: `sm`, `md` (default); `container`: `card` (default, muted/border/radius), `section` (transparent/no border/radius).
    - [ ] `highlightMode`: `auto` (default), `ranges`, `spans`.
    - [ ] Built-in language identifiers: `plaintext` (default/no highlighting), `ts`, `js`, `tsx`, `jsx`, `json`, `html`, `xml`, `svg`, `css`, `scss`, `less`, `python`, `bash`, `sh`, `zsh`, `shell`, `php`, `hack`, `yaml`, `markdown`, `c`, `cpp`, `c++`, `cc`, `cxx`, `hpp`, `hh`, `sql`, `rust`, `kotlin`, `swift`, `go`.
    - [ ] Separately imported language values: `graphql`, `java`, `thrift`, `objectiveC` (objective-c path), `toml`, `powershell`, `ruby`, `lua`, `dockerfile`; custom createCodeLanguage name/identifiers/tokenize supported, so language vocabulary is extensible, not a closed UI style set.
    - [ ] Per-instance syntaxTheme, nearest SyntaxTheme, or theme syntax colors; deprecated tokenizer callback still wins over language, upgrade rewrites static uses.
  - **Boolean modes, content, and async behavior**
    - [ ] `hasLanguageLabel=true` (hidden for plaintext), `hasLineNumbers=false`, `hasCopyButton=true`, `isWrapped=false`, `isDiff=false`, `isCollapsible=false` defaults.
    - [ ] Diff adds/removes/hunk/context and file headers over underlying language highlighting; label diff/<language> diff; copy exact source; Markdown diff/diff-lang fences select this mode.
    - [ ] Code string and optional title/header; startLineNumber default 1/clamped to 1; highlightLines uses same file coordinates, ignores out-of-range values.
    - [ ] Fit-content default width or arbitrary CSS/full width; maxHeight scrolls vertically; unwrapped long lines scroll horizontally.
    - [ ] endContent controls immediately before Copy in header or own inline-end code column; never overlays code or creates header/collapse eligibility.
    - [ ] Collapsible starts expanded, needs visible header and enough lines (threshold default 10); headerless never collapses, removing header expands collapsed content.
    - [ ] Copy success onCopy; rejection onCopyError once, cross plus accessible announcement Failed to copy value to clipboard, error detail not shown.
  - **Templates**
    - [ ] **Code — Snippet** (`CodeBlockBashCommand.doc.mjs`): headerless one-line terminal command/copy column.
    - [ ] **Code — Highlighted** (`CodeBlockHighlightedLines.doc.mjs`): TypeScript focus lines.
    - [ ] **Code — Config** (`CodeBlockJSONConfig.doc.mjs`): JSON filename header and gutter.
    - [ ] **Code — Scrollable** (`CodeBlockScrollableBlock.doc.mjs`): max-height long file.
    - [ ] **CodeBlock** (`CodeBlockShowcase.doc.mjs`): TypeScript title/gutter/copy.
    - [ ] **Code — Terminal** (`CodeBlockTerminal.doc.mjs`): bash in GitHub Dark SyntaxTheme, no gutter, copy enabled.

- [ ] **[Empty State](https://astryx.atmeta.com/components/EmptyState)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/EmptyState/EmptyState.doc.mjs). Templates in `EmptyState/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Layout and content**
    - [ ] `isCompact`: false (default) / true, reducing spacing and stacking actions vertically instead of horizontal default.
    - [ ] `headingLevel`: `1`, `2`, `3` (default), `4`, `5`, `6`; changes h1–h6 semantics, not title's fixed visual size.
    - [ ] Required title, optional description, decorative `icon`/illustration (`aria-hidden`), and action buttons below description.
    - [ ] Patterns: empty list, zero search results, first-time setup, cleared inbox; provide a next step, not an error-alert substitute.

  - **Named templates**
    - [ ] `EmptyStateActions` — **EmptyState — Actions**: icon/message with go-back, clear-filter, or retry-query actions.
    - [ ] `EmptyStateCompact` — **EmptyState — Compact**: sidebar/card/drawer constrained empty state.
    - [ ] `EmptyStateContainer` — **EmptyState — Container**: Card-wrapped first-time/onboarding state.
    - [ ] `EmptyStateShowcase` — **EmptyState**: no-results icon, message, CTA.

- [ ] **[Heading](https://astryx.atmeta.com/components/Heading)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Text/Heading.doc.mjs). Templates in `Heading/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Typography and semantics**
    - [ ] Required `level`: `1`, `2`, `3`, `4`, `5`, `6`, selecting h1–h6 and default themed visual style.
    - [ ] `type`: `display-1`, `display-2`, `display-3` visual overrides; theme extensions are open-ended and do not change semantic tag.
    - [ ] `accessibilityLevel`: `1`, `2`, `3`, `4`, `5`, `6`; differing value sets aria-level.
    - [ ] `weight`: `normal`, `medium`, `semibold`, `bold`; overrides type/level weight.
    - [ ] `color`: `primary` (default), `secondary`, `disabled`, `placeholder`, `accent`, `inherit`; custom theme colors remain open-ended.
    - [ ] `display`: `inline`, `block` (default); line clamp or Capsize forces block.
    - [ ] `justify`: `start` (default), `center`, `end`; logical RTL alignment.
    - [ ] `textWrap`: `wrap`, `nowrap`, `balance`, `pretty`; `wordBreak`: `break-word`, `break-all` (default for single-line truncation; break-word otherwise).
    - [ ] `hasCapsize` and `hasStrikethrough`: each false by default / true; optical trim and strike decoration respectively.

  - **Truncation and content**
    - [ ] `maxLines`: 0 default (unclamped), positive line limit; `hasTruncateTooltip`: true (default), false, `above`, `below`, `start`, `end`.
    - [ ] Required children; long tokens wrap even unclamped; flex child needs min-width:0 to shrink; inherited overflow-wrap requires Astryx's supported propagation.

  - **Named templates**
    - [ ] `HeadingCardGrid` — **Heading — Card Grid**: responsive cards with clamped headings/descriptions.
    - [ ] `HeadingPageLayout` — **Heading — Page Hierarchy**: h1–h3 with supporting text.
    - [ ] `HeadingShowcase` — **Heading**: showcase metadata, no additional described recipe.
    - [ ] `HeadingTruncation` — **Heading — Truncation**: single-/multi-line ellipsis.

- [ ] **[Icon](https://astryx.atmeta.com/components/Icon)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Icon/Icon.doc.mjs). Templates in `Icon/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Sizes, colors, artwork, and names**
    - [ ] `size`: `xsm` (12px), `sm` (16px), `md` (20px), `lg` (24px), at 16px root and scaling with root font. Explicit value wins owner context; otherwise contextual default, then md.
    - [ ] `color`: `primary`, `secondary`, `tertiary`, `disabled`, `accent`, `info`, `success`, `error`, `warning`, `inherit` (default), `blue`, `red`, `green`, `gray`, `cyan`, `teal`, `yellow`, `orange`, `pink`, `purple`.
    - [ ] `icon` accepts semantic name, registered namespaced key, or SVG component; unknown unregistered namespaced key renders nothing.
    - [ ] Documented semantic names: `close`, `chevronDown`, `chevronLeft`, `chevronRight`, `chevronsLeft`, `chevronsRight`, `check`, `success`, `error`, `warning`, `info`, `calendar`, `clock`, `externalLink`, `menu`, `moreHorizontal`, `search`, `upload`, `arrowUp`, `arrowDown`, `arrowsUpDown`, `funnel`, `eyeSlash`, `viewColumns`, `copy`, `checkDouble`, `wrench`, `stop`, `microphone`.
    - [ ] `variant` has **no built-in finite artwork names**: app-augmented IconVariantMap and theme iconVariants opt in; ignored for directly supplied component, not emitted to DOM.

  - **Accessibility and resolution**
    - [ ] Omitted/empty `label`: decorative aria-hidden; meaningful label: role img plus aria-label, unhidden. Interactive parent already names its own icon.
    - [ ] Theme-scoped overrides versus bootstrap process-wide registration; synchronous pure `{size, variant}` icon functions can return null; non-render reads use md.
    - [ ] Ref reaches actual SVG (or null for glyph with no SVG); own size/color styles compose with xstyle/class/style.

  - **Named templates**
    - [ ] `IconNonSemanticColors` — **Icon — Non-Semantic Colors**: palette hues.
    - [ ] `IconSemanticColors` — **Icon — Semantic Colors**: semantic role colors.
    - [ ] `IconShowcase` — **Icon**: glyph showcase.
    - [ ] `IconSizes` — **Icon — Size Variants**: xsm/sm/md/lg.
    - [ ] `IconStatusIcons` — **Icon — Status Indicators**: success/warning/error/info rows.
    - [ ] `useIconHookUsage` — **useIcon — Status Row**: current-theme versus overridden success/warning/error glyphs.

- [x] **[Kbd](https://astryx.atmeta.com/components/Kbd)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Kbd/Kbd.doc.mjs). Templates in `Kbd/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Key notation and platform modes**
    - [x] No finite size/color/style variants. Required `keys` string splits combinations with `+`; literal plus uses `plus`. Entirely empty shortcuts are rejected as a recoverable Rust boundary error; meaningful combinations retain all internal segments.
    - [x] Special names: `mod`, `ctrl`, `alt`, `shift`, `meta`, `enter`, `backspace`, `escape`, `tab`, `up`, `down`, `left`, `right`, `plus`; aliases `esc` and `return`.
    - [x] Apple versus other-platform symbols/text: mod becomes Command/Ctrl; meta Command/Win (Win before hydration); modifiers use Apple symbols versus text. Source `meta` spoken name remains META. Shared preferred Client Hints detector, no server platform reads, first-client non-Apple output and scoped post-render detection verified.
    - [x] Screen readers hear key words; component owns aria-label, caller aria-labelledby may associate visible text. Only displays shortcut, never handles key activation. Application-owned localized KbdText and pairwise formatting preserve spoken/visible names without claiming a completed locale provider. **[Verification](docs/kbd-port.md): 104 original group comparisons/184 keycap observations across both platforms/modes; 0/12 modeled text-contrast failures.**

  - **Named templates**
    - [x] `KbdInlineInstructions` — **Kbd — Inline Instructions**: keycaps in instruction prose.
    - [x] `KbdMenuShortcuts` — **Kbd — Menu Shortcuts**: action/shortcut pairs.
    - [x] `KbdModifierCombos` — **Kbd — Modifier Combinations**: modifiers and special keys.
    - [x] `KbdShowcase` — **Kbd**: shortcut-badge showcase.

- [ ] **[Markdown](https://astryx.atmeta.com/components/Markdown)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Markdown/Markdown.doc.mjs). Templates in `Markdown/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Rendering modes**
    - [ ] `display`: `block` (default), `inline`; `density`: `default` (default), `compact`.
    - [ ] `headingLevelStart`: `1` (default), `2`, `3`, `4`, `5`, `6`; shifted levels clamp at h6.
    - [ ] `isStreaming`: false (default) / true incremental parsing and chunk fade-in.
    - [ ] `citationStyle`: `label` (default title/icon/border chip), `number` (compact badge); `sources` resolves matching `[id]` / `【id】` markers.
    - [ ] `contentAlign`: `start` (default), `center`; contentWidth default 680, arbitrary prose cap; tables/code can use full container.
    - [ ] `autolink`: unset (unchanged literal behavior) or `gfm`; bare URLs, www, emails, bracket links, punctuation balancing; skips code/existing links/image alt.

  - **Syntax, extensions, and safety**
    - [ ] Required Markdown source string renders headings, paragraphs, lists/tasks, tables, blockquotes, rules, code/inline code, links/images/citations; CommonMark emphasis/strong nesting, depth >100 stays literal.
    - [ ] Components override `code`, `inlineCode`, `math`, `link`, `heading`, `paragraph`, `image`, `blockquote`, `hr`, `citation`; math override opts into inline `$…$` and block `$$…$$`, otherwise dollars literal.
    - [ ] Ordered stable createMarkdownPlugin entries: bounded syntax, immutable typed AST transforms, typed extension renderers; pure callbacks, child component for hooks; omitted/empty preserves baseline.
    - [ ] inlinePlugins transform prose regex matches, not inline/fenced code or math; protected prose helpers preserve links/images/citations/extension contexts.
    - [ ] codeLanguages imported/custom descriptors; matching fence beats built-in, later entry wins, diff-identifier adds diff layer; keep same languages in read/edit surfaces.
    - [ ] onLinkClick may return false to cancel; custom link renderer replaces handler. Shared URL rule plus **all data: links/images rejected**, rendered plain and never passed to handler/custom link renderer.
    - [ ] Plugin renderer failures/declines/suspension fall back for claimed fences to components.code or accessible copyable CodeBlock; custom output/accessibility remain renderer-owned, no raw HTML execution by math parsing.
    - [ ] First-party soft-break plugin; source-line plugin stamps block inclusive source lines (not inline/table rows/cells); decorations add nonvisual settled-document metadata without changing rendering/copy/name/ID/focus/navigation.
    - [ ] Frontmatter parser returns typed metadata, removes complete leading block, withholds unfinished streaming block; Remark adapter only synchronous transform-compatible subset, reports unsupported work and preserves last valid tree.
    - [ ] Server parser/plugin subpaths are client-free; function-bearing plugins cannot cross RSC serialization boundary into client Markdown. Character-reference helper decodes named/decimal/hex references with semicolons; callers protect code/escapes.
    - [ ] Heading-links plugin adds copy-only trailing # button, hover/focus/coarse-pointer reveal, no navigation/hash/scroll, check confirmation 1.5s; preserve incoming fragment IDs and shared depth-first Outline identity; share plugin/prefix across document surfaces.
    - [ ] Direct math parsing uses typed MathParseOptions/unions and incremental state<true>; default parser calls preserve legacy unions.

  - **Named inline examples**
    - [ ] **Inline display**: Markdown embedded in Text.
    - [ ] **GFM autolinks**: literal URL/email recognition.
    - [ ] **Linkable headings**: shared heading-links plugin and Outline namespace.
    - [ ] **First-party soft breaks**: meaningful single line endings.
    - [ ] **Source-line stamps**: rendered block source-line attributes.
    - [ ] **Text transform helper**: protected prose replacement.
    - [ ] **Semantic fence helper**: custom diagram extension renderer/toText.
    - [ ] **Source decoration helper**: annotate UTF-16 ranges, read later without visual changes.
    - [ ] **Native frontmatter**: parse typed metadata, remove frontmatter from rendering.
    - [ ] **Compatible Remark plugin**: synchronous text-rename transform adapter.
    - [ ] **Entity links**: inlinePlugins match identifiers only in prose.
    - [ ] **Math renderer**: caller-owned inline/block typesetting and accessible output.

  - **Named templates**
    - [ ] `MarkdownCitedContent` — **Markdown — Cited Content**: citation chips/external sources.
    - [ ] `MarkdownCompactAIResponse` — **Markdown — Compact AI Response**: compact density, shifted headings.
    - [ ] `MarkdownDataTable` — **Markdown — Data Table**: comparison table.
    - [ ] `MarkdownRichContent` — **Markdown — Rich Content**: headings/lists/code/tables/quotes/tasks.
    - [ ] `MarkdownShowcase` — **Markdown**: rich formatted prose.

- [ ] **[Text](https://astryx.atmeta.com/components/Text)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Text/Text.doc.mjs).

  - **Typography and finite choices**
    - [ ] `type`: `body` (default), `large`, `label`, `supporting`, `code`, `display-1`, `display-2`, `display-3`, `inherit`; themes may add types, so not a sealed global list.
    - [ ] `size`: `4xs`, `3xs`, `2xs`, `xsm`, `sm`, `base`, `lg`, `xl`, `2xl`, `3xl`, `4xl`; overrides type size, not its other typography.
    - [ ] `color`: `primary`, `secondary`, `disabled`, `placeholder`, `accent`, `inherit`; default secondary for supporting, primary otherwise; theme extension permitted.
    - [ ] `weight`: `normal`, `medium`, `semibold`, `bold`; `display`: `inline` (default), `block`; `as`: `span` (default), `p`, `div`, `label`.
    - [ ] `justify`: `start` (default), `center`, `end`; `textWrap`: `wrap`, `nowrap`, `balance`, `pretty`; `wordBreak`: `break-word`, `break-all` (single-line truncation default; otherwise break-word).
    - [ ] `hasCapsize`, `hasStrikethrough`, `hasTabularNumbers`: false (default), true. Capsize forces block display.

  - **Truncation and content**
    - [ ] `maxLines`: 0 (default, no truncation), one line, multiple lines; positive maxLines forces block. Tooltip only when measured truncated.
    - [ ] `hasTruncateTooltip`: true (default), false, `above`, `below`, `start`, `end`.
    - [ ] Inline mixed body/code text; long-token wrapping; flex child min-width:0 for shrinking. Use Heading, not Text, for document heading semantics.

  - **Named templates and discrepancy**
    - [ ] [`TextShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Text/TextShowcase.doc.mjs) — “Text”.
    - [ ] [`TextColors`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Text/TextColors.doc.mjs) — “Text — Colors”: metadata says primary/secondary/disabled/placeholder/active; API says accent, not active. Preserve the example name without inventing an active API color.
    - [ ] [`TextHeadingLevels`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Text/TextHeadingLevels.doc.mjs) — “Text — Heading Levels”: Heading h1–h6 family composition.
    - [ ] [`TextInline`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Text/TextInline.doc.mjs) — “Text — Inline”: mixed body/code.
    - [ ] [`TextTruncation`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Text/TextTruncation.doc.mjs) — “Text — Truncation”: single/multiline constrained content.
    - [ ] [`TextTypes`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Text/TextTypes.doc.mjs) — “Text — Types”: body/large/label/supporting/code.
    - [ ] [`TextWeight`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Text/TextWeight.doc.mjs) — “Text — Weight”: four weights.
    - [ ] [`TextWordBreak`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Text/TextWordBreak.doc.mjs) — “Text — Word Break”: break-word versus break-all.
    - [ ] [`TextWrap`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Text/TextWrap.doc.mjs) — “Text — Wrap”: four wrap modes.
    - [ ] [`useTruncationHookUsage`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Text/useTruncationHookUsage.doc.mjs) — “useTruncation — Tooltip on Cut-Off Text”: short/long table paths and two-line clamp.

- [ ] **[Thumbnail](https://astryx.atmeta.com/components/Thumbnail)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Thumbnail/Thumbnail.doc.mjs).

  - **States and content patterns**
    - [ ] `isLoading`/`isDisabled`: false (default), true; `showRemoveOn`: `always`, `hover` (default), only meaningful with onRemove.
    - [ ] Lifecycle: src image (square cover), no-src placeholder icon, loading without src skeleton, loading with src upload scrim/spinner.
    - [ ] Optional onClick adds button semantics/hover overlay; optional onRemove adds top-right close control. Hover-mode removal also appears on keyboard focus and remains visible with coarse primary pointer.
    - [ ] label names preview/remove affordance and supplies tooltip; alt describes image content. Omitted alt makes image explicitly decorative/aria-hidden; neither alt nor label triggers dev warning.
    - [ ] Disabled open/remove controls stay focusable, announced unavailable, and invoke nothing.

  - **Named templates and discrepancy**
    - [ ] [`ThumbnailShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Thumbnail/ThumbnailShowcase.doc.mjs) — “Thumbnail”: image and label.
    - [ ] [`ThumbnailDisabled`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Thumbnail/ThumbnailDisabled.doc.mjs) — “Thumbnail — Disabled”.
    - [ ] [`ThumbnailGallery`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Thumbnail/ThumbnailGallery.doc.mjs) — “Thumbnail — Gallery”: clickable labeled attachments.
    - [ ] [`ThumbnailRemovable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Thumbnail/ThumbnailRemovable.doc.mjs) — “Thumbnail — Removable”: removable previews. Its metadata mentions APCA luminance detection; current English anatomy instead specifies fixed overlay scrim/on-dark icon. Do not infer an APCA prop/mode.
    - [ ] [`ThumbnailStates`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Thumbnail/ThumbnailStates.doc.mjs) — “Thumbnail — States”: image, placeholder, skeleton, upload overlay.

- [ ] **[Timer](https://astryx.atmeta.com/components/Timer)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Timer/Timer.doc.mjs).

  - **Format, typography, and behavior**
    - [ ] `format`: `elapsed` (default; compact units, drops seconds after hour), `clock` (m:ss/h:mm:ss, always second-precise).
    - [ ] `startTime`: supplied Unix milliseconds versus omitted (mount time); active duration, not calendar date or timezone mode.
    - [ ] `type`: `body`, `large`, `label`, `supporting` (default), `code`, `display-1`, `display-2`, `display-3`, `inherit`.
    - [ ] `size`: `4xs`, `3xs`, `2xs`, `xsm`, `sm`, `base`, `lg`, `xl`, `2xl`, `3xl`, `4xl`; `color`: `primary`, `secondary` (default), `disabled`, `placeholder`, `accent`, `inherit`; weight `normal`, `medium`, `semibold`, `bold`.
    - [ ] Semantic time output without a React render per tick; elapsed updates per second below hour/per minute afterward. Avoid aria-live per tick unless task warrants it.

  - **Every named inline example and template**
    - [ ] “Elapsed duration”: default Timer.
    - [ ] “Stopwatch clock”: clock format.
    - [ ] “Operation that started before mount”: explicit startTime.
    - [ ] “Match surrounding text”: inherit type/color inline.
    - [ ] “Prominent elapsed time”: body/lg/primary/semibold.
    - [ ] [`TimerShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Timer/TimerShowcase.doc.mjs) — “Timer”.
    - [ ] [`TimerFormats`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Timer/TimerFormats.doc.mjs) — “Timer — Formats”: same start, elapsed versus clock.
    - [ ] [`TimerInline`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Timer/TimerInline.doc.mjs) — “Timer — Inline”: inherited status copy.
    - [ ] [`TimerTypography`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Timer/TimerTypography.doc.mjs) — “Timer — Typography”: default versus emphasis.

- [ ] **[Timestamp](https://astryx.atmeta.com/components/Timestamp)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Timestamp/Timestamp.doc.mjs).

  - **Formats and typography**
    - [ ] `format`: `relative`, `relative_short`, `auto` (default), `date`, `date_long`, `date_weekday`, `date_time`, `time`, `system_date`, `system_date_time`, `system_time`, `unix_seconds`.
    - [ ] `type`: `body`, `large`, `label`, `supporting` (default), `code`, `display-1`, `display-2`, `display-3`, `inherit`.
    - [ ] `size`: `4xs`, `3xs`, `2xs`, `xsm`, `sm`, `base`, `lg`, `xl`, `2xl`, `3xl`, `4xl`; color `primary`, `secondary` (default), `disabled`, `placeholder`, `accent`, `inherit`; weight `normal`, `medium`, `semibold`, `bold`.
    - [ ] Value accepts Unix seconds or ISO string; autoThreshold default 604800 seconds controls relative/date_time switch.

  - **Zones, tooltip modes, and accessibility**
    - [ ] `hasTooltip`: true (default), false; `isTimezoneShown`, `isLive`: false (default), true. Zone abbreviation applies only date_time/time, not machine-readable system formats.
    - [ ] Explicit IANA timezoneID versus omitted/`local` viewer zone; explicit zone stabilizes SSR absolute output. Unknown zone warns once/degrades to viewer zone; no fixed list of arbitrary zone strings.
    - [ ] Relative/relative_short/auto read now only on client; SSR/prerender displays labeled UTC full date/time until hydration. Absolute formats do not read current time; unix_seconds zone independent.
    - [ ] Provider calendar affects date-bearing formats/full accessible date/hover card, not time-only, relative, system_* or unix_seconds.
    - [ ] Default relative hover/focus card: single copyable full absolute row. Configured tooltipEntries attach card also to absolute formats; each optional label/zone/format row can be read-only (`isCopyable` false default) or copyable, with shared trailing action column.
    - [ ] Tooltip format choices: `full` plus `date`, `date_long`, `date_weekday`, `date_time`, `time`, `system_date`, `system_date_time`, `system_time`, `unix_seconds`; omitted format uses full absolute. Omitted entry zone inherits timestamp, explicit local means viewer. Label multiple-zone rows; copies use each formatted row value.

  - **Named templates and discrepancy**
    - [ ] [`TimestampShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Timestamp/TimestampShowcase.doc.mjs) — “Timestamp”.
    - [ ] [`TimestampAutoFormat`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Timestamp/TimestampAutoFormat.doc.mjs) — “Timestamp — Auto”.
    - [ ] [`TimestampColors`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Timestamp/TimestampColors.doc.mjs) — “Timestamp — Colors”: metadata names primary/secondary/disabled/active; API names accent, not active. No active API color inferred.
    - [ ] [`TimestampFormats`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Timestamp/TimestampFormats.doc.mjs) — “Timestamp — Formats”: absolute/system representations.
    - [ ] [`TimestampRelativeFormat`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Timestamp/TimestampRelativeFormat.doc.mjs) — “Timestamp — Relative”: relative recency with full-date hover.
    - [ ] [`TimestampTimezone`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Timestamp/TimestampTimezone.doc.mjs) — “Timestamp — Timezone”: appended zone.
    - [ ] [`TimestampTooltipTimezones`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Timestamp/TimestampTooltipTimezones.doc.mjs) — “Timestamp — Tooltip time zones”: multi-zone/multi-format rows.

- [ ] **[Token](https://astryx.atmeta.com/components/Token)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Token/Token.doc.mjs).

  - **Choices and content modes**
    - [ ] `size`: `sm`, `md` (default), `lg`; `color`: `default` (default), `red`, `orange`, `yellow`, `green`, `teal`, `cyan`, `blue`, `purple`, `pink`, `gray`.
    - [ ] `isDisabled`, `isLabelHidden`: false (default), true; disabled dims/blocks interaction, hidden label remains accessible.
    - [ ] Static metadata; clickable span with invisible accessible button via onClick; href anchor via shared Link navigation rule; removable onRemove X button.
    - [ ] Label, optional leading icon, trailing endContent before remove, accessible description via aria-description; label ellipsis, bidi character order preserved for mixed identifiers.

  - **Named templates**
    - [ ] [`TokenShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Token/TokenShowcase.doc.mjs) — “Token”.
    - [ ] [`TokenClickable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Token/TokenClickable.doc.mjs) — “Token — Clickable”: filters/details.
    - [ ] [`TokenColors`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Token/TokenColors.doc.mjs) — “Token — Colors”: eleven colors, normal/disabled.
    - [ ] [`TokenEndContent`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Token/TokenEndContent.doc.mjs) — “Token — End Content”: count/status.
    - [ ] [`TokenIcon`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Token/TokenIcon.doc.mjs) — “Token — Icon”: entity/category glyphs.
    - [ ] [`TokenRemovable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Token/TokenRemovable.doc.mjs) — “Token — Removable”: undoable selections.

- [ ] **[Rich Text View](https://astryx.atmeta.com/components/RichTextView)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/richtext/src/RichTextView.doc.mjs). No named inline examples or templates.

  - **Read-only content and failure paths**
    - [ ] No finite visual variants or editing modes; required serialized Lexical JSON `value`, view updates when it changes.
    - [ ] Same default nodes/tables/code/rules as editor; custom nodes must match authoring schema; markdownExtensions render plugin nodes, omitted plugin shows source; shared codeLanguages aligns read/edit styling.
    - [ ] Optional read-only composer plugins for hover cards/decorators; namespace default astryx-view.
    - [ ] Invalid JSON/unregistered node/render failure: onParseError reports, errorFallback replaces view (default null), does not throw by default.
    - [ ] Documented persistence recipe: stringify editorState.toJSON(), render as value; Markdown source goes to core Markdown, not this component.

## Feedback & Status

- [x] **[Badge](https://astryx.atmeta.com/components/Badge)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Badge/Badge.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Badge).

  - **Variants and content**
    - [x] Semantic `variant`: `neutral` (default), `info`, `success`, `warning`, `error` (solid backgrounds). Theme-added names stay open and validated, not collapsed to neutral.
    - [x] Category `variant`: `blue`, `cyan`, `green`, `orange`, `pink`, `purple`, `red`, `teal`, `yellow` (tinted backgrounds/colored text). **[Standalone audit](docs/badge-port.md#contrast-and-source-limitations): 0/112 text and 0/112 inherited-glyph failures on modeled core/Neutral page/surface backdrops; this does not supersede the prior 267/672 Badge-inside-Button composite findings.**
    - [x] Required ReactNode `label`, optional leading icon; no size or loading enum documented. Rust BadgeLabel preserves scalar/rich/empty distinctions; supplied icon size and semantics stay caller-owned.
    - [x] One-line, content-sized, grows with label text height, ellipsis when constrained; string/number label also supplies title for full hover text. **Original AR4/OQ1 limitation retained and documented:** clipped-tail visual recovery is pointer-only; no invented focus stop/tooltip or blanket accessibility-compliance claim.
    - [x] String/number labels retain RTL character order; caller marks rich mixed-direction content with dir/bdi. Text carries meaning alongside icon/color.
  - **Templates**
    - [x] **Badge — Colors** (`BadgeCategoryTags.doc.mjs`): nine non-semantic category colors.
    - [x] **Badge — Counts** (`BadgeCountBadges.doc.mjs`): notification/unread/task counts, not fixed count styles.
    - [x] **Badge — Variants** (`BadgeShowcase.doc.mjs`): semantic and color variants.
    - [x] **Badge — Status** (`BadgeStatusLabels.doc.mjs`): status labels in rows/details.

- [ ] **[Banner](https://astryx.atmeta.com/components/Banner)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Banner/Banner.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Banner).

  - **Choices, slots, and disclosure**
    - [ ] `status`: `info`, `warning`, `error`, `success`; status controls icon/color, not live-region behavior.
    - [ ] `container`: `card` (default, rounded), `section` (full-width, square); `elevation`: `none` (default), `low`, `med`, `high`.
    - [ ] `isDismissable=false` by default; enabled dismiss hides banner with or without `onDismiss`. `dismissLabel` names/tooltips control; default `Dismiss <title>` for string title, explicit label for ReactNode title.
    - [ ] Header title, description, default status icon/custom icon, trailing action `endContent`; actions wrap below narrow header; supporting children below header.
    - [ ] `collapsible=true` default, initially collapsed; `false` always shows children without toggle; `{defaultIsOpen:true}` uncontrolled initial open; `{isOpen,onOpenChange}` controlled.
  - **Accessibility and examples**
    - [ ] Async outcomes announced by caller's transition handler via `useAnnounce`; urgent `role="alert"` applies only to title/description, not controls/supporting content. Do not double-announce.
    - [ ] Inline **Announce an async outcome from its transition handler**: save success/error banner with explicit announcement.
  - **Templates**
    - [ ] **Banner — Collapsible** (`BannerCollapsibleContent.doc.mjs`): action, dismiss, collapsible detail; open-default and no-toggle options.
    - [ ] **Banner — Dismiss** (`BannerDismissable.doc.mjs`).
    - [ ] **Banner — Floating** (`BannerFloating.doc.mjs`): elevation med.
    - [ ] **Banner — Full Width** (`BannerSectionVariant.doc.mjs`): section container.
    - [ ] **Banner — Statuses** (`BannerShowcase.doc.mjs`): stacked four statuses.
    - [ ] **Banner — Statuses** (`BannerStatuses.doc.mjs`): separate four-status template.
    - [ ] **Banner — Action** (`BannerWithActionButton.doc.mjs`).

- [ ] **[Info Tip](https://astryx.atmeta.com/components/InfoTip)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/InfoTip/InfoTip.doc.mjs). Templates in `InfoTip/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Size, content, and toggletip behavior**
    - [ ] `size`: `xsm` (12px), `sm` (default, 16px), `md` (20px), `lg` (24px).
    - [ ] Required concise non-interactive content; accessible button label defaults to translated `More information`, caller can specialize it.
    - [ ] Hover/focus previews; click/tap/Enter/Space pins open; second press, Escape, outside press, or focus leaving dismisses.
    - [ ] Real Tab-reachable button owns disclosure/name; aria-expanded true when pinned, help announced once on pin; host data attributes reach button.
    - [ ] No documented color/placement/style variants; metric definitions, permissions, and field help are content patterns, not fixed styles.

  - **Named templates**
    - [ ] `InfoTipMetricDefinitions` — **InfoTip — Metric Definitions**: info after each metric label.
    - [ ] `InfoTipPermissionNotes` — **InfoTip — Permission Notes**: who can change read-only settings.
    - [ ] `InfoTipShowcase` — **InfoTip**: compact info-tip showcase.

- [x] **[Progress Bar](https://astryx.atmeta.com/components/ProgressBar)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/ProgressBar/ProgressBar.doc.mjs). Templates in `ProgressBar/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Semantic and progress modes**
    - [x] `variant`: `accent` (default), `success`, `warning`, `error`, `neutral`; validated open names retain theme extensions. Approved accent/on-accent fallback and inset track/fill boundaries resolve unpainted custom and standalone-graphic source gaps.
    - [x] Determinate default versus unknown progress modeled by ProgressBarState; reduced motion replaces 40%/1500ms travel with full-width 1↔.84 opacity, 2000ms per leg, without tempo scaling.
    - [x] isLabelHidden, hasValueLabel, isDisabled: each false by default / true; clipped names remain referenced, counters share aria-valuetext, and disabled paint adds no invented ARIA disabled state.
    - [x] Required accessible label; value default 0, max default 100; safe finite/range normalization, localized percentage template and custom formatter, native props/ref/event composition and recoverable text validation.
    - [x] Named finite/clamped marks are focusable images outside the progressbar; hover/focus/touch/click tooltip preview, source on-color/track/disabled ink, themed overhang and removal cleanup verified. Unknown progress cannot carry marks and ignores value labels/formatters. **[Contrast](docs/progress-bar-port.md#contrast-and-limits):** approved boundaries pass all modeled cases; 15/168 conditional disabled-marker findings remain explicit.
    - [x] Authoritative spec supersedes measured linear touch timing: every pointer uses full theme duration/easing, sampled retarget/reversal; reduced/forced-color updates land immediately. Coarse positive minimum is 16px capped to track, with matching mark coverage; nearest rendered direction, own-track resize and lifecycle cleanup verified. **[Verification](docs/progress-bar-port.md):** 18 source comparisons/five native frames, no idle Paint/Layout and compositor pixels through 700ms blocking in both busy modes. Icons/labels remain caller-composed outside the fill.

  - **Named templates**
    - [x] `ProgressBarCustomFormat` — **ProgressBar — Custom Format**: disk-usage GB value.
    - [x] `ProgressBarIndeterminate` — **ProgressBar — Indeterminate**: unknown-duration loading.
    - [x] `ProgressBarSemanticVariants` — **ProgressBar — Semantic Variants**: accent/success/warning/error/neutral.
    - [x] `ProgressBarShowcase` — **ProgressBar**: example 60% progress, not fixed style/default.
    - [x] `ProgressBarWithValueLabel` — **ProgressBar — With Value Label**: percentage text.

- [x] **[Skeleton](https://astryx.atmeta.com/components/Skeleton)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Skeleton/Skeleton.doc.mjs).

  - **Shape and loading patterns**
    - [x] `radius`: `none`, `0`, `1`, `2`, `3` (default), `4`, `rounded` (pill/circle/avatar). Rust SkeletonRadius preserves literal-zero versus zero-token behavior and the shared 3/4 container token; geometry is not a public theme axis.
    - [x] Width and height: pixel number or CSS string, both default `100%`; match known content geometry rather than invent fixed size variants. Open SkeletonLength CSS, finite/nonnegative pixels, zero/fractional/default/CSS-wide dimensions, native ref/events/ARIA overrides and recoverable declaration-boundary validation are verified.
    - [x] Animated shimmer; `index` default 0 staggers start by delay + stagger × index; single placeholder versus multi-placeholder wave. Original shimmer is an opacity pulse, not a gradient: ten held levels .25–.925, 0.01% linear ramps, alternating pre-tempo duration, 1000 + 100 × index ms initial delay. Reduced motion, increased contrast, GrayText forced-color precedence, AX exclusion, caller-owned busy state and drop cleanup verified. **[Verification](docs/skeleton-port.md): 66 actual source block comparisons/native frame and backdrop parity; no idle paint/layout, changing compositor pixels during a 700ms JS block. Decorative shape has no invented essential-edge/text contrast threshold.**

  - **Named templates**
    - [x] [`SkeletonShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Skeleton/SkeletonShowcase.doc.mjs) — “Skeleton”: loading placeholder.
    - [x] [`SkeletonCardSkeleton`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Skeleton/SkeletonCardSkeleton.doc.mjs) — “Skeleton — Card Loading”: avatar/name/content lines.
    - [x] [`SkeletonStaggeredList`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Skeleton/SkeletonStaggeredList.doc.mjs) — “Skeleton — Staggered List”: varied line widths.
    - [x] [`SkeletonTableRowSkeleton`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Skeleton/SkeletonTableRowSkeleton.doc.mjs) — “Skeleton — Table Rows”: column-width placeholders.

- [x] **[Spinner](https://astryx.atmeta.com/components/Spinner)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Spinner/Spinner.doc.mjs).

  - **Choices and labeling**
    - [x] `size`: `sm` (10px), `md` (14px, default), `lg` (18px), `xl` (28px).
    - [x] `shade`: `default` (default), `onMedia`, `subtle`, `inherit`. **[Source-color audit](docs/spinner-port.md#source-color-findings-and-limits): 0/28 arc/backdrop failures; 6/28 arc/track failures at 3:1 and 2/28 optional string-label failures at 4.5:1. Original colors retained under the existing fidelity decision, not claimed compliant.**
    - [x] Optional visible ReactNode label below spinner: string label names indicator via aria-labelledby; explicit aria-label or translated `Loading` fallback supports non-string/no label. Rust preserves string `label` and adds mutually exclusive `label_content`; localized fallback comes from `loading_label`. Shared polite announcements, owner silence, CSS theming, non-shrinking geometry, batched phase and reduced-motion behavior are verified in [Spinner notes](docs/spinner-port.md).

  - **Named templates**
    - [x] [`SpinnerShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Spinner/SpinnerShowcase.doc.mjs) — “Spinner”: large indicator.
    - [x] [`SpinnerOnMedia`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Spinner/SpinnerOnMedia.doc.mjs) — “Spinner — On Media Shade”: light/dark surfaces.
    - [x] [`SpinnerSizes`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Spinner/SpinnerSizes.doc.mjs) — “Spinner — Sizes”.
    - [x] [`SpinnerWithLabel`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Spinner/SpinnerWithLabel.doc.mjs) — “Spinner — With Label”: text and rich multiline labels.

- [x] **[Status Dot](https://astryx.atmeta.com/components/StatusDot)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/StatusDot/StatusDot.doc.mjs).

  - **Choices and accessibility**
    - [x] `variant`: `success`, `warning`, `error`, `accent`, `neutral`; required with no default. Validated open StatusDotVariant names preserve theme extensions. **Approved source-gap corrections:** neutral plate/ink for otherwise-unpainted custom variants and 1px CanvasText outline for every variant in forced colors; built-in status plates use icon roles, not strong fills.
    - [x] `isPulsing`: false (default), true; honors reduced motion. Exact 2s 1→.5→1 ease-in-out pulse and all source easing samples verified. **[Contrast findings](docs/status-dot-port.md#contrast-and-verification-limits): resting 0/60 plate and 0/60 glyph cases below 3:1; unchanged pulse minimum 33/60 conditional plate and 33/60 optional-glyph findings.**
    - [x] Accessible `label`, optional explanatory tooltip and centered currentColor icon; boolean/empty-string icons ignored, zero/rich content preserved. Pair visible text or distinct status shapes, not color alone. **[Verification](docs/status-dot-port.md):** 38 actual source controls/native frames, caller semantics/ref/events, AX hidden icons, spec-led standalone keyboard/mouse/touch tooltip pinning and nested hover-only/host naming, bounded 44px/midpoint hit bands, dynamic updates/removal/cleanup. Role/name policy stays StatusDot-owned; Tooltip, announcement and geometry/holder utilities retain their source owners. Supporting public migrations remain independent.

  - **Named templates**
    - [x] [`StatusDotShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/StatusDot/StatusDotShowcase.doc.mjs) — “StatusDot” / “Status Dot”: positive status.
    - [x] [`StatusDotPulsing`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/StatusDot/StatusDotPulsing.doc.mjs) — “StatusDot — Pulsing”: live/processing/error signals.
    - [x] [`StatusDotStatusIndicators`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/StatusDot/StatusDotStatusIndicators.doc.mjs) — “StatusDot — Status Indicators”: labeled presence list.
    - [x] [`StatusDotVariants`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/StatusDot/StatusDotVariants.doc.mjs) — “StatusDot — Variants”: all five colors.

## Form Controls

- [ ] **[Calendar](https://astryx.atmeta.com/components/Calendar)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Calendar/Calendar.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Calendar).

  - **Selection and display choices**
    - [ ] `mode`: `single` (default), `range`; `numberOfMonths`: `1` (default), `2`.
    - [ ] Controlled `value` with null for no selection; omitted/undefined value uses uncontrolled `defaultValue`; onChange reports selection.
    - [ ] Controlled visible `focusDate` / onFocusDateChange versus automatic selected-date/today clamped to min/max; imperative `navigateTo()` handle.
    - [ ] `hasOutsideDays=true`, `hasWeekNumbers=false`, `hasVariableRowCount=false` defaults; fixed six rows versus compact variable rows; week numbers remain ISO 8601 in every calendar.
    - [ ] `weekStartsOn`: `0`, `1`, `2`, `3`, `4`, `5`, `6`, `sun`, `mon`, `tue`, `wed`, `thu`, `fri`, `sat`; default 0/Sunday.
    - [ ] `calendar`: `gregory`, `iso8601`, `buddhist`, `roc`, `japanese`, `persian`, `indian`, `hebrew`, `chinese`, `dangi`, `coptic`, `ethiopic`, `ethioaa`, `islamic-civil`, `islamic-tbla`, `islamic-umalqura`; default provider calendar, else gregory.
    - [ ] Non-Gregorian month/year navigation and names; all external dates/callbacks stay Gregorian ISO. Calendars beyond first five need Temporal or fall back to Gregorian; same Temporal implementation server/client avoids Chinese/Dangi month-start mismatch.
    - [ ] min/max/custom dateConstraints; range max/min span count both endpoints (min default 1); second click on start commits one day if allowed, otherwise cancels unfinished selection. Before start, max span does not disable dates.
  - **Inline example and templates**
    - [ ] Inline **A custom month view in the Persian calendar**: navigation/days hooks share explicit calendar and visibleCalendarMonths (hooks do not inherit provider calendar).
    - [ ] **Calendar — Constraints** (`CalendarConstraints.doc.mjs`): bounds plus weekday rules.
    - [ ] **Calendar — Range** (`CalendarRangeWithValue.doc.mjs`).
    - [ ] **Calendar** (`CalendarShowcase.doc.mjs`): interactive single date.
    - [ ] **Calendar — Single** (`CalendarSingle.doc.mjs`).
    - [ ] **Calendar — Two Months** (`CalendarTwoMonths.doc.mjs`).
    - [ ] **useCalendarConstraints — Hotel Booking Range** (`useCalendarConstraintsHookUsage.doc.mjs`): season bounds, max stay, closed Mondays.
    - [ ] **useCalendarDays — Month Grid** (`useCalendarDaysHookUsage.doc.mjs`): compact seven-column table, outside days, Sunday/Monday switch.
    - [ ] **useCalendarNavigation — Month Pager** (`useCalendarNavigationHookUsage.doc.mjs`): localized label, previous/next, cross-month arrows/pendingFocus.

- [ ] **[Checkbox Input](https://astryx.atmeta.com/components/CheckboxInput)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/CheckboxInput/CheckboxInput.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/CheckboxInput).

  - **Choices, states, and labels**
    - [ ] `value`: `false`, `true`, `indeterminate`; onChange reports boolean and native event. No defaultValue/uncontrolled API documented.
    - [ ] `size`: `sm`, `md` (default); coarse-pointer labelled invisible hit area grows toward 44px, limited by neighbor gap, without changing visual size.
    - [ ] `isLabelHidden=false`, `isLoading=false`, `isDisabled=false`, `isReadOnly=false`, `isOptional=false`, `isRequired=false` defaults; optional and required mutually exclusive.
    - [ ] Read-only blocks change at full opacity; disabled dims; disabledMessage tooltip keeps disabled control focusable with aria-disabled and blocks toggling.
    - [ ] Async `changeAction` runs after unprevented onChange, spinner/interaction block while pending; explicit isLoading also blocks.
    - [ ] Required accessible label, optional description, labelIcon semantic/custom content, field width; status type `error`, `warning`, `success` with message; error sets aria-invalid.
    - [ ] htmlName participates in form, checked submits on; native-input ref/focus/blur callbacks.
  - **Templates**
    - [ ] **CheckboxInput — States** (`CheckboxInputBasic.doc.mjs`): checked/unchecked/disabled with descriptions.
    - [ ] **CheckboxInput — Indeterminate** (`CheckboxInputIndeterminateState.doc.mjs`): select-all partial dash, toggle group.
    - [ ] **CheckboxInput** (`CheckboxInputShowcase.doc.mjs`): interactive three states.
    - [ ] **CheckboxInput — Status** (`CheckboxInputStatusVariations.doc.mjs`): error/warning/success.

- [ ] **[Complex Selector](https://astryx.atmeta.com/components/ComplexSelector)**

  - Sources: [English doc, nested component API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/ComplexSelector/ComplexSelector.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ComplexSelector).

  - **Choices and state**
    - [ ] `variant`: `input` (default, bordered), `secondary` (neutral button), `ghost` (transparent button).
    - [ ] `size`: `sm` (28px), `md` (32px, default), `lg` (36px).
    - [ ] `placement`: `above`, `below` (default), `start`, `end`; `alignment`: `start` (default), `center`, `end`.
    - [ ] Status type `warning`, `error`, `success`; `statusVariant`: `attached`, `detached`, `tooltip`; default attached for input, detached for secondary/ghost (attached resolves detached there).
    - [ ] Required controlled value, onChange commit helper; async changeAction after onChange gives optimistic value/busy state; explicit isLoading and isDisabled supported (doc does not state their defaults).
    - [ ] Internal popup state, notification onOpenChange for trigger/keys/light-dismiss/Escape/close/handle; imperative open(), close(), toggle(), isOpen().
  - **Trigger/content/accessibility**
    - [ ] Labelled field and triggerLabel versus placeholder (default Select...), startIcon semantic/custom, width, content/container/control styling slots.
    - [ ] children render receives value, onChange, close, render state; caller supplies accessible grid/tree/list/date structure and keyboard semantics. Staged draft Apply commits, Cancel/dismiss does not; call close explicitly when desired.
    - [ ] renderTrigger replaces entire chrome with caller glyph/chip/icon control; spread given ref/id/click/key/role/ARIA props to retain combobox/dialog contract and anchoring.
    - [ ] DisabledMessage hover/focus reason describes disabled focusable trigger, including custom trigger; blocks opening. Avoid competing custom-control tooltip while disabled.
    - [ ] Built-in trigger onClick runs before open and can cancel; custom renderTrigger callback runs after toggle and cannot cancel. Built-in onKeyDown runs after handling including ArrowDown; custom path skips keys internally cancelled.
    - [ ] Owns popup focus restore; caller uses grid/tree/list focus hooks for content, preserving names/roles and visible/unobscured focus.
  - **Templates**
    - [ ] **ComplexSelector — Deadline Picker** (`ComplexSelectorDeadlinePicker.doc.mjs`): presets/custom date/time staged Apply flow.
    - [ ] **ComplexSelector** (`ComplexSelectorShowcase.doc.mjs`): two-axis fruit/ripeness grid with caller keyboard semantics.
    - [ ] **ComplexSelector — Tree Search** (`ComplexSelectorTreeSearch.doc.mjs`): search plus TreeList destination, close on folder selection.

- [ ] **[Date Input](https://astryx.atmeta.com/components/DateInput)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/DateInput/DateInput.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/DateInput).

  - **Finite choices and defaults**
    - [ ] `size`: `sm`, `md` (default), `lg`; status type `warning`, `error`, `success`; statusVariant `attached` (default), `detached`, `tooltip`.
    - [ ] `presentation`: `popover`, `bottom-sheet`, `native`, `adaptive-bottom-sheet`, `adaptive-native` (default). Adaptive fine pointer uses popover; coarse uses sheet/native respectively. No typed-only presentation for DateInput.
    - [ ] Deprecated nativePicker `touch` (default → adaptive-native), `always` (native), `never` (adaptive-bottom-sheet); explicit presentation wins.
    - [ ] `format`: `date`, `date_long` (default), `date_weekday`, `system_date`, custom formatter; first three use field calendar, system_date Gregorian ISO; formatting committed value only, custom output must be readable to recommit.
    - [ ] `numberOfMonths`: `1` (default), `2`; `weekStartsOn`: `0` (default), `1`, `2`, `3`, `4`, `5`, `6`, `sun`, `mon`, `tue`, `wed`, `thu`, `fri`, `sat`.
    - [ ] `calendar`: `gregory`, `iso8601`, `buddhist`, `roc`, `japanese`, `persian`, `indian`, `hebrew`, `chinese`, `dangi`, `coptic`, `ethiopic`, `ethioaa`, `islamic-civil`, `islamic-tbla`, `islamic-umalqura`; default provider calendar else gregory.
    - [ ] `entryErrorBehavior`: `complete`, `blur-or-enter`; default nearest FormLayout policy else complete.
  - **State, validation, and interaction**
    - [ ] `isLabelHidden=false`, `isOptional=false`, `isRequired=false`, `isDisabled=false`, `isLoading=false`, `hasClear=false` defaults; label/description/info tooltip, width and container/control styling seams.
    - [ ] Controlled Gregorian ISO value/onChange (undefined on clear); no defaultValue API documented. changeAction after onChange uses optimistic transition; pending spinner/aria-busy blocks typing/picking/clearing while preserving focus.
    - [ ] Disabled field remains focusable aria-disabled, typing/open blocked; disabledMessage hover/focus explanation. Clear returns focus to input.
    - [ ] min/max/dateConstraints reject dates; entry-error kind `unreadable`, `incomplete`, `nonexistent`, `outOfRange`, `unavailable`, or null when resolved. Callback includes typed text, ISO date for range/unavailable; avoids duplicate same rejection and clears when typed surface leaves.
    - [ ] Complete policy shows complete invalid dates immediately, unreadable/incomplete on Enter/blur; blur-or-enter delays message/ARIA/style/announcement/callback together, preserves shown wrong error until next commit, clears accepted/empty edits immediately. Native rejected dates show at Enter/blur under both.
    - [ ] Rejection overrides warning/success; with onEntryErrorChange, caller error message can replace own rejection message and is announced for a newly rejected entry even if unchanged.
    - [ ] Locale/calendar text entry supports any decimal digits, complete-only commit and era/leap-month years; external dates remain Gregorian. Beyond gregory/iso8601/buddhist/roc/japanese need Temporal else Gregorian fallback.
    - [ ] Non-Gregorian/non-ISO calendar substitutes popover on fine pointer/sheet on coarse even for native policies (explicit native warns). Native cannot express weekStartsOn/numberOfMonths/dateConstraints; choose Astryx surface when needed.
    - [ ] Fine-pointer dialog combobox: ArrowDown/Alt+ArrowDown opens, Escape closes/restores field focus. Native/base DOM event handlers run after own handler and cannot cancel built-ins; ref/id/ARIA target input.
  - **Templates**
    - [ ] **DateInput — Clearable** (`DateInputClearable.doc.mjs`).
    - [ ] **DateInput — Min/Max Constraints** (`DateInputDateRange.doc.mjs`): date window, not range-valued DateInput.
    - [ ] **DateInput — Formats** (`DateInputFormats.doc.mjs`): four named committed formats/custom function.
    - [ ] **DateInput** (`DateInputShowcase.doc.mjs`): typed/calendar entry.
    - [ ] **DateInput — Description** (`DateInputWithDescription.doc.mjs`).
    - [ ] **DateInput — Validation** (`DateInputWithValidation.doc.mjs`): three status states.

- [ ] **[Date Range Input](https://astryx.atmeta.com/components/DateRangeInput)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/DateRangeInput/DateRangeInput.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/DateRangeInput).

  - **Choices and modes**
    - [ ] `size`: `sm`, `md` (default), `lg`; status type `warning`, `error`, `success`; statusVariant `attached` (default), `detached`, `tooltip`.
    - [ ] `numberOfMonths`: `1`, `2` (default); `weekStartsOn`: `0` (default), `1`, `2`, `3`, `4`, `5`, `6`, `sun`, `mon`, `tue`, `wed`, `thu`, `fri`, `sat`.
    - [ ] `calendar`: `gregory`, `iso8601`, `buddhist`, `roc`, `japanese`, `persian`, `indian`, `hebrew`, `chinese`, `dangi`, `coptic`, `ethiopic`, `ethioaa`, `islamic-civil`, `islamic-tbla`, `islamic-umalqura`; provider default else gregory.
    - [ ] `isLabelHidden=false`, `isOptional=false`, `isRequired=false`, `isDisabled=false`, `isLoading=false`, `hasClear=true` defaults; label/description/info tooltip/status/width.
  - **Selection, constraints, and async behavior**
    - [ ] Controlled `{start,end}` Gregorian ISO DateRange or null; onChange(null) on clear. Import exported DateRange type; no defaultValue/uncontrolled API documented.
    - [ ] Button trigger/calendar popover, no typed entry and no presentation prop documented; placeholder default Select date range.
    - [ ] min/max/custom rules; minRangeSpan default 1 and maxRangeSpan count endpoints; clicking start again commits one day if permitted else cancels partial selection. Max does not rewrite externally supplied overwide range.
    - [ ] Preset data quick-select beside calendar or above when narrow; disabled if endpoints or min/max span violate constraints. Preset dates/names are caller data, not built-in fixed variants.
    - [ ] Async changeAction after onChange drives optimistic transition; loading blocks interaction/spinner. Disabled trigger retains focus/aria-disabled; disabledMessage describes it on hover/focus.
    - [ ] Non-first-five calendars need Temporal or Gregorian fallback/development warning; same implementation SSR/client for Chinese/Dangi. Trigger display follows calendar, values/min/max/presets remain Gregorian.
    - [ ] Base props/events/ref go to trigger, handlers run after own behavior and cannot cancel it; container/control styling targets field/box.
  - **Templates**
    - [ ] **DateRangeInput** (`DateRangeInputShowcase.doc.mjs`): button, two-month calendar, presets.
    - [ ] **DateRangeInput — With Presets** (`DateRangeInputWithPresets.doc.mjs`): quick analytics/report ranges.
    - [ ] **DateRangeInput — Validation** (`DateRangeInputWithValidation.doc.mjs`): error/warning/success.

- [ ] **[Date Time Input](https://astryx.atmeta.com/components/DateTimeInput)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/DateTimeInput/DateTimeInput.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/DateTimeInput).

  - **Presentation and finite options**
    - [ ] `presentation`: `popover`, `bottom-sheet`, `native`, `adaptive-bottom-sheet`, `adaptive-native` (default); adaptive uses fine-pointer typed/popover and coarse native or coordinated Date/Time sheet. No typed-only policy here.
    - [ ] Deprecated nativePicker `touch` (default → adaptive-native), `always` (native), `never` (adaptive-bottom-sheet); presentation wins.
    - [ ] `hourFormat`: `12h`, `24h`; omitted follows provider locale/hour-cycle extension. `hasSeconds=false` default.
    - [ ] `timeIncrement`: `1` (default), `5`, `10`, `15`, `30` for typed-arrow minute step; `timeOptionInterval`: `5`, `10`, `15`, `30`, `60` for fine-pointer preset list; touch sheet wheels ignore these cadences.
    - [ ] `size`: `sm`, `md` (default), `lg`; status `warning`, `error`, `success`; no statusVariant prop listed in this doc.
    - [ ] `numberOfMonths`: `1` (default), `2` on pointer popover only; touch Date panel always one swipe-paged month. `weekStartsOn`: `0` (default), `1`, `2`, `3`, `4`, `5`, `6`, `sun`, `mon`, `tue`, `wed`, `thu`, `fri`, `sat`; native ignores it.
    - [ ] `calendar`: `gregory`, `iso8601`, `buddhist`, `roc`, `japanese`, `persian`, `indian`, `hebrew`, `chinese`, `dangi`, `coptic`, `ethiopic`, `ethioaa`, `islamic-civil`, `islamic-tbla`, `islamic-umalqura`; provider default else gregory.
    - [ ] `entryErrorBehavior`: `complete`, `blur-or-enter`, nearest FormLayout default else complete.
  - **States, constraints, and entry**
    - [ ] `isLabelHidden=false`, `isOptional=false`, `isRequired=false`, `isDisabled=false`, `isLoading=false`, `hasClear=false` defaults; helper text/info tooltip/width; timeLabel default <label> time.
    - [ ] Controlled ISO datetime minute/second value and onChange(undefined) clear; no defaultValue documented; async changeAction after onChange uses optimistic transition/spinner and blocks interaction.
    - [ ] Two closed segments side-by-side at available width >=400px, full-width stacked below; independent of viewport. Date placeholder Select a date, time Select a time.
    - [ ] min/max constrain date and time, custom dateConstraints; values stay Gregorian, locale/calendar text supports decimal digits, eras/leap months; non-first-five calendars need Temporal else Gregorian.
    - [ ] Adaptive native retains Astryx time for seconds, non-default increment, or timeOptionInterval; explicit native uses native controls without those per-segment fallbacks. Non-gregory/non-iso calendar forces Astryx fine/coarse surfaces even under native policies and warns for explicit native.
    - [ ] Constraints enforced on commit; native min/max only hints; hourFormat formats closed text, OS picker follows user's locale.
    - [ ] Date rejection kinds `unreadable`, `incomplete`, `nonexistent`, `outOfRange`, `unavailable`, null on clear; deduplicated onEntryErrorChange includes text/date where applicable and clears if typed surface leaves. Native/touch have no typed-text callback rejection.
    - [ ] Complete policy shows complete rejected date/time immediately, other bad text Enter/blur; blur-or-enter delays both segments' message/ARIA/style/announcement and date callback until segment commit. Accepted/empty edit clears immediately; unchanged bad edit retains shown error until next commit; native date rejection Enter/blur in both policies.
    - [ ] Error status message can replace own error; warnings/success cannot mask rejection. Time errors remain typed, show unless date error or caller error message takes precedence.
    - [ ] Disabled fields stay focusable aria-disabled, opening/typing blocked; disabledMessage describes both; base DOM events run after built-ins, ref/id/ARIA target date input. Control styling targets row, never native input.
  - **Templates**
    - [ ] **DateTimeInput** (`DateTimeInputShowcase.doc.mjs`): metadata describes pointer calendar/time and touch Date/Time sheet; current documented default is adaptive-native, so do not infer default presentation from showcase description.
    - [ ] **DateTimeInput — Validation** (`DateTimeInputWithValidation.doc.mjs`): scheduling error/warning/success.

- [ ] **[Field](https://astryx.atmeta.com/components/Field)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Field/Field.doc.mjs). Templates in `Field/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Label and validation states**
    - [ ] `isGroupLabel`: false (default native label) / true (span for group); `isLabelHidden`: false (default) / true; `isDisabled`: false (default) / true label styling.
    - [ ] `isOptional`: false (default) / true; `isRequired`: false (default) / true; mutually exclusive.
    - [ ] `status.type`: `warning`, `error`, `success`, optionally message and messageID; absent status is neutral.
    - [ ] `statusVariant`: `attached` (default), `detached`, `tooltip`. Attached only suits opaque bordered fixed-height controls; tooltip announces without FieldStatus box.

  - **Slots, naming, and composition**
    - [ ] Required label/inputID and caller-owned control children; labelID/group association, description/descriptionID, status messageID for caller `aria-describedby` wiring.
    - [ ] Leading `labelIcon` as semantic icon or ReactNode; trailing `labelTooltip` info button named `About {label}`.
    - [ ] Help previews on hover/focus; tap/click/Enter/Space opens it without focusing or toggling associated control.
    - [ ] Whole-field `width` keeps label/control/status aligned and shrinkable; do not double-wrap Astryx inputs already owning Field shells.

  - **Named inline examples**
    - [ ] **Filter bar**: wrapping HStack, growing search field and shrinking selectors; narrow screen avoids horizontal overflow.
    - [ ] **Wrap a custom control**: native range control with description association and detached success status.

  - **Named templates**
    - [ ] `FieldRequired` — **Field — Required & Optional**: exception indicators side by side.
    - [ ] `FieldShowcase` — **Field**: label/description/status wrapper around input.
    - [ ] `FieldStatusVariants` — **Field — Validation States**: error, warning, success.
    - [ ] `FieldWithDescription` — **Field — Description**: format/constraint/data-use helper text.

- [ ] **[File Input](https://astryx.atmeta.com/components/FileInput)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/FileInput/FileInput.doc.mjs). Templates in `FileInput/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Modes and state**
    - [ ] `mode`: `input` (default compact), `dropzone` (drag-and-drop area).
    - [ ] `isMultiple`: false (default File/null) / true (File[]); required controlled `value` and `onChange`, including removal.
    - [ ] `isLabelHidden`, `isOptional`, `isRequired`, `isDisabled`, `isLoading`: each false by default; optional/required mutually exclusive; loading shows spinner/aria-busy.
    - [ ] `status.type`: `error`, `warning`, `success`; optional message, error sets `aria-invalid`.
    - [ ] `statusVariant`: `attached`, `detached`, `tooltip`; default attached for input, detached for dropzone. Explicit attached dropzone retains released rendering at this revision; docs announce a future normalization, not current behavior.

  - **Constraints, slots, and async behavior**
    - [ ] Required accessible label; description, labelTooltip, custom placeholder; default `Choose file` / `Choose files` follows multiplicity.
    - [ ] `accept` uses open-ended HTML MIME/extensions; `maxSize` rejects oversized files with error; `maxFiles` limits multi-file selection; folder/directory upload unsupported.
    - [ ] `changeAction` supports immediate async upload using React 19 transitions; caller owns upload operation.
    - [ ] `disabledMessage` keeps disabled trigger focusable with aria-disabled and hover/focus reason while picker remains blocked.
    - [ ] Whole-field width; container styling versus control-box styling, not native input styling; container overrides do not apply inside InputGroup or horizontal-labels FormLayout.

  - **Named templates**
    - [ ] `FileInputBasic` — **FileInput — Basic**: controlled single-file upload, accepted types, size limit, helper text.
    - [ ] `FileInputShowcase` — **FileInput**: showcase metadata supplies no further pattern description; do not infer extra modes from its title.

- [ ] **[Multi Selector](https://astryx.atmeta.com/components/MultiSelector)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/MultiSelector/MultiSelector.doc.mjs). Templates in `MultiSelector/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Trigger and validation choices**
    - [ ] `size`: `sm`, `md` (default), `lg`; `variant`: `input` (default), `secondary`, `ghost`.
    - [ ] `triggerDisplay`: `count` (default), `labels`, `badges`; formatValue customizes count/labels only; maxBadges default 3 before +N.
    - [ ] `indicatorPosition`: `start` (default), `end`, also moves select-all indicator.
    - [ ] `presentation`: `popover` (default), `bottom-sheet`, `adaptive` compact-touch sheet.
    - [ ] `status.type`: `error`, `warning`, `success`; `statusVariant`: `attached`, `detached`, `tooltip`; attached default only for input, detached default for secondary/ghost, attached requests there detach.
    - [ ] `isReadOnly`, `hasCreate`, `hasClear`, `isDefaultOpen`: each false by default / true.
    - [ ] `isDisabled`, `isLoading`, `isLabelHidden`, `isOptional`, `isRequired`, `hasSelectAll`, `hasSearch`: each on/off; defaults not stated in their prop entries.
    - [ ] Read-only retains full-opacity focusable combobox/form values but removes selection/clear/disclosure surface; disabled wins, is dimmed, disabledMessage describes focusable trigger with blocked activation.

  - **Controlled values, slots, and search**
    - [ ] Required controlled string[] value/onChange; changeAction follows onChange and can be async; one hidden input per value with htmlName.
    - [ ] Options: strings, value/label/icon/disabled objects, dividers, sections; custom renderOption only for actual selectable rows, not headings/dividers/select-all.
    - [ ] Option action slot changes popup into grid: Up/Down rows, inline-end arrow to action, Enter activates; pointer/touch action never changes selection.
    - [ ] Required accessible label; helper description, startIcon, width, loading spinner; placeholders default Select... and Search..., select-all label default Select all.
    - [ ] Search announces count/no-match politely, magnifier and query-clear button resets/focuses field; emptyText default No options, emptySearchText default No results found; rendered text announced with aria-hidden content excluded; empty initial list not shown while loading.
    - [ ] Create requires search: first Create-query row when no case-insensitive label equals query, or Enter with no highlight; onChange appends query plus `{type:'create',query}`, caller adds matching option same update; clears search; nothing offered while loading.
    - [ ] renderTrigger replaces entire field/status/clear/spinner with caller trigger spreading ref/ID/handlers/ARIA; popup stays named by label.
    - [ ] Imperative handle open/close/toggle/isOpen, not mirrored controlled visibility; onOpenChange for every opening/closing path. onClick follows toggle, onFocus precedes own handling, canceled built-in key events skip caller onKeyDown.
    - [ ] Container versus visual-control styles; container overrides ineffective in InputGroup/horizontal-labels form. Grouped multi-value display must remain single-line.

  - **Named templates**
    - [ ] `MultiSelectorBottomSheet` — **MultiSelector — Bottom Sheet**: list remains open while toggling selections.
    - [ ] `MultiSelectorColumnVisibilitySelector` — **MultiSelector — Column Visibility**: hidden label, search, select-all, count.
    - [ ] `MultiSelectorForm` — **MultiSelector — Form Composition**: required/optional fields.
    - [ ] `MultiSelectorGhostToolbar` — **MultiSelector — Ghost Toolbar**: ghost buttons plus borderless trigger.
    - [ ] `MultiSelectorSearchableMultiSelector` — **MultiSelector — Searchable**: filtering/select-all.
    - [ ] `MultiSelectorSectionedMultiSelector` — **MultiSelector — Sectioned Permissions**: titled sections.
    - [ ] `MultiSelectorShowcase` — **MultiSelector**: standard multi-select showcase.
    - [ ] `useMultiComboboxHookUsage` — **useMultiCombobox — Issue Labels**: input-owned focus, arrow active option, Enter/click toggles without closing, removable selected tokens.

- [ ] **[Number Input](https://astryx.atmeta.com/components/NumberInput)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/NumberInput/NumberInput.doc.mjs). Templates in `NumberInput/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Sizes and states**
    - [ ] `size`: `sm`, `md` (default), `lg`; status type `error`, `warning`, `success`; statusVariant `attached` (default), `detached`, `tooltip`.
    - [ ] Required controlled number/null/undefined value and onChange; hasClear false (default) / true extends change callback to null.
    - [ ] isReadOnly false (default) / true retains full-opacity submitted value and blocks arrow/wheel/stepper edits; isDisabled wins. disabledMessage keeps disabled field focusable/readonly with explanation.
    - [ ] isLabelHidden, isOptional, isRequired, isDisabled, isIntegerOnly, hasAutoFocus: on/off; optional/required mutually exclusive, defaults not explicitly given.
    - [ ] isWheelEnabled true (default) / false; hasNumberSteppers false (default) / true on fine pointers only; mouse/pen hold repeats, commits once when hold ends.

  - **Entry, formatting, and errors**
    - [ ] Arbitrary min/max/step (default 1), units, placeholder; formatValue shows formatted committed value blurred, raw number focused, aria-valuetext for formatting.
    - [ ] Valid text commits on Enter/blur; steps/clear change immediately; locale-aware formatted paste accepted, no typing interception. Ordinary out-of-range number clamps.
    - [ ] Rejection kinds: `unreadable`, `fractional` (integer-only), `outOfRange` (min>max). Rejected text retained at commit, announced and marked invalid, never shown while typing.
    - [ ] `entryErrorBehavior`: `complete`, `blur-or-enter`; nearest FormLayout default else complete; identical commit-only behavior for NumberInput.
    - [ ] onEntryErrorChange reports changed error or null, including valid/empty correction, value/constraint changes, disabling, and unmount with outstanding rejection; no duplicate identical reports.
    - [ ] Caller error status with message replaces own displayed error; warning/success or error without message do not; underlying rejection announcement/invalid state preserved.
    - [ ] htmlName posts committed number after rejected blur until text edited/new value; onFocus/onBlur/onKeyDown/onEnter hooks; label/icon/labelTooltip/description and control/container styling.

  - **Named templates**
    - [ ] `NumberInputClearableNumberInput` — **NumberInput — Clearable**: clear button, units, range.
    - [ ] `NumberInputRangeNumberInput` — **NumberInput — Range Constrained**: bounds/helper description.
    - [ ] `NumberInputShowcase` — **NumberInput**: quantity field.
    - [ ] `NumberInputStatuses` — **NumberInput — Status Variants**: error/warning/success.
    - [ ] `NumberInputWithUnits` — **NumberInput — With Units**: percentage suffix/range.

- [ ] **[Power Search](https://astryx.atmeta.com/components/PowerSearch)**

  - Sources: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/PowerSearch/PowerSearch.doc.mjs), [operator/config types](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/PowerSearch/types.ts). Templates in `PowerSearch/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Presentation and field states**
    - [ ] `presentation`: `popover`, `bottom-sheet`, `adaptive` (default). SSR/hydration begin popover; adaptive follows compact touch selection policy.
    - [ ] `size`: `sm`, `md` (default), `lg`; status type `warning`, `error`, `success`; statusVariant `attached` (default), `detached`.
    - [ ] isLabelHidden true (default) / false; hasAutoFocus false (default) / true; hasClear true (default) / false; isReadOnly and isDisabled false (default) / true.
    - [ ] Required config/controlled filters/onChange; changeType `add`, `edit`, `remove` plus affected index. Whole read-only blocks edits; per-filter read-only survives clear-all.
    - [ ] tokenOverflowBehavior `none` (default), `unfocusedInline`, `unfocusedLayer`; only popover field, warns/no surface in sheet; menuWidth likewise inapplicable in sheet.

  - **Actual typed editor/config choices**
    - [ ] Operator value discriminants: `empty`, `string`, `string_list`, `integer`, `float`, `time`, `date_absolute`, `date_relative`, `date_range`, `enum`, `enum_list`, `entity_list`, `custom`, `nested`; stored values match typed schema, not visual variants.
    - [ ] String/list/entity source supplied versus plain/arbitrary text; isArbitraryStringAllowed false default / true; list paste tokenization regex and sort on/off; custom entity renderer.
    - [ ] Integer/float numeric bounds and units; time bounds; enum single versus enum_list multiple caller-defined values; nested filter arrays; custom Editor/getString for serialized custom value.
    - [ ] date_absolute isDateOnly true/unset day picker versus false date/time instant; date_relative past/future permissions each true default / false; date_range interval/relative presets.
    - [ ] Date range parts `NOW`, `ABSOLUTE`, `RELATIVE`; relative unit `second`, `minute`, `hour`, `day`, `week`, `month`, `year`, optional anchorKey. Presets/field/operator labels are caller data, not a closed list of styles.
    - [ ] Operator display label is caller-localized text or i18nKey lookup, mutually exclusive; free-text contentSearchFieldKey and generated configuration/table filtering are documented composition modes.

  - **Slots, async search, and sheet workflow**
    - [ ] Accessible label defaults Search, placeholder Search..., edit-save label Apply; startIcon (name/component/node), trailing endContent; numeric resultCount formats N results or string as-is, polite live updates.
    - [ ] Defaults: maxTokenLength 40, maxOperatorMenuItems 10, maxSearchResults 10; empty browsing up to 1,000 fields. Search sources can provide entity/string suggestions; caller owns data.
    - [ ] timezoneID: date-only/absolute range parts use specified midnight, UTC if unset; date-time uses specified zone, viewer zone if unset. Provider calendar changes presentation, stored dates remain Unix seconds.
    - [ ] Disabled reason keeps primary input focusable/blocked; handle focusTypeahead/blurTypeahead; host data attributes target combobox in popover, trigger in sheet.
    - [ ] Sheet trigger shows display-only tokens; Filters rows edit/remove, Add filter, Done, confirmed Clear all preserving read-only; stacked Add/filter-value sheets stage operator then value.
    - [ ] Draft adds/edits/removes apply only on Done, onChange once per edit in order; changed-draft dismissal asks discard; dismissing upper step reveals lower; list values use Tokenizer sheet.
    - [ ] Documentation discrepancy: duplicate menuWidth/maxOperatorMenuItems entries describe main/search versus operator menus differently. Keep the limits above and verify exact menu targeting in source before implementation; do not create a second prop or extra enum from duplicate entries.

  - **Named inline example**
    - [ ] **Keep the popover on every device**: explicit presentation popover with caller filters state.

  - **Named templates**
    - [ ] `PowerSearchContentSearch` — **PowerSearch — Content Search**: free text maps contentSearchFieldKey to title field.
    - [ ] `PowerSearchFullFeatured` — **PowerSearch — Full Featured**: enum/multi-select/entity/text editors.
    - [ ] `PowerSearchPresetFilters` — **PowerSearch — Preset Filters**: initial status/priority tokens.
    - [ ] `PowerSearchSearchWithTable` — **PowerSearch — Search with Table**: generated config plus filtering.
    - [ ] `PowerSearchShowcase` — **PowerSearch**: prepopulated enum/text filters.
    - [ ] `usePowerSearchConfigHookUsage` — **usePowerSearchConfig — Filtered Directory Table**: typed definitions, config, applyFilters, Table rows.

- [ ] **[Radio List](https://astryx.atmeta.com/components/RadioList)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/RadioList/RadioList.doc.mjs). Templates in `RadioList/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Group layout and states**
    - [ ] `orientation`: `vertical` (default), `horizontal`; size `sm`, `md` (default).
    - [ ] Required controlled value/onChange, label, RadioListItem children; one-of-N visible choice, not multi-select.
    - [ ] isLabelHidden, isDisabled, isRequired, isOptional: each false by default / true; optional/required mutually exclusive.
    - [ ] status type `warning`, `error`, `success`, optional message; labelTooltip, description, whole-field width.
    - [ ] htmlName groups native radios for form submission; omitted gets unique internal grouping name.
    - [ ] Group disabledMessage keeps radios focusable via aria-disabled while selection blocked, applies to group disabled state rather than per-item reason.
    - [ ] Native radio input owns keyboard/accessibility behavior; circle edge/fill must reach 3:1 at rest/hover/pressed/selected; container/control style targeting follows field API.

  - **Named templates**
    - [ ] `RadioListHorizontalLayout` — **RadioList — Horizontal Layout**: compact row.
    - [ ] `RadioListPricingTier` — **RadioList — Pricing Tier**: price in end content.
    - [ ] `RadioListShowcase` — **RadioList**: radio-group showcase.
    - [ ] `RadioListWithDescriptions` — **RadioList — With Descriptions**: group/item helper text.
    - [ ] `RadioListWithValidation` — **RadioList — With Validation**: required empty/error state.

- [ ] **[Selector](https://astryx.atmeta.com/components/Selector)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Selector/Selector.doc.mjs).

  - **Finite choices and defaults**
    - [ ] `size`: `sm`, `md` (default), `lg`.
    - [ ] `variant`: `input` (default; bordered), `secondary` (neutral-filled), `ghost` (transparent).
    - [ ] `status.type`: `error`, `warning`, `success`, with or without message; `statusVariant`: `attached`, `detached`, `tooltip`. Default attached for input, detached for secondary/ghost; attached is valid only for input.
    - [ ] `indicatorPosition`: `start`, `end` (default); empty indicators consume no space.
    - [ ] `presentation`: `popover` (default), `bottom-sheet`, `adaptive` (sheet on compact touch screens, popover otherwise).

  - **State, content, and interaction**
    - [ ] `hasClear`, `hasSearch`, `isDisabled`, `isReadOnly`, `isLabelHidden`, `isOptional`, `isRequired`, `isLoading`: each defaults false.
    - [ ] Selected `value` with `onChange`; clearing emits null when enabled. No `defaultValue` mode is documented. `htmlName` submits the selected value through a hidden input.
    - [ ] Read-only retains focus, value submission, full-opacity appearance and `aria-readonly` combobox identity; removes selection surface, clear action and disclosure. Disabled wins over read-only; disabled trigger remains focusable with `aria-disabled`, blocks activation, and can show `disabledMessage` on hover/focus.
    - [ ] Options: strings; value/label objects with optional description, icon, disabled state; `{type: "divider"}`; `{type: "section", title, options}`. Cover selected, unselected, highlighted and disabled rows.
    - [ ] Label, description, optional/required marking; `startIcon` as semantic/icon-rendered or caller-rendered content; status icon versus collapsed/expanded chevron; standalone Field versus InputGroup shell.
    - [ ] Default options versus `renderOption` returning SelectorOption or bare content; custom `renderValue`. One-line/two-line triggers grow by a line; InputGroup fixes the row and forces SelectorOption inline/ellipsis, clipping taller custom nodes.
    - [ ] Placeholder default `Select...`; search placeholder `Search...`; empty list `No options`; empty search `No results found`. ReactNode empty text is announced from rendered DOM text, omitting aria-hidden parts; loading suppresses empty-list text.
    - [ ] Search magnifier and query-clear action; clearing search resets query and restores input focus. Result counts/no-results use a polite live region.
    - [ ] Closed-trigger typeahead selects directly; repeated first letters cycle, spaces participate. Open-list typing highlights and Enter commits; with search, typing opens and seeds the search field.
    - [ ] Trigger click runs before toggle (preventDefault does not stop it); focus callback precedes built-in focus handling; key callback follows built-in handling and skips claimed keys.
    - [ ] Width sizes the entire field; shrinking/truncating values and wrapping filter bars are documented content patterns.

  - **Named templates**
    - [ ] [`SelectorShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Selector/SelectorShowcase.doc.mjs) — “Selector” showcase.
    - [ ] [`SelectorBottomSheet`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Selector/SelectorBottomSheet.doc.mjs) — “Selector — Bottom Sheet”: single selection in a compact-touch sheet.
    - [ ] [`SelectorClearable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Selector/SelectorClearable.doc.mjs) — “Selector — Clearable”: reset selected value.
    - [ ] [`SelectorGhostToolbar`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Selector/SelectorGhostToolbar.doc.mjs) — “Selector — Ghost Toolbar”: ghost trigger beside ghost buttons.
    - [ ] [`SelectorOptionDescriptions`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Selector/SelectorOptionDescriptions.doc.mjs) — “Selector — Option descriptions”: two-line options and growing trigger versus fixed InputGroup row.
    - [ ] [`SelectorWithSections`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Selector/SelectorWithSections.doc.mjs) — “Selector — Grouped Sections”.
    - [ ] [`SelectorWithStatus`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Selector/SelectorWithStatus.doc.mjs) — “Selector — Validation States”: error, warning, success.
    - [ ] [`useComboboxHookUsage`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Selector/useComboboxHookUsage.doc.mjs) — “useCombobox — Assign To Field”: filtered teammate Item rows, arrows, Enter, Escape, focus retained in TextInput.

- [ ] **[Slider](https://astryx.atmeta.com/components/Slider)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Slider/Slider.doc.mjs).

  - **Modes, choices, and defaults**
    - [ ] Controlled `value`: number (single thumb) or `[number, number]` (range); `onChange` during drag, `onChangeEnd` at drag completion. No defaultValue API documented.
    - [ ] `orientation`: `horizontal` (default), `vertical`; `valueDisplay`: `tooltip` (default), `text`, `none`.
    - [ ] Numeric min/max/step default 0/100/1; `minStepsBetweenThumbs` default 0 prevents range overlap when configured; optional marks with/without labels, filled/unfilled by position.
    - [ ] `isDisabled`, `isOptional`, `isRequired`, `isLabelHidden` default false; status `warning`, `error`, `success`, with/without message.

  - **Content, input, and accessibility**
    - [ ] Required label, optional description and labelTooltip; formatted display also supplies `aria-valuetext`; hidden form input (two entries in range mode).
    - [ ] Disabled with `disabledMessage` remains focusable via aria-disabled; value changes blocked, hover/focus explains reason.
    - [ ] Touch: thumb-only drag through enlarged target, follows travel without jumping, coasts before onChangeEnd; track swipe scrolls page, track/mark tap sets value on lift with spring.
    - [ ] Mouse thumb drag begins beyond 10px movement, no lift/coast; primary track press moves value. Pen claims along track and ends without coast. Wheel/trackpad scrolling is not captured.
    - [ ] Keyboard interaction surface; thumb contrast at least 3:1 against track/surface in rest, hover and whole pressed drag.

  - **Named templates**
    - [ ] [`SliderShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Slider/SliderShowcase.doc.mjs) — “Slider”: single value showcase.
    - [ ] [`SliderFormattedValue`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Slider/SliderFormattedValue.doc.mjs) — “Slider — Formatted Value”: Fahrenheit formatting.
    - [ ] [`SliderRangeSlider`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Slider/SliderRangeSlider.doc.mjs) — “Slider — Range”: bounds selection.
    - [ ] [`SliderWithMarks`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Slider/SliderWithMarks.doc.mjs) — “Slider — With Marks”: labeled ticks.
    - [ ] [`SliderWithStatus`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Slider/SliderWithStatus.doc.mjs) — “Slider — Validation States”: error, warning, success.

- [ ] **[Switch](https://astryx.atmeta.com/components/Switch)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Switch/Switch.doc.mjs).

  - **Choices, state, and async**
    - [ ] `size`: `sm` (32×20px), `md` (40×24px, default); `labelPosition`: `start`, `end` (default); `labelSpacing`: `hug` (default), `spread`.
    - [ ] Controlled boolean `value`: off/on with onChange; changeAction runs afterward, optimistic state and loading spinner until Promise resolves.
    - [ ] `isLoading`, `isLabelHidden`, `isDisabled`, `isOptional`, `isRequired` default false; optional/required mutually exclusive.
    - [ ] Status `warning`, `error`, `success`, with/without message; error sets aria-invalid.

  - **Content and input behavior**
    - [ ] Required label; description, labelIcon, labelTooltip, field width; ref/focus/blur belong to input. htmlName checkbox submits `on` while on.
    - [ ] DisabledMessage exposes reason on hover/focus, keeps switch focusable with aria-disabled, blocks toggling.
    - [ ] Touch press lifts thumb, drag flips, release commits, tap toggles via click; thumb remains circular/same size. Coarse-pointer target grows toward 44×44px, bounded halfway to neighbors.
    - [ ] Reduced motion cross-fades on touch and changes immediately with mouse; on/off track and thumb contrast 3:1 at rest/hover/pressed.

  - **Named templates**
    - [ ] [`SwitchShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Switch/SwitchShowcase.doc.mjs) — “Switch”: notifications setting.
    - [ ] [`SwitchDisabled`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Switch/SwitchDisabled.doc.mjs) — “Switch — Disabled”: gated feature with description.
    - [ ] [`SwitchSettingsPanel`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Switch/SwitchSettingsPanel.doc.mjs) — “Switch — Settings Panel”: spread spacing in card.
    - [ ] [`SwitchWithDescription`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Switch/SwitchWithDescription.doc.mjs) — “Switch — With Description”.
    - [ ] [`SwitchWithStatus`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Switch/SwitchWithStatus.doc.mjs) — “Switch — With Status”: error/warning/success.

- [ ] **[Text Area](https://astryx.atmeta.com/components/TextArea)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/TextArea/TextArea.doc.mjs).

  - **Choices and states**
    - [ ] `size`: `sm`, `md` (default), `lg`; rows default 3 controls height rather than size.
    - [ ] Status `warning`, `error`, `success`, with/without message; `statusVariant`: `attached` (default), `detached`, `tooltip`.
    - [ ] `isLabelHidden`, `isOptional`, `isRequired`, `isDisabled`, `isReadOnly`, `isLoading`, `hasAutoFocus`: false (default), true; optional/required mutually exclusive. `hasSpellCheck`: true (default), false; `hasAutoCorrect`: browser default when unset, explicit true/false.
    - [ ] Controlled value/onChange; changeAction after onChange inside transition supports optimistic updates. Ref/focus/blur/paste reach textarea; htmlName submits value.
    - [ ] Read-only retains undimmed submitted value, disabled wins. DisabledMessage keeps disabled field focusable/aria-disabled and read-only, with hover/focus reason.

  - **Keyboard, limits, and content**
    - [ ] `enterKeyHint`: `enter`, `done`, `go`, `next`, `previous`, `search`, `send`; label only, not handler behavior. Keep Return as line break unless own handler implements another action.
    - [ ] `inputMode`: `none`, `text`, `tel`, `url`, `email`, `numeric`, `decimal`, `search`; keyboard request, not accepted-value validation.
    - [ ] `autoCapitalize`: `off`, `none`, `on`, `sentences`, `words`, `characters`; unset follows browser (iOS sentences); off/none never capitalize, on/sentences per sentence. autoComplete forwards unchanged.
    - [ ] Required accessible label, optional description/placeholder/labelTooltip, leading icon or caller start content, spinner, status icon/message.
    - [ ] `maxLength` soft grapheme counter: emoji/flag count one; over-limit red plus warning icon and spoken remaining/over-limit count, no native enforcement. Validate with exported characterCount.
    - [ ] `htmlMaxLength` separate native UTF-16 cap, applies before hydration, no counter coupling or truncation of controlled values; validate server-side too.

  - **Named templates**
    - [ ] [`TextAreaShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TextArea/TextAreaShowcase.doc.mjs) — “TextArea” / “Text Area”: placeholder.
    - [ ] [`TextAreaCharacterCount`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TextArea/TextAreaCharacterCount.doc.mjs) — “TextArea — Character Count”: under/over soft limit.
    - [ ] [`TextAreaStates`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TextArea/TextAreaStates.doc.mjs) — “TextArea — States”: required, disabled, loading.
    - [ ] [`TextAreaValidation`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TextArea/TextAreaValidation.doc.mjs) — “TextArea — Validation”: error/warning/success messages, error without message.
    - [ ] [`TextAreaWithIcon`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TextArea/TextAreaWithIcon.doc.mjs) — “TextArea — Icon”: chat/compose hints.

- [ ] **[Text Input](https://astryx.atmeta.com/components/TextInput)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/TextInput/TextInput.doc.mjs).

  - **Choices and state**
    - [ ] `type`: `text` (default), `password`, `email`, `search`, `tel`, `url`; `size`: `sm`, `md` (default), `lg`.
    - [ ] Status `error`, `warning`, `success`, with/without message; error aria-invalid. `statusVariant`: `attached` (default), `detached`, `tooltip`.
    - [ ] `isLabelHidden`, `isOptional`, `isRequired`, `isDisabled`, `isReadOnly`, `isLoading`, `hasClear`, `hasAutoFocus`: false (default), true; optional/required mutually exclusive.
    - [ ] `hasAutoCorrect` and `hasSpellCheck`: browser default when unset, explicit true/false.
    - [ ] Controlled value/onChange; changeAction after non-prevented onChange provides optimistic state and pending spinner. Loading also sets aria-busy.
    - [ ] Read-only full-opacity submitted value, disabled precedence; disabledMessage keeps field focusable/aria-disabled/read-only and explains on hover/focus.

  - **Input and content behavior**
    - [ ] `enterKeyHint`: `enter`, `done`, `go`, `next`, `previous`, `search`, `send`; `inputMode`: `none`, `text`, `tel`, `url`, `email`, `numeric`, `decimal`, `search`.
    - [ ] `autoCapitalize`: `off`, `none`, `on`, `sentences`, `words`, `characters`; browser default when unset (iOS sentences except URL). Keyboard hints/types have platform-specific return-key presentation, not new validation modes.
    - [ ] IME-safe onEnter does not fire for conversion commit; ordinary onKeyDown supported. autoComplete passes through for AutoFill without changing controlled value.
    - [ ] Required accessible label; optional description, placeholder, labelTooltip, start icon, status icon, spinner. Clear resets value and input focus; htmlName for submission.
    - [ ] Width aligns entire field; no-width field shrinks in row, wrapping HStack supports narrow filter bars.

  - **Named templates**
    - [ ] [`TextInputShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TextInput/TextInputShowcase.doc.mjs) — “TextInput” / “Text Input”.
    - [ ] [`TextInputIcon`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TextInput/TextInputIcon.doc.mjs) — “TextInput — Icon”: mail/lock/user hints.
    - [ ] [`TextInputSearch`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TextInput/TextInputSearch.doc.mjs) — “TextInput — Search”: hidden label, search icon, clear.
    - [ ] [`TextInputSizes`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TextInput/TextInputSizes.doc.mjs) — “TextInput — Sizes”: sm/md/lg.
    - [ ] [`TextInputStates`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TextInput/TextInputStates.doc.mjs) — “TextInput — States”: error/warning/success.
    - [ ] [`TextInputStatusVariant`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TextInput/TextInputStatusVariant.doc.mjs) — “TextInput — Status variant”: attached/detached messages.
    - [ ] [`TextInputTypes`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TextInput/TextInputTypes.doc.mjs) — “TextInput — Types”: text/password/email; tooltip, required/optional, description, disabled/loading.

- [ ] **[Time Input](https://astryx.atmeta.com/components/TimeInput)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/TimeInput/TimeInput.doc.mjs).

  - **Presentation, format, and defaults**
    - [ ] `presentation`: `text-input`, `popover` (same typed surface, no distinct popover), `bottom-sheet`, `native`, `adaptive-bottom-sheet`, `adaptive-native` (default).
    - [ ] Adaptive-native: typed on fine pointer, OS time picker on coarse, except seconds/custom increment retain typed field; native always OS without Astryx fallback. Adaptive-bottom-sheet: typed on fine, sheet wheels on coarse; bottom-sheet always wheels.
    - [ ] Deprecated `nativePicker`: `touch` (default → adaptive-native), `always` (native), `never` (text-input); explicit presentation wins.
    - [ ] `hourFormat`: `12h`, `24h`, or locale hour cycle (default). Provider locale preference tag controls user preference; native open picker follows device locale even when closed value is fixed-format.
    - [ ] `size`: `sm`, `md` (default), `lg`; `hasSeconds`, `hasClear`: false (default), true. Controlled ISO value HH:MM / HH:MM:SS; clearing reports undefined.
    - [ ] `increment` default 1 whole minute; arrows adjust by configured integer minutes, fractional increment leaves value unchanged; optional min/max reject out-of-range values, including native commit.

  - **Field state, validation, and accessibility**
    - [ ] `isLabelHidden`, `isOptional`, `isRequired`, `isDisabled`, `isLoading` default false; required/optional mutually exclusive; disabled remains focusable/aria-disabled, disabledMessage explains on hover/focus.
    - [ ] onChange then transition-wrapped changeAction for optimistic pending spinner; clock icon, clear, status icon and spinner content. InputGroup can supply prefix/suffix/timezone marker.
    - [ ] Status `warning`, `error`, `success`, optional message; `statusVariant`: `attached` (default), `detached`, `tooltip`.
    - [ ] `entryErrorBehavior`: `complete` (nearest FormLayout default else this fallback), `blur-or-enter`; own prop wins provider. Complete rejects complete out-of-range time immediately, other wrong text on Enter/blur; blur-or-enter defers message, invalid styling/aria-invalid, announcement and callback together.
    - [ ] Entry errors: `unreadable`, `outOfRange`; rejected text retained, accepted/empty edit clears immediately; under blur-or-enter a still-wrong edit retains shown error until next commit. 12h completeness includes day period unless hour 0 or 13–23.
    - [ ] onEntryErrorChange reports changed rejection and null on clearing/unmount; constraints/value/disable changes can update it, duplicate reports suppressed. Native/sheet do not report typed errors; native bounds rejection still has own error presentation.
    - [ ] Caller error status message can replace own rejection message, but rejection still announced; warning/success never override rejection.
    - [ ] Placeholder default `Select a time`, replaced by format hint while focused/empty. Required accessible label, optional description/labelTooltip. Base ref/id/aria/data/events target focused input; DOM handlers run after built-in handling and cannot cancel it.

  - **Named templates**
    - [ ] [`TimeInputShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TimeInput/TimeInputShowcase.doc.mjs) — “TimeInput” / “Time Input”: touch OS picker, fine-pointer typing.
    - [ ] [`TimeInputConstrained`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TimeInput/TimeInputConstrained.doc.mjs) — “TimeInput — Constrained”: min/max windows.
    - [ ] [`TimeInputFormats`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TimeInput/TimeInputFormats.doc.mjs) — “TimeInput — Formats”: 12h, 24h, seconds.
    - [ ] [`TimeInputIncrement`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TimeInput/TimeInputIncrement.doc.mjs) — “TimeInput — Increment”: custom arrow cadence.
    - [ ] [`TimeInputStates`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TimeInput/TimeInputStates.doc.mjs) — “TimeInput — States”: default, disabled, error, warning, success.

- [ ] **[Tokenizer](https://astryx.atmeta.com/components/Tokenizer)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Tokenizer/Tokenizer.doc.mjs).

  - **Choices and selection modes**
    - [ ] `size`: `sm`, `md` (default), `lg`; InputGroup owns size when grouped. Status `warning`, `error`, `success`; `statusVariant`: `attached` (default), `detached`, `tooltip` (info-tip control).
    - [ ] `presentation`: `popover`, `bottom-sheet`, `adaptive` (default; compact-touch sheet otherwise popover; SSR/hydration starts popover).
    - [ ] `tokenOverflowBehavior`: `none` (default), `unfocusedInline` (expands in flow), `unfocusedLayer` (overlay without layout shift), inline field only.
    - [ ] Controlled item-array value/onChange; changes `add`, `create`, `remove`, `reorder`, including affected item. Hidden input per selected item ID via htmlName.
    - [ ] `hasClear`, `hasEntriesOnFocus`, `hasAutoFocus`, `hasCreate`, `isDisabled`, `isLabelHidden`, `isRequired`, `isOptional`: false (default), true.

  - **Search, slots, and accessibility**
    - [ ] SearchSource search/bootstrap; synchronous/local versus async debounced search (150ms default; 0 for immediate local/context.update source). Updates retain shown rows first and append new matches; returned list final.
    - [ ] `minQueryLength` default 1; below threshold no search/menu, except hasCreate can still offer typed-text Create. `maxMenuItems` default 10 plus optional create row; maxEntries hides input at cap. Menu width optional pixels, never narrower than anchor.
    - [ ] EmptySearchText default `No results found`, ReactNode/link/create content; announce DOM text politely excluding aria-hidden. Null/undefined fall back, empty string suppresses text.
    - [ ] Default Token versus renderToken(item,onRemove); default TypeaheadItem versus renderItem; leading icon/name/component/arbitrary node; trailing endContent controls/count; placeholder only with no tokens. Label/description/labelTooltip and validation content.
    - [ ] Add/remove (including empty-input Backspace) announced politely; disabled input remains tab-stop/aria-disabled/read-only, no open/pick. UnfocusedLayer without disabledMessage uses native disabled/out-of-tab-order; disabledMessage restores focusable reason tooltip behavior.
    - [ ] Inline handle focus/blur/open/close; onOpenChange reports state, not controlled open API. Bootstrap list can remain focused/open after pick while next option exists.
    - [ ] `menu.role`: `listbox` (default; optional wrapper render with listbox/query/isBusy/isEmptySearch/close), `dialog` (label-named focus-taking custom render). Actionable listbox entries remain results for arrows; dialog Escape/outside/close restores prior focus. Reopen from onChange when switching menu role.

  - **Sheet-specific contract**
    - [ ] Field trigger opens tall modal sheet: search, full-width stable checkbox rows, optional Create row, pinned confirmed Clear all and Save. Draft edits commit one onChange at Save; changed draft close by swipe/scrim/Escape confirms discard.
    - [ ] Field still wraps renderToken/endContent; their own controls do not open sheet. Trigger receives ref/id/aria/data; ref should accept input or button despite released input-only prop type.
    - [ ] Sheet ignores/warns for renderItem, hasClear, hasEntriesOnFocus, menuWidth, menu, onOpenChange, tokenOverflowBehavior, hasAutoFocus; open/close handle ignored there. Force popover to retain inline-only APIs.

  - **Named templates**
    - [ ] [`TokenizerShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Tokenizer/TokenizerShowcase.doc.mjs) — “Tokenizer”: preset tags/source.
    - [ ] [`TokenizerClear`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Tokenizer/TokenizerClear.doc.mjs) — “Tokenizer — Clear”: inline clear-all versus sheet Clear all.
    - [ ] [`TokenizerCreatable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Tokenizer/TokenizerCreatable.doc.mjs) — “Tokenizer — Creatable”: free text and create-or-search.
    - [ ] [`TokenizerEndContent`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Tokenizer/TokenizerEndContent.doc.mjs) — “Tokenizer — End Content”: inline action.
    - [ ] [`TokenizerIcon`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Tokenizer/TokenizerIcon.doc.mjs) — “Tokenizer — Icon”: search hint.
    - [ ] [`TokenizerMaxEntries`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Tokenizer/TokenizerMaxEntries.doc.mjs) — “Tokenizer — Max Entries”: cap hides input.
    - [ ] [`TokenizerOverflow`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Tokenizer/TokenizerOverflow.doc.mjs) — “Tokenizer — Overflow”: unfocused inline/layer.
    - [ ] [`TokenizerStates`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Tokenizer/TokenizerStates.doc.mjs) — “Tokenizer — States”: disabled/error/warning/success.

- [ ] **[Typeahead](https://astryx.atmeta.com/components/Typeahead)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Typeahead/Typeahead.doc.mjs).

  - **Choices and state**
    - [ ] Single controlled selected item/null with onChange; no multi-select (Tokenizer owns that pattern).
    - [ ] `size`: `sm`, `md` (default), `lg`; status `warning`, `error`, `success`; `statusVariant`: `attached` (default), `detached`, `tooltip` (info-tip button).
    - [ ] `hasClear`: true (default), false; `hasEntriesOnFocus`, `isDisabled`, `isLabelHidden`, `isRequired`, `isOptional`, `hasAutoFocus`: false (default), true.
    - [ ] Disabled input remains focusable aria-disabled/read-only, opens/picks nothing; disabledMessage describes it through hover/focus tooltip.

  - **Search, slots, and menu modes**
    - [ ] SearchSource search/bootstrap; default debounce 150ms, minimum query 1, maximum menu 10. Below threshold no search/menu; configurable query callback and bootstrap-on-focus.
    - [ ] Async source may context.update local matches first, then final remote list retaining shown rows before new ones; debounce 0 for immediate local matches and source-owned server delay.
    - [ ] Required accessible label, description/labelTooltip/placeholder; leading semantic icon/component/arbitrary node; selected token outside edit mode, pending spinner and clear button.
    - [ ] Default TypeaheadItem versus renderItem or item.element inside stable result wrapper; grouped headings, selected trailing check, highlighted and disabled rows.
    - [ ] EmptySearchText default `No results found`; ReactNode empty state/link/create content politely announced from DOM text excluding aria-hidden. Null falls back, empty string renders/announces nothing.
    - [ ] `menu.role`: `listbox` (default, wrapper render receives listbox/query/isBusy/isEmptySearch/close), `dialog` (label-named, focus-taking render query/close). Listbox actionable choices remain results for arrow access; dialog Escape/outside/close restores original focus.
    - [ ] handleRef open/close controls internal menu; onOpenChange reports only, no controlled open state. Open dialog immediately versus show/load listbox results; switching role can keep open by open() from onChange.
    - [ ] Standalone Field shell versus InputGroup prefix/suffix row; full-field width aligns label/control/status.

  - **Named templates**
    - [ ] [`TypeaheadShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Typeahead/TypeaheadShowcase.doc.mjs) — “Typeahead”.
    - [ ] [`TypeaheadLimitedResults`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Typeahead/TypeaheadLimitedResults.doc.mjs) — “Typeahead — Limited Results”: cap of three (example data, not fixed style).
    - [ ] [`TypeaheadSearchField`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Typeahead/TypeaheadSearchField.doc.mjs) — “Typeahead — Search Field”: icon and focus suggestions.
    - [ ] [`TypeaheadWithHelperText`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Typeahead/TypeaheadWithHelperText.doc.mjs) — “Typeahead — With Helper Text”.
    - [ ] [`TypeaheadWithValidation`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Typeahead/TypeaheadWithValidation.doc.mjs) — “Typeahead — With Validation”: error message.

- [ ] **[Typeahead Item](https://astryx.atmeta.com/components/TypeaheadItem)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Typeahead/TypeaheadItem.doc.mjs).

  - **Content and state**
    - [ ] No finite visual variants documented. Required SearchableItem, optional icon/avatar and description; `isDisabled`: false (default), true (visual disabled state).
    - [ ] Default renderer versus custom renderItem reuse. `group` prop has no effect; item.auxiliaryData.group groups source results in Typeahead/BaseTypeahead instead.

  - **Named templates**
    - [ ] [`TypeaheadItemBasic`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TypeaheadItem/TypeaheadItemBasic.doc.mjs) — “TypeaheadItem — Basic”: secondary result descriptions.
    - [ ] [`TypeaheadItemShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TypeaheadItem/TypeaheadItemShowcase.doc.mjs) — “TypeaheadItem” / “Typeahead Item”: avatars/descriptions.

- [ ] **[Rich Text Editor](https://astryx.atmeta.com/components/RichTextEditor)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/richtext/src/RichTextEditor.doc.mjs). No named inline-example array or supplied template metadata; documented recipes below are usage prose, not invented template names.

  - **Field modes and extension slots**
    - [ ] size `sm`, `md` (default), `lg`; status type `warning`, `error`, `success`; statusVariant `attached` (default), `detached`, `tooltip` (focusable in-editor icon).
    - [ ] isLabelHidden, isOptional, isRequired, isReadOnly, isDisabled, hasAutoFocus: each false by default / true; optional/required mutually exclusive; required sets aria-required, error sets aria-invalid.
    - [ ] Read-only full opacity, Tab-reachable, announced; disabled wins, dimmed/out of Tab order, links inert/not tab stops; both update after mount.
    - [ ] Uncontrolled defaultValue serialized Lexical JSON read once on mount; onChange reports EditorState/editor for persistence, not controlled value prop.
    - [ ] Required label, description/labelTooltip/placeholder; minHeight default 4.5rem, namespace astryx-editor, arbitrary width/maxLength. Character counter signals excess without enforcing limit.
    - [ ] toolbar edge-to-edge before padded edit surface; plugins in composer; custom nodes beyond Heading/Quote/List/Link/Code; shared markdownExtensions/codeLanguages in editor/view.
    - [ ] hasMarkdownShortcuts true (default) / false; transformers custom array replaces default thematic rules, standard Lexical transforms, hard breaks, GFM tables (not additive).

  - **Keyboard, serialization, and rendering behavior**
    - [ ] Tab indents; Escape then Tab leaves; translated tabEscapeHint defaults Press Escape then Tab to move focus out of the editor., empty string omits hint.
    - [ ] Markdown shortcuts at paragraph/list start switch bullet/number/task lists, heading, quote; top-level list extraction preserves remaining list, one undo step back to typed marker.
    - [ ] GFM tables: header first row, delimiter alignment, fill short rows, own horizontal scroll; arrows change cells, Tab keeps editor indentation meaning; paste fills cells and expands grid, mixed-content paste becomes cell text.
    - [ ] Code Tab/Shift+Tab affects touched lines by existing tabs or smallest 2–8-space indent, else two spaces; preserves selection and single undo step.
    - [ ] Code-start Backspace converts block to multiline paragraph without merging prior heading; code paste inserts plain text only, no rich blocks.
    - [ ] Cmd/Ctrl+A in code/cell selects local text first, second press whole document; elsewhere whole document immediately.
    - [ ] Thematic breaks import ---/***/___ including spacing, export ---; dash line under paragraph stays paragraph-related; rule selection by click/arrows, Backspace removes.
    - [ ] Ref focus/clear/getEditorState/getMarkdown/getHTML/getEditor after mount; focus/clear no-op read-only/disabled, clear leaves empty paragraph.
    - [ ] Headless markdownToEditorStateJSON/editorStateJSONToMarkdown server subpath; unchanged blocks round-trip byte-for-byte, changed/new blocks canonicalized; same nodes/transformers across paths.
    - [ ] createRichTextExtension adopts syntax plugin, refuses transform-bearing/duplicate plugin; absent plugin shows source; supplied extension renders like core Markdown, atomic caret/delete/cut/copy/paste and exact-source export.
    - [ ] Toolbar recipe: undo/redo, block Selector, bold/italic/underline/strike/code/link controls, horizontal scroll not More menu, endContent extension, 4px gutter.
    - [ ] Link button/Cmd-Ctrl+K opens nonmodal URL popover; Enter applies, Remove/Cancel actions, Escape restores selection/focus; invalid URL keeps field focus/error; only http/https/mailto/tel, default new-tab safe rel.
    - [ ] AutoLinkPlugin in plugins links typed/pasted URLs/emails except code, with default safe new-tab metadata; richtext:* glyph keys themeable, undo/redo mirror RTL.
    - [ ] In-place read/edit recipe puts editor border/inset outside reading width via negative logical margins; assumes no status (status icon needs extra space); form editor retains normal aligned width.
    - [ ] Optional Lexical peer packages and same nodes/extensions are required for compatible author/read/serialize paths; no dependency installation performed for this inventory.

- [ ] **[Rich Text Editor Auto-Link Plugin](https://astryx.atmeta.com/components/RichTextEditorAutoLinkPlugin)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/richtext/src/RichTextEditorAutoLinkPlugin.doc.mjs). No named inline examples or templates.

  - **Actual plugin behavior**
    - [ ] No visual variants. Must render in RichTextEditor's plugins slot/composer; uses registered AutoLinkNode.
    - [ ] Default DEFAULT_LINK_MATCHERS recognizes typed/pasted URLs and emails, excludes code blocks; caller-supplied matchers may add patterns via createLinkMatcherWithRegExp.
    - [ ] Created links default new tab with target=_blank and rel=noopener noreferrer stored in node.
    - [ ] onChange(url,prevUrl) reports create/update/removal, null new URL for removal; recipe `plugins={<RichTextEditorAutoLinkPlugin />}` is documented usage, not an invented named example.

- [ ] **[Rich Text Editor Toolbar](https://astryx.atmeta.com/components/RichTextEditorToolbar)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/richtext/src/RichTextEditorToolbar.doc.mjs). No named inline examples or templates.

  - **Controls and modes**
    - [ ] size `sm` (default), `md`, `lg`; headingLevels ordered subset of `h1`, `h2`, `h3`, default all three.
    - [ ] hasLink true (default) / false; hasNewTabLinks true (default) / false (same tab); default translated label Text formatting, use label not aria-label.
    - [ ] Undo/redo; block paragraph/headings/lists/quotes; inline bold/italic/underline/strikethrough/code; link action; endContent extra controls after divider.
    - [ ] Must live in editor toolbar slot/composer; uses Astryx Toolbar/Selector/ToggleButton with horizontal overflow rather than hiding controls.
    - [ ] Built-in nonmodal link popover versus synchronous promptForUrl override for non-link selection; return null/empty to cancel, collapsed selection inserts URL as text, noncollapsed links text.
    - [ ] Only http/https/mailto/tel links; Enter applies, Remove existing link, Escape/Cancel restores editor selection/focus; safe target/rel stored in nodes when new-tab mode.
    - [ ] Root ref/style versus role-toolbar host data/ARIA/handlers; richtext:* theme glyph overrides (e.g. richtext:bold), not a finite color/visual-variant API.

## Layout

- [ ] **[App Shell](https://astryx.atmeta.com/components/AppShell)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/AppShell/AppShell.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/AppShell).

  - **Layout choices and slots**
    - [ ] `variant`: `wash`, `surface`, `section`, `elevated` (default); distinct navigation/content background, divider, and raised-content treatments.
    - [ ] `height`: `fill` (default, 100dvh and internal scrolling), `auto` (content growth, sticky nav).
    - [ ] `contentPadding`: `0` (default), `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`.
    - [ ] Slots: main `children`, `topNav`, `sideNav`, `banner`, mobile navigation; side-nav collapse configured on SideNav, not the shell.
    - [ ] `mobileNav`: `false` to disable, config object for automatic behavior, or custom ReactNode drawer.
    - [ ] Mobile config: `hasToggle`, controlled `isOpen` / `onOpenChange`, custom `content`, `defaultIsMobile`; breakpoint `sm`, `md` (default), `lg`, `xl`, `2xl`, `none`. Equality selects wider layout; `none` is always non-mobile and ignores `defaultIsMobile`.
  - **Accessibility and responsive behavior**
    - [ ] First-focusable skip link moves focus to main; single main and banner landmarks; separately named TopNav and SideNav landmarks; caller supplies page h1.
    - [ ] Overflowing main is keyboard reachable, with first-native-link/button Tab delegation; mobile drawer traps focus and returns it to toggle.
    - [ ] Phone safe-area treatment with `viewport-fit=cover`; header sides/top and content end inset, but landscape content/inline SideNav sides need consumer padding.
  - **Templates**
    - [ ] **AppShell — Content Only** (`AppShellContentOnly.doc.mjs`): no navigation, full-bleed/auth/embed.
    - [ ] **useAppShellMobile — Custom Mobile Trigger** (`AppShellMobileHookUsage.doc.mjs`): context-consuming custom trigger.
    - [ ] **AppShell** / **App Shell** (`AppShellShowcase.doc.mjs`): basic padded shell.
    - [ ] **AppShell — Side Nav Only** (`AppShellSideNavOnly.doc.mjs`): SideNav header carries identity.
    - [ ] **AppShell — Top Nav Only** (`AppShellTopNavOnly.doc.mjs`): no side navigation.
    - [ ] **AppShell — Top Nav with Side Nav** (`AppShellTopNavWithSideNav.doc.mjs`): identity plus page navigation.
    - [ ] **AppShell — With Banner** (`AppShellWithBanner.doc.mjs`): full navigation and dismissable info banner.

- [x] **[Aspect Ratio](https://astryx.atmeta.com/components/AspectRatio)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/AspectRatio/AspectRatio.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/AspectRatio).

  - Verified source-faithful baseline: [inventory, offline comparisons and remaining release/artwork boundaries](docs/aspect-ratio-port.md). Native demo framing does not complete Center/Grid/Layout/Text migrations.

  - **Shape, fitting, and content**
    - [x] `shape`: `rectangle` (default), `ellipse`; ratio 1 makes ellipse circular.
    - [x] `fit`: `cover` (crop), `contain` (letterbox), `center` (natural-size centered); omitted means child owns sizing.
    - [x] Required numeric `ratio` is open-ended, not a preset enum; documented examples use 1:1, 4:3, 16:9.
    - [x] One child for fitted media; single wrapper for overlays/captions. Definite ancestor width required; shrink-to-fit parents collapse; height-only limits break ratio unless paired with auto width.
    - [x] Responsive ratio via StyleX/default branch or unlayered CSS; child owns alt text and semantics, no role/name added by container.
  - **Templates**
    - [x] **AspectRatio — Circle Image** (`AspectRatioCircleImage.doc.mjs`): ellipse, ratio 1.
    - [x] **AspectRatio — Image Gallery** (`AspectRatioImageGallery.doc.mjs`): 4:3 image grid.
    - [x] **AspectRatio** / **Aspect Ratio** (`AspectRatioShowcase.doc.mjs`): equal-height 1:1, 4:3, 16:9 image containers.
    - [x] **AspectRatio — Square Image** (`AspectRatioSquareImage.doc.mjs`): square media.
    - [x] **AspectRatio — 16:9 Widescreen Image** (`AspectRatioWidescreen.doc.mjs`).
    - [x] **AspectRatio — Loading Skeleton** (`AspectRatioWithSkeleton.doc.mjs`): reserved media space while loading.

- [x] **[Divider](https://astryx.atmeta.com/components/Divider)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Divider/Divider.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Divider).

  - **Choices and content**
    - [x] `orientation`: `horizontal` (default), `vertical`; `variant`: `subtle` (default), `strong`. Theme-added names remain open. **[Original custom-paint gap/OQ2](docs/divider-port.md#authoritative-contract-versus-source-gaps) remains explicit: unmatched rules are unpainted, with no invented fallback/part API. [Source audit](docs/divider-port.md#contrast-and-remaining-verification-boundaries): 16/16 lines below conditional essential-boundary 3:1; 0/16 labels below 4.5:1. Original paint retained, not claimed compliant for essential separators.**
    - [x] `isFullBleed=false` default; true compensates container padding with negative margins. Both logical axes, asymmetric/absent/changed inset, LTR/RTL and narrow native Card frames verified.
    - [x] Optional centered ReactNode label; zero preserved, null/booleans/empty string omit label. No fixed label vocabulary. Stable named-separator references and explicit caller ARIA/name/role precedence follow authoritative FR2/FR6 rather than the documented source override bug.
  - **Templates**
    - [x] **Divider — Full Bleed** (`DividerFullBleed.doc.mjs`): edge-to-edge summary separation.
    - [x] **Divider — Variants** (`DividerShowcase.doc.mjs`): subtle/strong/labelled.
    - [x] **Divider — Variants** (`DividerVariants.doc.mjs`): separate card comparison, labelled alternative or.
    - [x] **Divider — Vertical** (`DividerVertical.doc.mjs`): metrics/toolbar group boundaries.

- [ ] **[Form Layout](https://astryx.atmeta.com/components/FormLayout)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/FormLayout/FormLayout.doc.mjs). Templates in `FormLayout/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Layout and inherited policy**
    - [ ] `direction`: `vertical` (default), `horizontal` (fractional grid tracks), `horizontal-labels` (logical-start labels, collapses vertically at viewport <=480px).
    - [ ] `defaultOptionality`: unset (per-field), `optional` (only required exceptions indicated), `required` (only optional exceptions indicated).
    - [ ] Required default supplies aria-required, not native required. CheckboxInput, Switch, CheckboxList do not inherit required; checkbox/switch require their own isRequired.
    - [ ] `defaultEntryErrorBehavior`: `complete` (also unset default), `blur-or-enter`; controls rejected-entry feedback for date/time/number fields, never caller status or validation policy.
    - [ ] Blur-or-enter defers message, invalid state, styling, announcement, error report until commit; valid/empty edits clear immediately. NumberInput commits under either mode.
    - [ ] Field policy overrides form policy; nested FormLayout replaces inherited default (unset becomes complete); layers such as Dialog reset it.
    - [ ] Children: Astryx inputs or Field-wrapped custom controls; nested horizontal rows inside vertical forms; horizontal-labels must be outermost. Layout owns no state/submission; native form remains caller-owned.

  - **Named templates**
    - [ ] `FormLayoutHorizontal` — **FormLayout — Horizontal**: paired fields side by side.
    - [ ] `FormLayoutHorizontalLabels` — **FormLayout — Settings Form**: labels beside inputs.
    - [ ] `FormLayoutMixedControls` — **FormLayout — Mixed Controls**: text input, selector, checkboxes.
    - [ ] `FormLayoutNested` — **FormLayout — Nested Address Form**: mixed vertical/horizontal grouping.
    - [ ] `FormLayoutShowcase` — **FormLayout**: vertical input layout.

- [ ] **[Grid](https://astryx.atmeta.com/components/Grid)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Grid/Grid.doc.mjs). Templates in `Grid/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Tracks, spacing, and alignment**
    - [ ] `columns`: fixed numeric N (never reflows) versus responsive `{minWidth, max?, repeat?}`; repeat `fill` (default, preserves empty tracks) or `fit` (collapses them, stretches items).
    - [ ] `gap`, `rowGap`, `columnGap`: each `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`; axis values override gap.
    - [ ] `align` and `justify`: each `start`, `center`, `end`, `stretch`; unset leaves browser `normal`, stretching naturally unsized items.
    - [ ] Open-ended width/height/maxWidth/minHeight; numeric rowHeight with GridSpan rows enables masonry-style layout.
    - [ ] Child content versus explicit GridSpan; pair minWidth with max for narrow-container reflow; CSS-variable track templates preserve xstyle/media overrides.

  - **Named templates**
    - [ ] `GridDashboardLayout` — **Grid — Dashboard Layout**: mixed-size widgets, full-width summary.
    - [ ] `GridGalleryExample` — **Grid — Card Gallery**: responsive consistent-width cards.
    - [ ] `GridResponsiveAutoFit` — **Grid — Responsive Auto-Fit**: cards stretch into remaining room, resizable demonstration.
    - [ ] `GridShowcase` — **Grid**: Card/Grid showcase; no further metadata description.
    - [ ] `GridWithGridSpan` — **Grid — Column Spanning**: featured multi-column/multi-row items.

- [ ] **[Layout](https://astryx.atmeta.com/components/Layout)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Layout/Layout.doc.mjs). Templates in `Layout/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Regions, sizing, and defaults**
    - [ ] Slots: `header`, `start`, `content`, `end`, `footer`; arbitrary nodes or LayoutHeader/LayoutPanel/LayoutContent/LayoutFooter wrappers.
    - [ ] `children` is content shorthand; explicit content wins. false/null/undefined hides a region and neighboring regions move to edge.
    - [ ] `height`: `fill` (default; needs height-constrained parent), `auto` (content growth).
    - [ ] `padding`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`. Own layout regions only; nested Layout does not inherit it, using enclosing container padding or spacing-4 default inset.
    - [ ] `defaultHasDividers`: true/false; unset inherits parent context and supplies only header/footer defaults, not panel divider policy.
    - [ ] `contentWidth` centers aligned region content, full-bleed dividers. No panel: scrollbar at outer edge; one panel: aligned frame with content extending open edge; two panels: width includes middle composition.
    - [ ] Percentage/intrinsic/bare-var arithmetic widths retain constrained composition; guaranteed length calc(var(...)) can use open-side arithmetic. Values such as 640/960 are recommendations, not finite widths.
    - [ ] Layout adds no role/name; caller gives region landmark role/label when appropriate. AppShell, not Layout, owns app navigation shell.

  - **Named inline example**
    - [ ] **Page with a details panel inside AppShell**: page header, complementary end panel, main content, default inner inset in full-bleed shell.

  - **Named templates**
    - [ ] `LayoutBasicCardLayout` — **Layout — Basic Card**: header/scroll content/action footer.
    - [ ] `LayoutContentOnlyLayout` — **Layout — Content Only**: minimal Card content, no header/footer.
    - [ ] `LayoutContentWidth` — **Layout — Content Width**: constrained centered content, full-bleed dividers.
    - [ ] `LayoutDualPanelLayout` — **Layout — Dual Panel**: folders, files, details.
    - [ ] `LayoutFullBleedContent` — **Layout — Full Bleed Content**: zero-padding table/image content.
    - [ ] `LayoutShowcase` — **Layout**: badge/button/list composition.
    - [ ] `LayoutSidebarLayout` — **Layout — Sidebar Navigation**: settings sidebar, header/content/footer.

- [ ] **[Resize Handle](https://astryx.atmeta.com/components/ResizeHandle)**

  - Sources: [Resizable family English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Resizable/Resizable.doc.mjs), [ResizeHandle.tsx](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Resizable/ResizeHandle.tsx), [resizeGesture.ts](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Resizable/resizeGesture.ts). No standalone doc or supplied templates.

  - **Direction and grip modes**
    - [ ] direction `horizontal` (default), `vertical`; must match region axis. Position `inline` (default flex-flow), `overlay` (absolute inside clipping panel), source-documented beyond family prop table.
    - [ ] isReversed false (default) / true for end/right/bottom panel; isDisabled false (default) / true; hasDivider false (default) / true 1px line, ignored overlay.
    - [ ] isAlwaysVisible true (default) / false hover/focus-only grip. Family prose calls grip invisible at rest, but prop table and implementation both default true; use that concrete default.
    - [ ] pillPlacement `start`, `end`, `center`, `auto` (default panel side, flips when collapsed); moves pill only, grab zone remains centered.
    - [ ] Custom children replace default grip; label defaults translated Resize handle; resizable connects useResizable region or props. Source permits absent region, but no resizing gesture exists then; family docs require meaningful connection.

  - **Resize and accessibility behavior**
    - [ ] Focusable role separator, axis orientation, resolved pixel current/min/max and collapsed text; nonfinite maximum handled by effective available basis for ARIA.
    - [ ] Pointer capture keeps drag over embedded frame; both grip and enlarged grab zone drag; short pane zones capped to half pane, collapsed pane keeps zone.
    - [ ] Arrow keys step 10px or Shift 50px; reversed/RTL change grow direction. With snaps, arrows move adjacent point (Shift two); collapsed pane grows only on grow arrow.
    - [ ] Home minimum; End configured finite maximum (no move for Infinity); Enter or double-click toggles collapse if region collapsible, restoring pre-collapse size where hook supports it.
    - [ ] Region exposes expanded/collapsed, snapping/bounded resize; collapsing adjacent panel transfers focus to handle and makes panel inert; cleanup interrupts in-flight gesture on unmount/lost region.

  - **Named family inline examples and source example**
    - [ ] **Structured fixed pixels**: pixel helper initial region size.
    - [ ] **One-time structured default**: percent initial size with pixel floor, handle using matching region.
    - [ ] **Any element, sized by the hook**: Card preview in Stack, labeled ResizeHandle, bounded region.
    - [ ] **Live percentage floor**: percentage minimum with pixel floor.
    - [ ] **Live percentage ceiling**: percentage maximum with pixel ceiling.
    - [ ] Source example: horizontal ResizeHandle with resizable region and divider.

- [ ] **[Scrollable Area](https://astryx.atmeta.com/components/ScrollableArea)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/ScrollableArea/ScrollableArea.doc.mjs). Templates in `ScrollableArea/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Axes and containment**
    - [ ] axis `inline`, `block` (default), `both`, mapped through writing-mode/direction; role `group` (default), `region` (explicit landmark).
    - [ ] overscroll `allow` (default ancestor chaining), `contain` (effective axes stop at edge).
    - [ ] stickyContainment `whenScrollable` (default, fitting content passes ownership outward), `always` (own sticky boundary even fitting).
    - [ ] isFullBleed false (default) / true lets viewport escape ancestor padding without changing content padding.
    - [ ] padding (default 0), paddingInline, paddingInlineStart, paddingInlineEnd, paddingBlock, paddingBlockStart, paddingBlockEnd: each `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`; edge wins axis wins all, publishes matching inset geometry.

  - **Content and accessibility behavior**
    - [ ] Required label names viewport only while requested axis effectively scrolls and enters Tab order; containment only on effective axes.
    - [ ] Native viewport plus observed normal-block content box, minimum 100% size; inline/both use max-content width, intentionally intrinsic overflow.
    - [ ] Arbitrary viewport width/height/maxWidth/minHeight, content children; use hook for existing direct flex/grid structure/percentage basis; preserve visible operable scroll affordance.

  - **Named templates**
    - [ ] `ScrollableAreaInlineRail` — **ScrollableArea — Inline Rail**: sideways cards, full-bleed viewport/aligned padding.
    - [ ] `ScrollableAreaOverscrollChaining` — **ScrollableArea — Overscroll**: allow versus contain.
    - [ ] `ScrollableAreaShowcase` — **ScrollableArea**: fixed-height file panel, heading scrolls with content.
    - [ ] `ScrollableAreaStickyGroupHeaders` — **ScrollableArea — Sticky Section Labels**: sticky labels pin inside viewport.
    - [ ] `ScrollableAreaStickyPassthrough` — **ScrollableArea — Sticky Passthrough**: fitting section passes sticky outward versus always boundary.
    - [ ] `ScrollableAreaTwoAxisBoard` — **ScrollableArea — Two-Axis Board**: oversized board, one viewport/two axes.

- [ ] **[Section](https://astryx.atmeta.com/components/Section)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Section/Section.doc.mjs). Templates in `Section/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Surfaces, borders, and content**
    - [ ] variant `section` (default), `transparent`, `muted`; page grouping, not discrete Card object.
    - [ ] dividers explicit sides `top`, `bottom`, `start`, `end`; optional independent sides, no invented combination matrix.
    - [ ] padding default 4; padding, paddingInline, paddingInlineStart, paddingInlineEnd, paddingBlock, paddingBlockStart, paddingBlockEnd each `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`; zero full-bleed, edge overrides axis overrides all.
    - [ ] Arbitrary width/height/maxWidth/minHeight and children; heading-plus-Stack, settings/form/sidebar regions; muted emphasizes particular area.

  - **Named inline example**
    - [ ] **Center a Section with a maximum width**: explicit auto marginInlineStart/marginInlineEnd in StyleX (shorthand may lose against compiled component margins).

  - **Named templates**
    - [ ] `SectionVariants` — **Section — Variants**: section/muted/transparent comparison.
    - [ ] `SectionWashHighlight` — **Section — Default with Wash**: default region plus full-width muted callout.
    - [ ] `SectionWithDividers` — **Section — With Dividers**: adjacent same-surface settings sections, bottom borders.

- [ ] **[Stack](https://astryx.atmeta.com/components/Stack)**

  - Sources: [English family API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Stack/Stack.doc.mjs), [original TSX](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Stack/Stack.tsx).

  - **Layout choices**
    - [ ] `direction`: `horizontal`, `vertical` (default); horizontal follows inline direction in LTR/RTL. HStack/VStack are configured wrappers, not additional direction strings.
    - [ ] Each of `gap`, `padding`, `paddingInline`, `paddingInlineStart`, `paddingInlineEnd`, `paddingBlock`, `paddingBlockStart`, `paddingBlockEnd`: numeric steps `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10` (not strings). Edge overrides axis, axis overrides all-side padding.
    - [ ] Main-axis alignment (`hAlign` horizontal, `vAlign` vertical, or `justify`): `start`, `center`, `end`, `between`, `around`, `evenly`.
    - [ ] Cross-axis alignment (`vAlign` horizontal, `hAlign` vertical, or `align`): `start`, `center`, `end`, `stretch`. Family docs expose combined hAlign/vAlign union; TSX documents these axis-specific valid choices. Explicit axis alignment wins its alias.
    - [ ] `wrap`: `nowrap` (default), `wrap`, `wrap-reverse`; `isScrollable`: false (default), true.

  - **Content patterns**
    - [ ] Polymorphic `as` (default div); arbitrary children with or without StackItem wrappers. Width/height/maxWidth/minHeight accept open-ended measurements and beat matching inline style fields.
    - [ ] HStack cross-axis vAlign and VStack cross-axis hAlign default stretch; no spacing default is asserted by the doc.
    - [ ] Use gap rather than child margins; fill item consumes leftover space; wrapping may avoid nested stacks.

  - **Named templates**
    - [ ] [`StackAlignment`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Stack/StackAlignment.doc.mjs) — “Stack — Alignment”: start/center/end buttons.
    - [ ] [`StackDirections`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Stack/StackDirections.doc.mjs) — “Stack — Directions”: horizontal/vertical badges (showcase).
    - [ ] [`StackFillItem`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Stack/StackFillItem.doc.mjs) — “Stack — Fill Item”: avatar, expanding text, button.

## Navigation

- [ ] **[Breadcrumbs](https://astryx.atmeta.com/components/Breadcrumbs)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Breadcrumbs/Breadcrumbs.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Breadcrumbs).

  - **Choices and semantics**
    - [ ] `variant`: `default` (default), `supporting` (smaller/secondary).
    - [ ] BreadcrumbItem children; `separator` default `/`, or custom ReactNode (chevrons/arrows/dots are examples, not finite enum); built-in slash mirrors in RTL.
    - [ ] Label default `Breadcrumb`; labelled nav and ordered list; current item aria-current page; multiple trails need distinct labels.
    - [ ] Menu-bearing crumb implements menu-button keyboard behavior: Enter, Space, ArrowDown opens; Escape closes.
  - **Templates**
    - [ ] **Breadcrumbs — Separators** (`BreadcrumbsCustomSeparator.doc.mjs`).
    - [ ] **Breadcrumbs — Deep Path** (`BreadcrumbsDeepHierarchy.doc.mjs`): five-level hierarchy.
    - [ ] **Breadcrumbs — Menu item** (`BreadcrumbsMenuItem.doc.mjs`): sibling destination menu, data or compound API.
    - [ ] **Breadcrumbs** (`BreadcrumbsShowcase.doc.mjs`): linked ancestors/current page.
    - [ ] **Breadcrumbs — Variants** (`BreadcrumbsSupportingVariant.doc.mjs`): default/supporting comparison.
    - [ ] **Breadcrumbs — Icons** (`BreadcrumbsWithIcons.doc.mjs`).

- [ ] **[Outline](https://astryx.atmeta.com/components/Outline)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Outline/Outline.doc.mjs). Templates in `Outline/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Density and active-state modes**
    - [ ] `density`: `default` (default), `compact`; sliding indicator tracks matching row height.
    - [ ] Flat ordered items with id/label/level `1`, `2`, `3`, `4`, `5`, `6`, IDs matching headings; levels indent, not expandable hierarchy.
    - [ ] Uncontrolled scroll-spy versus controlled activeId (disables spy); onActiveIdChange from spy or link selection.
    - [ ] `hasScrollOnClick`: true (default smooth scroll) / false (caller scrolling, still active/hash/callback updates).
    - [ ] label default Table of contents, named nav; every link in normal Tab order, Enter/Space activate.
    - [ ] Auto nearest scroll ancestor versus explicit scrollContainerRef; offset default 0 composes with heading scroll-margin-top for activation/landing below fixed header.
    - [ ] onNavigateStart/end paired exactly once including interrupted scroll; reduced motion jumps instantly; end is resolution, not guaranteed arrival.

  - **Named inline examples**
    - [ ] **Basic**: uncontrolled topmost visible heading tracking.
    - [ ] **Compact (density="compact")**: dense sidebar.
    - [ ] **Controlled active section**: caller state owns highlight.
    - [ ] **Generate items from markdown**: shared heading-links plugin, namespace, streaming finality.
    - [ ] **Scoped scroll container (split pane, modal, panel)**: explicit panel ref and fixed-header offset.
    - [ ] **Navigate callbacks (flash the heading on arrival)**: clear/set effect around paired callbacks, including interruption.

  - **Named templates**
    - [ ] `OutlineControlled` — **Outline — Controlled**: activeId/onActiveIdChange.
    - [ ] `OutlineDeepNesting` — **Outline — Deep Nesting**: levels 1–4 indentation example, API permits through 6.
    - [ ] `OutlineDensity` — **Outline — Density**: default/compact comparison.
    - [ ] `OutlineShowcase` — **Outline**: active sliding indicator.
    - [ ] `useOutlineFromDOMHookUsage` — **useOutlineFromDOM — Article**: appended heading adds linked entry.
    - [ ] `useOutlineFromMarkdownHookUsage` — **useOutlineFromMarkdown — Docs Article**: source-generated article/outline stay synchronized.

- [ ] **[Pagination](https://astryx.atmeta.com/components/Pagination)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Pagination/Pagination.doc.mjs). Templates in `Pagination/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Display and data modes**
    - [ ] `variant`: `pages` (default numbered/ellipsis), `count` (X–Y of Z), `compact` (Page X of Y), `dots`, `input` (editable page box), `none` (prev/next only).
    - [ ] `size`: `sm`, `md` (default); isDisabled false (default) / true.
    - [ ] Required controlled 1-based page/onChange; totalItems versus totalPages (items wins); unknown-total cursor mode via hasMore true/false.
    - [ ] pageSize default 10, positive integer normalization/nonfinite fallback; arbitrary pageSizeOptions shows selector; onPageSizeChange resets page 1.
    - [ ] hasFirstLast true default for input, false default for compact; ignored elsewhere, omitted for unknown totals. Input box disabled without known total; pageLabel custom noun, localized Page default.
    - [ ] step default 1 (invalid/fractional/<1 falls back); clamps overshoot, accessible labels reflect stride; siblingCount default 1 for pages.

  - **Links, async, and access**
    - [ ] getPageHref makes numbered/arrow/first/last controls real links with provider/custom as; dots/input/size selector remain controls; shared destination safety.
    - [ ] onChange still fires for links, do not navigate twice; page-size navigation belongs onPageSizeChange, not following onChange(1); docs warn against mixed navigation for dots/input variants.
    - [ ] changeAction follows change and supplies transition loading; label defaults Pagination for nav landmark; place under controlled content.

  - **Named templates**
    - [ ] `PaginationDotsCarousel` — **Pagination — Dots Carousel**: review-card carousel.
    - [ ] `PaginationPageSize` — **Pagination — Page Size Selector**: transactions footer/dropdown.
    - [ ] `PaginationVariants` — **Pagination — Variants**: dots/compact/count/pages showcase. Metadata says four; current API additionally documents input/none, both inventoried above.
    - [ ] `PaginationWithTable` — **Pagination — With Table**: client-side slicing, count/sm.

- [ ] **[Side Nav](https://astryx.atmeta.com/components/SideNav)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/SideNav/SideNav.doc.mjs).

  - **Layout and state modes**
    - [ ] Five zones: sticky `header`, `topContent`, scrollable `children`, `footer`, `footerIcons`. Footer controls inherit `sm` unless explicitly sized.
    - [ ] `collapsible`: false (default), true (uncontrolled with default toggle), config with `defaultIsCollapsed`, or controlled `isCollapsed` + `onCollapsedChange`; `hasButton` and `buttonLabel` configure the built-in toggle. Cover expanded/collapsed rails.
    - [ ] `resizable`: false (default), true (260px initial, 180–480px range), config (`defaultWidth`, `minWidth`, `maxWidth`, `autoSaveId`, `onWidthChange`), or a horizontal `useResizable` region. Widths are configurable measurements, not size variants.
    - [ ] Resize config can own uncontrolled/controlled collapse (`defaultIsCollapsed`, `isCollapsed`, `onCollapseChange`); it wins conflicting collapse state with a dev warning. `autoSaveId` persists width and collapse state in localStorage; resize handle hides while collapsed.
    - [ ] Hook region owns width/collapse and must have horizontal axis; its own collapsible option wins conflicting SideNav props, with warnings. External collapse button shares controlled config; deprecated `handleRef` is distinct from root ref.

  - **Accessibility and content patterns**
    - [ ] Navigation landmark; selected destination uses `aria-current="page"`; outline/filled icons distinguish selected state.
    - [ ] Nested disclosure toggles expose `aria-expanded`/`aria-controls`; collapsed group is inert.
    - [ ] Collapsed-rail child submenu: delayed hover open and leave close, click-open persists until dismissed, touch tap opens. Labels name icon-only items unless aria-label supplies context.

  - **Named templates**
    - [ ] [`SideNavShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/SideNav/SideNavShowcase.doc.mjs) — “SideNav” / “Side Nav”.
    - [ ] [`SideNavEndContent`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/SideNav/SideNavEndContent.doc.mjs) — “SideNav — End Content”: badges, counts, context menus.
    - [ ] [`SideNavNestedItems`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/SideNav/SideNavNestedItems.doc.mjs) — “SideNav — Nested Items”: collapsible hierarchy.
    - [ ] [`SideNavWithHeaderMenu`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/SideNav/SideNavWithHeaderMenu.doc.mjs) — “SideNav — Header with Menu”: account switcher.

- [ ] **[Stepper](https://astryx.atmeta.com/components/Stepper)**

  - Source: [English family API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Stepper/Stepper.doc.mjs).

  - **Choices and controlled progress**
    - [ ] Required controlled zero-based `activeStep` and Step children; preceding steps complete. No uncontrolled defaultActiveStep documented.
    - [ ] `orientation`: `horizontal` (default), `vertical`; `density`: `compact`, `balanced` (default), `spacious`.
    - [ ] `indicatorPosition`: `separated` (default), `on-track`; connector-gap token default 0px in on-track layout.
    - [ ] Horizontal collapse: `minimumStepWidth` default 112px; `collapsedVariant`: `withLabelAndControls` (default), `withLabel`, `hiddenLabel`. Controls appear only for first mode with onStepClick.

  - **Behavior and accessibility**
    - [ ] Ordered list, not nav landmark; label defaults localized `Progress`; all step names remain accessible when compact.
    - [ ] Without onStepClick display progress; with it enabled steps activate as buttons, disabled steps remain focusable/unavailable without activation. Compact Previous/Next skips disabled steps.
    - [ ] Narrow horizontal segmented track with optional current label/controls; on-track summary does not repeat indicator.
    - [ ] One-step advance animates progress fill at constant track speed; backward/jump/mid-flow mount/reduced-motion changes apply immediately. Status affects indicators, not connector progress.

  - **Named templates**
    - [ ] [`StepperShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Stepper/StepperShowcase.doc.mjs) — “Stepper — Checkout Progress”: equal horizontal segments, auto indicators, click-to-jump.
    - [ ] [`StepperCustomContent`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Stepper/StepperCustomContent.doc.mjs) — “Stepper — Custom Content”: active vertical fields/card/banner content.
    - [ ] [`StepperIndicatorModes`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Stepper/StepperIndicatorModes.doc.mjs) — “Stepper — Indicator Modes”: none, auto, number, custom icons.
    - [ ] [`StepperOnTrackHorizontal`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Stepper/StepperOnTrackHorizontal.doc.mjs) — “Stepper — On-Track Horizontal”: nodes on continuous rail, centered labels.
    - [ ] [`StepperOnTrackVertical`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Stepper/StepperOnTrackVertical.doc.mjs) — “Stepper — On-Track Vertical”: shared rail beside descriptions.
    - [ ] [`StepperStatus`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Stepper/StepperStatus.doc.mjs) — “Stepper — Validation Status”: success/error/accent, status announced as text.
    - [ ] [`StepperWidthResponsiveCollapse`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Stepper/StepperWidthResponsiveCollapse.doc.mjs) — “Stepper — Horizontal Narrow Collapsed”: compact summary/Previous/Next.

- [ ] **[Tab List](https://astryx.atmeta.com/components/TabList)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/TabList/TabList.doc.mjs).

  - **Choices and modes**
    - [ ] Controlled `value` + `onChange`; `size`: `sm`, `md` (default), `lg`; `layout`: `hug` (default), `fill`.
    - [ ] `hasDivider`, `isFullBleed`: false (default), true. Full bleed cancels nearest padded Layout inline insets and pads labels back; does not own block-edge docking.
    - [ ] `overflow`: `auto` (default, currently scroll), `scroll` (fades/hover-pointer arrows), `visible` (no overflow handling). Selected tab scrolls into view when scroll handling applies.
    - [ ] Default navigation landmark with aria-current versus explicit `role="tablist"` using tab/aria-selected and panelId-linked panels; only tabs in tabs-pattern strip and href ignored there. Other ARIA roles pass through, not a closed style enum.

  - **Content and accessibility**
    - [ ] Tab/TabMenu children in navigation pattern; passive badge/status/icon slots on tabs. Match adjacent action Button size when divided to share baseline.
    - [ ] Labels maintain 4.5:1 contrast in rest, hover, pressed, selected. Overflow menu is curated additional navigation, not automatic replacement of scrolling.

  - **Named templates**
    - [ ] [`TabListShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TabList/TabListShowcase.doc.mjs) — “TabList” / “Tab List”.
    - [ ] [`TabListTabsFillLayout`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TabList/TabListTabsFillLayout.doc.mjs) — “TabList — Fill Layout”: equal width plus divider.
    - [ ] [`TabListTabsWithActions`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TabList/TabListTabsWithActions.doc.mjs) — “TabList — With Actions”: tabs start/actions end.
    - [ ] [`TabListTabsWithBadge`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TabList/TabListTabsWithBadge.doc.mjs) — “TabList — With Badge”: urgent error and neutral counts.
    - [ ] [`TabListTabsWithIcons`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TabList/TabListTabsWithIcons.doc.mjs) — “TabList — With Icons”.
    - [ ] [`TabListTabsWithMenu`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TabList/TabListTabsWithMenu.doc.mjs) — “TabList — With Overflow Menu”.
    - [ ] [`TabListTabsWithStatusDot`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TabList/TabListTabsWithStatusDot.doc.mjs) — “TabList — With Status Dot”.

- [ ] **[Top Nav](https://astryx.atmeta.com/components/TopNav)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/TopNav/TopNav.doc.mjs).

  - **Layout and content patterns**
    - [ ] No finite visual variants documented. Heading/logo, startContent navigation/breadcrumbs, optional centerContent, endContent search/profile/actions; children alias startContent.
    - [ ] Center slot creates three-region centered layout while one row fits, then wraps; ordinary start/end layout otherwise.
    - [ ] Navigation landmark label default `Top navigation`; primary links, optional More menu, flexible search/CTA region. Recommended five or fewer visible destinations, shallow menus.

  - **Named templates**
    - [ ] [`TopNavShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNav/TopNavShowcase.doc.mjs) — “TopNav” / “Top Nav”.
    - [ ] [`TopNavCenteredNavigation`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNav/TopNavCenteredNavigation.doc.mjs) — “TopNav — Centered Navigation”: heading/end controls flank links.
    - [ ] [`TopNavEnterpriseDashboard`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNav/TopNavEnterpriseDashboard.doc.mjs) — “TopNav — Enterprise Dashboard”: icons, search, notifications, CTA.
    - [ ] [`TopNavHoverMenu`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNav/TopNavHoverMenu.doc.mjs) — “TopNav — Hover Menu”: product descriptions/icons.
    - [ ] [`TopNavMegaMenu`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNav/TopNavMegaMenu.doc.mjs) — “TopNav — Mega Menu”: full-width panel plus promotion.
    - [ ] [`TopNavMultipleDropdowns`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNav/TopNavMultipleDropdowns.doc.mjs) — “TopNav — Multiple Dropdowns”: switching auto-closes others.
    - [ ] [`TopNavWithLogo`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNav/TopNavWithLogo.doc.mjs) — “TopNav — With Logo”: brand link/profile action.

- [ ] **[Top Nav Mega Menu](https://astryx.atmeta.com/components/TopNavMegaMenu)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/TopNav/TopNavMegaMenu.doc.mjs).

  - **Surface and content patterns**
    - [ ] No finite visual style prop documented. Desktop full-width hover panel versus inline collapsible mobile drawer via render-mode context; items adapt themselves.
    - [ ] Trigger label; items slot usually TopNavMegaMenuItem but accepts arbitrary content; featured slot at desktop right versus below items in mobile drawer.
    - [ ] Hover show delay default 150ms/hide delay 250ms; onOpenChange reports open/closed state, not documented controlled isOpen.

  - **Named templates**
    - [ ] [`TopNavMegaMenuBasic`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNavMegaMenu/TopNavMegaMenuBasic.doc.mjs) — “TopNavMegaMenu — Basic”: rich destinations.
    - [ ] [`TopNavMegaMenuShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNavMegaMenu/TopNavMegaMenuShowcase.doc.mjs) — “TopNavMegaMenu” / “Top Nav Mega Menu”: items and featured card.

- [ ] **[Top Nav Mega Menu Featured Card](https://astryx.atmeta.com/components/TopNavMegaMenuFeaturedCard)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/TopNav/TopNavMegaMenuFeaturedCard.doc.mjs).

  - **Content patterns**
    - [ ] No finite visual variants documented. Title, optional description/image, CTA linkLabel/linkHref, custom children below body.
    - [ ] Informative image with imageAlt versus omitted alt (explicit decorative alt-empty/role=presentation/aria-hidden when title/description carry meaning).

  - **Named templates**
    - [ ] [`TopNavMegaMenuFeaturedCardBasic`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNavMegaMenuFeaturedCard/TopNavMegaMenuFeaturedCardBasic.doc.mjs) — “TopNavMegaMenuFeaturedCard — Basic”: promotional title/description/CTA.
    - [ ] [`TopNavMegaMenuFeaturedCardShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNavMegaMenuFeaturedCard/TopNavMegaMenuFeaturedCardShowcase.doc.mjs) — “TopNavMegaMenuFeaturedCard” / “Top Nav Mega Menu Featured Card”: inside menu.

- [ ] **[Top Nav Menu](https://astryx.atmeta.com/components/TopNavMenu)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/TopNav/TopNavMenu.doc.mjs).

  - **Behavior and content**
    - [ ] No finite visual variants documented. Label trigger and rich item-array icon/title/optional description hover popover.
    - [ ] Hover show delay default 150ms, hide delay 200ms; items follow shared Link destination rule, rejected href remains visible without navigation.
    - [ ] Keyboard/finger/pen pick closes and restores trigger focus; mouse pick leaves open. Per-item hasCloseOnSelect false keeps result-reporting action open.

  - **Named templates**
    - [ ] [`TopNavMenuBasic`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNavMenu/TopNavMenuBasic.doc.mjs) — “TopNavMenu — Basic”: grouped related destinations.
    - [ ] [`TopNavMenuShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNavMenu/TopNavMenuShowcase.doc.mjs) — “TopNavMenu” / “Top Nav Menu”: rich hover dropdown.

## Overlay

- [ ] **[Bottom Sheet](https://astryx.atmeta.com/components/BottomSheet)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/BottomSheet/BottomSheet.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/BottomSheet).

  - **Presentation and state**
    - [ ] Fully controlled standalone `isOpen` / `onOpenChange`; required accessible `label`; optional `finalFocusRef`. Deprecated switcher-managed alternative uses `sheetId` and omits standalone state props.
    - [ ] `purpose`: `info` (default: Escape, scrim, swipe, platform close), `form` (Escape/platform close, blocks scrim/swipe), `required` (no implicit dismissal, alertdialog); explicit controls may still close.
    - [ ] `height`: `hug` (content up to ~92%), `capped` (default ~62% scrolling panel), `tall` (~92% pinned), or arbitrary pixel/CSS budget; short viewports fill available height.
    - [ ] `snapPoints` omitted means open/close only; extra stops accept viewport fractions, percentages, absolute lengths. Own height is tallest stop; stops at/below quarter-sheet are peeks that slide away and thin scrim.
    - [ ] `padding`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`; omitted uses theme (spacing 4 fallback), top never below grab-bar height (spacing 6). Lone Section escapes padding; Table/Divider bleed compensation.
    - [ ] `hasScrim=true` default: showModal, inert background, focus containment, body lock, stacking. `false`: manual top-layer popover, no trap/lock/scrim; reduced fallback show() below Popover support floor, no page-level stacking guarantee.
  - **Focus, keyboard, stacking, and motion**
    - [ ] Scrollable named body is Tab-reachable only while overflowing; first native link/button may receive delegated entry; controls/composites/nested scrollers retain body stop.
    - [ ] data-autofocus takes priority on all openings/replacements; otherwise native dialog focus differs by browser. Return to revealed sheet opener or panel, else explicit finalFocusRef/opener; closing entire stack returns once to bottom opener/target.
    - [ ] Only fully expanded tall height accommodates mobile keyboard; shorter detents/hug/capped/custom heights leave browser reveal alone. Tall form scrolls focused field above visual keyboard and follows keyboard transitions.
    - [ ] Modal sheets stack by open order; simultaneous openings use tree order. Covered sheets remain mounted, preserve inputs/scroll, recede (nearest 0.96/8px, deeper 0.92/16px), and are blocked by top modal.
    - [ ] One dimming scrim across stack; only top dismisses. Closing covered host hides its nested sheets without rewriting their controlled state; unrelated sheets stay open. Hidden React sheets leave stack and re-enter on top; iframe documents have separate stacks.
    - [ ] Replacement requires exactly one top closing/one opening in same update, driven from inside old step; entering sheet rises, old aligns down if taller then fades inert/AT-hidden; focus returns to flow's original opener. Outside-driven/different-update changes use ordinary push/pop.
    - [ ] No-scrim sheets replace but do not stack/recede; focus returns only if still inside. Outside an unrelated modal they remain inert/beneath until it exits; descendants can enter top layer immediately.
    - [ ] Dialog/AlertDialog/Popover over a sheet do not recede it; a sheet opened from such a dialog can stack above underlying sheet.
  - **Named inline examples**
    - [ ] **Basic**: controlled filter sheet.
    - [ ] **Tall sheet (a list)**: pinned scrolling list.
    - [ ] **Collapsible to half the screen**: tall plus `[0.5]`.
    - [ ] **A peek, a working height, and full**: tall with `96px`/`50%` stops (example values, not enum).
    - [ ] **Hug height (fits content)**: share actions.
    - [ ] **Long scrolling content**: hug release notes.
    - [ ] **Multi-step flow (one sheet at a time, each step replaces the last)**: guarded one-step state.
    - [ ] **Stacked sheets (drill-in: list, order, edit)**: independent state and close-all.
    - [ ] **Detail sheet inside the list sheet that opens it**: nested lifetime without rewriting isOpen.
    - [ ] **Deep link into a stack (several sheets open in one update)**: tree-order stacking.
    - [ ] **Mobile keyboard**: tall comment form.
    - [ ] **No scrim**: live page behind sheet.
    - [ ] **Protect form input**: purpose form.
  - **Templates**
    - [ ] **Bottom Sheet — Height variants** (`BottomSheetHeights.doc.mjs`): hug/capped/tall.
    - [ ] **Bottom Sheet — Mobile keyboard** (`BottomSheetMobileKeyboard.doc.mjs`): tall scrolling form.
    - [ ] **Bottom Sheet — No scrim** (`BottomSheetNoScrim.doc.mjs`).
    - [ ] **Bottom Sheet** (`BottomSheetShowcase.doc.mjs`): filter surface.
    - [ ] **Bottom Sheet — Snap points** (`BottomSheetSnapPoints.doc.mjs`): half-height and peek.
    - [ ] **Bottom Sheet — Review flow** (`BottomSheetSwitcherReviewFlow.doc.mjs`): standalone two-step replacement.
    - [ ] **Bottom Sheet — Multi-step flow** (`BottomSheetSwitcherShowcase.doc.mjs`): standalone three heights, replacement for deprecated switcher.

- [ ] **[Bottom Sheet Switcher](https://astryx.atmeta.com/components/BottomSheetSwitcher)** — **deprecated upstream; confirm inclusion before porting**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/BottomSheet/BottomSheetSwitcher.doc.mjs). No own template directory supplied; replacement templates are inventoried under BottomSheet.

  - **Deprecated flow modes and behaviors**
    - [ ] Deprecated but still functional; controlled `activeSheet` matches unique child `sheetId`, `null` closes; `onActiveSheetChange(null)` reports allowed dismissal.
    - [ ] `hasScrim=true` default: modal dialogs, focus containment, one scroll lock/backdrop across flow. `false`: non-modal show(), no backdrop; avoid transformed/contained/clipping ancestors.
    - [ ] Active child purpose `info`, `form`, `required` determines implicit dismissal; `onCancel` may prevent native Escape/platform cancellation independently.
    - [ ] Previous step can remain visible/inert, align down behind shorter new step, then fade; autofocus field honored, reduced motion honored, toasts remain above/pressable.
    - [ ] Whole modal switcher is one stack level: recedes/blocks when covered, shares one scrim, only top dismisses. Step change does not reorder level. Nested standalone sheet leaves with inactive step without rewriting isOpen.
    - [ ] Dialog ref follows active sheet, then exiting/last dialog; id/test-id apply only there. Open SSR carries identifiers on active dialog; closed SSR carries none until synchronous client update; other dialog props apply to every step dialog.
  - **Migration and inline example**
    - [ ] Inline **Replacement: the same three-step flow as standalone sheets**: one step value, guarded per-sheet onOpenChange, hug heights, Continue/Back/Done.
    - [ ] Migrate hasScrim to every step; no sheetId/wrapper. Standalone onOpenChange has no dismissal reason: an Escape-only veto has no equivalent; purpose is per-path policy.
    - [ ] Move naming to each sheet label. Standalone ref/base props target panel, not implementation dialog; query closest dialog if necessary; dialog-level styling has no equivalent.

- [ ] **[Command Palette](https://astryx.atmeta.com/components/CommandPalette)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/CommandPalette/CommandPalette.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/CommandPalette).

  - **Modes, search, and slots**
    - [ ] Controlled isOpen/onOpenChange; searchable action/navigation launcher versus picker mode with controlled value/onValueChange and persistent check indicator.
    - [ ] SearchSource search/bootstrap, static createStaticSource or async search/loading spinner; automatic grouping by auxiliaryData.group retained with custom renderer.
    - [ ] Input slot defaults CommandPaletteInput, footer defaults CommandPaletteFooter; custom non-interactive item presentation for icons/descriptions/shortcuts (no nested buttons/links).
    - [ ] Empty search default No results; empty bootstrap default Type to search; ReactNode overrides.
    - [ ] Accessible dialog label default Command palette; width 640, maxHeight 480 (custom dimensions open-ended).
    - [ ] `isInline=false` default; inline preview disables modal behavior, autofocus and initial-highlight auto-scroll.
    - [ ] Query keeps keyboard focus, arrows update active descendant, Enter selects; preserve IME and native Home/End caret movement, not custom interception that breaks them.
  - **Templates**
    - [ ] **CommandPalette — Async Search** (`CommandPaletteAsyncSearch.doc.mjs`): server loading/custom empty.
    - [ ] **CommandPalette — Grouped** (`CommandPaletteAutoGrouped.doc.mjs`): data group labels.
    - [ ] **CommandPalette — Custom Footer** (`CommandPaletteCustomFooter.doc.mjs`): tip content.
    - [ ] **CommandPalette — Picker Mode** (`CommandPalettePickerMode.doc.mjs`): persistent single value/check.
    - [ ] **CommandPalette — Rich Items** (`CommandPaletteRichItems.doc.mjs`): icon/shortcut/keyword search.
    - [ ] **CommandPalette** (`CommandPaletteShowcase.doc.mjs`): static keyboard launcher.

- [ ] **[Dialog](https://astryx.atmeta.com/components/Dialog)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Dialog/Dialog.doc.mjs), [original position/focus contract](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Dialog/Dialog.tsx), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Dialog).

  - **Choices and state**
    - [ ] `variant`: `standard` (default), `fullscreen`.
    - [ ] `purpose`: `info` (default, Escape/backdrop), `form` (Escape, protects from backdrop after interaction per doc), `required` (no Escape/backdrop).
    - [ ] Controlled isOpen/onOpenChange, or imperative useImperativeDialog.show(content) plus rendered element; `isInline=false` default, true removes dialog/backdrop/modal behavior for preview.
    - [ ] Arbitrary children/structured Layout; width default 400, maxHeight 75dvh, standard viewport/container gutters/clamping; fullscreen safe-area fallback and fade entrance.
    - [ ] `padding`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`; no fixed numeric default in own prop table.
    - [ ] Centered position omitted; DialogPosition logical start/end and block top/bottom offsets accept pixel/CSS values (offset fields, not a placement enum); logical offsets mirror RTL.
    - [ ] Modal blocks page; explicit autofocus descendant takes precedence over header h2 focus. Initial/returned focus ring follows keyboard versus pointer modality; reduced-motion touch fades without travel/scale, mouse appears immediately.
  - **Templates**
    - [ ] **Dialog — Adaptive presentation** (`DialogAdaptivePresentation.doc.mjs`): optional wrapper, touchPresentation `dialog`, `fullscreen`, `bottom-sheet`; deterministic presentation override for tests/unusual environments. Sheet switch only lg-and-below/coarse/no-hover, not Dialog default and not destructive AlertDialog policy.
    - [ ] **Dialog — Confirmation** (`DialogConfirmationDialog.doc.mjs`): irreversible-action confirmation.
    - [ ] **Dialog — Form** (`DialogFormDialog.doc.mjs`): purpose form/inline edit.
    - [ ] **Dialog — Fullscreen** (`DialogFullscreenDialog.doc.mjs`): editor/wizard/media/document viewer.
    - [ ] **Dialog — Scrollable** (`DialogScrollingContent.doc.mjs`): long body with bounded height.
    - [ ] **Dialog** (`DialogShowcase.doc.mjs`): header/body/close.
    - [ ] **Dialog — Required** (`DialogWithSubtitle.doc.mjs`): explicit action required, no Escape/backdrop.

- [ ] **[Drawer](https://astryx.atmeta.com/components/Drawer)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Drawer/Drawer.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Drawer).

  - **Choices and composition**
    - [ ] Controlled isOpen/onOpenChange, required accessible label; no built-in close button, consumer must provide visible exit.
    - [ ] `side`: `start`, `end` (default); logical inline edge mirrors RTL, not a bottom placement.
    - [ ] `hasScrim=true` default: modal showModal/top layer, focus trap, scroll lock; false manual Popover top-layer non-modal, live interactive background/no trap.
    - [ ] `purpose`: `info` (default, Escape/scrim), `form` (Escape only), `required` (neither, consumes Escape and modal role alertdialog). No-scrim form/info equivalent; explicit controls can still close.
    - [ ] Width default 400, pixel/CSS budget; below 640px preserves 56px page reveal without exceeding budget. Experimental isFullWidthOnMobile false default, true covers viewport below breakpoint; unstable patch-level option.
    - [ ] `padding`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`; theme default spacing 4. Full bleed at 0; lone Section escapes, Table/Divider compensate, Layout redistributes insets; bottom safe area added in every mode.
    - [ ] Full-height scrolling children; Layout with DrawerHeader/header, LayoutContent/body, LayoutFooter/pinned actions. Keep selected content mounted through exit, rather than nulling immediately.
  - **Focus, stacking, and motion**
    - [ ] data-autofocus selects initial control; Escape closes and returns focus unless required. Sibling drawers stack last-opened on top, top-only Escape/innermost-first close; do not nest drawers.
    - [ ] Slide entrance/exit respects reduced motion; coarse-pointer touch/drag during entry blocked on panel while entry completes (non-modal background stays live), keys/AT stay live; mouse/trackpad activation retained per last-pointer/movement rule.
  - **Named inline examples**
    - [ ] **Basic**: trigger, modal details, Layout/DrawerHeader, Escape/scrim/close return focus.
    - [ ] **Modal or non-modal (hasScrim)**: compare trapped modal and live-page panel with close button.
    - [ ] **Edit form with footer actions**: purpose form and pinned Cancel/Save.
    - [ ] **Slide in from the start edge**: logical nav panel.
    - [ ] **Width budget: pixels or any CSS length**: pixel/rem/percentage sizing, mobile cap.
  - **Templates**
    - [ ] **Edit form with footer actions** (`DrawerForm.doc.mjs`): Layout/pinned footer/protected scrim.
    - [ ] **Modal or non-modal (hasScrim)** (`DrawerScrim.doc.mjs`).
    - [ ] **Basic** (`DrawerShowcase.doc.mjs`): trigger and focus return.
    - [ ] **Slide in from the start edge** (`DrawerStart.doc.mjs`).
    - [ ] **Width budget: pixels or any CSS length** (`DrawerWidths.doc.mjs`).

- [ ] **[Hover Card](https://astryx.atmeta.com/components/HoverCard)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/HoverCard/HoverCard.doc.mjs). Templates in `HoverCard/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Placement and trigger policy**
    - [ ] `placement`: `above` (default), `below`, `start`, `end`; `alignment`: `start`, `center` (default), `end`; logical RTL mirroring.
    - [ ] `focusTrigger`: `auto` (default), `always`, `never`; auto includes focus reached through roving tabindex.
    - [ ] `touchTrigger`: `auto` (default), `tap`, `none`; auto avoids stealing a working control's tap, but aria-disabled controls can reveal their reason.
    - [ ] `hasHoverIndication`: `auto` (default), true, false; dashed underline applies only to text triggers.
    - [ ] `isEnabled`: true (default) / false; show delay 300ms, hide delay 200ms by default.

  - **Open modes, content, and access**
    - [ ] Uncontrolled `isDefaultOpen` versus controlled `isOpen` true/false. Controlled Escape requests onOpenChange(false); actual closing waits for caller update, then returns inside focus to trigger.
    - [ ] Text/number/mixed text triggers get focusable inline span, ignoring false/null holes; element trigger wires first element without requiring ref acceptance.
    - [ ] Required rich `content`; optional label gives named dialog, otherwise unnamed group. Supplementary previews, not required interactive workflows.
    - [ ] Outside tap dismisses tap-opened card; block context preferred; paragraph/heading/link contexts use corrective portal; SVG foreignObject is unsupported as Chromium anchor.

  - **Named templates**
    - [ ] `HoverCardHookUsage` — **useHoverCard — Profile Preview**: direct trigger/render control with hook.
    - [ ] `HoverCardInlineTextHoverCard` — **HoverCard — Definition**: glossary term inside paragraph.
    - [ ] `HoverCardInteractiveContent` — **HoverCard — Link Preview**: summary/title/URL preview, not license for required interactive content.
    - [ ] `HoverCardProfileHoverCard` — **HoverCard — Profile Preview**: name, role, biography.
    - [ ] `HoverCardShowcase` — **HoverCard**: button-triggered avatar/profile, initially open preview; also showcase for useHoverCard.

- [ ] **[Lightbox](https://astryx.atmeta.com/components/Lightbox)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Lightbox/Lightbox.doc.mjs). Templates in `Lightbox/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Media and state**
    - [ ] Required controlled `isOpen` with onOpenChange; single media object versus array gallery with previous/next.
    - [ ] Media `type`: `image` (default), `video`; src, required alt, optional plain caption; video uses native controls and disables zoom/pan.
    - [ ] Gallery controlled `index`/onIndexChange versus uncontrolled `defaultIndex` (default 0).
    - [ ] `hasZoom`: false (default) / true for images; `hasAutoPlay`: false (default) / true for video.

  - **Keyboard and gesture behavior**
    - [ ] Close button, Escape, outside-media tap; phone/tablet downward photo drag from fitted size dismisses.
    - [ ] Gallery prev/next buttons, arrows, phone/tablet swipe update index.
    - [ ] Zoom via pinch/ctrl-wheel, double-tap/double-click at point; Enter/Space/+/- toggles 2x about center; 0 resets fitted size.
    - [ ] Zoomed drag/trackpad scroll or arrows pan; phone/tablet pan has momentum; alt text required, no interactive caption content.

  - **Named templates**
    - [ ] `LightboxGallery` — **Lightbox — Gallery**: thumbnail grid opens chosen index, browse without closing.
    - [ ] `LightboxHookUsage` — **useLightbox — Gallery Grid**: hook trigger props, keys, visibility.
    - [ ] `LightboxShowcase` — **Lightbox**: fullscreen image/caption.
    - [ ] `LightboxVideo` — **Lightbox — Video**: native video controls, no zoom/pan.
    - [ ] `LightboxZoom` — **Lightbox — Zoom**: double-click zoom, drag pan, double-click resets; close exits viewer.

- [ ] **[Overlay](https://astryx.atmeta.com/components/Overlay)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Overlay/Overlay.doc.mjs). Templates in `Overlay/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Reveal and scrim choices**
    - [ ] `showOn`: `hover`, `always` (default), `focus`, `hover-or-focus`; hover also reveals for keyboard focus, hover-or-focus aliases hover.
    - [ ] `isOpen` optional controlled true/false override takes precedence over reveal/touch toggle behavior.
    - [ ] `scrim`: `dark` (default), `light`, false (no scrim); `position`: `fill` (default), `bottom`, `top`; `align`: `start`, `center`, `end` (default).
    - [ ] Required overlay content (compact action/text), base children image/video/card/media; host attributes/handlers/ref reach bounded root; not an external anchored popover.

  - **Named templates**
    - [ ] `OverlayBottomStrip` — **Overlay — Bottom Strip**: supporting text in bottom scrim.
    - [ ] `OverlayHoverReveal` — **Overlay — Hover Reveal**: action on hover/focus.
    - [ ] `OverlayShowcase` — **Overlay**: always-visible scrim with centered action.
    - [ ] `useOverlayHookUsage` — **useOverlay — Product Quick View**: Card-owned container, bottom action strip, image link remains clickable and precedes Quick view in Tab order.

- [ ] **[Popover](https://astryx.atmeta.com/components/Popover)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Popover/Popover.doc.mjs). Templates in `Popover/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Placement and surfaces**
    - [ ] `placement`: `above`, `below` (default), `start`, `end`; alignment `start` (default), `center`, `end`, logical RTL.
    - [ ] `presentation`: `popover` (default), `bottom-sheet`, `adaptive`; compact coarse-pointer threshold below theme md width or height 500px; surface chosen on open and held until close.
    - [ ] `padding`: `0`, `0.5`, `1`, `1.5`, `2`, `3` (default), `4`, `5`, `6`, `8`, `10`; applied on painted target, theme can replace.
    - [ ] Width default auto, arbitrary size capped to viewport with safe gutters and internal scrolling.
    - [ ] `role`: `dialog` (default), `none` for self-semantic content; `isModal`: true default / false, applies only dialog role.

  - **Visibility, focus, and slots**
    - [ ] Internally managed versus controlled isOpen/onOpenChange; isEnabled true (default) / false ignores trigger.
    - [ ] hasCloseButton, hasAutoFocus, hasLightDismiss, hasEscapeDismiss: each true by default / false. Escape flag fully disables dismissal only with light-dismiss false because native light dismissal includes Escape.
    - [ ] Trigger children must contain button/role button; sibling mode uses external anchorRef; required interactive content; optional label, sheet falls back to trigger text.
    - [ ] Autofocus first actual content control or labeled surface; fallback close hidden until keyboard navigation; closeButtonLabel default Close popover.
    - [ ] Sheet ignores placement/alignment/width/role/isModal/hasAutoFocus, styles sheet panel; always outside-tap/swipe-down dismisses regardless light-dismiss flag; Escape follows own flag, fallback close hidden.
    - [ ] Coarse-pointer anchored surface grows/shrinks from trigger; reduced motion uses opacity-only crossfade <=200ms.

  - **Named inline examples**
    - [ ] **Flush surface for a list of rows**: padding 0, List owns edges.
    - [ ] **Roomier surface for a form**: padding 4 around focused save-search form.

  - **Named templates**
    - [ ] `PopoverBottomSheetAlternative` — **Popover — Bottom Sheet Alternative**: explicit modal touch alternative, built with BottomSheet rather than pretending identical anchor semantics.
    - [ ] `PopoverConfirmAction` — **Popover — Confirm Action**: destructive confirmation/cancel.
    - [ ] `PopoverFilterPanel` — **Popover — Filter Panel**: checkbox filters/apply/reset.
    - [ ] `PopoverHookUsage` — **usePopover — Quick Actions**: hook refs/ARIA/focus trap.
    - [ ] `PopoverKeyboardShortcuts` — **Popover — Keyboard Shortcuts**: key/description pairs.
    - [ ] `PopoverSettingsPanel` — **Popover — Settings Panel**: preference switches.
    - [ ] `PopoverShowcase` — **Popover**: standard panel, also usePopover showcase.

- [ ] **[Toast](https://astryx.atmeta.com/components/Toast)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Toast/Toast.doc.mjs).

  - **Choices and lifecycle**
    - [ ] `type`: `info` (default), `error`; isAutoHide defaults true for info, false for error, explicitly overridable either way. Duration default 5000ms; timed content must satisfy WCAG 2.2.1.
    - [ ] `collisionBehavior`: `overwrite` (default), `ignore`, keyed by uniqueID; repeated notifications replace versus retain first.
    - [ ] onHide/onDismiss reason `auto` or `manual`; inline visual preview versus production useToast/ToastViewport positioning, stacking, lifecycle and deduplication.
    - [ ] Top/bottom configured edge governs enter/exit and touch/pen vertical swipe: dominant edge-directed intent before suppressing native touch movement, manual reason. Mouse drag excluded to preserve text selection; close control remains.

  - **Content and accessibility**
    - [ ] ReactNode body, optional endContent button/link (persistent when action must remain available), default dismiss button.
    - [ ] Per-toast renderContent replaces internal layout, receives body/endContent/type/isAutoHide/autoHideDuration/dismiss; outer themed card/timer retained, custom renderer must provide own close control (none injected).
    - [ ] Viewport announces even above modal; card inside viewport is not itself live region. Safe-area/viewport gutters and wrapping long text; data-* reaches element but component owns role/live/theme attributes.

  - **Named templates**
    - [ ] [`ToastShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Toast/ToastShowcase.doc.mjs) — “Toast”: imperative viewport notifications.
    - [ ] [`ToastAction`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Toast/ToastAction.doc.mjs) — “Toast — Action”: persistent button/link.
    - [ ] [`ToastDeduplication`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Toast/ToastDeduplication.doc.mjs) — “Toast — Deduplication”: ignore versus overwrite.
    - [ ] [`ToastDismiss`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Toast/ToastDismiss.doc.mjs) — “Toast — Dismiss”: programmatic cleanup for long operation.
    - [ ] [`ToastStacking`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Toast/ToastStacking.doc.mjs) — “Toast — Stacking”: repeated enter/exit queue.
    - [ ] [`ToastTypes`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Toast/ToastTypes.doc.mjs) — “Toast — Types”: timed info/persistent error.
    - [ ] [`useToastHookUsage`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Toast/useToastHookUsage.doc.mjs) — “useToast — Settings Saved”: Undo, dedup ID and failure toast.

- [ ] **[Tooltip](https://astryx.atmeta.com/components/Tooltip)**

  - Source: [English family API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Tooltip/Tooltip.doc.mjs).

  - **Placement, triggers, and ownership**
    - [ ] `placement`: `above` (default), `below`, `start`, `end`; `alignment`: `start`, `center` (default), `end`; logical edges mirror using inherited popover direction in CSS.
    - [ ] `focusTrigger`: `auto` (default), `always`, `never`; `touchTrigger`: `auto` (default), `tap`, `none`.
    - [ ] Touch auto tap opens only when trigger has no own action; disabled aria-disabled control can reveal reason. Tap forces opening for reveal-only button, none never opens on touch; InfoTip for pinned info help.
    - [ ] `isEnabled`: true (default), false; `hasHoverIndication`: `auto` (default), true, false (dashed underline).
    - [ ] Uncontrolled `isDefaultOpen` initial state versus controlled isOpen with onOpenChange; controlled-closed focus requests true without showing, controlled-open Escape requests false without hiding until caller updates.
    - [ ] Show delay 200ms, hide delay 0ms; wrapped child trigger versus external anchorRef sibling; ReactNode content should remain short/noninteractive (links/actions belong to HoverCard/Popover).

  - **Named templates**
    - [ ] [`TooltipShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Tooltip/TooltipShowcase.doc.mjs) — “Tooltip”.
    - [ ] [`TooltipActionBarTooltips`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Tooltip/TooltipActionBarTooltips.doc.mjs) — “Tooltip — Action Bar”.
    - [ ] [`TooltipHookUsage`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Tooltip/TooltipHookUsage.doc.mjs) — “Tooltip — Hook Usage”: programmatic hook control.
    - [ ] [`TooltipInlineTextTooltips`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Tooltip/TooltipInlineTextTooltips.doc.mjs) — “Tooltip — Inline Text”: term definitions.

## Table & List

- [ ] **[List](https://astryx.atmeta.com/components/List)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/List/List.doc.mjs). Templates in `List/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Layout and row styles**
    - [ ] `density`: `compact`, `balanced` (default), `spacious`; coarse-pointer interactive hit targets can reach 44px without changing visible layout.
    - [ ] `hasDividers`: false (default) / true; optional header names list through aria-labelledby.
    - [ ] `listStyle`: `none` (default), `disc`, `decimal` (ol), `circle`; ordered `start` default 1, arbitrary count.
    - [ ] `edge`: `padded` (default), `flush`, `outset`, `bleed`; first row content aligns (marker/icon/avatar before label), not necessarily label itself.
    - [ ] Flush removes side padding; outset reaches highlights/dividers into container padding; bleed reaches container edge with conditional first/last rounded container corners.
    - [ ] Outset/bleed use nearest supported padding-sharing Card/Section/Dialog/BottomSheet/explicitly padded Layout/ScrollableArea; no intermediate extra padding. Drawer and automatic Layout region padding unsupported; no shared container falls back to padded.
    - [ ] Deprecated `edgeCompensation: 'inline'` means outset; explicit edge wins. Theme item inline padding overrides all modes, including flush.

  - **Motion and reordering**
    - [ ] Static updates without animatedLayout handle versus keyed insert/remove/reorder animation; itemKey-less rows change immediately; reduced motion removes travel.
    - [ ] `reorderable` from createReorderable: keyed eligible rows get handles; rejected/keyless rows keep place, boundary fixed rows bound moves; caller applies onReorder({key,to}).
    - [ ] Touch hold handle picks up, flick scrolls; mouse drags immediately; copy follows pointer, insertion line, edge autoscroll; one drop report, cancel restores. Touch rearrangement glides; mouse settles without motion.
    - [ ] Click/Space/Enter toggles handle reorder mode (aria-pressed); Up/Down commits each move; Escape/second activation ends while keeping moves.
    - [ ] Announce committed position/neighbors or refused fixed-row move; hint supplies move/Done buttons for tap-only access; start(key) enters from caller UI; disabled handle stays focusable/inert.
    - [ ] ListItem children with label/description/start/end content; interactive versus static rows must remain distinguishable; inline overflow clips swipe rows.

  - **Named templates**
    - [ ] `ListBasicList` — **List — Basic**: settings labels/descriptions.
    - [ ] `ListBulletedFeatures` — **List — Bulleted Features**: disc highlights.
    - [ ] `ListMessageList` — **List — Message List**: avatars, previews, unread badges.
    - [ ] `ListOrderedSteps` — **List — Ordered Steps**: decimal instructions.
    - [ ] `ListShowcase` — **List**: list showcase.

- [ ] **[Metadata List](https://astryx.atmeta.com/components/MetadataList)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/MetadataList/MetadataList.doc.mjs). Templates in `MetadataList/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Layout and disclosure**
    - [ ] `columns`: `single` (default), `multi`, arbitrary number; `orientation`: `vertical` (default), `horizontal` (wrapping row).
    - [ ] `label.position`: `start`, `top`; default start in vertical single-column, top in multi-column/horizontal; horizontal ignores requested position and always top.
    - [ ] Optional label width; maxNumOfItems enables collapsed/expanded show-more/less; count and widths are data, not finite styles.
    - [ ] Required MetadataListItem children; optional title does not itself name list/add heading, caller passes semantic heading if needed.
    - [ ] Each label/value remains term/definition semantics, including rich values such as tokens; no color or size enum.

  - **Named templates**
    - [ ] `MetadataListBasicMetadata` — **MetadataList — Basic**: single-column key/value rows.
    - [ ] `MetadataListCollapsibleMetadata` — **MetadataList — Collapsible**: show-more threshold.
    - [ ] `MetadataListHorizontalMetadata` — **MetadataList — Horizontal**: compact inline metadata.
    - [ ] `MetadataListMultiColumnMetadata` — **MetadataList — Multi-Column**: grid with token values.
    - [ ] `MetadataListShowcase` — **MetadataList**: metadata-list showcase.

- [ ] **[Overflow List](https://astryx.atmeta.com/components/OverflowList)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/OverflowList/OverflowList.doc.mjs). Templates in `OverflowList/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Collapse and measurement modes**
    - [ ] `collapseFrom`: `start`, `end` (default); `behavior`: `observeSelf` (default), `observeParent` (content-sized row measured against parent).
    - [ ] `gap`: `0`, `0.5`, `1`, `1.5`, `2` (default), `3`, `4`, `5`, `6`, `8`, `10`.
    - [ ] minVisibleItems default 0; maxVisibleItems unset/no cap; min wins contradictory max. Counts are arbitrary numeric bounds.
    - [ ] maxRows omitted or 1 is single-line, greater count wraps bounded rows assuming uniform row height; horizontal flow, not vertical list.
    - [ ] Required element children with stable React keys; custom overflowRenderer receives hidden child/index entries only while overflow exists.
    - [ ] onOverflowChange reports membership/order change even same count and empty array when widened; silent on initial/no overflow, unrelated render, or callback identity change; can feed existing menu instead of new indicator.

  - **Named templates**
    - [ ] `OverflowListCappedToolbar` — **OverflowList — Capped Toolbar**: fixed visible ceiling with dropdown remainder.
    - [ ] `OverflowListCollapseFromStartList` — **OverflowList — Collapse From Start**: retain newest/end items.
    - [ ] `OverflowListMultiRowTags` — **OverflowList — Multi-row Tags**: two-row cloud/count remainder.
    - [ ] `OverflowListOverflowBadges` — **OverflowList — Badge Tags**: resizable badge row and count.
    - [ ] `OverflowListOverflowDropdownActions` — **OverflowList — Dropdown Actions**: toolbar overflow menu.
    - [ ] `OverflowListShowcase` — **OverflowList**: buttons collapse to +N.

- [ ] **[Table](https://astryx.atmeta.com/components/Table)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/Table.doc.mjs).

  - **Finite choices and modes**
    - [ ] `density`: `compact`, `balanced` (default), `spacious`; `dividers`: `rows` (default), `columns`, `grid`, `none`.
    - [ ] `verticalAlign`: `middle` (default), `top`, `bottom`; `textOverflow`: `wrap` (default), `truncate`. Headers always truncate; default truncated body cells show hover tooltip.
    - [ ] `isStriped` and `hasHover`: false (default), true; stripe even rows, hover applies on pointer devices.
    - [ ] Data-driven rows/columns versus children mode with TableHeader/TableBody/TableFooter → TableRow → cells. Never direct tr under table: parser-inserted tbody otherwise causes hydration mismatch. Data-driven mode has no automatic footer.
    - [ ] Auto-generated columns versus explicit key/header/width/align/renderCell definitions; `pixel(n)` fixed versus `proportional(n)` weighted widths. Omitted width: equal share/60px floor; explicit proportional default floor 120px, overridable minWidth. No fixed column-data examples are styles.
    - [ ] Explicit `idKey` property or key function; legacy index fallback. Virtualization requires unique valid string/finite-number identities.

  - **Capabilities, content, and accessibility**
    - [ ] Named plugin pipeline for sorting, selection, filtering, pagination, column management and structure; add only needed plugins.
    - [ ] Geometry-only row virtualization, column virtualization, or at most one of each; invalid lists fall back to full rendering. Positive finite rowHeight/initialViewportHeight; complete flat data; bounded parent height required or all rows render.
    - [ ] Column windowing mounts nearby unpinned columns; pins remain. Custom renderers, wrapping, structural plugins, incompatible offsets/identity/geometry can force full rendering; default virtualized text truncates.
    - [ ] Semantic sections/rows/cells, horizontal scroll region (also vertical with bounded row windowing); scroll region enters keyboard order when overflowing.
    - [ ] Sort controls/direction glyphs and multi-sort priority; checkbox header/body controls; row expansion toggle/glyph/detail panel; rich cells with Badge, StatusDot, Avatar/Link; default compact empty state unless replaced/disabled.
    - [ ] `rowIndexStart` default 1 and optional `rowCount` support global ARIA ordinals independent of visible index columns; explicit offset without known total uses aria-rowcount=-1; logical positions include header.
    - [ ] RSC-safe data/width helpers versus client wrapper required for function props such as renderCell.

  - **Every associated named template**
    - [ ] [`TableShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableShowcase.doc.mjs) — “Table”: proportional/pixel columns, hover.
    - [ ] [`ColumnResizeHookUsage`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/ColumnResizeHookUsage.doc.mjs) — “useTableColumnResize — Draggable Columns”: release commits widths; last proportional column flexes.
    - [ ] [`ColumnsVirtualizationHookUsage`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/ColumnsVirtualizationHookUsage.doc.mjs) — “useTableColumnsVirtualization - Wide Table”: pinned sensor, horizontal windowing; caller retains all columns/data.
    - [ ] [`InfiniteScrollHookUsage`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/InfiniteScrollHookUsage.doc.mjs) — “useTableInfiniteScroll - Load More Products”: automatic or command loading, caller-owned Promise/loading/rows, command gone at completion.
    - [ ] [`RowsVirtualizationHookUsage`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/RowsVirtualizationHookUsage.doc.mjs) — “useTableRowsVirtualization - Fixed Height Rows”: bounded parent and visible row mounting.
    - [ ] [`StickyColumnsHookUsage`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/StickyColumnsHookUsage.doc.mjs) — “useTableStickyColumns — Pinned Columns”: logical start/end pins and shadows.
    - [ ] [`TableBulkActionsTable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableBulkActionsTable.doc.mjs) — “Table: Bulk Actions”: controlled selection, in-flow toolbar, actions start/count-clear end.
    - [ ] [`TableColumnSettingsTable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableColumnSettingsTable.doc.mjs) — “Table — Column Settings”: visibility picker.
    - [ ] [`TableFilterableTable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableFilterableTable.doc.mjs) — “Table — Popover Filters”: header-icon triggers.
    - [ ] [`TableFloatingBulkActionsTable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableFloatingBulkActionsTable.doc.mjs) — “Table: Floating Bulk Actions”: caller-sticky toolbar in capped scroll region without selection layout shift.
    - [ ] [`TableGridDividersTable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableGridDividersTable.doc.mjs) — “Table — Grid Dividers”: compact numeric grid.
    - [ ] [`TableGroupedRowsTable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableGroupedRowsTable.doc.mjs) — “useTableGroupedRows — Collapsible Groups”: full-width group headers/counts.
    - [ ] [`TableInCard`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableInCard.doc.mjs) — “Table — In Card”: heading and container bleed.
    - [ ] [`TableInlineFilterTable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableInlineFilterTable.doc.mjs) — “Table — Inline Filters”: controls below headings.
    - [ ] [`TableLeadingMarkSelection`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableLeadingMarkSelection.doc.mjs) — “Table — Selection in the Leading Mark”: avatar-slot checkboxes on hover/focus/any selection, always on touch; no automatic aria-selected/highlight without selection plugin.
    - [ ] [`TablePaginatedTable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TablePaginatedTable.doc.mjs) — “Table — Paginated Data”.
    - [ ] [`TableResizableTable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableResizableTable.doc.mjs) — “Table — Resizable Columns”.
    - [ ] [`TableRichCellTable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableRichCellTable.doc.mjs) — “Table — Rich Cell Content”: email Link, role Badge.
    - [ ] [`TableRowExpansionTable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableRowExpansionTable.doc.mjs) — “useTableRowExpansion — Tree Table”: inherited columns, depth indentation, chevron/right-click expansion.
    - [ ] [`TableRowIndexTable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableRowIndexTable.doc.mjs) — “useTableRowIndex — Numbered Rows”: right-aligned monospaced indices.
    - [ ] [`TableRowStatusTable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableRowStatusTable.doc.mjs) — “useTableRowStatus - Semantic and Custom Markers”: themed status glyph/tone versus custom color/icon/dot.
    - [ ] [`TableSelectableTable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableSelectableTable.doc.mjs) — “Table — Row Selection”: row and select-all checkboxes.
    - [ ] [`TableSortableTable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableSortableTable.doc.mjs) — “Table — Sortable Columns”: ascending/descending.
    - [ ] [`TableStripedTable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableStripedTable.doc.mjs) — “Table — Striped Rows”: alternating color plus hover.
    - [ ] [`TableTreeTable`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/TableTreeTable.doc.mjs) — “useTableTreeData: Tree Table”: nested state/data hooks, header expand-all/collapse-all, collapsed branches unmount.
    - [ ] [`useTableColumnSettingsStateHookUsage`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/useTableColumnSettingsStateHookUsage.doc.mjs) — “useTableColumnSettingsState — Column Visibility Picker”: checkbox menu and Reset.
    - [ ] [`useTableSortableStateHookUsage`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Table/useTableSortableStateHookUsage.doc.mjs) — “useTableSortableState — Multi-Sort Orders”: click then Shift-click priority, current state readout.

- [ ] **[Table Selection Toolbar](https://astryx.atmeta.com/components/TableSelectionToolbar)**

  - Sources: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/TableSelectionToolbar.doc.mjs), [SectionVariant definition in English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Section/Section.doc.mjs).

  - **Choices and state ownership**
    - [ ] `size`: `sm` (default), `md`, `lg`, cascaded to actions; `variant`: `section`, `transparent`, `muted` (default).
    - [ ] Shared `TableSelectionState`: selectedKeys, selectedCount, hasSelection, clearSelection; selected/no-selection controls count/visibility and clearing of complete selection.
    - [ ] State from useTableSelectionState versus caller-owned cross-server-page state; pair selectionConfig with Table plugin and selectionState with toolbar.
    - [ ] Product actions in logical-start `startContent`; selection count and clear at end. Label default `Bulk actions`; visible count default `${count} selected`; clearLabel `Unselect All`; custom renderSelectionLabel for localization/domain copy.
    - [ ] In-flow versus caller-positioned/sticky xstyle; component owns neither table nor sticky layout and is not injected into table scroll wrapper.

  - **Named inline example and associated family patterns**
    - [ ] “Select every matching row across server pages”: explicit picks versus all-matching minus exclusions, app-calculated count, loaded-only selectedKeys, complete clear; stable focusable switch between all matches and current page beside Archive. Bulk action sends query/exclusions, not a fabricated complete client key list.
    - [ ] No metadata files in this identifier's own directory. Related inspected Table templates: `TableBulkActionsTable` (“Table: Bulk Actions”) and `TableFloatingBulkActionsTable` (“Table: Floating Bulk Actions”).

- [ ] **[Tree List](https://astryx.atmeta.com/components/TreeList)**

  - Source: [English family API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/TreeList/TreeList.doc.mjs).

  - **Choices, data, and motion**
    - [ ] `density`: `compact`, `balanced` (default), `spacious`; `variant`: `lineGuides` (default), `noGuides`, independent of density.
    - [ ] Recursive items id/label, optional children/isExpanded and row xstyle/className/style; branch expanded/collapsed, leaf and fully-flat trees. No top-level controlled expansion pair documented here.
    - [ ] Optional header names tree via aria-labelledby; item label, description, start/end content, branch chevron/glyph and optional connector guides.
    - [ ] Leaves reserve chevron column when any branch expands; fully-flat tree is flush. Preexpanded important branches, selected-navigation and clickable/link item patterns in metadata.
    - [ ] Touch expansion glides rows/fades children and reverses for collapse; animatedLayout handle coordinates following content to container boundary across pointers; without handle content after changed row takes its place immediately.

  - **Named templates**
    - [ ] [`TreeListShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TreeList/TreeListShowcase.doc.mjs) — “TreeList” / “Tree List”.
    - [ ] [`TreeListFileTreeWithIcons`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TreeList/TreeListFileTreeWithIcons.doc.mjs) — “TreeList — File Tree With Icons”.
    - [ ] [`TreeListInteractiveSettings`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TreeList/TreeListInteractiveSettings.doc.mjs) — “TreeList — Interactive Settings”: clickable rows/documentation link.
    - [ ] [`TreeListMailboxTree`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TreeList/TreeListMailboxTree.doc.mjs) — “TreeList — Mailbox Tree”: unread badges.
    - [ ] [`TreeListNavigationTree`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TreeList/TreeListNavigationTree.doc.mjs) — “TreeList — Navigation Tree”: current page selection.
    - [ ] [`TreeListVariants`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TreeList/TreeListVariants.doc.mjs) — “Tree List — Variants”: guide lines versus indentation only.

## Utility

- [ ] **[VisuallyHidden](https://astryx.atmeta.com/components/VisuallyHidden)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/VisuallyHidden/VisuallyHidden.doc.mjs).

  - **Semantics and content patterns**
    - [ ] Truly no visual variants, sizes, or colors: intentionally invisible but exposed to accessibility tree; `xstyle` accepted and ignored only for 0.7.2 compatibility.
    - [ ] `as` is open ElementType, default `span`; documented block `div` for aria-live region; children supply accessible content.
    - [ ] Accessible name for icon-only control; polite dynamic announcement/result-count/drag-drop region; supplementary screen-reader context; structural hidden heading.
    - [ ] Not equivalent to hidden/remove-from-everyone; do not put interactive controls inside invisible, non-pointer-reachable content.
  - **Every associated template**
    - [ ] **VisuallyHidden — Structural Heading** (`VisuallyHiddenStructuralHeading`): name visually implicit section for screen-reader navigation. [Metadata](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/VisuallyHidden/VisuallyHiddenStructuralHeading.doc.mjs).
    - [ ] **VisuallyHidden — Live Region** (`VisuallyHiddenLiveRegion`): polite aria-live announces visual-only state changes. [Metadata](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/VisuallyHidden/VisuallyHiddenLiveRegion.doc.mjs).
    - [ ] **VisuallyHidden — Supplementary Context** (`VisuallyHiddenSupplementaryContext`): stat-card up/down arrows get explanatory screen-reader text (“increase from last month”), not just terse numerical change. [Metadata](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/VisuallyHidden/VisuallyHiddenSupplementaryContext.doc.mjs).
    - [ ] **VisuallyHidden** showcase (`VisuallyHiddenShowcase`): composition with IconButton/Card/stacks/Text/Icon; metadata lists download/share/audio/delete glyph aliases but no additional finite styling modes. [Metadata](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/VisuallyHidden/VisuallyHiddenShowcase.doc.mjs).

## Additional Components and Subcomponents from the Sidebar

### Avatar and Breadcrumbs

- [ ] **[Avatar Group Overflow](https://astryx.atmeta.com/components/AvatarGroupOverflow)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/AvatarGroup/AvatarGroupOverflow.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/AvatarGroupOverflow).

  - **Content and behavior**
    - [ ] No independent style/size enum; inherits group's size/shape. Default `+N` versus custom short children text; circle grows to pill for wider counts.
    - [ ] Required real `count` drives accessible label even with custom text; custom label must not contradict it.
    - [ ] Non-action indicator versus focusable button when `onClick` provided (member list/popover/detail trigger), with hover/focus states.
  - **Inline examples and templates**
    - [ ] Inline **Default overflow count**: sliced avatars followed by computed hidden count.
    - [ ] Inline **Custom count text**: custom `12+` while count remains 12.
    - [ ] **AvatarGroupOverflow — Custom Text** (`AvatarGroupOverflowCustomText.doc.mjs`).
    - [ ] **AvatarGroupOverflow — Default Count** (`AvatarGroupOverflowDefault.doc.mjs`).
    - [ ] **AvatarGroupOverflow** (`AvatarGroupOverflowShowcase.doc.mjs`): default and custom counts.

- [ ] **[Avatar Status Dot](https://astryx.atmeta.com/components/AvatarStatusDot)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Avatar/AvatarStatusDot.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/AvatarStatusDot).

  - **Choices and semantics**
    - [ ] `variant`: `success` (default, green filled), `neutral` (gray ring), `error` (red minus); distinct shapes prevent color-only status.
    - [ ] Size comes from avatar context, not a separate size prop; optional custom `icon` replaces shape glyph and hides at tiny sizes.
    - [ ] Accessible `label` joins parent avatar name; standalone dot exposes labelled `role="img"`.
  - **Templates**
    - [ ] **AvatarStatusDot** (`AvatarStatusDotShowcase.doc.mjs`): presence and automatic sizing.
    - [ ] **AvatarStatusDot — Variants** (`AvatarStatusDotVariants.doc.mjs`): success, neutral, error in status slot.

- [ ] **[Breadcrumb Item](https://astryx.atmeta.com/components/BreadcrumbItem)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Breadcrumbs/BreadcrumbItem.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/BreadcrumbItem).

  - **Modes and content**
    - [ ] Link `href`, action `onClick`, current plain location, or sibling-menu trigger; `menu` takes precedence over href/onClick.
    - [ ] `isCurrent=true` sets aria-current page; omitted auto-detects last when no explicit current; `false` opts out.
    - [ ] Children label and optional `startIcon`; custom link `as` overrides LinkProvider only for non-current items.
    - [ ] `menu`: DropdownMenuOption array or composed content; trigger chevron and aria-haspopup menu.
    - [ ] `menuSize`: `sm`, `md`, `lg`; default sm under supporting breadcrumbs, md otherwise. Enter/Space/ArrowDown opens menu, Escape closes.
  - **Templates**
    - [ ] **BreadcrumbItem — Basic** (`BreadcrumbItemBasic.doc.mjs`): ancestors linked, explicit current last item.
    - [ ] **BreadcrumbItem** (`BreadcrumbItemShowcase.doc.mjs`): links/icons/current/custom links.

### Chat

- [ ] **[Chat Composer Drawer](https://astryx.atmeta.com/components/ChatComposerDrawer)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Chat/ChatComposerDrawer.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ChatComposerDrawer).

  - **Disclosure and slots**
    - [ ] No visual variant/size enum; arbitrary attachment/context/preview children.
    - [ ] Omit `count` for always expanded; provide count to enable collapse toggle and collapsed count Badge/label (label default Items).
    - [ ] `isCollapsed` / `onCollapsedChange` controlled versus `defaultIsCollapsed=false` uncontrolled.
    - [ ] `collapsedSummary` replaces entire visual Badge/label summary but keeps disclosure/accessibility naming. Essential actions belong in persistent composer footer/send slots.
  - **Templates**
    - [ ] **ChatComposerDrawer — Attachments** (`ChatComposerDrawerAttachments.doc.mjs`): thumbnail carousel plus removable files, no count/always open.
    - [ ] **ChatComposerDrawer — Collapsible** (`ChatComposerDrawerCollapsible.doc.mjs`): count and toggle.
    - [ ] **ChatComposerDrawer — Feedback** (`ChatComposerDrawerFeedback.doc.mjs`): prompt and lettered selectable choices.
    - [ ] **ChatComposerDrawer** (`ChatComposerDrawerShowcase.doc.mjs`): file tokens, collapse, header actions.
    - [ ] **ChatComposerDrawer — With Progress** (`ChatComposerDrawerWithProgress.doc.mjs`): context budget progress in header.

- [ ] **[Chat Composer Input](https://astryx.atmeta.com/components/ChatComposerInput)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Chat/ChatComposerInput.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ChatComposerInput).

  - **Entry modes and behaviors**
    - [ ] Controlled value/onChange or composer-managed input; serialized string includes token placeholders. Default placeholder Type a message..., accessible label Message input, maxRows 8.
    - [ ] Trigger definitions (documented @ mentions and / commands): search source/onSelect token; `hasMultiWordQuery=true` extends query through spaces to line break/~64 characters; default query ends at first space; per-trigger ReactNode emptySearchText.
    - [ ] `debounceMs=150`; async suggestions. `hasHistory=true` default enables ArrowUp/Down recall; `isDisabled=false` default.
    - [ ] Enter without Shift submits, Shift+Enter does not use that submit path; IME composition never submits mid-composition. Custom onKeyDown runs after open trigger-menu handling but before Enter/history; preventDefault overrides submit, e.g. touch newline or Cmd/Ctrl+Enter.
    - [ ] `enterKeyHint`: `enter`, `done`, `go`, `next`, `previous`, `search`, `send` (default); use enter when custom touch Return inserts newline.
    - [ ] Plain paste onPaste can return true to take over; default >200-character paste becomes expandable token; `pasteAsToken=false` leaves all inline or hook customizes threshold/label.
    - [ ] `onFiles` handles pasted/dropped attachments. Handle methods: insertToken, expandToken, insertText, focus, getValue.
  - **Templates**
    - [ ] **ChatComposerInput — Controlled** (`ChatComposerInputControlledInput.doc.mjs`): live value display.
    - [ ] **ChatComposerInput — Disabled** (`ChatComposerInputDisabled.doc.mjs`).
    - [ ] **ChatComposerInput — Mentions** (`ChatComposerInputMentionTrigger.doc.mjs`): @ selection to token.
    - [ ] **ChatComposerInput — Multiple Triggers** (`ChatComposerInputMultipleTriggers.doc.mjs`): @ and / with distinct token colors.
    - [ ] **ChatComposerInput** (`ChatComposerInputShowcase.doc.mjs`): named showcase; metadata has no further description.
    - [ ] **ChatComposerInput — Slash Commands** (`ChatComposerInputSlashCommands.doc.mjs`).
    - [ ] **useChatDictation — Voice input** (`useChatDictationHookUsage.doc.mjs`): provider locale, partial ghost text, finalized phrase inserted once; unsupported stays off.
    - [ ] **useChatPasteAsToken — Long paste** (`useChatPasteAsTokenHookUsage.doc.mjs`): threshold 40/Pasted note, short paste inline (example threshold, not fixed style).
    - [ ] **useSpeechRecognition — Transcript** (`useSpeechRecognitionHookUsage.doc.mjs`): support/listening, partial words, finalized onResult.

- [ ] **[Chat Composer Token Element](https://astryx.atmeta.com/components/ChatComposerTokenElement)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Chat/ChatComposerTokenElement.doc.mjs). No associated template metadata supplied.

  - **Token content and behavior**
    - [ ] Structured badge config `{value,label,variant?,icon?}` versus custom `{value,render}`; no independent visual size/orientation enum.
    - [ ] Badge variant alternatives are `neutral`, `info`, `success`, `warning`, `error`, `blue`, `cyan`, `green`, `orange`, `pink`, `purple`, `red`, `teal`, `yellow` (Badge's documented default neutral), not invented token styles.
    - [ ] Renders chip outside contentEditable in data-astryx-token span; preserves token serialization and consistency with composer tokens. No separately named inline examples or templates in own doc.

- [ ] **[Chat Dictation Button](https://astryx.atmeta.com/components/ChatDictationButton)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Chat/ChatDictationButton.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ChatDictationButton).

  - **States and options**
    - [ ] `size`: `sm`, `md` (default); composer sendActions placement.
    - [ ] Dictation/speech-recognition return object owns listening, volume, frequency bands, toggle; idle microphone versus animated listening bars, start/stop activation.
    - [ ] `isHiddenWhenUnsupported=true` default hides; false leaves unsupported disabled button.
    - [ ] Accessible label override or state-derived Start dictation / Stop dictation; use composer with or without browser speech support.
  - **Templates**
    - [ ] **ChatDictationButton — Basic** (`ChatDictationButtonBasic.doc.mjs`): useChatDictation in composer sendActions.
    - [ ] **ChatDictationButton** (`ChatDictationButtonShowcase.doc.mjs`): live SpeechRecognition integration.

- [ ] **[Chat Layout Scroll Button](https://astryx.atmeta.com/components/ChatLayoutScrollButton)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Chat/ChatLayoutScrollButton.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ChatLayoutScrollButton).

  - **States and content**
    - [ ] `isVisible` hidden/visible; compact icon fades in when scrolled up, optional label expands for new messages. Label text is application content, not fixed variant values.
    - [ ] onClick typically scrolls to bottom and clears unread indicator; ChatLayout supplies by default, custom scrollButton replaces, null removes. No independent size/color enum documented.
  - **Templates**
    - [ ] **ChatLayoutScrollButton — Labels** (`ChatLayoutScrollButtonLabels.doc.mjs`): new-message/unread-reply/generic prompts.
    - [ ] **ChatLayoutScrollButton** (`ChatLayoutScrollButtonStates.doc.mjs`): hidden, visible icon, expanded labelled.

- [ ] **[Chat Message Bubble](https://astryx.atmeta.com/components/ChatMessageBubble)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Chat/ChatMessageBubble.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ChatMessageBubble).

  - **Choices and composition**
    - [ ] `variant`: `filled` (default, sender background), `ghost` (transparent but retains alignment padding).
    - [ ] `group`: `first`, `middle`, `last`, or omitted standalone full radius; grouped corners reduce on sender side.
    - [ ] Sender/density inherited from context; documented densities `compact`, `balanced`, `spacious`, sender styles user/assistant/system from ChatMessage.
    - [ ] Arbitrary children (text/Markdown/custom UI), optional first-bubble name and last-bubble metadata aligned to padding.
    - [ ] Null/undefined/booleans/empty string omit aligned wrappers, numeric zero remains; raw first/last content uses parent slots instead.
    - [ ] Optional width replaces default max(80%, 280px) cap; ghost width 100% supports full-column artifact/attachments.
  - **Templates**
    - [ ] **ChatMessageBubble — Custom Content** (`ChatMessageBubbleCustomContent.doc.mjs`): full-width ghost artifact and timestamp.
    - [ ] **ChatMessageBubble — Density** (`ChatMessageBubbleDensity.doc.mjs`): three padding/radius/spacing densities.
    - [ ] **ChatMessageBubble — Grouping** (`ChatMessageBubbleGrouping.doc.mjs`): first/middle/last.
    - [ ] **ChatMessageBubble — Metadata** (`ChatMessageBubbleMetadata.doc.mjs`): first name/last metadata.
    - [ ] **ChatMessageBubble** (`ChatMessageBubbleShowcase.doc.mjs`): filled user group/ghost agent.
    - [ ] **ChatMessageBubble — Variants** (`ChatMessageBubbleVariants.doc.mjs`): filled/ghost for user and assistant.

- [ ] **[Chat Message List](https://astryx.atmeta.com/components/ChatMessageList)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Chat/ChatMessageList.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ChatMessageList).

  - **Layout and async behavior**
    - [ ] `density`: `compact`, `balanced` (default), `spacious`, provided to messages.
    - [ ] Optional gap override: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`; omitted follows density.
    - [ ] `align`: `top`, `bottom` (default); spacer bottom-aligns short conversations only, overflow/auto-scroll remains same.
    - [ ] Message/system-message children or emptyState; `scrollToTopAction():Promise<void>` loads history, transition spinner at top while pending.
    - [ ] `role="log"`, aria-live polite; `isStreaming=false` default, true sets aria-busy to announce completed message once rather than every token.
  - **Templates**
    - [ ] **ChatMessageList — Density** (`ChatMessageListDensity.doc.mjs`): three densities and independent row gap.
    - [ ] **ChatMessageList — Full Featured** (`ChatMessageListFullFeatured.doc.mjs`): system/date dividers, grouped/ghost bubbles, Markdown/code, metadata.
    - [ ] **ChatMessageList** (`ChatMessageListShowcase.doc.mjs`): basic alternating conversation/date divider.

- [ ] **[Chat Send Button](https://astryx.atmeta.com/components/ChatSendButton)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Chat/ChatSendButton.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ChatSendButton).

  - **States and overrides**
    - [ ] `size`: `sm`, `md` (default).
    - [ ] Send (primary) versus stop (secondary) through `isStopShown`; default context value; disabled versus ready through isDisabled (default !context.canSend).
    - [ ] Context-wired onSend/onStop or standalone explicit states/callbacks; send defaults context onSubmit, stop defaults context onStop.
    - [ ] Default sendIcon arrowUp, stopIcon stop from registry, either customizable ReactNode.
  - **Templates**
    - [ ] **ChatSendButton — Custom Icon** (`ChatSendButtonCustomIcon.doc.mjs`): paper-plane/sparkles/check examples, not fixed icon enum.
    - [ ] **ChatSendButton — In Composer** (`ChatSendButtonInComposer.doc.mjs`): content readiness from context.
    - [ ] **ChatSendButton** (`ChatSendButtonShowcase.doc.mjs`): ready/custom/streaming.
    - [ ] **ChatSendButton — States** (`ChatSendButtonStates.doc.mjs`): disabled/ready/streaming at both sizes.

- [ ] **[Chat Tokenized Text](https://astryx.atmeta.com/components/ChatTokenizedText)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Chat/ChatTokenizedText.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ChatTokenizedText).

  - **Projection modes**
    - [ ] Plain string children and token definitions: structured Badge token versus caller-rendered custom token. No independent size/placement enum.
    - [ ] Structured token Badge variants: `neutral`, `info`, `success`, `warning`, `error`, `blue`, `cyan`, `green`, `orange`, `pink`, `purple`, `red`, `teal`, `yellow`; Badge default neutral.
    - [ ] Matches non-empty values literally; preserves unmatched text exactly; ignores empty token values to terminate safely.
  - **Templates**
    - [ ] **ChatTokenizedText — Basic** (`ChatTokenizedTextBasic.doc.mjs`): @mention values become display-name badges.
    - [ ] **ChatTokenizedText — Colors** (`ChatTokenizedTextColors.doc.mjs`): blue people/red bugs/green features, example taxonomy rather than prescribed semantic mapping.
    - [ ] **ChatTokenizedText** (`ChatTokenizedTextShowcase.doc.mjs`): named showcase, no further metadata description.

### Selection and Collapsible

- [ ] **[Checkbox List](https://astryx.atmeta.com/components/CheckboxList)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/CheckboxList/CheckboxList.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/CheckboxList).

  - **Choices and group modes**
    - [ ] `density`: `compact`, `balanced` (default), `spacious`; `hasDividers=false` default.
    - [ ] Collection mode value string array/onChange; without value, children may manage direct standalone checked state. No group defaultValue documented.
    - [ ] `isLabelHidden=false`, `isOptional=false`, `isRequired=false`, `isDisabled=false`, `isReadOnly=false` defaults; optional/required mutually exclusive.
    - [ ] Required group needs at least one selected; browser blocks submit and focuses first enabled editable checkbox, unless disabled/read-only group; caller supplies error text with status.
    - [ ] Async changeAction uses optimistic update; only toggled item spinner/aria-busy while pending.
    - [ ] Whole-group disabledMessage tooltip keeps checkboxes focusable and blocks toggles, not per-item reason; read-only full opacity/no change.
    - [ ] Label, description, CheckboxListItem children; status type `warning`, `error`, `success`, optional message, width. Container/control style slots target field/visual box, not native input; no root style props.
  - **Templates**
    - [ ] **CheckboxList — Select All With Indeterminate** (`CheckboxListSelectAllPattern.doc.mjs`): bulk select partial state.
    - [ ] **CheckboxList** (`CheckboxListShowcase.doc.mjs`): named showcase without further metadata description.
    - [ ] **CheckboxList — With End Content** (`CheckboxListWithEndContent.doc.mjs`): option price/status badges.

- [ ] **[Checkbox List Item](https://astryx.atmeta.com/components/CheckboxListItem)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/CheckboxList/CheckboxListItem.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/CheckboxListItem).

  - **Modes, slots, and interaction**
    - [ ] Collection mode requires identity value (throws without it); parent's value/onChange owns selection. Standalone in List/value-less CheckboxList uses isChecked/onCheck; ignored in collection mode.
    - [ ] Standalone `isChecked`: `false`, `true`, `indeterminate`; `isDisabled=false`, `isLoading=false` defaults. Loading spinner blocks item; parent pending change also supplies loading automatically.
    - [ ] `align`: `center` (default), `start`, `first-line` (checkbox aligned to wrapped first line).
    - [ ] Rich label, description, trailing endContent; label-derived accessible name and description via aria-describedby. Explicit aria-label when needed must preserve all visible label words; embedded links/buttons act independently without toggling.
    - [ ] Coarse pointer treats box/label/description as one pressed row target; fine pointer box has own press.
  - **Inline example and templates**
    - [ ] Inline **Rich label with an overriding aria-label**: Pro plan/Recommended badge retained in spoken name.
    - [ ] **CheckboxListItem — Basic** (`CheckboxListItemBasic.doc.mjs`): controlled notification choices with descriptions.
    - [ ] **CheckboxListItem** (`CheckboxListItemShowcase.doc.mjs`): label/description states including disabled.

- [ ] **[Collapsible Group](https://astryx.atmeta.com/components/CollapsibleGroup)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Collapsible/CollapsibleGroup.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/CollapsibleGroup).

  - **Group choices**
    - [ ] `type`: `single` (default, string open value), `multiple` (array of open values).
    - [ ] Controlled value/onChange or uncontrolled defaultValue; coordinates Collapsible children by their identifiers.
    - [ ] `hasDividers=false` default: no wrapper DOM; true adds wrapper/hairlines and default balanced padding density. Wrapper refs/style/test-id only apply when wrapper exists.
    - [ ] `density`: `compact`, `balanced`, `spacious`; omitted unpadded without dividers, balanced with dividers.
    - [ ] Shared `chevronPosition`: `start`, `end` (default); individual child may override, logical RTL behavior retained.
  - **Templates**
    - [ ] **CollapsibleGroup — Density** (`CollapsibleGroupAccordion.doc.mjs`): compact sidebar/balanced page/spacious primary content rows; hasDividers activates balanced.
    - [ ] **CollapsibleGroup** (`CollapsibleGroupShowcase.doc.mjs`): internally owned single-open state, one Card per child.

### Command Palette, Menus, and Dialogs

- [ ] **[Command Palette Empty](https://astryx.atmeta.com/components/CommandPaletteEmpty)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/CommandPalette/CommandPaletteEmpty.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/CommandPaletteEmpty).

  - **Content and behavior**
    - [ ] No finite visual choices documented; children message/content for no search results or empty no-query bootstrap, automatically rendered by parent palette.
  - **Templates**
    - [ ] **CommandPaletteEmpty — Basic** (`CommandPaletteEmptyBasic.doc.mjs`): custom emptyBootstrapText explains next step.
    - [ ] **CommandPaletteEmpty** (`CommandPaletteEmptyShowcase.doc.mjs`): no commands available.

- [ ] **[Command Palette Footer](https://astryx.atmeta.com/components/CommandPaletteFooter)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/CommandPalette/CommandPaletteFooter.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/CommandPaletteFooter).

  - **Content alternatives**
    - [ ] No visual variant/size enum; omitted children show built-in Kbd hints for arrows, Enter, Esc; children replace guidance with custom footer.
    - [ ] Parent renders default automatically; pass footer only to replace content; optional xstyle layout styling.
  - **Templates**
    - [ ] **CommandPaletteFooter — Basic** (`CommandPaletteFooterBasic.doc.mjs`): default navigation hints.
    - [ ] **CommandPaletteFooter** (`CommandPaletteFooterShowcase.doc.mjs`): custom tip.

- [ ] **[Command Palette Group](https://astryx.atmeta.com/components/CommandPaletteGroup)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/CommandPalette/CommandPaletteGroup.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/CommandPaletteGroup).

  - **Grouping content**
    - [ ] No finite visual variant documented; heading string and CommandPaletteItem children inside CommandPaletteList; optional layout xstyle.
    - [ ] Composed group alternative to data-driven auxiliaryData.group, not a separate selection mode.
  - **Templates**
    - [ ] **CommandPaletteGroup — Basic** (`CommandPaletteGroupBasic.doc.mjs`): composed labelled command sections.
    - [ ] **CommandPaletteGroup** (`CommandPaletteGroupShowcase.doc.mjs`): data-driven and composed grouping forms.

- [ ] **[Command Palette Input](https://astryx.atmeta.com/components/CommandPaletteInput)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/CommandPalette/CommandPaletteInput.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/CommandPaletteInput).

  - **Entry modes and slots**
    - [ ] Popup-backed query in palette or independent search field without popup; context value/onValueChange when omitted versus explicit controlled values.
    - [ ] `hasAutoFocus=true` default, automatically off in inline palette.
    - [ ] Placeholder default Search…; accessible label defaults translated Search commands, never placeholder-derived; caller may override.
    - [ ] Trailing endContent after spinner for clear control or shortcut hint; no size/color variant documented.
  - **Templates**
    - [ ] **CommandPaletteInput — With End Content** (`CommandPaletteInputBasic.doc.mjs`): custom placeholder and keyboard badge.
    - [ ] **CommandPaletteInput** (`CommandPaletteInputShowcase.doc.mjs`): search field/shortcut hint.

- [ ] **[Command Palette Item](https://astryx.atmeta.com/components/CommandPaletteItem)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/CommandPalette/CommandPaletteItem.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/CommandPaletteItem).

  - **Item states and selection**
    - [ ] `isHighlighted=false`, `isSelected=false`, `isDisabled=false` documented defaults; omitted highlighted/selected derived from palette context. Highlight (active result) differs from chosen picker selection.
    - [ ] Unique value identity; non-interactive children label/icon/description/shortcut; disabled suppresses pointer activation.
    - [ ] onSelect reports pointer activation; palette keyboard selection reports parent onValueChange. Standalone item only explicit visual state/pointer, no independent keyboard model.
    - [ ] In palette, listbox option with query focus/aria-activedescendant arrows and Enter selection; descendants presentational, so never nest interactive controls.
  - **Templates**
    - [ ] **CommandPaletteItem — Basic** (`CommandPaletteItemBasic.doc.mjs`): manually composed highlighted/disabled items.
    - [ ] **CommandPaletteItem** (`CommandPaletteItemShowcase.doc.mjs`): inline palette renderItem labels/shortcuts.

- [ ] **[Command Palette List](https://astryx.atmeta.com/components/CommandPaletteList)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/CommandPalette/CommandPaletteList.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/CommandPaletteList).

  - **Container behavior**
    - [ ] No finite style enum; scrollable result list with item/group/empty-state children, label default Commands.
    - [ ] Standalone listbox, and listbox inside palette while selectable results exist; grouping is composition rather than visual variant.
  - **Templates**
    - [ ] **CommandPaletteList — Item States** (`CommandPaletteListBasic.doc.mjs`): flat highlighted/selected/disabled options.
    - [ ] **CommandPaletteList** (`CommandPaletteListShowcase.doc.mjs`): grouped scrolling list with highlight, no full palette.

- [ ] **[Context Menu](https://astryx.atmeta.com/components/ContextMenu)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/ContextMenu/ContextMenu.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ContextMenu).

  - **Presentation, data, and states**
    - [ ] `presentation`: `popover` (default, cursor anchored), `bottom-sheet`, `adaptive` (sheet on coarse primary pointer with viewport below theme md/default 768px or height below 500px).
    - [ ] `size`: `sm`, `md` (default), `lg`; menuWidth default 160px, label default Context menu.
    - [ ] `triggerAs`: `div` (default block), `span` (inline); children trigger region and triggerXstyle.
    - [ ] Exactly one of data `items` or compound `menuContent`; item kinds action, `{type:"divider"}`, `{type:"section",title?,items}`.
    - [ ] Action label/onClick/icon/isDisabled/variant/nested items; destructive variant error color; nested items fly out in popover, drill to new view in sheet.
    - [ ] `isDisabled=false` default; true restores native browser context menu; onOpenChange reports open/closed.
    - [ ] Right-click pointer and long-press touch with adaptive/sheet; supply visible alternate actions for keyboard-only users and important mobile actions.
  - **Templates**
    - [ ] **ContextMenu — Basic** (`ContextMenuBasic.doc.mjs`): adaptive long-press sheet/right-click cursor menu.
    - [ ] **ContextMenu — Bottom Sheet** (`ContextMenuBottomSheet.doc.mjs`): forced sheet and alternate visible trigger guidance.
    - [ ] **ContextMenu** (`ContextMenuShowcase.doc.mjs`): adaptive context target.

- [ ] **[Context Menu Item](https://astryx.atmeta.com/components/ContextMenuItem)**

  - Sources: [own English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/ContextMenu/ContextMenuItem.doc.mjs), [alias export](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/ContextMenu/index.ts), [owning English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/DropdownMenu/DropdownMenuItem.doc.mjs), [original TSX](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/DropdownMenu/DropdownMenuItem.tsx), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/ContextMenuItem).

  - **Alias choices and behavior**
    - [ ] Re-export of DropdownMenuItem, not separate style API. `variant`: `default` (default), `destructive` (error label/description/icon).
    - [ ] `isDisabled=false` default: true aria-disables, removes href, suppresses activation; `hasCloseOnSelect=true` default, false keeps open for in-place result.
    - [ ] Action onClick versus href real menuitem anchor via LinkProvider; target `_blank`, `_self`; _blank adds noopener noreferrer.
    - [ ] Enter/Space synthesize modifier-preserving click; href callback runs before ordinary navigation but not modified/middle clicks, which remain browser navigation.
    - [ ] Non-interactive label, icon (semantic or custom source-supported ReactNode), description, endContent; no nested focusable controls.
    - [ ] Parent owns menu keyboard navigation and size `sm`, `md` (fallback), `lg`; item ref/host attributes target owned role menuitem row, focus/hover/tab semantics retained.
  - **Templates**
    - [ ] **ContextMenuItem — Basic** (`ContextMenuItemBasic.doc.mjs`): labels/secondary descriptions.
    - [ ] **ContextMenuItem** (`ContextMenuItemShowcase.doc.mjs`): custom icon/description rows.

- [ ] **[Alert Dialog](https://astryx.atmeta.com/components/AlertDialog)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/AlertDialog/AlertDialog.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/AlertDialog).

  - **Choices and content**
    - [ ] `actionVariant`: Button alternatives `primary`, `secondary`, `ghost`, `destructive`; default `destructive`.
    - [ ] Controlled `isOpen` / `onOpenChange`: open and closed; `isInline=false` by default, or inline preview with `role="group"` rather than modal semantics.
    - [ ] `isActionLoading`: spinner versus idle; `onAction` does not auto-close. Hold open during work and explicitly close after settlement.
    - [ ] Required title, consequence description, action label; configurable cancel label (default `Cancel`).
    - [ ] Width default 400; omitted width on a compact coarse-pointer phone uses 320; viewport/container clamping and scrolling body.
  - **Interaction and accessibility**
    - [ ] `alertdialog`, title via `aria-labelledby`, consequence via `aria-describedby`; initial focus on Cancel, focus returns to trigger; Escape cancels, outside click does not.
    - [ ] Above 640px actions are horizontal and may wrap; at/below 640px destructive action precedes Cancel and both fill width.
    - [ ] Touch/pen action-group slide tracking: release over action activates it, release outside both activates neither; mouse/keyboard/AT retain button activation.
    - [ ] Touch entrance scale/fade and exit fade; entrance blocks touch only, exit blocks every activation; reduced motion cross-fades; mouse keeps theme entrance and immediate close.
    - [ ] Imperative alternative: `useImperativeAlertDialog().show(options)` plus rendered `element`.
  - **Templates**
    - [ ] **AlertDialog — Loading** (`AlertDialogAsyncAction.doc.mjs`): pending confirmation spinner.
    - [ ] **AlertDialog — Delete** (`AlertDialogDeleteConfirmation.doc.mjs`): delete trigger and confirmation; also demonstrates imperative alert hook.

- [ ] **[Dialog Header](https://astryx.atmeta.com/components/DialogHeader)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Dialog/DialogHeader.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/DialogHeader).

  - **Content, focus, and choices**
    - [ ] Title non-empty/non-interactive inline ReactNode in focusable h2; receives initial focus and labels dialog unless parent supplies explicit aria-label/labelledby.
    - [ ] Subtitle supports inline Link, not block elements; optional startContent (e.g. back), endContent before close.
    - [ ] onOpenChange present renders close button calling false; absent omits button. `hasDivider=true` default versus no border.
    - [ ] `endContentEdgeCompensation`: `inline`, `block`, `all`; omitted preserves automatic close-action compensation. No independent size/color enum documented.
  - **Named inline examples and templates**
    - [ ] Inline **Basic**: title/subtitle, no close callback.
    - [ ] Inline **With close button**: callback false.
    - [ ] Inline **With a link in the subtitle**: sharing policy link.
    - [ ] Inline **With start and end content**: leading chevron and trailing Badge.
    - [ ] **DialogHeader — Basic** (`DialogHeaderBasic.doc.mjs`): Layout header/title/subtitle/close.
    - [ ] **DialogHeader** (`DialogHeaderShowcase.doc.mjs`): structured slots showcase.

- [ ] **[Drawer Header](https://astryx.atmeta.com/components/DrawerHeader)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Drawer/DrawerHeader.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/DrawerHeader).

  - **Content, options, and semantics**
    - [ ] Title inline/non-interactive h2, optional inline subtitle (Link permitted), startContent, endContent before close; no independent size/color enum.
    - [ ] onOpenChange present gives close button calling false, absent none; Drawer itself has no close control.
    - [ ] `hasDivider` true/false; omitted inherits Layout.defaultHasDividers, unlike DialogHeader's explicit true default.
    - [ ] `endContentEdgeCompensation`: `inline`, `block`, `all`; omitted automatic close-action compensation.
    - [ ] Compose in Layout header to avoid double inset. Unlike DialogHeader, does not autofocus or label drawer; parent Drawer label/data-autofocus owns those contracts.
  - **Named inline examples and templates**
    - [ ] Inline **With close button**: controlled Drawer and Layout header.
    - [ ] Inline **With subtitle and end content**: host title/location/status Badge.
    - [ ] **DrawerHeader** (`DrawerHeaderShowcase.doc.mjs`): structured slots showcase.
    - [ ] **With subtitle and end content** (`DrawerHeaderSubtitleEndContent.doc.mjs`): status before close.
    - [ ] **With close button** (`DrawerHeaderWithCloseButton.doc.mjs`): visible exit callback.

- [ ] **[Dropdown Menu Checkbox Item](https://astryx.atmeta.com/components/DropdownMenuCheckboxItem)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/DropdownMenu/DropdownMenuCheckboxItem.doc.mjs). No own named inline examples or template files.

  - **State, content, and semantics**
    - [ ] Controlled boolean `value` with `onChange(checked)`; independently checked/unchecked, not one-of-N selection.
    - [ ] `isDisabled`: false (default) / true; `aria-disabled`, activation blocked, arrow keys/typeahead still reach row, pointer highlight skips it.
    - [ ] `hasCloseOnSelect`: false (default, toggle several options) / true.
    - [ ] Label, secondary description, leading Icon, trailing `endContent` such as shortcut/badge; ref reaches row root.
    - [ ] Row owns `role="menuitemcheckbox"`; shared checkbox indicator is decorative. Family pattern: selectable rows inside a titled DropdownMenuGroup.

- [ ] **[Dropdown Menu Divider](https://astryx.atmeta.com/components/DropdownMenuDivider)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/DropdownMenu/DropdownMenuDivider.doc.mjs). No own named inline examples or templates.

  - **Actual behavior, without invented variants**
    - [ ] No documented size/color/orientation choice: a horizontal separator between compound-menu row groups.
    - [ ] `role="separator"`; never an arrow-key stop. Data equivalent `{type: 'divider'}` uses the same component/theme target.
    - [ ] Host ID/data/ARIA attributes and handlers reach separator; role/orientation remain component-owned; `xstyle` customizes spacing/layout, not a fixed visual variant.

- [ ] **[Dropdown Menu Group](https://astryx.atmeta.com/components/DropdownMenuGroup)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/DropdownMenu/DropdownMenuGroup.doc.mjs). Templates in `DropdownMenuGroup/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Grouping and content**
    - [ ] No finite style variants. Titled group uses visible non-interactive `title` as `aria-labelledby`; omitted title gives unnamed `role="group"`.
    - [ ] Children: DropdownMenuItem, DropdownMenuCheckboxItem, DropdownMenuRadioGroup, DropdownMenuSubMenu.
    - [ ] Headings skipped by arrow keys/typeahead; interactive controls belong in menu header/footer, not title.
    - [ ] Data equivalent `{type: 'section', title, items}` shares heading treatment; host props and ref reach group root.

  - **Named templates**
    - [ ] `DropdownMenuGroupShowcase` — **DropdownMenuGroup**: named compound row groups, headings skipped during navigation.
    - [ ] `DropdownMenuGroupWithSelectable` — **DropdownMenuGroup — Selectable rows**: titled radio and checkbox groups unavailable through the data API.

- [ ] **[Dropdown Menu Item](https://astryx.atmeta.com/components/DropdownMenuItem)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/DropdownMenu/DropdownMenuItem.doc.mjs). Templates in `DropdownMenuItem/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Variants and states**
    - [ ] `variant`: `default` (default), `destructive` (error-colored label, description, icon).
    - [ ] `isDisabled`: false (default) / true; disabled exposes ARIA state, removes link destination, blocks activation.
    - [ ] `hasCloseOnSelect`: true (default) / false for in-place results.
    - [ ] Action callback versus `href` destination; `target`: `_blank`, `_self`; `_blank` merges `noopener noreferrer`.

  - **Content and interaction**
    - [ ] Required label, optional leading icon, secondary description, non-interactive trailing `endContent`; no nested focusable controls.
    - [ ] Parent owns keyboard navigation; keyboard activation synthesizes click with modifiers. For links, ordinary callback precedes navigation; modified/middle clicks retain browser behavior and skip callback.
    - [ ] Host props, composed handlers, style, and ref reach `role="menuitem"` root; LinkProvider participates in link rendering.

  - **Named templates**
    - [ ] `DropdownMenuItemBasic` — **DropdownMenuItem — Basic**: label and secondary description.
    - [ ] `DropdownMenuItemShowcase` — **DropdownMenuItem**: compound items with icons/descriptions.

- [ ] **[Dropdown Menu Radio Group](https://astryx.atmeta.com/components/DropdownMenuRadioGroup)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/DropdownMenu/DropdownMenuRadioGroup.doc.mjs). No own named inline examples/templates.

  - **Controlled choice and close policy**
    - [ ] Required controlled string `value` and `onChange`; `undefined` means no selection yet, not an uncontrolled default.
    - [ ] `hasCloseOnSelect`: true (default single-choice commit) / false for continued interaction.
    - [ ] Required accessible `label` supplies `aria-label` fallback even with visible `aria-labelledby`; group contains DropdownMenuRadioItems only.
    - [ ] `role="group"` wraps `menuitemradio` choices; parent DropdownMenu owns arrow navigation; group owns value propagation and spacing. Ref reaches group root.
    - [ ] Family example pattern: titled selectable group alongside checkbox rows (`DropdownMenuGroupWithSelectable`).

- [ ] **[Dropdown Menu Radio Item](https://astryx.atmeta.com/components/DropdownMenuRadioItem)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/DropdownMenu/DropdownMenuRadioItem.doc.mjs). No own named inline examples/templates.

  - **Selection, disabled state, and slots**
    - [ ] Required item `value` matches parent's controlled value: checked versus unchecked; must be inside DropdownMenuRadioGroup.
    - [ ] `isDisabled`: false (default) / true; still reachable by arrow/typeahead and announced disabled; pointer skips it and selection is blocked.
    - [ ] Required label; optional description, leading Icon, trailing metadata/badge; ref reaches `role="menuitemradio"` root.
    - [ ] No own finite visual variant/size/color API; close policy comes from radio group. Family pattern: single-select rows in titled compound groups.

- [ ] **[Dropdown Menu Submenu](https://astryx.atmeta.com/components/DropdownMenuSubMenu)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/DropdownMenu/DropdownMenuSubMenu.doc.mjs). No own named inline examples/templates; `DropdownMenuWithSubmenu` demonstrates the family pattern.

  - **Presentation and loading**
    - [ ] `presentation`: `flyout`, `drill-in`, `adaptive` (default; coarse-pointer decision taken when menu opens).
    - [ ] `isDisabled`: false (default) / true, never opens when disabled; `hasSpinner`: false (default) / true, replacing caret while lazy children load.
    - [ ] Hover `delay` default 150ms; `hideDelay` default 200ms; keyboard/touch do not wait.
    - [ ] Custom minimum `menuWidth`, default intrinsic sizing with 160px minimum; can grow for content but capped to viewport.

  - **Content and navigation**
    - [ ] Required label and child menu rows, optional icon/description; nested submenus and selectable children supported. Data API uses nested `items` instead.
    - [ ] Flyout opens inline-end and auto-flips; Right/Enter/Space opens and focuses first row; Left/Escape closes and restores trigger focus; arrows swap in RTL.
    - [ ] Drill-in replaces rows in same surface, with `Back to <parent>`; Back/Escape/ArrowLeft return to parent row.
    - [ ] Optional header/footer outside rows: surface becomes modal dialog named by trigger, `aria-haspopup="dialog"`, Tab/Shift+Tab wrap, Escape returns; pointer leave closes only after focus also leaves. ContextMenu does not show these slots.
    - [ ] `onOpenChange` reports visibility; row host props/ref and separate `menuDataTestId`; own open keys and disabled-click behavior cannot be overwritten by caller handlers.

### Fields and Indicators

- [ ] **[Field Label](https://astryx.atmeta.com/components/FieldLabel)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Field/FieldLabel.doc.mjs). Templates in `FieldLabel/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Modes and content**
    - [ ] `isGroupLabel`: false (default native label) / true span naming caller's group through labelID/aria-labelledby.
    - [ ] `isLabelHidden`: false (default) / true, hiding label and description visually but retaining both for assistive technology.
    - [ ] `isDisabled`: false (default) / true; `isOptional` and `isRequired`: each false by default, localized indicators suppressed when matching FormLayout default.
    - [ ] Required label/inputID; optional description with descriptionID, icon before label, tooltip after label. Supporting text remains separate from accessible name.
    - [ ] Tooltip named `About {label}`; hover/focus preview and tap/click/Enter/Space disclosure do not activate/focus underlying control.

  - **Named templates**
    - [ ] `FieldLabelBasic` — **FieldLabel — Basic**: custom-control labels, required/optional indicators, description.
    - [ ] `FieldLabelShowcase` — **FieldLabel**: required, optional, tooltip, icon variations.

- [ ] **[Field Status](https://astryx.atmeta.com/components/FieldStatus)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/FieldStatus/FieldStatus.doc.mjs). Templates in `FieldStatus/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Feedback choices and announcements**
    - [ ] Required `type`: `error`, `warning`, `success`; required message; optional ID for `aria-describedby`.
    - [ ] `variant`: `attached` (default, overlapping input), `detached` (separate feedback below).
    - [ ] Persistent live regions: errors assertive, others polite; initial conditional mount and message/type changes announce safely.
    - [ ] Repeated identical failed submission does not reannounce each field: form announces once with useAnnounce and focuses first invalid field; first failure relies on fields' announcements.

  - **Named templates**
    - [ ] `FieldStatusBasic` — **FieldStatus — Basic**: detached error/success for checkboxes, switches, custom controls.
    - [ ] `FieldStatusShowcase` — **FieldStatus**: error/warning/success and attached/detached.

- [ ] **[Input Group](https://astryx.atmeta.com/components/InputGroup)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/InputGroup/InputGroup.doc.mjs). Templates in `InputGroup/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Shared field state**
    - [ ] `size`: `sm`, `md` (default), `lg`, inherited by inputs.
    - [ ] `isLabelHidden`, `isDisabled`, `isOptional`, `isRequired`: each false by default / true.
    - [ ] `status.type`: `warning`, `error`, `success`; optional message below group; group border does not automatically change (theme can use data-status).
    - [ ] Required accessible label and children; helper description and labelTooltip.

  - **Connected content patterns**
    - [ ] Prefix/suffix InputGroupText (text/icon), attached Button action; focused member raised so focus ring stays visible.
    - [ ] Compatible controls: TextInput, NumberInput, TimeInput, DateInput, Typeahead, Tokenizer, Selector, MultiSelector.
    - [ ] Inner specific labels combine with group label and inherit description/status; multi-value controls must remain short/summarized on fixed-height single line.
    - [ ] Not TextArea, Slider, Switch, CheckboxInput, RadioList or unrelated multiple text fields.

  - **Named templates**
    - [ ] `InputGroupBasic` — **InputGroup — Basic**: currency input, static prefix and suffix.
    - [ ] `InputGroupShowcase` — **InputGroup**: connected text-input group showcase.

- [ ] **[Input Group Text](https://astryx.atmeta.com/components/InputGroupText)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/InputGroup/InputGroupText.doc.mjs). No own named inline examples/templates.

  - **Actual content and behavior**
    - [ ] No finite visual variants. Prefix versus suffix placement inside InputGroup; required short text or decorative icon children.
    - [ ] Plain non-interactive text, single line, does not shrink; not part of input accessible name, read in ordinary order.
    - [ ] Currency symbol, protocol prefix, unit suffix are sample content, not a fixed style enumeration; input label carries essential unit meaning.
    - [ ] Plain text preserves character order in RTL; rich mixed-direction content uses caller dir/bdi. Actions use Button addon instead.
    - [ ] Family example: currency prefix/suffix (`InputGroupBasic`); do not use outside InputGroup.

- [ ] **[Checkbox Indicator](https://astryx.atmeta.com/components/CheckboxIndicator)**

  - Sources: [owning English Indicator family doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Indicator/Indicator.doc.mjs), [original TSX](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Indicator/CheckboxIndicator.tsx), [type contract](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Indicator/types.ts). Own ID doc is absent because API belongs to Indicator family; resolved from pinned family/source. No own templates supplied.

  - **Decorative states and customization**
    - [ ] Required state `unchecked`, `checked`, `indeterminate`; persistent empty box, checkmark, bar respectively. This is multiSelection, not a native input.
    - [ ] `size`: `sm` (20px), `md` (24px, default); `isDisabled=false` default, purely visual.
    - [ ] `children` replaces mark inside chrome, e.g. pending Spinner. Use shallow isRenderable: null/undefined/booleans/empty string do not replace; elements/containers do even if descendants render nothing.
    - [ ] Always aria-hidden, non-interactive; owning control retains role/name/state/keyboard and paints focus ring on single indicator root; replacement root radius determines ring shape. Hover/press comes from owner's ancestor CSS marker, not extra props.
    - [ ] Theme restyle checkbox-indicator/check/dash targets or replace `indicators.checkbox`; token-driven colors/radius/borders, forced-color-visible check/bar and reduced-motion transition handling in source.
  - **Named family/source examples**
    - [ ] **Restyle an indicator (the common path)**: rounded box, thicker border, checked/disabled styling through theme targets.
    - [ ] **Replace an indicator with your own component**: BrandCheckbox, token styles, aria-hidden and children-first replacement handling.
    - [ ] **Use radio visuals for single selection**: family example changes check indicator, not checkbox's three-state contract.
    - [ ] TSX example: indeterminate sm checkbox visual.

- [ ] **[Check Indicator](https://astryx.atmeta.com/components/CheckIndicator)**

  - Sources: [owning English family doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Indicator/Indicator.doc.mjs), [original TSX](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Indicator/CheckIndicator.tsx), [types](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Indicator/types.ts). Own doc absent; family ownership resolved. No own template metadata supplied.

  - **Decorative choices and behavior**
    - [ ] Required state `unchecked`, `checked`; default unchecked renders nothing/no reserved chrome, checked draws glyph. No indeterminate state in singleSelection family.
    - [ ] `size`: `sm`, `md` (default); both source mappings use sm Icon. `isDisabled=false` default; accent versus disabled icon foreground.
    - [ ] Renderable children replace mark in either state, e.g. pending Spinner even when unchecked; shallow renderability excludes null/undefined/booleans/empty string, not empty-rendering elements.
    - [ ] Always decorative aria-hidden, no role/name/keyboard/focus ownership; host owns semantics. Check has no separate chrome/theme target: host target shares Icon element; replacement-content branch forwards props to span.
    - [ ] Theme `indicators.check` replacement can draw both states, e.g. RadioIndicator gives unchecked empty circle; host always mounts indicator rather than hiding it based on selection.
  - **Named family/source examples**
    - [ ] **Restyle an indicator (the common path)**: family target override recipe.
    - [ ] **Replace an indicator with your own component**: family replacement contract, token use, children-first behavior.
    - [ ] **Use radio visuals for single selection**: map check to RadioIndicator globally.
    - [ ] Source examples: checked/unchecked selection expression with size sm; replace check with RadioIndicator through defineTheme.

- [ ] **[Radio Indicator](https://astryx.atmeta.com/components/RadioIndicator)**

  - Sources: [Indicator family English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Indicator/Indicator.doc.mjs), [RadioIndicator.tsx](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Indicator/RadioIndicator.tsx). No standalone doc or supplied templates.

  - **Decorative state and size**
    - [ ] Required state `unchecked`, `checked`; no partial/indeterminate public state in singleSelection family.
    - [ ] `size`: `sm` (20px circle/8px dot), `md` (default, 24px circle/10px dot); isDisabled false (default) / true visual dimming only.
    - [ ] Empty circle when unchecked versus filled circle/dot checked; circle remains circular independent of radius-full theme token.
    - [ ] children replace inner state mark when shallow isRenderable accepts them; excludes null/undefined/booleans/empty string, not empty-rendering element/container descendants.
    - [ ] Decorative aria-hidden root, no input/role/focus/keyboard ownership; owner supplies semantics/focus ring; hover/press via ancestor CSS, reduced-motion no transition, forced-color dot stays perceivable.
    - [ ] Theme restyling radio-indicator/radio-indicator-dot versus whole radio replacement; mapping check to RadioIndicator also draws unchecked circles for selection slots.

  - **Named family inline examples and source example**
    - [ ] **Restyle an indicator (the common path)**: shared circle/dot component targets, state overrides.
    - [ ] **Replace an indicator with your own component**: family replacement recipe, token styles and isRenderable; example uses checkbox, not a claimed extra radio state.
    - [ ] **Use radio visuals for single selection**: theme check indicator replaced by RadioIndicator.
    - [ ] Source example: checked md RadioIndicator.

- [ ] **[Item](https://astryx.atmeta.com/components/Item)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Item/Item.doc.mjs). Templates in `Item/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Layout and state**
    - [ ] `align`: `center` (default), `start`, `first-line`; first-line aligns small marks with first label line, taller slots move text down.
    - [ ] `density`: `compact` (4px block padding), `balanced` (default, 8px), `spacious` (12px block/inline).
    - [ ] `layout`: `stacked` (default), `inline` (single row, description ellipsizes first).
    - [ ] `as`: `div` (default), `li`, `span`, or open-ended element/component; custom root link requires role and no interactive start/end descendants.
    - [ ] `isHighlighted`, `isSelected`, `isDisabled`: each false by default / true; disabled interactive row remains keyboard-reachable but inert.
    - [ ] Label/description line limits are arbitrary counts, not finite sizes.

  - **Slots and activation modes**
    - [ ] Required label as string (automatic truncation) or rich ReactNode; marker before startContent; leading icon/avatar/checkbox; secondary description; trailing badge/timestamp/metadata/action.
    - [ ] Static row versus onClick button surface versus href invisible anchor; root already a link avoids extra anchor. Target `_blank`/`_self`; _blank merges safe rel.
    - [ ] `interactiveRef` delegates row surface to nested control with no extra invisible control/tab stop; mutually exclusive onClick/href are ignored.

  - **Swipe behavior**
    - [ ] `swipe={createSwipe(...)}` with leading and/or trailing actions, `swipeBehavior`: `reveal` or `commit`; action `variant`: `neutral`, `accent`, `destructive`; action enabled/disabled and `hasRemoval` hold-out versus spring-back.
    - [ ] Action fields: stable ID, label, optional icon, onActivate({id,itemKey}); plain Item itemKey undefined. Swipe definitions can be module-scoped/memoized.
    - [ ] Finger, pen, trackpad two-finger swipe work; mouse drag/wheel notches do not; mostly vertical movement remains scrolling. Leading reveals toward inline-end, trailing toward inline-start.
    - [ ] Open row closes on its touch content tap (without activation/press paint), press elsewhere, scrolling with focus outside, Escape.
    - [ ] Swipe never sole action path: duplicate verbs in accessible row controls/menu; allowed on listitem/no-role rows, not option/menuitem/native-radio delegation rows; host clips inline overflow.
    - [ ] Same action IDs preserve open/settling state; changed IDs close. Modern createSwipe code is import-gated and available first paint.
    - [ ] Deprecated `swipeActions` and `swipeBehavior` still work, default reveal, ignored when modern swipe supplied; legacy lazy-load path retained.

  - **Named templates**
    - [ ] `ItemBasicItem` — **Item — Basic**: label, description, trailing timestamp.
    - [ ] `ItemDocumentTabs` — **Item — Document Tabs**: compact selected document rows, leading file glyph, reveal-on-hover reserved close controls, separators, external add button, horizontal scroll.
    - [ ] `ItemShowcase` — **Item**: avatar/badge/icon/text row compositions.
    - [ ] `ItemWithMedia` — **Item — Start Content**: leading avatars/icons.
    - [ ] `ItemWithMetadata` — **Item — Metadata**: build/memory/sync status, timestamp/badge end content.

### Layout

- [ ] **[Center](https://astryx.atmeta.com/components/Center)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Center/Center.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Center).

  - **Layout choices**
    - [ ] `axis`: `both` (default), `horizontal`, `vertical`; single-axis behavior follows logical flex main/cross axes in vertical writing, not guaranteed physical names.
    - [ ] `isInline=false` default versus inline-flex for inline icon/badge content.
    - [ ] Each of `padding`, `paddingInline`, `paddingInlineStart`, `paddingInlineEnd`, `paddingBlock`, `paddingBlockStart`, `paddingBlockEnd` accepts `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`; axis overrides all-padding, edge overrides axis.
    - [ ] Children and arbitrary width/height/maxWidth/minHeight; selected axis needs available space. Layout only, adds no role/name; children retain semantics.
  - **Templates**
    - [ ] **Center — Horizontal Center** (`CenterHorizontal.doc.mjs`): formatting controls, writing-mode caveat.
    - [ ] **Center — Vertical & Horizontal Center** (`CenterInsideACard.doc.mjs`): fixed-height empty state icon/heading/description.
    - [ ] **Center** (`CenterShowcase.doc.mjs`): both-axis fixed-height content.

- [ ] **[Grid Span](https://astryx.atmeta.com/components/GridSpan)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Grid/GridSpan.doc.mjs). Templates in `GridSpan/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Span behavior**
    - [ ] `columns`: arbitrary numeric count or finite sentinel `full` (all explicit tracks); `rows`: numeric span. Counts are layout data, not a closed size scale.
    - [ ] Child content inside Grid; explicit columns/rows replace corresponding grid-column/grid-row inline styles; wrapper ref and host styling supported.
    - [ ] No other documented size/color/variant API.

  - **Named templates**
    - [ ] `GridSpanColumns` — **GridSpan — Columns**: two-of-three-column asymmetric composition.
    - [ ] `GridSpanShowcase` — **GridSpan**: multiple column/row spanning and masonry-style composition.

- [ ] **[H Stack](https://astryx.atmeta.com/components/HStack)**

  - Sources: [Stack family English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Stack/Stack.doc.mjs), [HStack.tsx](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/HStack/HStack.tsx). No standalone doc; family provides the exact HStack contract. Templates in `HStack/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Horizontal layout choices**
    - [ ] Fixed horizontal direction along inline axis: LTR left-to-right, RTL right-to-left; do not invent a vertical HStack mode.
    - [ ] `hAlign` / `justify`: `start`, `center`, `end`, `between`, `around`, `evenly`; explicit hAlign wins alias.
    - [ ] `vAlign` / `align`: `start`, `center`, `end`, `stretch` (default); explicit vAlign wins alias.
    - [ ] `wrap`: `nowrap` (default), `wrap`, `wrap-reverse`; `isScrollable`: false (default) / true.
    - [ ] Each of `gap`, `padding`, `paddingInline`, `paddingInlineStart`, `paddingInlineEnd`, `paddingBlock`, `paddingBlockStart`, `paddingBlockEnd`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10` as numbers, not strings. Edge override beats axis override, which beats broad padding.
    - [ ] Polymorphic `as` (default div), arbitrary children, width/height/maxWidth/minHeight; dimension props beat same inline-style properties; StackItem fill can occupy spare room.

  - **Named templates and source example**
    - [ ] `HStackBasic` — **HStack — Basic**: horizontal siblings, consistent gap, centered vertical alignment.
    - [ ] `HStackShowcase` — **HStack**: varying gaps/alignments.
    - [ ] Source example: HStack with gap 2 and sibling items; gap 2 is example data, not a documented universal default.

- [ ] **[Layout Content](https://astryx.atmeta.com/components/LayoutContent)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Layout/LayoutContent.doc.mjs). Templates in `LayoutContent/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Content-region modes**
    - [ ] `isScrollable`: true (default) / false; arithmetic contentWidth scrollport extends through open sides with aligned child insets.
    - [ ] `padding`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`, overriding enclosing layout padding.
    - [ ] Arbitrary children and optional landmark role/label; scroll region label defaults to translated `Content` if omitted; root ref supported.
    - [ ] No own color/size/variant enum.

  - **Named templates**
    - [ ] `LayoutContentBasic` — **LayoutContent — Basic**: scrolling main content below fixed header.
    - [ ] `LayoutContentShowcase` — **LayoutContent**: automatic padding/scroll containment between header/footer.

- [ ] **[Layout Footer](https://astryx.atmeta.com/components/LayoutFooter)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Layout/LayoutFooter.doc.mjs). Templates in `LayoutFooter/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Footer modes and content**
    - [ ] `hasDivider`: false documented default / true top border; unset follows Layout.defaultHasDividers.
    - [ ] `padding`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`; replaces inherited inset, zero full-bleed.
    - [ ] Arbitrary height and children: action bars, pagination, status bars; optional landmark role/label, forwarded ref. No own finite color/size variant.

  - **Named templates**
    - [ ] `LayoutFooterActions` — **LayoutFooter — Actions**: fixed end-aligned Save/Cancel footer under scroll content.
    - [ ] `LayoutFooterShowcase` — **LayoutFooter**: bottom-pinned persistent action/navigation slot.

- [ ] **[Layout Header](https://astryx.atmeta.com/components/LayoutHeader)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Layout/LayoutHeader.doc.mjs). Templates in `LayoutHeader/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Header modes and content**
    - [ ] `hasDivider`: false documented default / true bottom border; unset follows Layout.defaultHasDividers.
    - [ ] `padding` and `paddingBlockEnd`: each `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`; block-end overrides that edge only, zero docks last child at bottom.
    - [ ] Arbitrary height and children: page title, app bar, toolbar; optional landmark role/label and root ref. No own finite color/size variant.

  - **Named templates**
    - [ ] `LayoutHeaderShowcase` — **LayoutHeader**: top-pinned title/navigation/actions.
    - [ ] `LayoutHeaderWithActions` — **LayoutHeader — With Actions**: title plus primary action above scroll content.

- [ ] **[Layout Panel](https://astryx.atmeta.com/components/LayoutPanel)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Layout/LayoutPanel.doc.mjs). Templates in `LayoutPanel/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Panel states and regions**
    - [ ] Logical start versus end slot: divider faces main content, mirrors in RTL.
    - [ ] `hasDivider`: false (default) / true; independent of Layout.defaultHasDividers, leave false beside handle drawing its own divider.
    - [ ] `isScrollable`: true (default) / false; optional role/label, scroll name defaults to translated Content.
    - [ ] `padding`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`; replaces automatic inset/shared gap; zero full-bleed.
    - [ ] Fixed arbitrary `width` versus `resizable` region/props, which owns width; region axis `horizontal`/`vertical` must match adjacent ResizeHandle.
    - [ ] Hook-owned region slides collapsed/expanded from handle edge; hand-built props do not slide. Both move contained focus to handle and make collapsed panel inert, also supported for adjacent panel sized from region.size.
    - [ ] Arbitrary navigation/settings/inspector children; no finite background variants.

  - **Named templates**
    - [ ] `LayoutPanelNavigation` — **LayoutPanel — Navigation**: fixed-width nav list beside content.
    - [ ] `LayoutPanelShowcase` — **LayoutPanel**: sidebar/detail/secondary-content slot.

- [ ] **[Stack Item](https://astryx.atmeta.com/components/StackItem)**

  - Sources: [owning Stack family API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Stack/Stack.doc.mjs), [original StackItem TSX](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Stack/StackItem.tsx). No standalone English doc exists; the family contains its full member documentation, so this is resolved rather than a missing API.

  - **Choices and behavior**
    - [ ] `size`: `static` (default; intrinsic, does not grow/shrink), `fill` (remaining space).
    - [ ] `crossAlignSelf`: `start`, `center`, `end`, `stretch`; overrides parent cross-axis alignment.
    - [ ] `isScrollable`: false (default), true; min-width/min-height reset means fill + scrolling forms a complete flex scroll region.
    - [ ] Polymorphic `as` (default div); div/span overflow region becomes a group named by aria-label or translated `Content`; other element types preserve their own role. Optional children and root ref/style integration.

  - **Examples and named templates**
    - [ ] TSX example: HStack with static Logo, fill Content, static Actions.
    - [ ] [`StackItemFill`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/StackItem/StackItemFill.doc.mjs) — “StackItem — Fill”: fixed-width neighbor and remaining-space item.
    - [ ] [`StackItemShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/StackItem/StackItemShowcase.doc.mjs) — “StackItem” / “Stack Item”: optional wrapper inside HStack/VStack for sizing/alignment.

- [ ] **[V Stack](https://astryx.atmeta.com/components/VStack)**

  - Sources: [owning English Stack family doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Stack/Stack.doc.mjs), [original VStack TSX](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/VStack/VStack.tsx), [Stack implementation](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Stack/Stack.tsx).

  - **Direction, alignment, and wrapping**
    - [ ] Fixed vertical/top-to-bottom wrapper; direction omitted from VStackProps, not a horizontal/vertical variant selector.
    - [ ] `hAlign` cross-axis: `start`, `center`, `end`, `stretch` (documented default).
    - [ ] `vAlign` main-axis: `start`, `center`, `end`, `between`, `around`, `evenly`.
    - [ ] `align` cross-axis alias: `start`, `center`, `end`, `stretch`; `justify` main-axis alias: `start`, `center`, `end`, `between`, `around`, `evenly`; explicit hAlign/vAlign override aliases.
    - [ ] `wrap`: `nowrap` (default), `wrap`, `wrap-reverse`.
  - **Spacing and dimensions**
    - [ ] `gap`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`; numeric JSX values, not strings; playground's gap `2` is sample content, not documented API default.
    - [ ] `padding`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`.
    - [ ] `paddingInline`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`.
    - [ ] `paddingInlineStart`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`.
    - [ ] `paddingInlineEnd`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`.
    - [ ] `paddingBlock`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`.
    - [ ] `paddingBlockStart`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`.
    - [ ] `paddingBlockEnd`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`; edge overrides axis, axis overrides all-edge padding, respecting logical RTL edges.
    - [ ] `width`, `height`, `maxWidth`, `minHeight`: arbitrary number px or CSS string, override matching inline style properties; no finite size presets.
  - **Content and scroll behavior**
    - [ ] `isScrollable`: `false` (default) / `true` overflow auto; when flex child must scroll use parent StackItem fill/scrollable min-size reset.
    - [ ] Polymorphic `as` default `div`, arbitrary children; optional StackItem controls individual fill/static/cross-alignment behavior rather than adding VStack variants.
    - [ ] StyleX layout customization, className, style and forwarded ref supported; family theme target `astryx-stack` reflects direction/gap/wrap. No color/elevation/type variants documented.
    - [ ] Family playground: three Cards with padding `3` at gap `2`; unnamed source example: two Items inside VStack gap `2`.
  - **Every associated template**
    - [ ] **VStack — Basic** (`VStackBasic`): heading and paragraphs with consistent top-to-bottom gap. [Metadata](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/VStack/VStackBasic.doc.mjs).
    - [ ] **VStack** / display name **V Stack** showcase (`VStackShowcase`): vertically arranged items with different gaps, composed with Layout/Badge/Text; metadata does not enumerate additional gap presets. [Metadata](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/VStack/VStackShowcase.doc.mjs).

### Lists and Navigation

- [ ] **[List Item](https://astryx.atmeta.com/components/ListItem)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/List/ListItem.doc.mjs). Templates in `ListItem/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Alignment, selection, and content**
    - [ ] `align`: `center` (default), `start`, `first-line`; inherited List density/edge treatment, not an independent size/color enum.
    - [ ] `isDisabled`: false (default) / true; keyboard-reachable aria-disabled row cannot activate.
    - [ ] `isSelected`: false (default) / true; uses aria-current=true, never aria-selected on listitem.
    - [ ] Required label and optional description: strings auto single-line truncate; rich nodes own wrapping/clamping. Start icon/avatar, end badge/chevron/count/timestamp.
    - [ ] Static versus onClick invisible button versus href invisible anchor; target/rel and safe _blank tokens; shared Link URL rule.
    - [ ] interactiveRef delegates to nested control with no duplicate tab stop; onClick/href ignored in that mode.
    - [ ] itemKey supplies animation/reordering/swipe identity separate from React key; missing key means immediate updates/no reorder handle.

  - **Swipe modes**
    - [ ] Modern createSwipe leading/trailing entries and `reveal`/`commit`; `neutral`/`accent`/`destructive` action variants, disabled and removal-hold states; forwards itemKey to action callback.
    - [ ] Same touch/pen/trackpad, inline clipping, dismissal, and keyboard/mouse alternative-action requirements as Item.swipe.
    - [ ] Deprecated swipeActions/swipeBehavior retained (reveal default) but ignored when modern swipe provided.

  - **Named templates**
    - [ ] `ListItemBasicItem` — **List Item — Basic**: label/description collection.
    - [ ] `ListItemShowcase` — **ListItem**: icons, descriptions, end slots.
    - [ ] `ListItemWithMedia` — **List Item — Media**: leading avatar/icon.
    - [ ] `ListItemWithMetadata` — **List Item — Metadata**: trailing badge/count/time/status.

- [ ] **[Metadata List Item](https://astryx.atmeta.com/components/MetadataListItem)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/MetadataList/MetadataListItem.doc.mjs). Templates in `MetadataListItem/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Actual content and inherited layout**
    - [ ] No own finite visual variants: required string label and rich value children; optional leading icon before label.
    - [ ] Parent chooses side-by-side versus stacked layout; style/ref target label dt side-by-side, wrapper when stacked.
    - [ ] Test-ID pattern: side-by-side label/value get -label/-value suffix; stacked wrapper gets unsuffixed ID.

  - **Named templates**
    - [ ] `MetadataListItemBasic` — **MetadataListItem — Basic**: titled deployment record, text/status Badge values.
    - [ ] `MetadataListItemShowcase` — **MetadataListItem**: text, badges, links as values.

- [ ] **[Mobile Nav](https://astryx.atmeta.com/components/MobileNav)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/MobileNav/MobileNav.doc.mjs). Templates in `MobileNav/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Drawer presentation and control**
    - [ ] `side`: `start`, `end`, `auto` (default based on trigger position); logical side mirrors RTL.
    - [ ] Width default 320px, capped at 85vw; arbitrary width, not a finite size scale.
    - [ ] AppShell context-owned open state versus explicit isOpen/onOpenChange outside AppShell.
    - [ ] Required SideNavSection/SideNavItem or arbitrary children; optional header string versus custom logo/search/ReactNode versus omitted title.
    - [ ] Backdrop, Escape, close button request dismissal; focus returns to opener after dialog closes.
    - [ ] On coarse primary pointer, entry slide ignores taps/drags until settled; keyboard/assistive technology remain active.
    - [ ] MobileNavToggle reads context automatically, not passed isOpen/onOpenChange; desktop uses persistent SideNav.

  - **Named templates**
    - [ ] `MobileNavBasicMobileNav` — **MobileNav — Basic Drawer**: sectioned navigation, menu-button trigger.
    - [ ] `MobileNavEndSideMobileNav` — **MobileNav — End Side Drawer**: end-side slide (metadata describes right in LTR).
    - [ ] `MobileNavShowcase` — **MobileNav**: icon/button/SideNav composition.
    - [ ] `MobileNavWithoutTitleMobileNav` — **MobileNav — Without Title**: no title header.

- [ ] **[Mobile Nav Toggle](https://astryx.atmeta.com/components/MobileNavToggle)**

  - Sources: [MobileNav family English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/MobileNav/MobileNav.doc.mjs), [MobileNavToggle.tsx](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/MobileNav/MobileNavToggle.tsx). No standalone doc. Templates in `MobileNavToggle/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Actual states and content**
    - [ ] No own finite style/size variants; source fixes ghost icon-only Button, default menu glyph versus custom children.
    - [ ] Accessible label defaults translated `Open navigation`; override for custom control meaning.
    - [ ] Context-managed open/closed via AppShell; accepts neither isOpen nor onOpenChange. Button owns aria-expanded/aria-controls; caller onClick runs after toggle.
    - [ ] Renders nothing above mobile breakpoint, without mobile context, or when nav disabled/absent/custom mobileNav node replaces automatic nav.
    - [ ] Host props/ref/styles reach button; source examples place toggle in custom toolbar or supply custom icon/label.

  - **Named templates**
    - [ ] `MobileNavToggleBasic` — **MobileNavToggle — Basic**: custom icon/label, AppShell mobile context opens drawer.
    - [ ] `MobileNavToggleShowcase` — **MobileNavToggle**: standalone hamburger pattern. Metadata componentsUsed lists MobileNav/Button/SideNav rather than the toggle itself; treat as pattern metadata, not evidence of extra toggle API.

- [ ] **[Nav Heading Menu](https://astryx.atmeta.com/components/NavHeadingMenu)**

  - Sources: [NavMenu family English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/NavMenu/NavMenu.doc.mjs), [NavHeadingMenu.tsx](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/NavMenu/NavHeadingMenu.tsx). No standalone doc. Template in `NavHeadingMenu/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Size and content**
    - [ ] `size`: `sm` (160px min), `md` (default, 200px min), `lg` (240px min); flows item padding context.
    - [ ] Arbitrary minWidth override wins size minimum; children NavHeadingMenuItem, dividers/custom content; pass menu to SideNavHeading or TopNavHeading.
    - [ ] No controlled open prop on menu itself; parent heading injects close callback; selection dismisses where parent policy provides it.

  - **Navigation and source examples**
    - [ ] role menu, arrow navigation, Home/End, Escape close; first-character typeahead includes disabled rows; Enter/Space synthesize activation, disabled rows stay inert.
    - [ ] Held press highlight follows pointer, release acts on enabled row underneath; mouse released outside dismisses, including expanded heading context.
    - [ ] Caller capture cancellation is tracked; own accessibility keyboard handling composes with supported host handlers; ref reaches root.
    - [ ] Source example: large product-switcher menu supplied to SideNavHeading with destination rows.

  - **Named template**
    - [ ] `NavHeadingMenuShowcase` — **NavHeadingMenu**: SideNavHeading product-switcher popover.

- [ ] **[Nav Icon](https://astryx.atmeta.com/components/NavIcon)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/NavIcon/NavIcon.doc.mjs). Templates in `NavIcon/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Actual behavior**
    - [ ] No finite variants/size/color props: circular accent-background container with required icon ReactNode.
    - [ ] Display-only navigation-header visual anchor, not button; Icon or similarly sized glyph; TopNavHeading/PageNavHeader composition.

  - **Named templates**
    - [ ] `NavIconBasic` — **NavIcon — Basic**: semantic icons as logo/accent visuals.
    - [ ] `NavIconShowcase` — **NavIcon**: circular accent containers.

- [ ] **[Side Nav Collapse Button](https://astryx.atmeta.com/components/SideNavCollapseButton)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/SideNav/SideNavCollapseButton.doc.mjs).

  - **Choices, content, and ownership**
    - [ ] `size`: `sm`, `md`, `lg`; defaults to container size (`sm` in SideNav footer), otherwise `md`.
    - [ ] Default icon-only ghost button; `label` changes it to text plus chevron; `children` overrides default icon/label.
    - [ ] Inside SideNav reads context; outside receives the same controlled `collapsible` config (`isCollapsed`, `onCollapsedChange`, optional default/button settings). Deprecated imperative `handleRef` alternative is documented.
    - [ ] Toggle both expanded and collapsed rail states; manually positioned button uses `hasButton: false` to suppress the built-in one.

  - **Named templates**
    - [ ] [`SideNavCollapseButtonBasic`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/SideNavCollapseButton/SideNavCollapseButtonBasic.doc.mjs) — “SideNavCollapseButton — Basic”: footer placement.
    - [ ] [`SideNavCollapseButtonShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/SideNavCollapseButton/SideNavCollapseButtonShowcase.doc.mjs) — “SideNavCollapseButton” / “Side Nav Collapse Button”: collapsible rail.

- [ ] **[Side Nav Heading](https://astryx.atmeta.com/components/SideNavHeading)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/SideNav/SideNavHeading.doc.mjs).

  - **Content and interaction patterns**
    - [ ] No finite visual variant/size/color is documented. Required heading; optional icon, superheading, subheading; each text tier has its own optional href.
    - [ ] Optional menu popover; smart link/menu interaction boundaries rather than one nested interactive target.
    - [ ] `headerEndContent` between text and chevron: badge, status indicator or compact action; hidden when rail collapses.

  - **Named templates**
    - [ ] [`SideNavHeadingBasic`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/SideNavHeading/SideNavHeadingBasic.doc.mjs) — “SideNavHeading — Basic”: icon and linked title in header slot.
    - [ ] [`SideNavHeadingShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/SideNavHeading/SideNavHeadingShowcase.doc.mjs) — “SideNavHeading” / “Side Nav Heading”: logo, heading, superheading and subheading.

- [ ] **[Side Nav Item](https://astryx.atmeta.com/components/SideNavItem)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/SideNav/SideNavItem.doc.mjs).

  - **Choices and state**
    - [ ] `size`: `sm`, `md` (default), `lg`; `isSelected` and `isDisabled` default false.
    - [ ] Icon and selectedIcon (outline versus filled); selected item marks current page. Link via href/custom `as`, or click callback.
    - [ ] `collapsible`: false (default), true (uncontrolled, initially expanded), or config with `defaultIsCollapsed` / controlled `isCollapsed` + `onCollapsedChange`.
    - [ ] Nested children occupy full nav width with one content-indent step per level; expanded/collapsed disclosure and collapsed-rail flyout are family behaviors.

  - **Slots and accessibility**
    - [ ] Required label; passive `endContent` for badges/counts; interactive `actions` are sibling controls after disclosure toggle and before children in DOM/focus order.
    - [ ] Actions own accessible names, inherit row control size unless overridden, and hide when rail collapses.

  - **Named templates**
    - [ ] [`SideNavItemBasic`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/SideNavItem/SideNavItemBasic.doc.mjs) — “SideNavItem — Basic”: icon links and current page.
    - [ ] [`SideNavItemShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/SideNavItem/SideNavItemShowcase.doc.mjs) — “SideNavItem” / “Side Nav Item”: selected, icon, disabled, nested states.

- [ ] **[Side Nav Section](https://astryx.atmeta.com/components/SideNavSection)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/SideNav/SideNavSection.doc.mjs).

  - **Content and state**
    - [ ] No finite style variants documented. Title (required in prop table, despite description saying optional), optional subtitle, item children, header `endContent` such as icon/badge.
    - [ ] `isHeaderHidden`: false (default), true; visually hidden header remains accessible.

  - **Named templates**
    - [ ] [`SideNavSectionBasic`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/SideNavSection/SideNavSectionBasic.doc.mjs) — “SideNavSection — Basic”: titled navigation clusters.
    - [ ] [`SideNavSectionShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/SideNavSection/SideNavSectionShowcase.doc.mjs) — “SideNavSection” / “Side Nav Section”: grouped items.

- [ ] **[Top Nav Heading](https://astryx.atmeta.com/components/TopNavHeading)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/TopNav/TopNavHeading.doc.mjs).

  - **Content and interaction modes**
    - [ ] No finite style variants documented. Logo as image/NavIcon/arbitrary node; heading; optional superheading/subheading and independent href for each; trailing headerEndContent badge/status.
    - [ ] No menu and only headingHref: entire heading is one link. Menu with no hrefs: entire header triggers popover. Menu with hrefs: independent inline links plus chevron trigger.
    - [ ] Menu adds automatic chevron; custom link `as` overrides LinkProvider and must accept href/className/style/children.

  - **Named templates**
    - [ ] [`TopNavHeadingBasic`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNavHeading/TopNavHeadingBasic.doc.mjs) — “TopNavHeading — Basic”: linked logo heading.
    - [ ] [`TopNavHeadingShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNavHeading/TopNavHeadingShowcase.doc.mjs) — “TopNavHeading” / “Top Nav Heading”: plain versus linked.

- [ ] **[Top Nav Item](https://astryx.atmeta.com/components/TopNavItem)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/TopNav/TopNavItem.doc.mjs).

  - **States, semantics, and content**
    - [ ] `isSelected`, `isDisabled`, `isIconOnly`: false (default), true; selected sets aria-current=page; disabled sets aria-disabled, removes href/target and blocks interaction.
    - [ ] Label/link with optional icon versus icon-only square (requires icon, label becomes aria-label); children replace label text; custom `as` link overrides provider.
    - [ ] No independent finite size/color/variant documented; normal/hover/selected appearance belongs to item semantics.

  - **Named templates**
    - [ ] [`TopNavItemBasic`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNavItem/TopNavItemBasic.doc.mjs) — “TopNavItem — Basic”: selected current page.
    - [ ] [`TopNavItemShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNavItem/TopNavItemShowcase.doc.mjs) — “TopNavItem” / “Top Nav Item”: default, icon, selected, disabled.

- [ ] **[Top Nav Mega Menu Item](https://astryx.atmeta.com/components/TopNavMegaMenuItem)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/TopNav/TopNavMegaMenuItem.doc.mjs).

  - **Behavior and content**
    - [ ] No finite size/color/style variants documented. Desktop popover grid versus mobile drawer via context; title, optional description/left icon.
    - [ ] href navigation/custom `as` link versus onClick action; `hasCloseOnSelect`: true (default), false. Keyboard/finger/pen pick closes and returns trigger focus by default; mouse pick leaves open; false supports in-place action result.

  - **Named templates**
    - [ ] [`TopNavMegaMenuItemBasic`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNavMegaMenuItem/TopNavMegaMenuItemBasic.doc.mjs) — “TopNavMegaMenuItem — Basic”: rich link cards in grid.
    - [ ] [`TopNavMegaMenuItemShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TopNavMegaMenuItem/TopNavMegaMenuItemShowcase.doc.mjs) — “TopNavMegaMenuItem” / “Top Nav Mega Menu Item”: icons/titles/descriptions in menu.

- [ ] **[Radio List Item](https://astryx.atmeta.com/components/RadioListItem)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/RadioList/RadioListItem.doc.mjs). Templates in `RadioListItem/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Item state and slots**
    - [ ] `align`: `center` (default), `start`, `first-line`; isDisabled false (default) / true for individual choice.
    - [ ] Required rich label and string value; checked/unchecked derives from controlled RadioList, size inherits group (sm/md).
    - [ ] Description may contain independently working links/buttons; optional content before circle and after label; non-interactive row surface delegates to radio.
    - [ ] Label text names radio; optional aria-label targets control, should preserve visible label words or supply name for textless rich label.
    - [ ] Coarse-pointer enabled invisible hit area grows toward 44px, never below row size or past half neighbor gap; row ::after owns it, decorate child instead.

  - **Named templates**
    - [ ] `RadioListItemBasic` — **RadioListItem — Basic**: controlled shipping-method-style label/description choices.
    - [ ] `RadioListItemShowcase` — **RadioListItem**: descriptions and disabled items.

- [ ] **[Segmented Control Item](https://astryx.atmeta.com/components/SegmentedControlItem)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/SegmentedControl/SegmentedControlItem.doc.mjs). Templates in `SegmentedControlItem/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Item state and content**
    - [ ] Required value/label, selection from parent controlled value; radio within group, no independent selected boolean.
    - [ ] isLabelHidden false (default) / true: visible label versus icon-only with label as aria-label; icon optionally precedes text.
    - [ ] isDisabled false (default) / true for individual segment; disabled selected segment keeps group's Tab stop.
    - [ ] Size/layout inherited from parent (sm/md/lg, hug/fill), not extra item variants.

  - **Named templates**
    - [ ] `SegmentedControlItemBasic` — **SegmentedControlItem — Basic**: label-only view options, parent state owns selection.
    - [ ] `SegmentedControlItemShowcase` — **SegmentedControlItem**: text/icons plus disabled item.

- [ ] **[Selector Option](https://astryx.atmeta.com/components/SelectorOption)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Selector/SelectorOption.doc.mjs).

  - **Layout and content**
    - [ ] `layout`: `stacked` (default), `inline`; InputGroup forces inline. Selector trigger padding accommodates either layout.
    - [ ] Required ReactNode label; optional semantic icon, ReactNode description, trailing `endContent`.
    - [ ] Use through Selector `renderOption`, not direct Selector JSX children; selection/disabled interaction belongs to the owning row, not invented helper variants.

  - **Named templates**
    - [ ] [`SelectorOptionBasic`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/SelectorOption/SelectorOptionBasic.doc.mjs) — “SelectorOption — Basic”: secondary descriptions via renderOption.
    - [ ] [`SelectorOptionShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/SelectorOption/SelectorOptionShowcase.doc.mjs) — “SelectorOption” / “Selector Option”: icons and descriptions.

- [ ] **[Step](https://astryx.atmeta.com/components/Step)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Stepper/Step.doc.mjs).

  - **Progress, indicators, and state**
    - [ ] Progress derives from step index versus parent activeStep: completed, active/current, not-started/upcoming; not directly set on Step.
    - [ ] `status`: unset (progress-derived coloring), `accent`, `success`, `warning`, `error`; semantic color is independent of progress.
    - [ ] `indicator`: `auto` (default), `number`, `none`, custom ReactNode. Prop prose says number until complete/check after; family templates additionally describe current auto as a ring. Treat that as progress-dependent auto behavior, not an invented extra indicator value.
    - [ ] `density`: `compact`, `balanced`, `spacious`, or inherit parent; `isDisabled`/`isOptional` default false.

  - **Content and examples**
    - [ ] Label is one-line ellipsized without losing accessible full name; optional description, optional marker and trailing endContent.
    - [ ] Children sit below description, aligned with label and outside clickable label area; render only active content for expanding vertical workflows. Compact horizontal hides but retains mounted local state.
    - [ ] [`StepShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Step/StepShowcase.doc.mjs) — “Step”: all parts on one Step in its owning Stepper.
    - [ ] [`StepContent`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Step/StepContent.doc.mjs) — “Step — Content Slot”: fields/buttons, gated by active step.
    - [ ] [`StepIndicator`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Step/StepIndicator.doc.mjs) — “Step — Indicator”: auto, always-number, custom icon, none; stable 16px box; live inherit-shade Spinner indicator.
    - [ ] [`StepStates`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Step/StepStates.doc.mjs) — “Step — States”: completed/current/upcoming, disabled, status axis, including completed warning.

### Table, Tabs, and Typeahead

- [ ] **[Table Body](https://astryx.atmeta.com/components/TableBody)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/TableBody.doc.mjs).

  - **Behavior and composition**
    - [ ] No independent finite visual variants. Semantic tbody wrapper with row children and optional xstyle; supplied by caller in children mode, automatic in data mode.
    - [ ] Keep body rows inside section to avoid server/client implied-tbody mismatch. Inherits owning Table context rather than declaring new styles.
    - [ ] No own named inline examples or associated template metadata; family Table patterns above cover composition.

- [ ] **[Table Cell](https://astryx.atmeta.com/components/TableCell)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/TableCell.doc.mjs).

  - **Behavior and content**
    - [ ] No independent finite visual variants. Semantic td with arbitrary children; TableContext supplies density padding/font and divider borders.
    - [ ] Inherited density `compact`, `balanced`, `spacious`; divider `rows`, `columns`, `grid`, `none` belong to Table, not local cell props.
    - [ ] No own named inline examples or associated template metadata.

- [ ] **[Table Footer](https://astryx.atmeta.com/components/TableFooter)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/TableFooter.doc.mjs).

  - **Behavior and composition**
    - [ ] No independent finite visual variants. Semantic tfoot for summary/totals rows, optional xstyle.
    - [ ] Children mode only; data-driven path renders header/body but never footer. Keep rows in section to avoid hydration mismatch.
    - [ ] No own named inline examples or associated template metadata.

- [ ] **[Table Header](https://astryx.atmeta.com/components/TableHeader)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/TableHeader.doc.mjs).

  - **Behavior and composition**
    - [ ] No independent finite visual variants. Semantic thead with TableRow/TableHeaderCell children and optional xstyle.
    - [ ] Caller supplies in children mode; data mode creates it when columns are supplied. Keep rows in section for matching server/client trees.
    - [ ] No own named inline examples or associated template metadata.

- [ ] **[Table Header Cell](https://astryx.atmeta.com/components/TableHeaderCell)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/TableHeaderCell.doc.mjs).

  - **Behavior and content**
    - [ ] No independent finite visual variants. Semantic th with header children; TableContext supplies density padding, semibold weight, secondary color and divider borders.
    - [ ] Inherited density `compact`, `balanced`, `spacious` and dividers `rows`, `columns`, `grid`, `none` are parent choices.
    - [ ] No own named inline examples or associated template metadata.

- [ ] **[Table Row](https://astryx.atmeta.com/components/TableRow)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/TableRow.doc.mjs).

  - **Behavior and content**
    - [ ] No independent finite visual variants. Semantic tr holding cells; reads TableContext for stripes, hover and divider styles.
    - [ ] Inherited striped/unstriped, hover-enabled/disabled and divider `rows`, `columns`, `grid`, `none`; place inside header/body/footer section.
    - [ ] No own named inline examples or associated template metadata.

- [ ] **[Tab](https://astryx.atmeta.com/components/Tab)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/TabList/Tab.doc.mjs).

  - **State, semantics, and slots**
    - [ ] Selected/unselected derives from required value and owning TabList; `isLabelHidden` false (default)/true uses label as aria-label.
    - [ ] Button versus href anchor/custom link `as`; explicit role=tablist on parent ignores href. `panelId` wires aria-controls only in tabs pattern; navigation-pattern use warns and has no effect.
    - [ ] Default icon versus selectedIcon (fallback to icon); optional passive endContent badge/status/decorative info icon. Do not place interactive controls inside a tab.

  - **Named inline examples and templates**
    - [ ] “Underline the whole tab in one strip”: nested theme best-effort tab-indicator insetInline=0, editor navigation strip, inherited color mode; not a guaranteed theme target property.
    - [ ] “Describe a tab with help text”: external ref-anchored sibling Tooltip, decorative endContent, panelId-linked tabpanels.
    - [ ] [`TabShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Tab/TabShowcase.doc.mjs) — “Tab”: labels, icons, selected icons, end slots.
    - [ ] [`TabWithSelectedIcon`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Tab/TabWithSelectedIcon.doc.mjs) — “Tab — Selected Icon”.

- [ ] **[Tab Menu](https://astryx.atmeta.com/components/TabMenu)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/TabList/TabMenu.doc.mjs).

  - **Content and behavior**
    - [ ] No independent finite visual choices documented. Required label plus TabMenuOption array; unselected trigger shows label, selected option replaces trigger text; label also supplies heading divider.
    - [ ] Closed/open dropdown state; host id/aria/data and DOM handlers reach trigger button, not popup/options; component retains role, tab stop and disclosure attributes. Selection belongs to TabList.

  - **Named templates**
    - [ ] [`TabMenuBasic`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TabMenu/TabMenuBasic.doc.mjs) — “TabMenu — Basic”: secondary tabs behind end-of-strip dropdown.
    - [ ] [`TabMenuShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/TabMenu/TabMenuShowcase.doc.mjs) — “TabMenu” / “Tab Menu”: selected option label as trigger.

- [ ] **[Base Typeahead](https://astryx.atmeta.com/components/BaseTypeahead)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Typeahead/BaseTypeahead.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/BaseTypeahead).

  - **Modes and search**
    - [ ] Bare combobox engine, no wrapper/border/selected-value token; caller supplies named input chrome and focus treatment.
    - [ ] `size`: `sm`, `md` (default), `lg` for result padding; controlled selected `value: T|null` / `onChange` and query/open notifications.
    - [ ] `menu.role`: `listbox` (default) with optional surrounding render content, or labelled focus-taking `dialog` with custom render. Interactive listbox actions stay results for arrow reachability.
    - [ ] Open state is internal, not controlled: `handleRef.open()` / `close()`, `onOpenChange` reporting. Listbox-to-dialog switch can keep menu open by calling open from onChange.
    - [ ] `hasEntriesOnFocus=false` default; bootstrap results when enabled. `minQueryLength=1`, `maxMenuItems=10`, `debounceMs=150`; zero debounce for synchronous or self-debouncing progressive source.
    - [ ] SearchSource search/bootstrap; `context.update(items)` supplies early matches, returned list is final; later lists preserve shown-row ordering and append new rows.
    - [ ] `emptySearchText` default `No results found`; ReactNode/link/create row supported; null/undefined fall back, empty string suppresses visible text. DOM text is politely announced, excluding aria-hidden content.
  - **States and interaction**
    - [ ] `isDisabled=false`, `isFocusableDisabled=false`, `hasAutoFocus=false` defaults; focusable-disabled uses aria-disabled/readOnly, blocks entry/open/selection including already-open results; Escape/Tab can still close.
    - [ ] `onKeyDown` runs before internal navigation; preventDefault suppresses internal handling. Dialog Escape/outside/close restores pre-open focus.
    - [ ] Custom result rendering; default placeholder `Search…`; custom anchorRef and requested menuWidth (viewport-clamped).
    - [ ] Legacy input aliases `inputTabIndex`, `inputId`, `ariaDescribedBy`, `ariaLabelledBy` override corresponding native attributes when supplied.
  - **Templates**
    - [ ] **BaseTypeahead — Custom Search Bar** (`BaseTypeaheadCustomSearch.doc.mjs`): own chrome/icon, wrapper anchor.
    - [ ] **BaseTypeahead — Menu** (`BaseTypeaheadMenu.doc.mjs`): listbox field picker becomes value-editing dialog; imperative reopen and Escape return.
    - [ ] **BaseTypeahead** (`BaseTypeaheadShowcase.doc.mjs`): metadata-rich renderer with option semantics/keyboard retained.

### Providers and Themes

- [ ] **[Internationalization Provider](https://astryx.atmeta.com/components/InternationalizationProvider)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/i18n/InternationalizationProvider.doc.mjs). Templates in `InternationalizationProvider/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Actual provider options**
    - [ ] Required open-ended BCP47 `locale`; regional fallback to base language, then shipped English; without provider uses English defaults.
    - [ ] Optional shipped/generated or compatible local `messages` catalogs; sparse locale-keyed `overrides`; live locale changes choose corresponding catalog/overrides.
    - [ ] `dir`: `ltr`, `rtl`, or omitted locale-derived direction via Intl.Locale.getTextInfo. Provider does not set DOM dir; keep provider and html/wrapper direction synchronized.
    - [ ] `calendar`: `gregory`, `iso8601`, `buddhist`, `roc`, `japanese`, `persian`, `indian`, `hebrew`, `chinese`, `dangi`, `coptic`, `ethiopic`, `ethioaa`, `islamic-civil`, `islamic-tbla`, `islamic-umalqura`.
    - [ ] Omitted calendar is Gregorian regardless of locale/default/Unicode ca keyword; component calendar overrides provider; stored dates stay Gregorian ISO.
    - [ ] gregory/iso8601/buddhist/roc/japanese need no Temporal; other calendars require Temporal/polyfill on server and browser, otherwise render Gregorian and warn in development while context retains request.
    - [ ] Children receive localized component strings and calendar formatting for Calendar/date inputs/Timestamp/PowerSearch/Schedule. No visual size/color variants.

  - **Named inline examples**
    - [ ] **Load a shipped Astryx locale catalog**: import generated catalog, supply locale/messages.
    - [ ] **Mirror the layout for right-to-left text**: same derived direction on provider and DOM wrapper.
    - [ ] **Override one Astryx string**: sparse locale-keyed selector placeholder override.
    - [ ] **Provide a local fallback catalog**: typed MessagesByLocale for unshipped/local catalog.
    - [ ] **Show Calendar in the Persian calendar**: explicit calendar with Gregorian stored values.
    - [ ] **Show Calendar and DateInput in another calendar**: Japanese calendar, shared controlled ISO date.

  - **Named templates**
    - [ ] `InternationalizationProvider01ShippedLocale` — **Load a shipped Astryx locale catalog**: switch locale live.
    - [ ] `InternationalizationProvider02Overrides` — **Override one Astryx string**: sparse override, not entire replacement catalog.
    - [ ] `InternationalizationProvider03RtlDirection` — **Mirror the layout for right-to-left text**: provider plus VStack dir, mirrored pagination chevrons.
    - [ ] `InternationalizationProvider04Calendar` — **Show Calendar and DateInput in another calendar**: alternate calendars and Temporal requirement.
    - [ ] `useDirectionHookUsage` — **useDirection — Hook Usage**: ambient-direction-aware canvas sparkline.

- [ ] **[Layer Provider](https://astryx.atmeta.com/components/LayerProvider)**

  - Sources: [Layer family English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Layer/Layer.doc.mjs), [LayerProvider.tsx](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Layer/LayerProvider.tsx), [LayerContext.ts](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Layer/LayerContext.ts). No standalone doc or supplied templates.

  - **Actual configuration, not UI variants**
    - [ ] `toast.position`: `topEnd`, `topStart`, `bottomEnd` (default), `bottomStart`; `maxVisible` default 5, arbitrary numeric count.
    - [ ] Provider `inset`: `blockStart`, `blockEnd`, `inlineStart`, `inlineEnd` as pixel numbers/CSS lengths; default zero each edge. Adds persistent-app-bar space to layer gutters and toast viewport.
    - [ ] Toast-only inset keys `top`, `bottom`, `start`, `end` as numbers; explicit edge replaces provider inset for toast only, omitted edge follows provider.
    - [ ] Optional provider versus lazy self-mounted fallback; no empty toast viewport until Toast imported; children are application subtree, not rendered overlay content.
    - [ ] Nested provider retains outer toast config and mounts no second viewport; nested declared inset applies to subtree anchored layers, root toast viewport remains root-owned.
    - [ ] Anchored layers use max(spacing-4, safe-area) plus declared inset; viewport cap, flip, then slide when neither side fits; rendered layer content resets surface/group membership and body typography, not application subtree.

  - **Source examples**
    - [ ] Provider with toast topEnd/maxVisible 3 around app.
    - [ ] Provider with blockEnd inset for persistent floating bottom bar. These are examples, not alternative defaults.

- [ ] **[Link Provider](https://astryx.atmeta.com/components/LinkProvider)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Link/LinkProvider.doc.mjs). Template in `LinkProvider/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Actual provider behavior**
    - [ ] No finite UI variants: required `component` and children replace native anchors with framework/client-router links throughout subtree.
    - [ ] Per-link `as` wins, otherwise nearest provider wins, then native anchor fallback; disabled Link bypasses provider.
    - [ ] Both href/to checked under shared Link URL policy, including structured protocol without changing object identity; rejection of either makes inert content without invoking router or falling back to other destination.

  - **Named template**
    - [ ] `LinkProviderCustomLink` — **Link Provider — Custom Link Component**: intercept navigation instead of full page load.

- [ ] **[Media Theme](https://astryx.atmeta.com/components/MediaTheme)**

  - Source: [English docs](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/theme/MediaTheme.doc.mjs). Templates in `MediaTheme/` under [pinned root](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components).

  - **Local surface modes**
    - [ ] Required `mode`: `dark`, `light`, `auto`, `off`; dark means light text on dark surface, light means dark text on light surface, off disables context.
    - [ ] `fallback`: `dark` (default), `light`, only for auto when SSR/first frame/unknown backdrop prevents measuring.
    - [ ] Auto leaves ambient context unchanged when existing text reaches 3:1, otherwise chooses more readable side; background-image pixels need useImageMode sampling.
    - [ ] Always renders element so switching contexts does not remount children; supplies tokens/color-scheme, not background; caller supplies surface.
    - [ ] Children inherit media tokens; theme onDark/onLight token/component overrides; local inversion, not app-wide Theme dark/system setting.

  - **Named templates**
    - [ ] `MediaThemeImageOverlay` — **MediaTheme — Image Overlay**: dark gradient text/actions with dark mode.
    - [ ] `MediaThemeLightScrim` — **MediaTheme — Light Scrim**: light image scrim with dark-on-light text/ghost buttons.
    - [ ] `MediaThemeShowcase` — **MediaTheme — Media Overlay**: text/icons/badges/button variants on image-backed dark surface.

- [ ] **[Syntax Theme](https://astryx.atmeta.com/components/SyntaxTheme)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/theme/SyntaxTheme.doc.mjs).

  - **Theme alternatives**
    - [ ] Region-level theme provider versus theme-wide `defineTheme({syntax: ...})` colors versus single CodeBlock `syntaxTheme` shorthand; supplied children inherit.
    - [ ] Explicitly named presets: `oneDarkPro`, `dracula`, `monokai`, `nord`, `tokyoNight`, `catppuccinMocha`, `githubLight`, `githubDark`, `solarizedLight`, `oneLight`.
    - [ ] Custom `defineSyntaxTheme`; each token accepts a fixed value or `[light, dark]` tuple for automatic scheme adaptation.
    - [ ] Fourteen semantic token roles: `keyword`, `string`, `comment`, `number`, `function`, `type`, `variable`, `operator`, `constant`, `tag`, `attribute`, `property`, `punctuation`, `background`; values remain custom data.

  - **Named templates and source note**
    - [ ] [`SyntaxThemeShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/SyntaxTheme/SyntaxThemeShowcase.doc.mjs) — “SyntaxTheme — Compact Preset”: One Dark Pro.
    - [ ] [`SyntaxThemeDarkPreset`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/SyntaxTheme/SyntaxThemeDarkPreset.doc.mjs) — “SyntaxTheme — Dark Preset”: Dracula-style dark region.
    - [ ] [`SyntaxThemeLightPreset`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/SyntaxTheme/SyntaxThemeLightPreset.doc.mjs) — “SyntaxTheme — Light Preset”: light documentation surface.
    - [ ] [`useSyntaxThemeHookUsage`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/SyntaxTheme/useSyntaxThemeHookUsage.doc.mjs) — “useSyntaxTheme — Keyword color”: const colored by hook inside GitHub Light.
    - [ ] Documentation says tokens were validated against 11 community themes, but explicitly lists only the ten presets above; no unnamed eleventh preset is inferred.

- [ ] **[Theme](https://astryx.atmeta.com/components/Theme)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/theme/Theme.doc.mjs).

  - **Provider modes and theme content**
    - [ ] `mode`: `light`, `dark`, `system` (default); system follows page/root scheme, normally reader preference unless host overrides.
    - [ ] Static production built themes/CSS via theme build versus dynamic runtime defineTheme; nested region versus app-wide provider; identical children with different theme objects.
    - [ ] Theme motion `character`: `crisp`, `standard`, `expressive`; `tempo` continuous range 0.75–1.5, not finite style presets.
    - [ ] Tokens use `--` CSS custom property names; fixed string versus `[light, dark]` tuple. Accent/surface/body/text/radius/spacing tokens are customizable, not enumerated fixed palettes.

  - **Inline example and named templates**
    - [ ] Usage's defineTheme example: custom `ocean` theme with paired accent/surface/primary-text values and single radius value; example name/colors are arbitrary data, not standard themes.
    - [ ] [`ThemeApply`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Theme/ThemeApply.doc.mjs) — “Theme — Apply Theme”: subtree inheritance.
    - [ ] [`ThemeNested`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Theme/ThemeNested.doc.mjs) — “Theme — Nested Theme”: local override.
    - [ ] [`ThemeShowcase`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Theme/ThemeShowcase.doc.mjs) — “Theme — Distinct Themes”: two providers, same content.
    - [ ] [`ThemeSwitcher`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Theme/ThemeSwitcher.doc.mjs) — “Theme — Switch Themes”: state selects theme object.
    - [ ] [`useThemeHookUsage`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Theme/useThemeHookUsage.doc.mjs) — “useTheme — Token Values”: resolved values for SVG/non-CSS consumers.
    - [ ] [`useThemeNameHookUsage`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/Theme/useThemeNameHookUsage.doc.mjs) — “useThemeName — Active theme”: provider name readout.

## Supporting Hooks and Utilities

### Component-specific Hooks

- [ ] **[useAppShellMobile](https://astryx.atmeta.com/components/useAppShellMobile)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/AppShell/useAppShellMobile.doc.mjs).

  - **Actual state and controls**
    - [ ] No parameters or visual variants. Read `isMobile` at same AppShell nav breakpoint, `isMobileNavOpen`, `isMobileNavEnabled`, `hasAutoToggle`.
    - [ ] `mobileNavId` for custom toggle aria-controls, undefined without managed drawer; wire open state into accessible custom disclosure.
    - [ ] `toggleMobileNav`, `openMobileNav`, `closeMobileNav`; open/toggle no-op when disabled; close after route changes.
    - [ ] Managed nav versus false/no nav/custom mobileNav node (isMobileNavEnabled false); hasToggle false disables auto toggle. Combine mobile/enabled/auto-toggle flags for custom trigger visibility.
    - [ ] Outside provider returns safe defaults/no-op callbacks, not exception. Standard trigger uses MobileNavToggle; unrelated responsive state uses useMediaQuery.
    - [ ] No own named inline examples or associated template metadata.

- [ ] **[useCalendarConstraints](https://astryx.atmeta.com/components/useCalendarConstraints)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Calendar/useCalendarConstraints.doc.mjs).

  - **Actual rules and modes**
    - [ ] Options combine optional inclusive ISO min/max, custom native-Date predicates, minRangeSpan/maxRangeSpan and PlainDate/null rangeAnchor; no UI styles.
    - [ ] Before anchor, span rules inactive; afterward spans count both endpoints. Any predicate false disables date; before min/after max disables date.
    - [ ] Minimum-span exception: anchor itself remains enabled, so picker forbidding single-day selection must reject second anchor click itself. Maximum span bounds both directions from anchor.
    - [ ] Returns isDateDisabled plus parsed minDate/maxDate or null; synchronous per-cell predicates, not async network validation.
    - [ ] Documented patterns: recurring closed weekdays, holidays/blackout/sold-out dates, bespoke scheduling/range picker. No own named inline examples or associated templates.

- [ ] **[useCalendarDays](https://astryx.atmeta.com/components/useCalendarDays)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Calendar/useCalendarDays.doc.mjs).

  - **Actual grid options**
    - [ ] Required calendar-relative year and 1-based month; `weekStartsOn`: `0` Sunday (default), `1` Monday, `2` Tuesday, `3` Wednesday, `4` Thursday, `5` Friday, `6` Saturday.
    - [ ] `hasVariableRowCount`: false (default; 6 rows/42 cells), true (needed weeks only; Gregorian 4–6, short Coptic/Ethiopic month 1–2).
    - [ ] CalendarSystem default `gregory`; docs explicitly name non-Temporal-supported `gregory`, `iso8601`, `buddhist`, `roc`, `japanese` and illustrate `hebrew`, `coptic`, `ethiopic` behavior. Other calendars require Temporal or fall back to Gregorian numbering with once-only dev warning. Provider calendar is not read; no undocumented exhaustive CalendarSystem union inferred.
    - [ ] Returns days with PlainDate/ISO/isOutside/dayNumber, weeks of seven, locale dayNames starting at chosen first day, totalCells; names abbreviated where they fit, narrow otherwise.
    - [ ] Outside-month filler versus in-month data, locale header versus calendar numbering; rendering, selection, keyboard and grid semantics remain caller-owned. Use Calendar for full picker semantics.
    - [ ] No own named inline examples or associated template metadata; documented use cases compact availability grid/mini calendar.

- [ ] **[useCalendarNavigation](https://astryx.atmeta.com/components/useCalendarNavigation)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Calendar/useCalendarNavigation.doc.mjs).

  - **State modes and options**
    - [ ] Uncontrolled initialValue versus controlled focusDate/onFocusDateChange; ISO dates choose visible focus month.
    - [ ] `numberOfMonths`: `1` (default), `2`; `calendar` default gregory, documented examples gregory/persian/hebrew. gregory/iso8601/buddhist/roc/japanese need no Temporal; others do. Provider calendar not read; CalendarSystem remains referenced type, not invented style list.
    - [ ] Returns Gregorian baseMonth/visibleMonths, active-calendar year/month pairs in visibleCalendarMonths, localized monthYearLabel.
    - [ ] navigateMonth positive/negative steps; optional focusedDate and offset set cross-month pendingFocus (default offset 7; horizontal-arrow pattern 1, vertical 7); clearPendingFocus once target cell receives focus.
    - [ ] Previous/Next custom header or dual-month availability picker patterns; no rendering/UI variants, own named inline examples, or associated templates.

- [ ] **[useCollapsible](https://astryx.atmeta.com/components/useCollapsible)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Collapsible/useCollapsible.doc.mjs).

  - **Actual state-machine modes**
    - [ ] isCollapsible false (disabled behavior), true (uncontrolled, initially open), or config for custom defaults/controlled state.
    - [ ] Three ownership modes: group-controlled with unique value in CollapsibleGroup; controlled isOpen/onOpenChange; uncontrolled defaultIsOpen.
    - [ ] Returns isEnabled/isOpen and toggle; toggle dispatches to group, controlled callback or internal state. Value-bound group ignores local isOpen/defaultIsOpen/onOpenChange notification path.
    - [ ] Card/Section internals, bespoke collapsible wrapper or accordion group patterns; no visual styles, own named inline examples, or associated templates.

- [ ] **[useImperativeAlertDialog](https://astryx.atmeta.com/components/useImperativeAlertDialog)**

  - Sources: [own doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/AlertDialog/useImperativeAlertDialog.doc.mjs), [AlertDialog options](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/AlertDialog/AlertDialog.doc.mjs), [original hook](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/AlertDialog/useImperativeAlertDialog.tsx), [Button variant vocabulary](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Button/Button.doc.mjs).

  - **Imperative state and delegated options**
    - [ ] Internally owned open state (initially closed): `show(options)`, `hide()`, observable `isOpen`; render `element` in JSX (null before first options).
    - [ ] Options are AlertDialog props except `isOpen` / `onOpenChange`; no external controlled-open mode on this hook.
    - [ ] Required title, consequence description, action label, `onAction`; Cancel label defaults to `Cancel`.
    - [ ] `actionVariant`: `primary`, `secondary`, `ghost`, `destructive` (AlertDialog default).
    - [ ] `isActionLoading`: spinner/busy action; `onAction` does not auto-close. Complete asynchronous work before calling `hide()`; loading remains caller-supplied.
    - [ ] Requested `width` default `400`; clamps to container/dynamic viewport with token gutters; omitted width on a compact coarse-pointer phone gets `320`.
    - [ ] `isInline`: `false` (default modal) / `true` (preview-only nonmodal `group`).
  - **Accessibility and interaction**
    - [ ] Modal `alertdialog`, level-2 title/`aria-labelledby`, description/`aria-describedby`, Cancel initially focused, return focus on close.
    - [ ] Escape cancels; backdrop never dismisses. Above `640px` actions are horizontal/wrappable; at/below `640px` destructive action precedes Cancel and both fill width; constrained body scrolls.
    - [ ] Touch/pen actions track as one group: highlight follows contact; release activates current action, release off both acts on nothing. Mouse/keyboard/AT retain button clicks.
    - [ ] Touch entrance scales/fades and blocks touch during entry; exit blocks all activation while raised toast stays live; reduced motion cross-fades; mouse uses theme entrance and immediate close.
  - **Source example**
    - [ ] Unnamed TSX example: Delete trigger calls `alert.show({title, description, actionLabel, onAction: async ...})`; await deletion then `alert.hide()`, rendering `alert.element` alongside trigger.

- [ ] **[useImperativeDialog](https://astryx.atmeta.com/components/useImperativeDialog)**

  - Sources: [own doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Dialog/useImperativeDialog.doc.mjs), [Dialog options](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Dialog/Dialog.doc.mjs), [original hook](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Dialog/useImperativeDialog.tsx).

  - **Imperative state and delegated choices**
    - [ ] `show(content, options?)`, `hide()`, `isOpen`; render `element`. Hook owns initially closed state; content is arbitrary React content, not a finite visual style.
    - [ ] Options exclude Dialog `isOpen`, `onOpenChange`, `children`; source also accepts hook-level `defaultOptions` and merges later show options into stored options (own doc omits this argument).
    - [ ] `variant`: `standard` (default), `fullscreen`.
    - [ ] `purpose`: `info` (default, Escape/backdrop dismiss), `form` (backdrop disabled after interaction), `required` (Escape/backdrop disabled).
    - [ ] `padding`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`.
    - [ ] Preferred `width=400`, `maxHeight='75dvh'`; width clamps with viewport/container gutters. Optional static `DialogPosition`, centered when omitted, uses logical start/end offsets under RTL; dimensions/offsets are open-ended, not size presets.
    - [ ] `isInline`: `false` (default native modal) / `true` (preview without dialog/backdrop/modal behavior).
  - **Content, accessibility, and examples**
    - [ ] Header with title/optional subtitle/close control; title labels dialog and receives focus; body supports text/forms/lists/layout; optional action footer; backdrop blocks page.
    - [ ] Theme body-text defaults and complete surface/group boundary reset; intentional groups/providers belong inside.
    - [ ] Reduced motion: no slide/scale, touch fades within `200ms`, mouse appears immediately; entering touch/drag is blocked, keyboard/AT remain live, mouse/trackpad clicks follow documented last-pointer handling.
    - [ ] Unnamed source examples: Open button with `<MyContent />`; show output in a `CodeBlock` with `language="bash"`; render `dialog.element`.

- [ ] **[useHoverCard](https://astryx.atmeta.com/components/useHoverCard)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/HoverCard/useHoverCard.doc.mjs).

  - **Placement and trigger choices**
    - [ ] `placement`: `above` (default), `below`, `start`, `end`; logical sides mirror in inherited RTL.
    - [ ] `alignment`: `start`, `center` (default), `end`.
    - [ ] `delay`: hover opening delay, default `300` ms; `hideDelay`: leaving delay, default `200` ms.
    - [ ] `focusTrigger`: `auto` (default; trigger itself or a currently focusable enclosing trigger, including roving items), `always`, `never`.
    - [ ] `touchTrigger`: `auto` (default; opens unless trigger has its own action; aria-disabled control has none), `tap` (always opens), `none`.
    - [ ] `isEnabled`: `true` (default) / `false` for hover/focus triggers.
  - **State, content, and accessibility**
    - [ ] Controlled `isOpen=true` force-shows / `false` force-hides; omitted lets hover/focus manage visibility; `isDefaultOpen=false` by default.
    - [ ] `onShow` / `onHide` visibility notifications; imperative `show()` / `hide()` act immediately.
    - [ ] Combined trigger `ref` or split `positionRef` + `interactionRef`; `anchorId` supports advanced anchoring.
    - [ ] Compose `describedBy` into trigger `aria-describedby`; `label` supplied exposes named `dialog`, omitted exposes `group`.
    - [ ] `renderHoverCard(children, props)` accepts custom rich content but excludes `positioning` opt-out; placement/alignment always own positioning.
    - [ ] Rich preview patterns: user profile, entity summary, link preview; safe hover/focus intent behavior. Simple text hints belong in Tooltip instead.

- [ ] **[useLinkComponent](https://astryx.atmeta.com/components/useLinkComponent)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Link/useLinkComponent.doc.mjs).

  - **Renderer resolution and safety**
    - [ ] Explicit `as` component wins; otherwise nearest LinkProvider renderer; otherwise native `a`. Resolved component identity stays stable while choice is unchanged.
    - [ ] Custom router receives `to={href}` unless explicit `to` is supplied; pass `as='a'` for downloads/out-of-app destinations.
    - [ ] Validate both `href` and `to` under Link's destination rule before invoking renderer; rejected destination yields destination-less native anchor and never invokes custom router.
    - [ ] Shared custom-component links follow provider routing without direct router imports. No visual variants or finite router-name list.

- [ ] **[useLinkify](https://astryx.atmeta.com/components/useLinkify)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Link/useLinkify.doc.mjs).

  - **Detection modes and rendering**
    - [ ] `hasBuiltins`: `true` (default, HTTP(S) URL + email detection) / `false` (custom patterns exclusively).
    - [ ] Optional `patterns`: global-flag regex + href builder with captures, optional label builder and `isExternal`; documented custom-reference patterns are issue numbers (`#123`) and ticket IDs, not style presets.
    - [ ] Overlap precedence: earliest match wins; at same starting position custom beats built-in.
    - [ ] Return interleaved plain strings and Link nodes; empty text returns one-element array containing itself.
    - [ ] Matched URLs use external links, emails `mailto:`; all links respect LinkProvider. Render directly inside text container for activity feeds, comments, descriptions.
    - [ ] Regex must use `g` to advance scanning; do not preparse markup or use a non-global pattern that can stall rendering.

- [ ] **[useOutlineFromDOM](https://astryx.atmeta.com/components/useOutlineFromDOM)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Outline/useOutlineFromDOM.doc.mjs).

  - **Heading extraction behavior**
    - [ ] No visual variants: scan `h1`, `h2`, `h3`, `h4`, `h5`, `h6` inside required article `containerRef`; null ref returns empty array.
    - [ ] Include only headings with existing ID and nonempty trimmed text; output document-order `{id,label,level}` with level `1`–`6` for Outline.
    - [ ] Update on added/removed/renamed headings or changed IDs; never invent heading IDs or include unrelated page headings.
    - [ ] Non-Markdown article pattern; Markdown strings use the parser-aware companion instead.

- [ ] **[useOutlineFromMarkdown](https://astryx.atmeta.com/components/useOutlineFromMarkdown)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Outline/useOutlineFromMarkdown.doc.mjs).

  - **Parse modes and matching**
    - [ ] Required Markdown string; optional ordered `plugins` identical to Markdown's list, or omit for plugin-free rendering; memoize list reference to avoid recomputation.
    - [ ] `isFinal`: `true` (default complete document) / `false` during Markdown streaming.
    - [ ] Return root-heading items in document order with parser-matching slugs, text label, depth `1`–`6`; ignore fenced code and nested headings such as blockquotes.
    - [ ] Memoize on source/options; outside React use `parseOutlineFromMarkdown`.
    - [ ] Multiple-document pattern uses one `createMarkdownHeadingLinks` per document with `headingIdPrefix` to avoid ID collisions.
    - [ ] Streaming can temporarily omit/change targets due to reveal/unfinished syntax; sources-citation IDs are not matched because hook receives no sources (`# Intro [ref]`: renderer `intro`, hook `intro-ref`). These are documented limitations, not unresolved source gaps.

- [ ] **[useOverlay](https://astryx.atmeta.com/components/useOverlay)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Overlay/useOverlay.doc.mjs).

  - **Visibility and appearance choices**
    - [ ] `showOn`: `hover`, `always` (default), `focus`, `hover-or-focus` (alias of hover).
    - [ ] Hover also reveals on focus-within; on hover-none touch devices tap toggles hover overlay.
    - [ ] Controlled `isOpen=true` / `false` overrides showOn and touch toggle; omitted uses trigger policy.
    - [ ] `scrim`: `dark` (default), `light`, `false` (no background).
    - [ ] `position`: `fill` (default), `bottom`, `top`; `align`: `start`, `center`, `end` (default).
  - **Content and accessibility patterns**
    - [ ] Supply `content` for pre-rendered `element`, or call `renderOverlay(children)` for changing/self-mounted content; never render both. Without content, `element=null`.
    - [ ] Attach `containerRef` and `containerProps` to existing Card/media container (positioning/clipping scope, touch click/mouseup); scrim covers nearest positioned ancestor, including an inner image section.
    - [ ] Place focusable title/link before overlay content so Tab reveals it and next Tab reaches actions; hidden overlay content is not focusable.
    - [ ] Revealed scrim intercepts its covered area: keep revealing control outside it or in uncovered part of top/bottom strip. Use Overlay component when no container already exists.

- [ ] **[usePopover](https://astryx.atmeta.com/components/usePopover)**

  - Sources: [English hook doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Popover/usePopover.doc.mjs), [inherited render-prop choices](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Layer/useLayer.tsx).

  - **Dismissal, focus, and semantic modes**
    - [ ] `hasLightDismiss`: `true` (default) / `false`; `hasEscapeDismiss`: `true` (default) / `false` (fully suppresses Escape only with light dismissal disabled, since native light-dismiss also handles Escape).
    - [ ] `hasAutoFocus`: `true` (default) / `false`; first genuine content control gets focus, otherwise labeled dialog surface; generated close control excluded.
    - [ ] `hasCloseButton`: `true` (default) / `false`; hidden close appears for keyboard users; `closeButtonLabel='Close popover'` by default.
    - [ ] `role`: `dialog` (default) / `none` (menu/listbox content exposes own role and keeps trigger focus).
    - [ ] `isModal`: `true` (default) / `false`, applicable to dialog role; `dialogLabel` supplies accessible name without visible title.
  - **Surface and positioning choices**
    - [ ] `hasSurface`: `true` (default background/radius/shadow) / `false`; `xstyle` composes after defaults.
    - [ ] `padding`: `0`, `0.5`, `1`, `1.5`, `2`, `3`, `4`, `5`, `6`, `8`, `10`; omitted hook paints no padding (Popover component supplies `3`, flush `0`).
    - [ ] `render(children, props)` placement: `above`, `below`, `start`, `end`; alignment: `start`, `center`, `end`; inherited source defaults above/center and logical RTL mirroring.
    - [ ] Inherited context positioning `anchor` / `custom`, `as=div` / `span`, optional flip-safe offset default `0`; custom mode owns insets/RTL.
    - [ ] Every painted surface emits canonical `popover` theme target; optional open-ended `surfaceTarget` adds owned refinement such as `selector-popup`, not a new anatomy part.
  - **State and composition**
    - [ ] `show({skipAutoFocus?})`, `hide()`, `toggle()`, `isOpen`; skipAutoFocus preserves input-trigger focus. `onShow` / `onHide` allow lifecycle/focus-return composition.
    - [ ] Attach `triggerRef`, content focus-trap ref; `anchorId`, unique `id`; spread `triggerProps` with expanded/controls and haspopup `dialog` or `true` according to role.
    - [ ] Interactive form/picker/menu/command-panel patterns combine Layer positioning, focus trapping, auto-focus, optional close, and dismissal; noninteractive previews use HoverCard/Tooltip instead.

- [ ] **[useResizable](https://astryx.atmeta.com/components/useResizable)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Resizable/useResizable.doc.mjs).

  - **Size forms, direction, and bounds**
    - [ ] `direction`: `horizontal` (default, inline basis) / `vertical` (block basis); match ResizeHandle direction.
    - [ ] `defaultSize=250`, `minSize=50`, `maxSize=Infinity` by default; pixel forms are number, exact `Npx`, `pixel(value)`.
    - [ ] Percentage forms: exact `N%` without extra pixel bound; `percent(value,{min:pixel(...)})` floor or `{max:pixel(...)}` ceiling, exactly one (XOR), options required.
    - [ ] Default percentage resolves once to pixels; percentage min/max remain live as basis changes and clamp current size.
    - [ ] Basis is caller-supplied `containerRef`, otherwise viewport; follows ref-node changes. Unlaid-out/display-none/detached basis temporarily uses `1200px` rather than zero and persists nothing from that temporary value.
  - **Collapse, persistence, and controls**
    - [ ] `collapsible`: `false` (default) / `true`; release/key step below threshold collapses to zero; pixel `snaps` configure drag snapping.
    - [ ] Uncontrolled `defaultIsCollapsed=false`; persisted `autoSaveId` entry wins. Controlled `isCollapsed` reports requests via `onCollapseChange` instead of changing internal state.
    - [ ] `autoSaveId` persists pixel size and collapse state to localStorage; dimensions, callbacks, persistence, paint and ARIA use resolved pixels.
    - [ ] Return `size`, `isCollapsed`, `collapse()`, `expand()`, `resize(px)`, and `props` for region/ResizeHandle.
    - [ ] Collapse-change paths: release, keyboard threshold step, drag/key reopen, Enter, double-click, imperative collapse/expand/resize; callback once per change.
    - [ ] Single-region and multi-region/cascade ordering patterns; Layout/AppShell sidebar integration; import server-safe percent/pixel from Resizable/utils. Documented bounded examples are 40% with 333px floor and 10% with 400px ceiling, not finite size presets.

- [ ] **[useCombobox](https://astryx.atmeta.com/components/useCombobox)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Selector/useCombobox.doc.mjs).

  - **Actual options and ownership**
    - [ ] Required ordered selectableItems (string value/optional disabled), caller-owned isOpen/onOpen/onClose and listboxId; optional selected value/onSelect, onClear. onOpen returning false refuses without changing highlight.
    - [ ] `hasSearch`, `isDisabled`: false (default), true; optional onSearchSeed receives printable non-Space characters on trigger, opening first when needed.
    - [ ] Disabled choices cannot pick and are skipped. Opening highlights enabled selected value, otherwise first/last enabled for down/up; no enabled choices leaves -1. Search press/Enter opening does not prehighlight; arrows still do.
    - [ ] onClose resets highlight first; caller-driven outside close should setHighlightedIndex(-1). Returned highlightedIndex/setHighlightedIndex/getItemId, click/select/hover handlers allow button-control wiring.

  - **Keyboard and accessibility contracts**
    - [ ] Manual trigger handler: arrows open/move; Enter and non-search Space open/pick; Escape/Tab close; Home/End/PageUp/PageDown jump edges. Editable field omits caret Home/End from this manual handler; no action while IME composing.
    - [ ] Prop-getter field contract: combobox/aria-autocomplete=list, focus stays in input, open-list arrows wrap; Enter claimed only when an active option is picked; Escape (any modifiers) closes even empty open list without also closing containing layer. Other chords/keys reach field.
    - [ ] getFieldProps composes own onKeyDown after hook unless claimed; references listbox only while rendered/nonempty, active descendant only for existing active option. getListboxProps preserves field focus on press; caller names listbox/field.
    - [ ] getOptionProps: id/option role, aria-selected on active option, aria-disabled, pointer move activates without focus/scroll, press picks and closes with field focus retained.
    - [ ] Closed nonsearch Delete/Backspace invokes onClear; ignored open/hasSearch. Type-to-select composes useTypeahead first, fallback to hook keys.
    - [ ] For new query caller sets active index or -1 and politely announces count/no-results via useAnnounce; empty messages/non-option controls live beside, never inside listbox.

  - **Named associated family example**
    - [ ] No metadata in hook directory; `useComboboxHookUsage` under Selector — “useCombobox — Assign To Field”: TextInput filters teammate Item rows, input focus with arrows/Enter/Escape. Hook renders nothing; works with in-flow page results or dialog people-picker, not only popup styling.

- [ ] **[useToast](https://astryx.atmeta.com/components/useToast)**

  - Sources: [English hook doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Toast/useToast.doc.mjs), [delegated Toast options/behavior](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Toast/Toast.doc.mjs).

  - **Notification choices and lifecycle**
    - [ ] No arguments; returns `showToast(options)` which returns imperative dismiss function. LayerProvider supplies viewport, or hook self-mounts fallback; call from child of provider, not same component rendering it.
    - [ ] `type`: `info` (default), `error`; `isAutoHide` true by default for info, false for error, explicitly set either boolean as needed.
    - [ ] `autoHideDuration` default `5000ms`; timed content must satisfy WCAG 2.2.1; persistent message/action uses `isAutoHide=false`.
    - [ ] `uniqueID` deduplicates; `collisionBehavior`: `overwrite` (default), `ignore`.
    - [ ] Dismiss/hide reasons: `auto`, `manual`; position/stacking/auto-dismiss/dedup handled by viewport rather than caller-owned open state.
  - **Content and accessibility patterns**
    - [ ] Required `body` ReactNode (documented Text body slot); optional `endContent` such as Undo button/link, icon or Badge.
    - [ ] Per-toast `renderContent` replaces inner layout, receives body/endContent/type/resolved timing/dismiss; retains card/theme target and auto-hide, custom renderer owns controls with no injected fallback close.
    - [ ] Shared custom layout wraps useToast and passes renderContent on each call; library toasts without renderer remain standard.
    - [ ] Viewport announces even over modal, card itself not live region there; viewport/safe-area gutters and wrapping protect long messages.
    - [ ] Top-edge toast swipe dismisses upward, bottom-edge downward; finger/pen dominant edge-directed intent required before native touch cancellation; mouse drag excluded for text selection, visible close remains; swipe reports manual dismissal.
    - [ ] Transient save/delete/status feedback rather than blocking critical errors or field validation; action must remain available when needed.

- [ ] **[useTooltip](https://astryx.atmeta.com/components/useTooltip)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Tooltip/useTooltip.doc.mjs).

  - **Placement and trigger choices**
    - [ ] `placement`: `above` (default), `below`, `start`, `end`; inherited logical RTL mirroring.
    - [ ] `alignment`: `start`, `center` (default), `end`; `delay=200ms`, `hideDelay=0ms` by default.
    - [ ] `focusTrigger`: `auto` (default, trigger/currently focusable containing trigger including roving items), `always`, `never`.
    - [ ] `touchTrigger`: `auto` (default, unless trigger performs own action; aria-disabled control has none), `tap` (always), `none`.
    - [ ] `isEnabled`: `true` (default) / `false`; `isDefaultOpen=false` by default.
  - **Controlled and imperative behavior**
    - [ ] Controlled `isOpen=true` force-shows / `false` force-hides; omitted hover/focus-managed. Controlled false still calls `onShow` for focus that would reveal.
    - [ ] `show()` immediately holds open until hide/Escape/outside press/focus leaving trigger, announces text once and calls onShow; disabled show does nothing.
    - [ ] `hide()` calls onHide; under controlled state show/hide only notify callbacks rather than changing state.
  - **Trigger, content, and accessibility patterns**
    - [ ] Combined `ref` or split `positionRef` / `interactionRef`; anchorId for positioning; compose describedBy into trigger aria-describedby.
    - [ ] `renderTooltip(children,props)` always uses placement/alignment, excludes custom-positioning opt-out.
    - [ ] Brief labels for icon buttons, truncated text, abbreviations, compact controls; no interactive tooltip content (use Popover/HoverCard).
    - [ ] Custom toggle trigger calls show/hide on press, sets aria-describedby/aria-controls to describedBy, tracks aria-expanded through onHide, retains independent accessible name; info-icon pattern uses InfoTip.

- [ ] **[useIcon](https://astryx.atmeta.com/components/useIcon)**

  - Sources: [English hook doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Icon/useIcon.doc.mjs), [Icon vocabulary](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Icon/Icon.doc.mjs).

  - **Registry choices and resolution**
    - [ ] Semantic names: `close`, `chevronDown`, `chevronLeft`, `chevronRight`, `chevronsLeft`, `chevronsRight`, `check`, `success`, `error`, `warning`, `info`, `calendar`, `clock`, `externalLink`, `menu`, `moreHorizontal`, `search`, `upload`, `arrowUp`, `arrowDown`, `arrowsUpDown`, `funnel`, `eyeSlash`, `viewColumns`, `copy`, `checkDouble`, `wrench`, `stop`, `microphone`.
    - [ ] Namespaced extension key (documented example `richtext:bold`): application/library-registered open vocabulary, not a closed style list; unregistered namespaced glyph renders nothing.
    - [ ] Resolve nearest Theme icons; outside Theme use the registered theme named by document root; then `registerIcons()` override; finally built-in default.
    - [ ] Return raw React node without size/color wrapper; built-ins are `1em` squares in `currentColor`; function entries are evaluated at `{size: 'md'}` and may intentionally return `null`.
    - [ ] No hook size/color/label modes: use `<Icon icon={name}>` for another size, color, or accessible label. Theme-aware semantic success/warning/error glyphs are the documented custom-component pattern.

- [ ] **[useLightbox](https://astryx.atmeta.com/components/useLightbox)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Lightbox/useLightbox.doc.mjs).

  - **Media modes and state**
    - [ ] Required `media`: single `LightboxMedia` object (single item) or array (gallery); documented image and video behaviors, not fixed media URLs.
    - [ ] `hasZoom`: `false` (default) / `true` for images only; double-click or Enter/Space/+/- zooms; when zoomed drag/arrows pan.
    - [ ] `hasAutoPlay`: `false` (default) / `true` video autoplay on opening.
    - [ ] Internally owned `isOpen` and gallery `index`; `open(index?)` defaults to index `0`, `close()` dismisses; do not maintain duplicate state.
  - **Trigger and accessibility patterns**
    - [ ] Single `triggerProps` opens index `0`; `getTriggerProps(index)` supports per-thumbnail gallery triggers.
    - [ ] Trigger gets button role, `tabIndex=0`, `aria-haspopup=dialog`; click/Enter/Space activate.
    - [ ] Render returned `element` once alongside triggers; prewired Lightbox owns dismissal, next/previous navigation, and focus restoration.

- [ ] **[useMultiCombobox](https://astryx.atmeta.com/components/useMultiCombobox)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/MultiSelector/useMultiCombobox.doc.mjs).

  - **Ownership and finite modes**
    - [ ] Caller-controlled `isOpen`, `onOpen`, `onClose`, `onToggle`, ordered `selectableItems`, `listboxId`; `onOpen=false` refuses opening without changing highlight; closing resets highlight first.
    - [ ] `hasSearch`: `false` (default) / `true`; `hasValue`: `false` (default) / `true`; `isDisabled`: `false` (default) / `true` ignores clicks/keys.
    - [ ] `isGrid`: `false` (default listbox) / `true` option+action-cell rows; `rowHasAction` controls action availability, `onActivateAction` runs action without selecting.
    - [ ] Highlight cell alternatives `option` / `action`; index `-1` means none; `setHighlightedIndex()` resets to option cell.
  - **Keyboard and pointer behavior**
    - [ ] Disabled options cannot toggle and are skipped; ArrowDown/press/Enter/Space opening reaches first enabled option, ArrowUp last; with search, press/Enter opens without prehighlight while arrows still select ends.
    - [ ] Up/Down open or navigate; Enter and nonsearch Space toggle without closing; Escape/Tab close; Home/End/PageUp/PageDown reach first/last.
    - [ ] Without search, type-to-select prefix matching cycles repeated letters and starts single-letter search after current item; Space remains toggle.
    - [ ] With search, Space types, Delete/Backspace edit query, typeahead disabled; search input may stop Home/End for caret and use PageUp/PageDown for list boundaries.
    - [ ] Optional `onClear`: Delete/Backspace clears closed, nonsearch control only if `hasValue`; ignored while open or searchable.
    - [ ] Grid inline-end arrow moves to action, inline-start back to option, mirrored in RTL; Enter on action invokes action callback.
    - [ ] `onItemMouseEnter(item,index,cell?)` highlights option/action without scrolling or moving focus.
  - **Accessibility wiring**
    - [ ] Headless output sets no DOM attributes: focus stays on control; caller wires `activeDescendantId` and `getItemId()` to aria-activedescendant/option IDs, `getActionCellId()` to grid action cells.
    - [ ] Active descendant undefined when closed/unhighlighted; announce changed query result count with `useAnnounce`; multiselect intentionally remains open across toggles.

- [ ] **[usePowerSearchConfig](https://astryx.atmeta.com/components/usePowerSearchConfig)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/PowerSearch/usePowerSearchConfig.doc.mjs), [original field/operator definitions](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/PowerSearch/usePowerSearchConfig.ts).

  - **Field types and canonical operator choices**
    - [ ] Field types: `string`, `number`, `boolean`, `date`, `enum`, `enum_list`, `string_list`; definitions carry key/type, optional label, enumValues required for enum/enum_list. Consumer enum entries are data, not finite component styles.
    - [ ] String operators: `contains` (default), `not_contains`, `starts_with`, `not_starts_with`, `ends_with`, `not_ends_with`, `is`, `is_not`.
    - [ ] Number operators: `equals` (default), `not_equals`, `greater_than`, `less_than`, `greater_than_or_equal`, `less_than_or_equal`.
    - [ ] Date operators: `before`, `after` (default), `between`; boolean: `is_true` (default), `is_false`.
    - [ ] Enum: `is` (default), `is_not`; enum-list and string-list: `is_any_of` (default), `is_none_of`.
  - **Memoization and usage**
    - [ ] Optional `configName` defaults to `PowerSearchConfig`; return memoized config plus `applyFilters(filters,data)` for client-side predicate filtering.
    - [ ] Declare definitions as const outside component or memoize; pass config to PowerSearch and filtered results to Table/List. No rendered UI or visual variants of its own.

- [ ] **[useSnapPaging](https://astryx.atmeta.com/components/useSnapPaging)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/SnapPaging/useSnapPaging.doc.mjs).

  - **Paging choices and lifecycle**
    - [ ] `pages`: `view` (default, one viewport toward nearest item-start stop) / `item` (one item, useful with several visible).
    - [ ] Finger/trackpad flick uses iOS-like paging on native snap stops; mouse wheel keeps native snapping; ordinary `hasSnap` instead preserves multi-item momentum.
    - [ ] Pass stable return value as Carousel `hasSnap`; capability stable for component lifetime, Carousel retains first attached value, changed pages applies on next flick.
    - [ ] Paging code loads only for participating carousel; documented patterns are onboarding views and photo strips, not rapidly scanned rows.

- [ ] **[useTruncation](https://astryx.atmeta.com/components/useTruncation)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Text/useTruncation.doc.mjs).

  - **Detection modes and state**
    - [ ] Required `maxLines`: `0` disables (`isTruncated=false`, `fullText=''`); `1` measures scrollWidth/offsetWidth; above `1` measures actual Range height past line clamp (not clamped scrollHeight).
    - [ ] `isExpanded`: `false` (default) / `true` while displaying full text, still report whether clamped form would truncate.
    - [ ] Return measured ref, isTruncated, live fullText; update for size changes and text swaps even without resize.
    - [ ] Shared ResizeObserver + MutationObserver support many cells; initial measurement batches in postcommit microtask; ref reattachment cleans previous observation.
  - **Content feedback pattern**
    - [ ] Caller applies matching nowrap/line-clamp; compose full-content Tooltip only when actually truncated and use measured fullText.
    - [ ] Text/Heading already integrate; direct hook for custom table/status text. No finite number-of-lines style presets beyond these documented algorithm modes.

### Table Hooks

- [ ] **[useTableColumnResize](https://astryx.atmeta.com/components/useTableColumnResize)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/useTableColumnResize.doc.mjs).

  - **Width ownership and interaction**
    - [ ] Controlled `columnWidths` record overrides declared widths for present keys; `onColumnResizeEnd(updates)` reports every changed key on pointerup/Enter for caller merge.
    - [ ] `columns` definitions distinguish pixel vs proportional widths and supply per-column minima; proportional resize preserves full-width table by resizing neighbor, including documented last-column behavior.
    - [ ] Global `minWidth` overrides per-column defaults when present; global `maxWidth` defaults `Infinity`; widths are numeric data, not size presets.
    - [ ] Returned TablePlugin adds header-border draggable handles, RTL and pointer/keyboard WAI-ARIA window-splitter operation; commit on release, no visual variant prop.

- [ ] **[useTableColumnSettings](https://astryx.atmeta.com/components/useTableColumnSettings)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/useTableColumnSettings.doc.mjs).

  - **Visibility and ordering modes**
    - [ ] Required columns metadata (`key`, `label`, optional `isAlwaysVisible`, `group`), controlled ordered `activeColumnKeys`, `onChangeActiveColumnKeys`.
    - [ ] Toggle visibility/reorder active columns; essential `isAlwaysVisible` columns versus toggleable columns; optional grouping metadata for picker.
    - [ ] `defaultColumnKeys` defines reset set; omitted reset shows every column.
    - [ ] Headless filtered columns/toggle helpers/prebuilt MultiSelector options for column picker; no standalone size/color/style variants.

- [ ] **[useTableColumnSettingsState](https://astryx.atmeta.com/components/useTableColumnSettingsState)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/plugins/columnSettings/useTableColumnSettingsState.doc.mjs).

  - **Controlled state and commands**
    - [ ] Required available columns metadata, ordered `activeColumnKeys`, `onChangeActiveColumnKeys`; columns optionally grouped and marked `isAlwaysVisible`.
    - [ ] `toggleColumn`: remove active / append inactive; no-op for always-visible. `isColumnActive()` and `isColumnToggleable()` query current modes.
    - [ ] `showAllColumns()` activates all; `resetToDefault()` restores provided `defaultColumnKeys` or all columns when omitted.
    - [ ] `setActiveColumnKeys()` accepts ordered strings while enforcing always-visible keys remain active; no direct array mutation.
    - [ ] Return pass-through active keys and `columnSettingsConfig` for useTableColumnSettings; renderer-agnostic custom picker UI, no visual variants.

- [ ] **[useTableColumnsVirtualization](https://astryx.atmeta.com/components/useTableColumnsVirtualization)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/useTableColumnsVirtualization.doc.mjs).

  - **Windowing capability and supported modes**
    - [ ] Required `initialViewportWidth` plans SSR/premeasurement window only, not Table size; measured Table scroll area replaces it after mount.
    - [ ] Pass opaque capability to `Table.virtualization`, retain complete data/columns/plugins and explicit stable idKey; combine `[rows, columns]` for two-axis windowing.
    - [ ] Pinned start/end runs always mounted; middle columns window with overscan, declared widths retained at overflowing table minimum width; sufficiently wide viewport mounts all.
    - [ ] Focused cell pins its column until focus moves to another cell or leaves Table; ordering/width/visibility/pinning changes preserve first-visible column and offset.
  - **Fallbacks and accessibility**
    - [ ] Supported: one header row, flat rows, single-line content, no spanning cells.
    - [ ] Full-render fallback: explicit wrapping, arbitrary custom renderers, nonflat data, invalid identity, custom scrollWrapper, min-width wider than column floors.
    - [ ] Mounted cells retain native table semantics/full-table column positions; no screen-reader discovery promise for unmounted columns.
    - [ ] Keep capability present for entire Table lifetime; adding/removing remounts and resets focus/scroll, changing initialViewportWidth does not. No visual variant/size enum.

- [ ] **[useTableFiltering](https://astryx.atmeta.com/components/useTableFiltering)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/useTableFiltering.doc.mjs).

  - **Filter choices and state**
    - [ ] `variant`: `popover` (default), `inline`, `inline-compact`.
    - [ ] Documented filter kinds: text, select, multi-select, date, number through PowerSearch field definitions.
    - [ ] Required controlled `filters` column-key/value map and `onFilterChange(columnKey,value)`; `null` clears filter.
    - [ ] Table plugin adds inline column filters; pair useTableFilterState for managed state. Hidden from overview upstream, not absent or unsupported documentation.

- [ ] **[useTableFilterState](https://astryx.atmeta.com/components/useTableFilterState)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/useTableFilterState.doc.mjs).

  - **Managed filter behavior**
    - [ ] No documented visual variations: optional `initialState` seeds filter map; returns current map and onChange handler for useTableFiltering.
    - [ ] Initial map omitted versus caller-seeded map; no external controlled-state mode or additional finite options documented here. Upstream hides API from overview.

- [ ] **[useTableGroupedRows](https://astryx.atmeta.com/components/useTableGroupedRows)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/useTableGroupedRows.doc.mjs).

  - **Group state and header content**
    - [ ] Required flat `data`, `groupBy`, caller-controlled `collapsedGroups`, `onToggleGroup`; distinct group keys become sections, not finite visual variants.
    - [ ] Expanded shows members; collapsed hides data rows but retains full-width header with chevron, label, count. Disclosure ARIA belongs to chevron button, not native table row.
    - [ ] Default header content `<groupKey> (<count>)`, or custom `renderGroupHeader(groupKey,count,collapsed)` right of chevron.
    - [ ] `getRowKey` uses stable real-row key, positional fallback if omitted; `groupOrder` explicitly orders listed groups, unlisted follow first-seen order.
    - [ ] Return `{data,plugin,idKey}` for Table data/plugins/idKey.
  - **Filtering, sorting, and pagination pattern**
    - [ ] Pipeline: filter → sort → slice → group; sort group key first, user sort keys second to keep sections contiguous.
    - [ ] Appending paginated results should extend bottom, not splice above reader; mid-section cut counts only loaded members. With full result set, carry cut to section end for complete sections/total counts.

- [ ] **[useTableInfiniteScroll](https://astryx.atmeta.com/components/useTableInfiniteScroll)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/useTableInfiniteScroll.doc.mjs).

  - **Controlled request states and automatic/manual modes**
    - [ ] Required `hasMore`, `isLoading`, `onLoadMore: () => Promise<void>`; returned stable plugin goes in Table `plugins.infiniteScroll`; caller owns accumulated rows/cursors/errors/stale-response protection.
    - [ ] `hasMore=true`: localized Load more Button and end-sentinel automatic observation; false removes chrome/observation, focused removed command transfers focus to shared Table viewport without scroll.
    - [ ] Loading true blocks requests; shared outstanding-request latch holds busy state through Promise settlement and subsequent committed false, without requiring a true loading phase.
    - [ ] Button remains mounted/focusable while busy and owns loading announcement; automatic and manual paths share request lifecycle.
    - [ ] IntersectionObserver uses Table scroll area for row-windowed Table, browser viewport otherwise; zero lookahead, no ancestor discovery; absent observer keeps manual command operable.
  - **Async failures and supported data**
    - [ ] Visible short batches can request again in later task; exhausted/empty batches must update hasMore accurately.
    - [ ] Throw, reject, or non-Promise result pauses automatic loading; manual retry allowed once controlled loading false; changing callback identity does not reset pending/failure pause.
    - [ ] Hook neither renders product errors nor cancels transport. New dataset remounts owner of both hook and Table, not Table alone; caller prevents stale response commits.
    - [ ] Flat rows only; children mode/unsupported row models do not load and emit deduplicated development warning; body modifiers must declare compatible flat row model. No visual style variants.

- [ ] **[useTablePagination](https://astryx.atmeta.com/components/useTablePagination)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/useTablePagination.doc.mjs).

  - **Control choices and ownership**
    - [ ] `variant`: `pages` (default), `count`, `compact`, `dots`, `none`.
    - [ ] `position`: `below` (default), `above`, `both`, `none`.
    - [ ] `align`: `start`, `center` (default), `end`.
    - [ ] Controlled 1-based `page` and `onPageChange`; totals via `totalItems`, `totalPages`, or unknown-total cursor `hasMore`.
    - [ ] `pageSize` default `10`; consumer-defined `pageSizeOptions` show selector, with `onPageSizeChange` handling user change. Supplied numbers are not fixed style sizes.
    - [ ] `label` default `Table pagination`; both-position landmarks get distinct `(top)` / `(bottom)` suffixes.
    - [ ] Return TablePlugin for `plugins.pagination`; headless config composition, not a separate data-fetching mode.

- [ ] **[useTableRowExpansion](https://astryx.atmeta.com/components/useTableRowExpansion)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/useTableRowExpansion.doc.mjs).

  - **Detail-panel choices and state**
    - [ ] `panelVariant`: `transparent` (default, table/Card/zebra shows through) / `muted` (commentary wash, chiefly light-theme effect).
    - [ ] Required controlled `expandedKeys`, `onToggle`, stable `getRowKey`, `renderExpanded(item)`; full-width detail panel aligns content after chevron to first data column, density-sensitive inset.
    - [ ] `getIsItemExpandable`: omitted all expandable / predicate excludes chevron/context action/panel for ineligible rows.
    - [ ] `hasRowClickExpansion`: `false` (default) / `true`; ignores interactive content/text selection and ineligible rows; pointer convenience only, chevron remains keyboard/AT control.
    - [ ] `getRowLabel` human-readable identity localizes Expand/Collapse name; omitted/blank falls back `Expand row` / `Collapse row`.
  - **Motion, content, and named examples**
    - [ ] Chevron and right-click Expand/Collapse action toggle panel; documented content patterns: order details, forms, charts, nested tables. Same-column child rows use tree APIs instead.
    - [ ] Optional `animatedLayout`: glide subsequent rows/content and entrance panel/dividers together; absent jumps. Reduced motion fades in place on touch, immediate mouse; column/grid dividers currently force immediate changes.
    - [ ] **Master-detail order rows**: controlled Set toggle, stable item ID/name and OrderDetails renderer, plugin passed to Table.
    - [ ] **Migrating from tree rows to the tree plugin**: replace old row-expansion-state tree composition with useTableTreeState + useTableTreeData; childrenKey replaces getChildren, defaultExpandedIds or controlled expandedIds, expand-all and row-click tree options.

- [ ] **[useTableRowIndex](https://astryx.atmeta.com/components/useTableRowIndex)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/useTableRowIndex.doc.mjs).

  - **Numbering modes and content**
    - [ ] No finite visual variants: prepend right-aligned monospaced row-number column; `label` default `#`, `startFrom` default `1`.
    - [ ] Without `data`, use Table-supplied positions; with `data`, follow its ordering; optional `getRowKey` matches by unique string, otherwise reference identity.
    - [ ] Windowed rows use parent logical Table position directly; nondefault startFrom opts into full rendering.
    - [ ] Memoize getRowKey for stable plugin identity; custom header ReactNode and numerical starting point are content/data, not enumerated styles.

- [ ] **[useTableRowStatus](https://astryx.atmeta.com/components/useTableRowStatus)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/useTableRowStatus.doc.mjs).

  - **Exclusive status branches and paint choices**
    - [ ] Required `getStatus(item)` returns semantic `{status,label}`, custom `{color,icon?,label}`, or `null` for empty status cell.
    - [ ] Semantic `status`: `success`, `warning`, `error`; theme resolves matching glyph and tone, cannot combine with custom color/icon branch.
    - [ ] Custom named paint choices documented in type: `accent`, `success`, `error`, `warning`, `red`, `orange`, `green`, `yellow`, `blue`, `gray`; arbitrary CSS string also accepted, not a closed palette.
    - [ ] Custom marker without icon is stable `8px` dot, even for semantic-named colors; explicit icon uses caller glyph. Named custom icons retain released Icon color mapping; raw CSS color icons inherit exact paint.
    - [ ] Explicit icon names: `close`, `chevronDown`, `chevronLeft`, `chevronRight`, `chevronsLeft`, `chevronsRight`, `check`, `success`, `error`, `warning`, `info`, `calendar`, `clock`, `externalLink`, `menu`, `moreHorizontal`, `search`, `upload`, `arrowUp`, `arrowDown`, `arrowsUpDown`, `funnel`, `eyeSlash`, `viewColumns`, `copy`, `checkDouble`, `wrench`, `stop`, `microphone`.
  - **Accessibility and stability**
    - [ ] Required `label` becomes role-img accessible name plus supplemental hover tooltip; column header visually blank with localized screen-reader `Row status` name (`@astryx.table.rowStatus.columnHeader`).
    - [ ] Memoize getStatus with useCallback for stable plugin identity; status color alone does not replace label.

- [ ] **[useTableRowsVirtualization](https://astryx.atmeta.com/components/useTableRowsVirtualization)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/useTableRowsVirtualization.doc.mjs).

  - **Capability, sizing, and fallback behavior**
    - [ ] Required positive finite `rowHeight` and `initialViewportHeight` CSS px; row pitch includes density padding/collapsed borders and must fit text/controls without shrinking them.
    - [ ] Initial viewport height plans SSR/premeasurement only; Table fills parent-given height and measured viewport replaces estimate, accounting for headers/chrome.
    - [ ] Pass opaque capability to Table.virtualization; retain full data/columns/plugins and explicit stable idKey. Parent needs fixed/relative height or suitable flex child with min-height zero; no height renders all rows.
    - [ ] Flat fixed-height single-line rows supported; wrapping/custom renderers/nonflat rows/invalid identity/custom scrollWrapper fall back to full rendering.
    - [ ] All rows uniformly taller (zoom/enlarged text): measured height used with development warning; unequal heights render all.
    - [ ] Native mounted-row semantics and full-table positions retained; unmounted rows not promised screen-reader discovery. No visual variant enum.

- [ ] **[useTableSelection](https://astryx.atmeta.com/components/useTableSelection)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/useTableSelection.doc.mjs).

  - **Selection and row modes**
    - [ ] Controlled callbacks `getIsItemSelected`, `onSelectItem({item,isSelected})`, `onSelectAll({isAllSelected})`, `getIsAllSelected`.
    - [ ] Optional `getIsIndeterminate` exposes partial selection in header checkbox, alongside all/none states.
    - [ ] `getIsItemSelectable`: default always true; false omits row checkbox. `getIsItemEnabled`: default always true; false renders disabled checkbox.
    - [ ] `hasRowHighlight`: `true` (default accent wash) / `false` (no wash); checked rows retain `aria-selected` either way. Inline wash cannot be overridden from userland.
    - [ ] Optional `getRowLabel` yields hidden `Select <identity>` checkbox name; omitted fallback `Select row`; React Context enables independent checkbox rerenders.
  - **Every named inline example**
    - [ ] **Accessible per-row checkbox names**: selected-ID set, item/all toggle callbacks and row name identity passed into Table selection plugin.
    - [ ] **Opt out of the checked-row wash**: `hasRowHighlight=false` preserves background's “open detail panel” meaning while checkbox and aria-selected still signal selection.

- [ ] **[useTableSelectionState](https://astryx.atmeta.com/components/useTableSelectionState)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/useTableSelectionState.doc.mjs).

  - **Controlled selection behavior**
    - [ ] Required full `data`, `idKey` property-name or unique-string function, controlled `selectedKeys` Set and `setSelectedKeys` state setter.
    - [ ] `getIsItemSelectable` default always true; false excludes from select-all. `getIsItemEnabled` default always true; disabled rows remain frozen through select-all/deselect-all.
    - [ ] Return `selectionConfig` for behavior plugin and `selectionState` for TableSelectionToolbar/selection-aware UI; no independent visual variants or documented uncontrolled mode.

- [ ] **[useTableSortable](https://astryx.atmeta.com/components/useTableSortable)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/useTableSortable.doc.mjs).

  - **Sort states and interaction modes**
    - [ ] Required controlled ordered `sort` array and `onSortChange`; directions `ascending`, `descending` (not asc/desc), empty array unsorted; first entry primary, remainder tiebreakers.
    - [ ] `allowUnsortedState`: `false` (default ascending → descending → ascending) / `true` (ascending → descending → unsorted).
    - [ ] `isMultiSortEnabled`: `false` (default) / `true` Shift+click multi-sort; regular click replaces complete sort state.
    - [ ] Headless TablePlugin passed as `plugins.sort`; header clicks report next state, caller owns actual data sorting including server-side operation.

- [ ] **[useTableSortableState](https://astryx.atmeta.com/components/useTableSortableState)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/plugins/sortable/useTableSortableState.doc.mjs).

  - **Ownership, directions, and defaults**
    - [ ] Uncontrolled `defaultSort=[]` initial unsorted; controlled `sort` with required `onSortChange` ignores defaultSort.
    - [ ] Directions `ascending`, `descending`; ordered primary/tiebreakers, empty array unsorted.
    - [ ] `allowUnsortedState`: `true` (state-hook default, third click clears) / `false` (cycles back ascending); differs from bare useTableSortable default false.
    - [ ] `isMultiSortEnabled`: `false` (default) / `true`; Shift+click adds/steps tiebreaker instead of replacing sort.
  - **Comparator and result behavior**
    - [ ] Required `data` is never mutated; memoized `sortedData` copy keeps input order for ties.
    - [ ] Optional ascending `comparators` per key; descending flips result. Default numbers compare numerically; other values use numeric collator in active provider locale.
    - [ ] Null/undefined/NaN sort last ascending and first descending; custom comparator pattern for status ranks.
    - [ ] Return `sort`, `sortConfig` for behavior plugin, `applySort(otherData)` for second source/subset or tree sibling sorting.
    - [ ] In-memory/full-dataset local sort only; server-side/paginated-on-server sorting uses bare behavior plugin instead.

- [ ] **[useTableStickyColumns](https://astryx.atmeta.com/components/useTableStickyColumns)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/useTableStickyColumns.doc.mjs).

  - **Pinning modes**
    - [ ] `startKeys`: contiguous run from first column through last listed key; logical inline-start (left LTR).
    - [ ] `endKeys`: contiguous run from first listed key through final column; logical inline-end (right LTR).
    - [ ] Start only, end only, or both supported; empty `{}` explicitly valid no-op pins nothing (not arbitrary noncontiguous pinning).
    - [ ] Cumulative offsets and soft scroll-aware shadow over scrolling content; no size/color/elevation enum or separate content slots.

- [ ] **[useTableTreeData](https://astryx.atmeta.com/components/useTableTreeData)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/useTableTreeData.doc.mjs).

  - **Tree options and expansion states**
    - [ ] Required `getRowMeta(item)` supplies `{id,level,hasChildren,isExpanded}` (level zero-based), `onToggleItem`, and `hasExpandableRows`.
    - [ ] `hasExpandableRows=false`: no-op with no expander/indent/tree ARIA, visually identical flat Table; true enables tree transforms.
    - [ ] `indent`: `sm` (spacing-3), `md` (default, spacing-4), `lg` (spacing-6).
    - [ ] `treeColumnKey` defaults first column; `getRowLabel` localizes identity-based expand/collapse name, omitted/blank generic `Expand row` / `Collapse row`.
    - [ ] `hasExpandAllControl`: `false` (default) / `true`, requires `isAllExpanded` (`true`, `false`, `indeterminate`) plus `onExpandAll` / `onCollapseAll`.
    - [ ] `hasRowClickExpansion`: `false` (default) / `true`; pointer convenience ignores interactive cells/text selection, leaf rows and flat data inert.
  - **Composition, accessibility, and inline example**
    - [ ] Expanded/collapsed disclosure button owns `aria-expanded` and expanded-only `aria-controls` of child rows; indentation visual hierarchy, not an invented treegrid keyboard model.
    - [ ] Accept treeConfig from state hook or direct server-driven/preflattened config; canonical order tree before selection places checkbox left of indented tree column.
    - [ ] Known limit: wrapped tree cell content does not get `textOverflow='truncate'` tooltips.
    - [ ] **File tree with expandable folders**: nested children, idKey, initially expanded `src` folder, human-readable item name, visibleData + tree plugin passed to Table.

- [ ] **[useTableTreeState](https://astryx.atmeta.com/components/useTableTreeState)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Table/useTableTreeState.doc.mjs).

  - **Ownership and flattening modes**
    - [ ] Required nested `data`, `idKey` property or unique-string/number function; `childrenKey` default `children`; entirely flat data yields plugin no-op.
    - [ ] Uncontrolled initial `defaultExpandedIds` iterable; controlled `expandedIds` ReadonlySet with `onExpandedIdsChange`, ignoring default IDs.
    - [ ] Change callback fires in both modes; lazy-load children here; `isItemExpandable` overrides default nonempty-children test so unloaded parents can expand.
    - [ ] `sortSiblings` sorts each sibling group without separating child from parent; compose `applySort` from sortable-state hook.
    - [ ] `indent`: `sm` (spacing-3), `md` (default spacing-4), `lg` (spacing-6), forwarded to tree plugin; treeColumnKey defaults first column.
    - [ ] Return visible flattened data, ready tree config, expandAll/collapseAll and aggregate `isAllExpanded=true` / `false` / `indeterminate`.
    - [ ] Collapsed subtrees unmount, not CSS-hide; cell-local state is lost, so lift persistent state. No additional visual variants.

### Shared Behavior and Utility APIs

- [ ] **[createAnimator](https://astryx.atmeta.com/components/createAnimator)**

  - Sources: [English utility doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/animator/createAnimator.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/createAnimator).

  - **Channels, operations, and lifecycle**
    - [ ] Non-React single-element animator, no UI styles/sizes; channels `x`, `y` (px), `scale` (1 normal), `rotate` (degrees), `opacity` (0–1); one animator per element.
    - [ ] `animate(target,{velocity?})` spring to specified channels, supplied units/second velocity or current on-screen velocity; Promise resolves at arrival or takeover, interruption does not jump.
    - [ ] `decay(velocity)` coasts with momentum; resolves at stop or takeover, opacity clamps endpoints. `set(values)` immediately ends channel motion to follow drag.
    - [ ] `get()` on-screen values, `getVelocity()` velocities; `stop()` freezes all channels; `destroy()` stops/releases, preserves displayed values, required when no longer used.
    - [ ] Writes translate/scale/rotate/opacity and temporary merged will-change, restored 500ms after last motion; await exit before removal then destroy.
  - **Accessibility and reduced motion**
    - [ ] Provide keyboard/AT action controls and move focus before removing/flinging focused element; use animate for equivalent button actions, decay for actual user throw.
    - [ ] Reduced motion cannot be disabled: touch move/scale cross-fades in place then lands; mouse lands immediately; x/y decay retains throw momentum, scale/rotate/opacity decay planned as move.
  - **Templates**
    - [ ] **createAnimator — Playing cards in plain JavaScript** (`AnimatorPlayingCardsVanilla.doc.mjs`): native pointers follow drag, gentle spring return/fast fling, keyboard Discard; React only mounts.

- [ ] **[createDragToSnapPoints](https://astryx.atmeta.com/components/createDragToSnapPoints)**

  - Sources: [English utility doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/dragToSnapPoints/createDragToSnapPoints.doc.mjs), [templates](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/createDragToSnapPoints).

  - **Options and settle behavior**
    - [ ] Non-React drag utility, not visual variants; required element, `axis`: `x`, `y`, required snapPoints/onSettle.
    - [ ] Snap offsets are pixels, sorted before indexing (unlike viewport-fraction BottomSheet stops); empty array disables drag. Arbitrary offsets are data, not preset detents.
    - [ ] Nearest stop after slow release, next in direction after flick, return to origin when finger moves back; release velocity continues into settle. `flickThreshold` default 1008px/s.
    - [ ] onSettle(index) after drag or settleTo completes; interrupted settle calls nothing. `settleTo(index)` supports non-drag control; invalid index no-op/development warning.
    - [ ] destroy removes listeners and stops current position; React alternative useDragToSnapPoints; prefer built-in BottomSheet snapPoints/Carousel hasSnap for those components.
  - **Accessibility and example**
    - [ ] Buttons/keys for every position, announce changed content. Reduced-motion touch/coarse-pointer settles cross-fade, fine-pointer mouse/pen lands instantly; dragging still follows finger.
    - [ ] **createDragToSnapPoints — Playing card in plain JavaScript** (`DragToSnapPointsCardVanilla.doc.mjs`): three positions, sideways drag/flick, keyboard buttons/live region; React mounts only.

- [ ] **[createPanGesture](https://astryx.atmeta.com/components/createPanGesture)**

  - Sources: [English utility doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/gestures/createPanGesture.doc.mjs), [shared template metadata](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/usePanGesture/PanDraggableBox.doc.mjs), [related advanced metadata](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/usePanGesture/PanPlayingCard.doc.mjs). No own template directory; own doc explicitly points to usePanGesture's shared example.

  - **Options and gesture lifecycle**
    - [ ] Reports values only, no element movement/UI variant. `options.axis`: `x`, `y`, or omitted both; orthogonal movement ignored/zero so cross-axis page scroll remains; both-axis touch owns element drag, trackpad still scrolls page.
    - [ ] onPan event state `began`, `changed`, `ended`, `cancelled`; translation px excludes initial claim travel; velocity px/s, release velocity at ended (zero after rest), zero at cancelled.
    - [ ] Finger/mouse/pen; touch claims after 10pt, not tap/press/long-press. Mouse/pen on selectable text selects instead of panning; use user-select none on intended draggable text.
    - [ ] Moves nothing itself; writes axis-required touch-action only. Caller offsets original position by translation and passes release velocity to settling animation.
    - [ ] destroy removes listeners/touch-action and cancels ongoing gesture with cancelled event.
    - [ ] No intrinsic keyboard/AT path: supply buttons/keys/menu alternative. Caller-driven settle/throw must respect reduced motion; use component built-in drag props where available.
  - **Shared and related examples**
    - [ ] **usePanGesture — A draggable box** (`usePanGesture/PanDraggableBox.doc.mjs`, explicitly alsoExampleFor createPanGesture): hook and factory side by side, follow drag/spring back.
    - [ ] **usePanGesture — A playing card (advanced)** (`usePanGesture/PanPlayingCard.doc.mjs`, related hook-only pattern, not claimed as own factory template): gentle return/fast throw and keyboard Discard/announcement.

- [ ] **[createReorderable](https://astryx.atmeta.com/components/createReorderable)**

  - Source: [English utility doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/reorderable/createReorderable.doc.mjs). No associated template metadata or named inline examples supplied in own doc.

  - **Reorder options and input modes**
    - [ ] No UI variant/size/color choices; required onReorder({key,to}) once per committed drop or keyboard move, not cancellation; caller renders new order in same task, latest callback used.
    - [ ] Optional isReorderable(key) rejects fixed rows (no handle/pickup); fixed endpoints bound other rows and refuse keyboard entry; omitted allows every row.
    - [ ] Pass returned behavior to List.reorderable and stable string/number itemKey to each ListItem (not React key alone/index); one memoized object per collection preferred.
    - [ ] `start(key)` enters reorder mode/focuses handle from context menu/toolbar; absent/unmounted/fixed key no-op/warns. Shared object can serve independent collections, start targets last mounted.
    - [ ] Coarse-pointer finger/pen hold on handle picks up, quick flick still scrolls; mouse/fine-pointer pen starts immediately. Dim original row, floating copy/drop line, stationary other rows, edge autoscroll; drop rearranges, cancel restores immediately.
    - [ ] Click/Space/Enter toggles reorder mode; arrows move one place immediately; Escape/re-activation ends mode and keeps changes. Move/neighbors announced; hint offers move/Done buttons for tap-only use.
    - [ ] Reorder code loads with factory; collection without behavior loads none. These documented List/handle integration recipes are behaviors, not missing cosmetic variants.

- [ ] **[useAnimatedLayout](https://astryx.atmeta.com/components/useAnimatedLayout)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useAnimatedLayout.doc.mjs).

  - **Actual options and behavior (no UI styles)**
    - [ ] Required options.containerRef to existing panel/scroll viewport; ScrollableArea ref already targets native viewport. Returns opaque AnimatedLayout, no public members.
    - [ ] One shared handle per container passed to size-changing TreeList/Collapsible; changed component and all following mixed Astryx/custom content glide together, no extra opt-in wrapper.
    - [ ] Fixed/stuck-sticky content remains still, scroll position unchanged, compositor translate rather than height animation; mouse as well as touch, reduced motion honored.
    - [ ] Without handle following content jumps. No own named inline examples or associated template metadata.

- [ ] **[useAnimator](https://astryx.atmeta.com/components/useAnimator)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useAnimator.doc.mjs).

  - **Channels, methods, and lifetime**
    - [ ] Required ref to one HTMLElement; stable handle follows node after every render, share ref with gesture hook. Unmount/replacement stops motion/resets rest; calls before mount/after unmount do nothing and promises resolve immediately.
    - [ ] Channels: `x`, `y` pixels; `scale` (rest 1); `rotate` degrees; `opacity` 0–1. Not component variant names.
    - [ ] `animate(target, {velocity?})` springs on state-change motion; `decay(velocity)` coasts; both Promise-complete. `set(values)` jumps; `get()`, `getVelocity()`, `stop()` read/halt; no React rerenders per call.

  - **Accessibility and named pattern**
    - [ ] Invoke from handlers/effects, seed drag-following animation with release velocity; non-drag button/key path reaches every action and focus moves before flinging target away.
    - [ ] Reduced motion: touch move/scale short cross-fade in place, mouse immediate; x/y decay retains throw momentum; scale/rotate/opacity decay uses planned move behavior. No opt-out.
    - [ ] [`AnimatorPlayingCards`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/useAnimator/AnimatorPlayingCards.doc.mjs) — “useAnimator — Playing cards”: follow drag, gentle spring-back, fast flick discard, keyboard Discard and announcement.

- [ ] **[useAnnounce](https://astryx.atmeta.com/components/useAnnounce)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useAnnounce.doc.mjs).

  - **Actual modes and delivery behavior**
    - [ ] Optional `hasReplace`: false (default, join messages), true (withdraw this instance's not-yet-spoken message across politeness modes, replace with latest). Independent status types use separate instances.
    - [ ] Returned announce(message, politeness): `polite` (default), `assertive` (interrupting/error urgency); empty string clears lingering/joined pending status instead of speaking.
    - [ ] Same-frame messages join in order; duplicate identical repeats kept once. Spoken messages not retroactively withdrawn; other instances continue joining.
    - [ ] Caller outside modal uses persistent body live region; caller inside open Astryx modal uses persistent internal region so native modal isolation does not hide speech. Delivery during dismissal routes to exposed region afterward.
    - [ ] Regions first mount empty and remain mounted; messages clear after a couple seconds. Outcomes/result counts/loading/saved/validation status patterns, avoid duplicate announcement of already labeled content.
    - [ ] No own named inline examples or associated template metadata.

- [ ] **[useClickableContainer](https://astryx.atmeta.com/components/useClickableContainer)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useClickableContainer.doc.mjs).

  - **Actual activation modes**
    - [ ] Required containerRef; action onClick, href/optional target navigation, or proxy to interactiveRef when neither action nor href supplied; `disabled`: false (default), true.
    - [ ] Attach returned onClick/onMouseUp/onPointerDown; nested interactive targets and text selection do not activate container. Supports middle-click and Ctrl/Cmd navigation.
    - [ ] With usePressFeedback, nested-control touch press drops parent press paint; mouse active paint unchanged. Without it click rules persist, pressed state not altered.
    - [ ] Headless pressable-surface recipe: same-element usePressFeedback/interactionOverlayStyles.backgroundColor, visually-hidden named native button/link as interactiveRef, focusOutlineStyles.focusWithin. Standard alternatives ClickableCard/SelectableCard; a single interactive element just uses button/anchor.
    - [ ] No finite UI styles or own named inline/template metadata; usage explicitly references the `Core/Hooks/useClickableContainer` story for full recipe.

- [ ] **[useClipboard](https://astryx.atmeta.com/components/useClipboard)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useClipboard.doc.mjs).

  - **Actual options and async outcomes**
    - [ ] Optional localized announce string versus omitted (no spoken confirmation); resetAfterMs default 2000, configurable duration, not style.
    - [ ] copy(text) Promise<boolean>: success true sets isCopied and optional polite announcement, restarts reset timer; failure false silently leaves state unchanged.
    - [ ] Async Clipboard API versus browser copy-command fallback inside click/key gesture for sandbox/permissions-policy refusal; true only when browser copied.
    - [ ] isCopied transient true/false drives icon/check or text confirmation; rapid recopy restarts full window, timer cleaned on unmount.
    - [ ] Ghost IconButton common pattern retains `Copy` tooltip while icon changes; also menu/text-button/value-chip copy patterns; CodeBlock/Timestamp use same implementation. No own named inline examples or associated templates.

- [ ] **[useCollator](https://astryx.atmeta.com/components/useCollator)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/i18n/useCollator.doc.mjs).

  - **Actual options and behavior**
    - [ ] Optional Intl.CollatorOptions for numeric ordering, sensitivity, punctuation handling and case order. Upstream delegates to Intl and does not enumerate option-value unions/defaults here; do not invent Astryx-specific sort styles.
    - [ ] Returns memoized Intl.Collator using active InternationalizationProvider locale; recreated when locale or option changes.
    - [ ] Use collator.compare for user-visible string ordering/custom Table comparators rather than direct Intl.Collator/localeCompare bypassing provider contract.
    - [ ] No own named inline examples or associated template metadata; no UI variations.

- [ ] **[useContainerReveal](https://astryx.atmeta.com/components/useContainerReveal)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useContainerReveal.doc.mjs).

  - **Actual trigger and content modes**
    - [ ] `isEnabled`: true (default), false (inert, no styles, content always shown), reactive after mount.
    - [ ] getContainerProps: optional hoverDelay dwell gate; forceState `active` or `inactive` versus natural hover/focus; inactive still yields to keyboard focus/coarse pointer.
    - [ ] getContentRevealProps: isRevealInverted true/false for conceal-on-hover versus reveal; isLayoutPreserved true/false for reserved box/opacity-only; forceVisibility `shown` or `hidden` versus natural state, hidden yields to focus.
    - [ ] CSS-only scoped state, no hover React rerender; nested container shadows parent. Concealed content stays mounted/tabbable/accessibility-visible (not display:none), focus-within reveals, coarse primary pointer always sees affordances; reduced motion honored.
    - [ ] Row edit/copy/remove and media/Thumbnail overlay controls; hoverDelay recommended 100–250ms, keyboard/touch immediate; force state for drag/scroll/open-menu gates. Essential actions remain discoverable.
    - [ ] No UI style presets, own named inline examples, or associated templates.

- [ ] **[useDevWarning](https://astryx.atmeta.com/components/useDevWarning)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useDevWarning.doc.mjs).

  - **Actual diagnostic behavior**
    - [ ] Required component prefix/message; condition true (default) or false. Once-per-mount warning while condition holds, development only, stripped in production; no return value/UI styles.
    - [ ] Render-safe ref/effect rather than repeated render-body logging/state rerender; message explains misuse and fix, including mutually exclusive props types cannot express.
    - [ ] Imperative devWarn outside components; not user-facing error handling. No own named inline examples or associated templates.

- [ ] **[useDirection](https://astryx.atmeta.com/components/useDirection)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/i18n/useDirection.doc.mjs).

  - **Actual modes and usage**
    - [ ] No parameters; returns `ltr` or `rtl` from InternationalizationProvider, fallback `ltr` outside provider.
    - [ ] Last-resort render-time direction for canvas/non-CSS coordinate calculations. Prefer CSS logical properties, then shared mirrored directional-icon styles, then DOM reads for gesture/keyboard logic; not conditional margin/padding style variants.
    - [ ] No own named inline examples or associated template metadata.

- [ ] **[useDragToSnapPoints](https://astryx.atmeta.com/components/useDragToSnapPoints)**

  - Source: [English API](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useDragToSnapPoints.doc.mjs).

  - **Actual options and settle modes**
    - [ ] Required element ref, `axis`: `x` or `y`, pixel-offset snapPoints array and onSettle(index). Offsets sort before use; empty array inert; these are not BottomSheet viewport fractions or fixed style presets.
    - [ ] Slow release settles nearest; flick faster than flickThreshold (default 1008px/s) advances in direction; release moving back returns where started. Settle carries release velocity.
    - [ ] Changing axis, points or threshold recreates drag without relocating element, including in-flight settle; next interaction starts there. Latest onSettle callback used each render.
    - [ ] Stable settleTo(index) offers same positions without dragging; state/announcement follows onSettle. Use component-owned BottomSheet/Carousel snapping when applicable; createDragToSnapPoints outside React.

  - **Accessibility and named template**
    - [ ] Button/key alternatives for every position and announcement of changed content; dragging never sole activation path.
    - [ ] Reduced motion: post-touch/coarse-device settle cross-fades; mouse/pen elsewhere lands immediately. settleTo fades on coarse primary pointer, immediate on fine; finger tracking remains direct during drag.
    - [ ] [`DragToSnapPointsCard`](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/useDragToSnapPoints/DragToSnapPointsCard.doc.mjs) — “useDragToSnapPoints — Playing card on three places”: nearest/flick/return settle, keyboard position buttons and announcement.

- [ ] **[useEntryAnimation](https://astryx.atmeta.com/components/useEntryAnimation)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useEntryAnimation.doc.mjs).

  - **Presets and lifecycle**
    - [ ] `preset`: `slideDown` (default), `slideUp`, `fadeIn`, `scaleIn`.
    - [ ] Dynamically inserted after initial paint: return entry StyleX style using Astryx motion duration/easing tokens.
    - [ ] Initial page load and server-rendered hydration: return `null`, with no entry animation; post-hydration mounts animate.
    - [ ] Compose returned style with other `stylex.props()` styles; documented content patterns are validation messages, toasts, and expanding sections.

- [ ] **[useFocusTrap](https://astryx.atmeta.com/components/useFocusTrap)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useFocusTrap.doc.mjs).

  - **Activation and keyboard behavior**
    - [ ] Required `isActive`: active/inactive trap; attach `containerRef` and call `focusFirst()` when opening.
    - [ ] Tab and Shift+Tab wrap when navigation from inside would escape; Tab originating outside remains page-owned.
    - [ ] Optional `onEscape` handles closing from inside the container.
    - [ ] Deactivation/unmount restores the previously focused element unless focus already moved elsewhere or never entered the trap; trigger-focused comboboxes remain unaffected.
    - [ ] Outside mouse clicks are not intercepted: compose light dismissal separately. Intended for modal/dialog focus management, not tooltips or ordinary dropdowns.

- [ ] **[useGridFocus](https://astryx.atmeta.com/components/useGridFocus)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useGridFocus.doc.mjs).

  - **Geometry and focus ownership**
    - [ ] Required `columns`; configurable `cellSelector` defaults to `button:not([disabled]), [tabindex]:not([tabindex="-1"])`.
    - [ ] Match every grid position, including disabled/empty cells, to preserve geometry; optional `isCellFocusable` filters destinations (omitted: all matches focusable).
    - [ ] `getFocusTarget`: focus the matched cell itself when omitted, or a resolved inner element such as a gridcell's button.
    - [ ] `hasRovingTabIndex`: `false` (default, focus movement only) / `true` (single repaired `0` tab stop, other targets `-1`); attach `handleFocus`, seed one target at `0`.
  - **Navigation and patterns**
    - [ ] Attach `gridRef` and `handleKeyDown`; arrows navigate cells, vertical movement uses column count.
    - [ ] Home/End reach row boundaries; Ctrl+Home/Ctrl+End reach grid boundaries.
    - [ ] `onNavigateBefore` / `onNavigateAfter` receive column and offset for cross-grid navigation.
    - [ ] `onPageUp` / `onPageDown` receive the key event; calendar pattern uses month navigation and Shift for a larger step such as a year.
    - [ ] `focusCell(index)` clamps to valid range; `focusFirst()` and `focusLast()` target focusable cells.

- [ ] **[useHotkeys](https://astryx.atmeta.com/components/useHotkeys)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useHotkeys.doc.mjs).

  - **Registration modes and behavior**
    - [ ] Per-registration `keys` and `onPress`; documented shortcut patterns: `mod+k` command palette, `?` help overlay, `escape`, and navigation shortcuts.
    - [ ] `mod` resolves to Command on macOS and Ctrl elsewhere; display the same combo with `Kbd`.
    - [ ] `allowInInputs`: `false` (default, skip input/textarea/select/contenteditable) / `true` (allow while typing).
    - [ ] `isDisabled`: suspend/resume an entry without unmounting, such as while a modal or wizard owns the keyboard.
    - [ ] Skip already-default-prevented events; matching calls `preventDefault()` before `onPress(KeyboardEvent)`.
    - [ ] Punctuation uses the produced `event.key` character (`?`, not `shift+/`); one window listener per instance, ref-held handlers avoid resubscription, SSR-safe.
    - [ ] Global shortcut utility, not focus-scoped list/grid navigation; no visual variants or rendered output.

- [ ] **[useImageMode](https://astryx.atmeta.com/components/useImageMode)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useImageMode.doc.mjs).

  - **Analysis modes and asynchronous states**
    - [ ] Result/fallback choices: `dark`, `light`, `null`; `fallback` default `null`, used while loading or on error and for null/undefined source.
    - [ ] Sample whole image by default or normalized `{x, y, width, height}` `region` for a text-overlay area.
    - [ ] Luminance `threshold` default `0.5`: below dark / above light.
    - [ ] OffscreenCanvas sampling with APCA perceptual lightness, sRGB linearization and power curve; no visible canvas or paint-path layout work.
    - [ ] Source changes fetch/reanalyze; graceful CORS/network-error fallback. Pair with `MediaTheme`; not intended for rapidly changing video frames.

- [ ] **[useIndicatorFocusRing](https://astryx.atmeta.com/components/useIndicatorFocusRing)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useIndicatorFocusRing.doc.mjs).

  - **Focus behavior**
    - [ ] `isDisabled`: `false` (default) / `true` (skip ring).
    - [ ] `containerRef` wraps only the indicator, with one element child; imperatively ring that child so its own border radius shapes the outline.
    - [ ] Spread returned `focusProps` (`onFocus`, `onBlur`) on hidden-input owner, not input itself; keyboard `:focus-visible` paints ring, pointer clicks stay quiet.
    - [ ] Checkbox/radio visually hidden-input pattern; theme replacement cannot remove visible focus by ignoring a supplied ring prop. No visual style variants.

- [ ] **[useInputContainer](https://astryx.atmeta.com/components/useInputContainer)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useInputContainer.doc.mjs).

  - **Delegation modes**
    - [ ] Required wrapper `containerRef` and inner `inputRef`; attach both returned `onClick` and `onMouseUp` to wrapper.
    - [ ] `disabled`: `false` (default) / `true`.
    - [ ] Noninteractive icon/padding/status area delegates to text-like input/textarea `.focus()`; other types (checkbox, radio, file) receive `.click()`.
    - [ ] Nested clear buttons/calendar toggles/links remain independently interactive without focus stealing; input-family wrapper behavior, not a bare-input visual variant.

- [ ] **[useInputStatusIcon](https://astryx.atmeta.com/components/useInputStatusIcon)**

  - Sources: [English hook doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useInputStatusIcon.doc.mjs), [status types](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/FieldStatus/FieldStatus.doc.mjs), [icon sizes](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Icon/Icon.doc.mjs).

  - **Status and presentation choices**
    - [ ] `status`: type `error`, `warning`, or `success` plus message; undefined means no status.
    - [ ] `statusVariant`: `attached` (default, plain glyph and separate message box), `detached` (no on-field icon; message box owns icon), `tooltip` (focusable info-tip button and message tooltip).
    - [ ] `isInGroup`: `false` (default) / `true` (InputGroup owns status rendering); allow hook to return no icon.
    - [ ] `size`: `xsm`, `sm`, `md` (default), `lg`.
  - **Accessibility and output**
    - [ ] Render `statusIcon` inside bordered input; tooltip works by keyboard, pointer, touch, and AT.
    - [ ] Merge `describedBy` into input `aria-describedby`; supplied exactly while tooltip DOM exists, avoiding dangling IDs.
    - [ ] Attached/detached modes still require a message box; glyph alone is not validation messaging. Hook is for input affordance, not field-level status.

- [ ] **[useInteractiveRole](https://astryx.atmeta.com/components/useInteractiveRole)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useInteractiveRole.doc.mjs).

  - **Semantic resolution**
    - [ ] Return alternatives: `link` (anchor), `button`, `inert` (noninteractive span/div).
    - [ ] Precedence: `href` → `onClick` → parent interactive-trigger context → inert.
    - [ ] `isDisabled`: `false` (default) / `true` (ignore href, then use click/context/inert resolution; do not create disabled navigation).
    - [ ] Native link rendering preserves keyboard/middle-click behavior; trigger contexts include Popover/DropdownMenu; documented consumers Token, Thumbnail, Item, ClickableCard. No independent size/color variants.

- [ ] **[useKeyboardHint](https://astryx.atmeta.com/components/useKeyboardHint)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useKeyboardHint.doc.mjs).

  - **Modes and lifecycle**
    - [ ] `orientation`: `horizontal` (default, left/right arrows), `vertical` (up/down), `both` (all four).
    - [ ] `isEnabled`: `true` (default) / `false`, including disabled/read-only widget patterns.
    - [ ] `dismissAfterMs`: omitted by default (no time limit); optional timed dismissal carries WCAG 1.4.13 caution.
    - [ ] First outside keyboard-focus-visible entry shows hint; arrow press, Escape, or blur dismisses; does not re-show for that instance.
    - [ ] Internal focus movement reanchors; Escape dismisses only hint through layer stack and keeps focus unchanged.
  - **Composition and accessibility**
    - [ ] Render `hintElement` unconditionally as last container child; `Kbd` arrows, manual top-layer popover, CSS anchor to focused item, fixed positioning, `aria-hidden`, no layout/accessibility-tree impact.
    - [ ] Attach `onFocus`, `onBlur`, `onKeyDown`; call hint key handler before own navigation, without preventDefault/stopPropagation.
    - [ ] Custom roving-tabindex composites only; Toolbar, TabList, SegmentedControl already integrate it.

- [ ] **[useLayer](https://astryx.atmeta.com/components/useLayer)**

  - Sources: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Layer/useLayer.doc.mjs), [original render-prop types](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/Layer/useLayer.tsx).

  - **Positioning choices**
    - [ ] Required `mode`: `context` (trigger/CSS anchor) or `fixed` (explicit `x`, `y`; trigger `ref` undefined).
    - [ ] Context `placement`: `above` (source default), `below`, `start`, `end`; `alignment`: `start`, `center` (source default), `end`.
    - [ ] Context `positioning`: `anchor` (default derived styles) or `custom` (caller insets/anchor-size cover; ignore placement/alignment and own RTL behavior).
    - [ ] Context `offset`: CSS length / number in px; default flush `0`; applied flip-safely on placement axis. `as`: `div` (default) or `span`.
    - [ ] Context logical sides mirror inherited direction; viewport overflow flips to opposite placement, centered layers additionally slide along alignment axis.
  - **State, mounting, and content**
    - [ ] `lightDismiss`: `false` (default) / `true` native outside dismissal; `onShow` / `onHide`, `show()`, `hide()`, `isOpen`.
    - [ ] Context `lazyMount`: `false` (default resolve after hydration, keep closed DOM) / `true` (resolve/mount on show, unmount on hide, retain marker).
    - [ ] Render inert template marker with matching SSR/client markup; safe parent hosts content inline, unsafe paragraphs/links/buttons/inline/restricted structures portal to nearest safe ancestor.
    - [ ] Preserve live CSS variable inheritance, JSX-position direction/writing mode, natural focus order; Popover API top layer escapes clipping/stacking independently of host.
    - [ ] `render(children, props)` supports arbitrary content, StyleX/class/inline styles; context role/accessible name and pointer-enter/leave handlers enable caller-owned ARIA/hover behavior.
    - [ ] `anchorId` and unique `id` support positioning/ARIA composition; both modes reset theme body text and ancestor surface/group membership including group-owned state, not unrelated contexts/explicit props.
    - [ ] Patterns: custom popover, hover card, tooltip, fixed-coordinate overlay. Caller owns complete accessibility pattern; prefer higher-level APIs for standard cases.

- [ ] **[useListFocus](https://astryx.atmeta.com/components/useListFocus)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useListFocus.doc.mjs).

  - **Navigation modes and defaults**
    - [ ] `orientation`: `horizontal` (left/right), `vertical` (default, up/down), `both` (four arrows); RTL supported.
    - [ ] `wrap`: `true` (default) / `false`; `hasHomeEnd`: `true` (default) / `false` first/last enabled item jumps.
    - [ ] `hasPaging`: `false` (default) / `true`; PageDown reaches last fully visible enabled item, then next viewport; PageUp mirrors; no wrapping, nonscrolling list acts as End/Home.
    - [ ] `hasRovingTabIndex`: `false` (default, never change tabindex) / `true` (one `0`, rest `-1`; repair mounting/removal/disabled changes).
    - [ ] `hasCaretGuard`: `false` (default) / `true`; preserve text-input/textarea caret until travel boundary, preserve selection, never steal contenteditable arrows.
  - **Scoping, output, and patterns**
    - [ ] `itemSelector` default `[role="menuitem"]`; optional `boundarySelector` such as `[role="menu"]` scopes nested menus to their own level.
    - [ ] Optional `onEscape` calls callback and consumes key; omitted passes Escape to enclosing layer.
    - [ ] Attach `listRef`, `handleKeyDown`, and `handleFocus` (roving synchronization, otherwise no-op).
    - [ ] `focusItem(index)` clamps; `focusFirst()` / `focusLast()` return whether enabled item focused; `ownsEvent()` guards consumer Enter/Space/typeahead against nested levels; `getItems()` returns own-level DOM-order targets.
    - [ ] Patterns: vertical dropdown/menu, horizontal toolbar/tab strip, listbox, segmented single-tab-stop composite; combine caret guard for inline editors.

- [ ] **[useLocale](https://astryx.atmeta.com/components/useLocale)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/i18n/useLocale.doc.mjs).

  - **Locale behavior**
    - [ ] No parameters or visual variations: return authoritative InternationalizationProvider BCP 47 locale, or `en` without a provider.
    - [ ] Pass locale to pure formatting helpers/sibling-package APIs; do not invent another locale from `navigator.language` or hardcoded fallback. Locale strings are open data, not a finite style vocabulary.

- [ ] **[useLongPress](https://astryx.atmeta.com/components/useLongPress)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useLongPress.doc.mjs).

  - **Gesture options and cancellation**
    - [ ] Required `onLongPress({x,y})` receives touch-start coordinates; `delayMs` default `500`, `moveCancelPx` default `10` on either axis.
    - [ ] `disabled`: `false` (default) / `true` inert handlers.
    - [ ] Spread `onTouchStart`, `onTouchMove`, `onTouchEnd`, `onTouchCancel`; single-finger hold cancels on movement, lift before threshold, multitouch, or unmount.
    - [ ] Touch-event timestamps preserve a completed hold even when busy-page handling delays lift; report long press before lift acts.
    - [ ] Touch-only path supplements pointer contextmenu for cursor-positioned ContextMenu, particularly iOS Safari which does not synthesize that event. Not the only path to a primary action.

- [ ] **[useMediaQuery](https://astryx.atmeta.com/components/useMediaQuery)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useMediaQuery.doc.mjs).

  - **Query states and SSR**
    - [ ] Required arbitrary CSS `query`; returned `matches` is `false` on first render for SSR safety, then follows `window.matchMedia` changes.
    - [ ] Missing matchMedia retains server default without throwing; matching/nonmatching are behavior states, not visual variants.
    - [ ] Documented usage categories: viewport-width layout switching, color-scheme preference, motion preference. Prefer existing responsive tokens/props; not for guaranteed matching first paint.

- [ ] **[useMenuPress](https://astryx.atmeta.com/components/useMenuPress)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useMenuPress.doc.mjs).

  - **Pointer model and options**
    - [ ] Required `menuRef` and enabled-row `itemSelector`; dividers/headings/disabled rows clear highlight.
    - [ ] Optional `triggerRef` and `onTriggerPress(pointerType)` open under held pointer; return false if press closed menu instead.
    - [ ] Mouse opens immediately on press and drags to rows; finger/pen opens while down, immediately by default or after `150ms` content-touch delay in scroller without scrolling.
    - [ ] Optional `longPressDelayMs` replaces default delay everywhere; earlier released tap opens through trigger click; default touch picking requires movement of `10px`.
    - [ ] `isEnabled`: `true` (default) / `false`.
  - **Highlight, release, scrolling, and composition**
    - [ ] Default `onHighlight` focuses row with preventScroll, or menu root for null; pointer focus is not focus-visible. Custom callback supports aria-activedescendant listbox without moving DOM focus.
    - [ ] Default `onActivate` dispatches click carrying release button/modifiers; activate row under release, not original press target.
    - [ ] `onDismiss`: mouse released outside with no action closes; finger released outside leaves menu open.
    - [ ] `getScroller`: custom scroller or overflowing menu root by default; held pointer near top/bottom autoscrolls.
    - [ ] Spread `menuProps` (pointerdown + press-model marker), `triggerProps` (pointerdown/contextmenu); guard trigger click with `isTriggerClickFromPress()`; `cancel()` ends gesture without action when menu closes.
    - [ ] Document-level pointerId tracking follows off-row movement; swallow synthetic touch-end click to prevent double action. Use `isMenuPressActivation()`, not `detail===0`, to distinguish pointer activation.
    - [ ] Menu touch-action pattern: `pinch-zoom` when fitting; `pan-y pinch-zoom` when scrolling. Browser owns scroll-vs-press arbitration and pinch zoom.
    - [ ] Already integrated by DropdownMenu, ContextMenu, submenus, Selector, menu bottom sheet; custom menu/listbox only, no independent visual variants.

- [ ] **[useMergedRefs](https://astryx.atmeta.com/components/useMergedRefs)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useMergedRefs.doc.mjs).

  - **Ref behavior**
    - [ ] No visual variations: combine up to six object refs, callback refs, or undefined entries so all receive same element.
    - [ ] Returned callback stays stable until an input ref changes; unrelated rerenders do not detach/reattach.
    - [ ] Documented pattern: forwarded consumer ref plus internal refs on one element; use hook instead of calling `mergeRefs` in JSX each render.

- [ ] **[useOverflow](https://astryx.atmeta.com/components/useOverflow)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useOverflow.doc.mjs).

  - **Measurement modes**
    - [ ] Required `itemCount`; `gap=0` px, `minVisibleItems=0` by default (minimum can remain visible even when not fitting).
    - [ ] `collapseFrom`: `start`, `end` (default).
    - [ ] `behavior`: `observeParent` (parent width, content-sized visible container) / `observeSelf` (default).
    - [ ] Hidden `measureRef` container contains all items, plus optional final `data-overflow-indicator` item such as “+N more”; visible `containerRef` renders fitting subset.
    - [ ] `visibleCount` and `hasOverflow` update through ResizeObserver on container/measured-child changes without flicker; horizontal-width only, not vertical overflow.

- [ ] **[usePanGesture](https://astryx.atmeta.com/components/usePanGesture)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/usePanGesture.doc.mjs).

  - **Gesture modes and event states**
    - [ ] `axis`: `x`, `y`, or omitted (both); orthogonal drag not reported under axis restriction and orthogonal values are `0`.
    - [ ] Required `onPan`: states `began`, `changed`, `ended`, `cancelled`; translation in px since recognition excludes threshold travel; velocity in px/s, release velocity at ended, `0` after resting or cancellation.
    - [ ] Finger/mouse/pen input; touch recognized at `10pt`, taps/long presses remain separate; mouse/pen on selectable text selects it without panning (use user-select none for drag surface).
    - [ ] Restricted axis allows cross-axis page scrolling; unrestricted claims any drag from element; trackpad scroll remains page scroll.
    - [ ] Hook returns nothing; ref-node/axis changes recreate recognizer, unmount destroys it; teardown ends active drag without calling `onPan(cancelled)` from effect. Handler ref updates do not recreate.
  - **Motion and accessible alternatives**
    - [ ] Consumer moves element using start position + translation, settles with release velocity; not a built-in animation or visual size/color variant.
    - [ ] Supply equivalent buttons/arrows/menu for keyboard and AT (WCAG 2.5.7); reduced-motion settle/throw goes to destination without travel.
    - [ ] Prefer BottomSheet `snapPoints` or Carousel `hasSnap` for built-in patterns; non-React equivalent is `createPanGesture(element, options)` with `destroy()`.
  - **Every associated template**
    - [ ] **usePanGesture — A draggable box** (`PanDraggableBox`): side-by-side hook/factory cards, same StyleX styles, follow finger/mouse/pen and spring back; draggable tile/swipe/drag-release starter. [Metadata](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/usePanGesture/PanDraggableBox.doc.mjs).
    - [ ] **usePanGesture — A playing card (advanced)** (`PanPlayingCard`): follow drag, settle on gentle release, fly off on quick flick; keyboard Discard deals and announces next card; swipe-dismiss/deck/flick pattern driven by React state and StyleX. [Metadata](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/cli/assets/templates/blocks/components/usePanGesture/PanPlayingCard.doc.mjs).

- [ ] **[usePressFeedback](https://astryx.atmeta.com/components/usePressFeedback)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/usePressFeedback.doc.mjs).

  - **Press-paint composition and input states**
    - [ ] No parameters/visual variant prop; spread `data-astryx-pressable` on paint-owning element.
    - [ ] Compose `interactionOverlayStyles.backgroundColor` or `.backgroundImage` when system owns hover+press; `.pressedBackgroundColor` when hover is custom. Press token is `--color-overlay-pressed`.
    - [ ] Desktop mouse keeps CSS `:active`; coarse-primary-pointer stylesheet drops active and controller paints finger, connected mouse/trackpad, and Space-key presses.
    - [ ] Touch onset immediate except content-touch delay in scroller; quick tap answers at lift; hold stays painted; release fades; scroll cancellation clears without fade, flick paints nothing.
    - [ ] One shared document controller installed by first mounted pressable, removed by last; no per-element React state/listener. Innermost marked element owns nested press.
    - [ ] Custom local pressable pattern only: built-in Button/Item/ClickableCard already marked; do not add conflicting coarse-pointer active paint.

- [ ] **[useScrollableArea](https://astryx.atmeta.com/components/useScrollableArea)**

  - Sources: [English hook doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useScrollableArea.doc.mjs), [original option types](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useScrollableArea.ts), [owning scroll-family doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/ScrollableArea/ScrollableArea.doc.mjs).

  - **Actual scroll options**
    - [ ] Required `axis`: `inline`, `block`, `both`; map via direction/writing mode. Hook type requires axis (family component's block default is not a hook default).
    - [ ] Required `keyboardAccess.owner`: `content`, `viewport`, `contentOrViewport`; latter two require label, optional role `group` / `region`.
    - [ ] `overscroll`: `allow` (default) / `contain`; `stickyContainment`: `whenScrollable` (default) / `always`.
    - [ ] Per-axis state for `inline` and `block`: `isScrollable`, `atStart`, `atEnd`; effective only if computed overflow scroll-capable and geometry exceeds shared `1px` tolerance.
  - **Keyboard ownership and structure**
    - [ ] `viewport` joins tab order only when effectively scrollable; give concise accessible name and visible focus indicator.
    - [ ] `content` assumes integration provides keyboard access to full scroll range.
    - [ ] `contentOrViewport` checks current content at each keyboard entry: forward Tab delegates to first sequential native link/button only if native scroll keys preserved; inputs/composites/nested scroll areas retain viewport stop.
    - [ ] Shift+Tab from delegated first child skips viewport; pointer/programmatic focus remains viewport; no continuous focusability tracking.
    - [ ] Pass fully resolved caller props, xstyle, refs through both `getViewportProps()` and `getContentProps()`, spread each once; real observed content box required.
    - [ ] Getter owns fitting clip/active overflow/Sticky containment/accessibility/chaining/owner registration and removes conflicting inline overflow. Both boxes observed; stable logical scroll/edge state returned.

- [ ] **[useScrollLock](https://astryx.atmeta.com/components/useScrollLock)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useScrollLock.doc.mjs).

  - **Lock states and input behavior**
    - [ ] Required `isLocked=true` / `false`; no returned UI or visual variants. Pair with modal/dialog visibility, not nonmodal popovers/tooltips.
    - [ ] Lock root overflow, or body when it owns viewport overflow; root/body overscroll-behavior-y none prevents bounce; preserve scroll offset and iOS toolbar without jump.
    - [ ] Wheel/keyboard/scrollbar page scrolling blocked; stable scrollbar gutter prevents desktop content/fixed chrome shifting.
    - [ ] Coarse-pointer single-finger page pans cancelled for iOS Safari; overlay scrollers that can consume pan, native range dragging, pinch zoom and panning already-zoomed view remain available.

- [ ] **[useScrollOverflow](https://astryx.atmeta.com/components/useScrollOverflow)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useScrollOverflow.doc.mjs).

  - **Logical edge states**
    - [ ] No parameters or styles: attach `scrollRef` to horizontal overflow-auto/scroll container; expose `overflowStart`, `overflowEnd`, `hasOverflow` booleans.
    - [ ] Start means left in LTR/right in RTL; end mirrors; total overflow compares scrollWidth/clientWidth with `1px` tolerance.
    - [ ] Update on scrolling, resize, added/removed/resized child, gap/padding changes through scroll listeners and ResizeObserver/MutationObserver.
    - [ ] Carousel/horizontal-list patterns: show/hide navigation buttons and edge fades; vertical tracking unsupported.

- [ ] **[useStreamingText](https://astryx.atmeta.com/components/useStreamingText)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useStreamingText.doc.mjs).

  - **Reveal choices and stream states**
    - [ ] `speed`: `natural` (default, steady about 2 chars/frame), `fast` (backlog-scaled about 4 chars/frame), `instant` (full text, no animation).
    - [ ] Required accumulated `targetText` and `isStreaming`; true gradually reveals, false immediately returns full target.
    - [ ] requestAnimationFrame decouples burst arrival from display, advances at word/syntax boundaries to avoid mid-word/Markdown glitches.
    - [ ] Theme motion tokens through useTheme with outside-provider fallback; use instant for reduced-motion/nonanimated contexts. Documented AI/chat/Markdown stream smoothing, not static-text styling.

- [ ] **[useTheme](https://astryx.atmeta.com/components/useTheme)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/theme/useTheme.doc.mjs).

  - **Resolved modes and token output**
    - [ ] No parameters or independent theme presets: nearest theme `name`, default theme name when no provider; arbitrary theme names are not finite variants.
    - [ ] Effective `mode`: `light`, `dark`; system preference already resolved to one of these.
    - [ ] `token(name)` resolves one current CSS value; `tokens` resolves defaults + theme overrides and remains stable until active theme/mode changes, matching resolveThemeTokens.
    - [ ] JavaScript-value patterns: SVG attributes, canvas, Vega/D3/maps/charts; use data-visualization tokens such as color-data-categorical-blue rather than arbitrary UI paint.
    - [ ] Prefer CSS variables/StyleX for ordinary styling; local media-surface/external CSS cascade overrides may not be represented. Non-React equivalent resolveThemeTokens.

- [ ] **[useThemeName](https://astryx.atmeta.com/components/useThemeName)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/theme/useThemeName.doc.mjs).

  - **Identity sources and absence**
    - [ ] No parameters or visual variations: read nearest ThemeContext name, otherwise document-root theme name.
    - [ ] Return `null` when neither exists; null does not mean theme named `default`.
    - [ ] Theme-identity/semantic-icon-selection pattern without resolving tokens/mode; useTheme supplies those when needed.

- [ ] **[useTranslator](https://astryx.atmeta.com/components/useTranslator)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/i18n/useTranslator.doc.mjs).

  - **Translation behavior**
    - [ ] No parameters/visual variations: return stable translator bound to current InternationalizationProvider context.
    - [ ] Call with message key and optional ICU values of `string`, `number`, `Date`; resolves active locale, provider overrides, shipped English fallback catalog.
    - [ ] Hook called in component; translator safe in render, event handlers, effects and other parent-lifecycle code. Keys/locales/values are open content, not a finite visual vocabulary.

- [ ] **[useTreeFocus](https://astryx.atmeta.com/components/useTreeFocus)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useTreeFocus.doc.mjs).

  - **Configuration and defaults**
    - [ ] `itemSelector` default `[role="treeitem"]` in visible DOM order; `isItemDisabled` defaults data-tree-disabled/aria-disabled; `getLevel` defaults 1-based aria-level.
    - [ ] `hasRovingTabIndex`: `false` (default) / `true` single repaired `0`/`-1` stop, preserving initial `0` seed; handleFocus synchronizes pointer focus.
    - [ ] `typeahead`: `true` (default) / `false`; prefix matching ignores case/accents, stroked đ/ł/ø/ħ/ŧ/ƀ and Turkish ı/İ consistently across locales.
  - **Tree keyboard and callback behavior**
    - [ ] Up/Down/Home/End move over visible enabled items; Right expands collapsed parent or moves to first child; Left collapses expanded parent or moves to parent.
    - [ ] Enter/Space invokes `onActivate(item,id)`; true means handled, false/undefined falls back to parent expansion via `onToggleExpand(id)`.
    - [ ] `onActiveChange` reports moved-focus ID; attach treeRef and handleKeyDown to role-tree container plus handleFocus when roving; focusFirst/focusLast target enabled visible items.
    - [ ] Hierarchical TreeList/custom-tree pattern, not linear-list/grid traversal; consumer wires expansion state.

- [ ] **[useTypeahead](https://astryx.atmeta.com/components/useTypeahead)**

  - Source: [English doc](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/packages/core/src/hooks/useTypeahead.doc.mjs).

  - **Search modes and options**
    - [ ] Required `getItemLabels()` DOM-order array and `onMatch(index)`; null/undefined/empty label preserves unmatchable positional slot.
    - [ ] `getCurrentIndex` default `() => -1`; provide current item so repeated-letter presses cycle through matches rather than sticking on first.
    - [ ] `resetMs` default `750ms`; optional `isDisabled(index)` skips entries; `reset()` clears buffer when collection closes.
    - [ ] Printable characters build prefix buffer; repeated same letter cycles; case/accents/stroked đ/ł/ø/ħ/ŧ/ƀ/Turkish ı/İ folded consistently; typed accent prefers exact match.
  - **Focus and keyboard composition**
    - [ ] Returned onKeyDown accepts React/native KeyboardEvent, returns true when printable key consumed; call before arrow-navigation handler and fall through only on false.
    - [ ] Hook moves nothing; onMatch delegates to own list/grid focus manager. Menu/listbox/select/tree patterns, not text-input use where typing would conflict. No UI style variants.
