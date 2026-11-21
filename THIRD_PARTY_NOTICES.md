# Third-party notices

## Meta Astryx

The implementations under `src/toast/`, `src/avatar/`, `src/token/`, `src/stack_item/`, `src/aspect_ratio/`, `src/blockquote/`, `src/badge/`, `src/button/`, `src/button_group/`, `src/card/`, `src/clickable_card/`, `src/selectable_card/`, `src/private_attributes/`, `src/layout/`, `src/divider/`, `src/icon/`, `src/icon_button/`, `src/progress_bar/`, `src/motion/`, `src/visually_hidden/`, `src/skeleton/`, `src/spinner/`, `src/status_dot/`, `src/size_context/`, `src/input_group/`, `src/kbd/`, `src/layer/`, `src/link/`, `src/text/`, `src/theme/`, `src/hooks/`, `src/tooltip/`, `src/interactive_role_context/`, `src/i18n/`, and `src/utils/`, together with `assets/button-group-controller.js` and `assets/astryx.css` adapt Meta Astryx's Button, Link/Text typography and navigation, Spinner, icon sizing/lookup, grouping/context contracts, tooltip/press behavior, design tokens, and implemented theme values:

- Repository: https://github.com/meta/astryx
- Reference revision: `476a3153417dbf75c565f7df05b0784e94ddd59e`
- Sources: `packages/core/src/Button/Button.tsx`, `packages/core/src/Badge/Badge.tsx` (contrast-audit mappings), `packages/core/src/Spinner/{Spinner.tsx,spinnerRotor.ts}`, `packages/core/src/Icon/IconSize.stylex.ts`, `packages/core/src/Tooltip/useTooltip.tsx`, `packages/core/src/utils/{pressFeedback.ts,pressGesture.ts}`, `packages/core/src/theme/tokens.stylex.ts`, `packages/themes/neutral/src/{neutralTheme.ts,neutralPaletteRefs.generated.ts}`, and `apps/docsite/src/themes/astryxTheme.ts`.
- ButtonGroup additionally follows `packages/core/src/ButtonGroup/{ButtonGroup.tsx,ButtonGroupContext.ts}`, `packages/core/src/hooks/{useListFocus.ts,useGroupPressTracking.ts}`, `packages/core/src/IconButton/IconButton.tsx`, and `packages/core/src/Layer/layerScopedContext.tsx` at the same revision.
- Link additionally follows `packages/core/src/Link/`, `Text/`, `InteractiveRoleContext/`, `hooks/{useInteractiveRole.ts,usePressFeedback.ts}`, `Icon/globalIconRegistry.tsx`, `theme/types.ts`, and `utils/safeUrl.ts` at the pinned revision.
- Spinner's standalone motion and announcement support additionally follows `Spinner/{Spinner.tsx,spinnerRotor.ts}` and `hooks/{useAnnounce.ts,useAnnounceRenderedText.ts}` at the same revision. Shared polite/native-dialog support remains under the hooks owner.
- Badge follows `Badge/{Badge.tsx,index.ts}`, shared color tokens, Neutral's Badge/palette overrides, `utils/textDirection.ts`, and `i18n/firstStrongDirection.ts` at the same revision. The static script-property range table is derived from that original expression and retains its source/engine provenance.
- Divider follows `Divider/{Divider.tsx,index.ts}` and its authoritative specification. Its showcase-only native Card/Section/Stack framing follows those original owners and the shared container-padding protocol; it does not claim those components are migrated.
- Kbd follows `Kbd/Kbd.tsx`, `utils/isApplePlatform.ts`, and the Kbd messages in `packages/core/locales/en.json`. Localized text props adapt the catalog boundary without claiming a completed internationalization provider. Keycaps retain the pinned group-only theme target despite the published catalog's additional key class.
- Skeleton follows `Skeleton/{Skeleton.tsx,index.ts}`, its authoritative specification, shared radius/color/duration tokens and the pre-tempo duration contract. The exact held opacity keyframes retain the source's compositor-friendly linear ramps. Native Card/Stack example framing does not claim independent Card/Layout migrations.
- StatusDot follows `StatusDot/{StatusDot.tsx,index.ts}`, its authoritative specification and shared semantic icon tokens. Neutral fallback for unpainted variants and universal forced-color outlines are explicitly approved source-gap corrections. Spec-led tooltip role/host-name policy stays with StatusDot; shared surfaces/show-hide remain Tooltip-owned, hit-area/watch and ref-counted holder remain separate `utils/{hitArea.ts,touchHitArea.ts}` owners, and announcements consume existing hook support. This does not claim public Tooltip/Theme/announcement migrations complete.
- ProgressBar follows `ProgressBar/{ProgressBar.tsx,ProgressBarTouch.ts,ProgressBarMarkTooltip.tsx}`, its authoritative specification, `VisuallyHidden.tsx`, `motion/{themeTiming.ts,uikit/progressBar.ts,uikit/timing.ts}`, `utils/{sharedResizeObserver.ts,touchQueries.ts}`, and the source percentage-message contract. Approved inset boundaries/custom fallback are documented separately from source paint. Shared timing, geometry and tooltip responsibilities retain their upstream owners; supporting public migrations remain independent.
- Card follows `Card/{Card.tsx,index.ts}`, its authoritative specification and `docs/architecture/container-padding.md`. Shared `utils/types.ts` and `Layout/{padding.stylex.ts,container.stylex.ts,containerCorners.stylex.ts,containerBlockBleed.stylex.ts}` retain their upstream ownership; FR3 corrects the documented applied/published inset mismatch. Its six static examples and inline payout row use source-derived native Heading/Stack/Grid/Layout framing without claiming those public migrations, Toast, ClickableCard or SelectableCard complete.
- SelectableCard follows `SelectableCard/{SelectableCard.tsx,selectableCardInput.ts}`, its specification, shared clickable-container/press/focus/overlay owners, and the three SelectableCard templates under the Card directory. Its source-local before-layer compatibility policy and private input channel retain their upstream ownership; no Lab group migration is claimed.
- ClickableCard follows `ClickableCard/{ClickableCard.tsx,index.ts}`, `hooks/useClickableContainer.ts`, `utils/{disabledInteraction.ts,interactionOverlay.stylex.ts,pressFeedback.ts,pressGesture.ts}`, `motion/themeTiming.ts`, `privateAttributes/layerRoot.stylex.ts`, and `Layer/layerSurfacePress.ts` at the same revision. Native owner scoping and weak layer-anchor metadata are documented Dioxus adaptations; the shared press architecture's every-pointer/nested rules supersede the recorded runtime gap. Its three named patterns remain owned by Card's template directory.
- Heading's checkpoint follows `Heading/Heading.tsx`, `Text/{text.stylex.ts,Heading.doc.mjs}`, its specification and shared theme tokens at the same revision. Its four Heading templates plus `Text/TextHeadingLevels.tsx` are adapted in the showcase and offline source-derived fixture; native Stack/Layout adapters and the shared Tooltip bridge are not completed public migrations. See `docs/heading-port.md` for exact source hashes and fidelity boundaries.
- Text's checkpoint follows `Text/{Text.tsx,text.stylex.ts,useTruncation.ts,TruncationTooltip.tsx}`, its specification, `utils/{sharedResizeObserver.ts,sharedMutationObserver.ts,triggerFocus.ts}`, `Tooltip/{Tooltip.tsx,useTooltip.tsx}` and `Layer/{useLayer.tsx,gestureCounter.ts,useTouchTrigger.ts}` at the same revision. The approved shared lifecycle follow-up retains those owners and the private Text-owned Button cut-label adapter. The Text showcase and offline source-derived fixture adapt the eight Text-only CLI templates and the three hook example paths; the native Table/Stack framing and existing shared Tooltip bridge do not claim completed public migrations. See `docs/text-port.md` for verified blob provenance and unresolved fidelity gaps.
- Private Text/Heading truncation presentation additionally follows `Layer/{layerSlideRules.ts,layerViewportInset.stylex.ts,layerAnimations.stylex.ts,layerHost.ts,layerTextReset.stylex.ts,layerStructureReset.stylex.ts}`, `motion/{reducedMotion.stylex.ts,themeTempo.ts}`, `theme/themeMotion.ts` and `utils/moduleLoader.ts` at the same revision. Separate Dioxus assets implement only the required internal subset; source-derived HTML fixtures are not an executed React/StyleX build.
- Blockquote follows `Blockquote/{Blockquote.tsx,index.ts}`, its component documentation/tests/authoritative specification and Storybook stories at the same pin. Its three templates retain the original quote/author strings, with native demo-only Stack/Grid/GridSpan framing and the existing Card. Verified source evidence in `tests/fixtures/blockquote-upstream/` additionally retains the original MIT notices for shared BaseProps/renderability/merge/theme helpers, tokens, Neutral theme/palette, and Card/Grid/Stack owners. This is a pinned-source comparison, not a verified release or published catalog measurement.
- AspectRatio follows `AspectRatio/{AspectRatio.tsx,index.ts}`, its authoritative specification, documentation, tests and all six templates at the pinned revision. `tests/fixtures/aspect_ratio_source/` retains hash-verified upstream files, including shared BaseProps/merge/theming/tokens and demo framing sources; they remain covered by the MIT notice below. Browser references are source-derived, not published measurements. Its image artwork is a locally authored replacement, not copied catalog assets. Native demo frames do not complete Center/Grid/Layout/Text migrations.
- Stack follows `Stack/{Stack.tsx,stack.stylex.ts,index.ts}`, the authoritative `docs/specs/components/Stack/Stack.spec.md`, shared `Layout/padding.stylex.ts`, `utils/{types.ts,themeProps.ts,mergeProps.ts}`, and `BaseProps.ts` at the same revision. `src/stack/` retains that owner; shared sizing/padding remain with their original owners. `tests/fixtures/stack-source-reference.json` includes 25 hash-verified source/spec/test/doc/template texts with their copyright headers, including inspected HStack/VStack/StackItem boundaries; inclusion is not a claim of those migrations. The three Stack named examples retain their upstream template ownership. Token/Avatar/StackItem private exact-pattern slices derive from `Token/Token.tsx`, `Avatar/Avatar.tsx` and `Stack/{StackItem.tsx,stackItem.stylex.ts}`, not public supporting ports. Named Avatar tooltips reuse the existing Tooltip owner. `tests/fixtures/stack-actual/` separately records actual Rust renders and raster comparisons; `stack-published-interactions.json` records original semantics/interaction observations. Exact Stack templates use no profile images, so no external image asset is bundled. The source-derived fixture claims no release equivalence. Separate `tests/fixtures/stack-published.json` and `stack-published/*.png` preserve observed published original DOM/styles/code and screenshots from astryx.atmeta.com (displayed v0.7.16), with timestamp/resource and image digests; the deployment commit is unknown. These Meta original captures are evidence, not screenshots of the port. See `docs/stack-port.md`.
- HStack and VStack follow their respective `HStack/HStack.tsx` and `VStack/VStack.tsx` sources and Basic and Showcase CLI templates at the same pin. Both delegate layout to Stack. See `docs/h-stack-port.md` and `docs/v-stack-port.md`.
- The public StackItem follows `Stack/{StackItem.tsx,stackItem.stylex.ts}`, `hooks/{StackItemScrollRegion.tsx,useScrollRegionProps.ts}`, and the overflow observation contract in `AppShell/observeMainOverflow.ts` at the same pin. It replaces Stack's former demo-only item wrapper and includes both StackItem CLI templates. See `docs/stack-item-port.md`.
- Stack's private Toast demo follows `Toast/{Toast.tsx,ToastViewport.tsx,useToast.tsx,useToastGesture.ts}` and `theme/{MediaTheme.tsx,onMediaTokens.ts}` at the same pin. `tests/fixtures/stack-toast/source.json` preserves twelve hash-verified originals with copyright headers; `published.json` and three PNGs record separate published observations, not deployment/pin equivalence. Shared announcements remain hook-owned; no public Toast migration is claimed. See `docs/stack-toast-port.md`.
- CheckIndicator follows `Indicator/CheckIndicator.tsx` and its specification at the same pin, with Icon retaining ownership of the glyph root. See `docs/check-indicator-port.md`.
- Avatar follows `Avatar/Avatar.tsx`, its component spec, and the standalone catalog arrangements at the same pin. The offline portraits in `tests/fixtures/avatar/` are pinned catalog assets. See `docs/avatar-port.md`.
- AvatarGroup follows `AvatarGroup/AvatarGroup.tsx`, its component spec, and both showcase templates at the same pin. See `docs/avatar-group-port.md`.
- AvatarGroupOverflow follows `AvatarGroup/AvatarGroupOverflow.tsx`, its component spec, and three catalog templates at the same pin. See `docs/avatar-group-overflow-port.md`.
- AvatarStatusDot follows `Avatar/AvatarStatusDot.tsx` and `Avatar/AvatarSizeContext.ts` at the same pin. See `docs/avatar-status-dot-port.md`.
- CheckboxIndicator follows `Indicator/CheckboxIndicator.tsx` and the Indicator family contract at the same pin. See `docs/checkbox-indicator-port.md`.
- RadioIndicator follows `Indicator/RadioIndicator.tsx` and `Indicator/indicator.markers.stylex.ts` at the same pin. See `docs/radio-indicator-port.md`.
- VisuallyHidden follows `VisuallyHidden/VisuallyHidden.tsx`, its specification, and all four original templates at the same pin. See `docs/visually-hidden-port.md`.
- Breadcrumbs follows `Breadcrumbs/Breadcrumbs.tsx`, its component spec, and the five non-menu catalog arrangements at the same pin. See `docs/breadcrumbs-port.md`.
- BreadcrumbItem follows the non-menu forms of `Breadcrumbs/BreadcrumbItem.tsx`, its component spec, and two catalog templates at the same pin. See `docs/breadcrumb-item-port.md`.
- Token follows `Token/{Token.tsx,TokenLink.tsx}`, its component spec, and six catalog templates at the same pin. See `docs/token-port.md`.
- MetadataList follows `MetadataList/MetadataList.tsx`, its component spec, and five catalog templates at the same pin. See `docs/metadata-list-port.md`.
- MetadataListItem follows `MetadataList/MetadataListItem.tsx`, its component spec, and two catalog templates at the same pin. See `docs/metadata-list-item-port.md`.
- EmptyState follows `EmptyState/EmptyState.tsx`, its component spec, and four catalog templates at the same pin. See `docs/empty-state-port.md`.
- Timer follows `Timer/Timer.tsx`, its component spec, and four catalog templates at the same pin. See `docs/timer-port.md`.
- Thumbnail follows `Thumbnail/Thumbnail.tsx` and its five catalog templates at the same pin. The offline PNGs in `tests/fixtures/thumbnail/` come from the embedded gallery assets. See `docs/thumbnail-port.md`.
- FieldStatus follows `FieldStatus/{FieldStatus.tsx,useFieldStatusAnnouncement.ts}`, the Core and Neutral status tokens and glyphs, and its two catalog templates at the same pin. See `docs/field-status-port.md`.
- FieldLabel follows `Field/FieldLabel.tsx`, the field optionality and input-container hooks, and two catalog arrangements at the same pin. See `docs/field-label-port.md`.
- NavIcon follows `NavIcon/NavIcon.tsx`, its spec, and both catalog templates at the same pin. The showcase adapts fallback glyphs from `Icon/defaultIcons.tsx`. See `docs/nav-icon-port.md`.
- InputGroupText follows `InputGroup/InputGroupText.tsx`, its spec, and adapted arrangements from the InputGroup templates at the same pin. See `docs/input-group-text-port.md`.
- Citation follows `Citation/Citation.tsx`, its spec, and three catalog templates at the same pin. See `docs/citation-port.md`.
- ToggleButtonGroup follows `ToggleButton/ToggleButtonGroup.tsx`, its spec, and the single- and multi-select catalog arrangements at the same pin. See `docs/toggle-button-group-port.md`.
- ToggleButton follows the standalone form of `ToggleButton/ToggleButton.tsx`, its spec, and adapted catalog examples at the same pin. See `docs/toggle-button-port.md`.
- TopNavMegaMenuItem follows the desktop form of `TopNav/TopNavMegaMenuItem.tsx`, its spec, and both catalog templates at the same pin. See `docs/top-nav-mega-menu-item-port.md`.
- TopNavMegaMenuFeaturedCard follows `TopNav/TopNavMegaMenuFeaturedCard.tsx`, its spec, and both catalog templates at the same pin. See `docs/top-nav-featured-card-port.md`.
- TopNavHeading follows the non-menu forms of `TopNav/TopNavHeading.tsx`, its spec, and both catalog templates at the same pin. See `docs/top-nav-heading-port.md`.
- SideNavCollapseButton follows the controlled form of `SideNav/SideNavCollapseButton.tsx`, its spec, and both catalog templates at the same pin. See `docs/side-nav-collapse-button-port.md`.
- SideNavSection follows `SideNav/SideNavSection.tsx`, its spec, and both catalog templates at the same pin. See `docs/side-nav-section-port.md`.
- SideNavHeading follows the non-menu forms of `SideNav/SideNavHeading.tsx`, its spec, and both catalog templates at the same pin. See `docs/side-nav-heading-port.md`.
- MediaTheme follows `theme/MediaTheme.tsx`, its on-media tokens and automatic surface selection, and three catalog templates at the same pin. The offline images are pinned catalog assets. See `docs/media-theme-port.md`.
- FormLayout follows `FormLayout/{FormLayout.tsx,FormLayoutContext.ts}`, its component spec, and five catalog arrangements at the same pin. See `docs/form-layout-port.md`.
- DropdownMenuGroup follows `DropdownMenu/DropdownMenuGroup.tsx`, its spec, and both catalog arrangements at the same pin. See `docs/dropdown-menu-group-port.md`.
- DropdownMenuDivider follows `DropdownMenu/DropdownMenuDivider.tsx` and its component spec at the same pin. See `docs/dropdown-menu-divider-port.md`.
- ScrollableArea follows `ScrollableArea/ScrollableArea.tsx`, `useScrollableArea.ts`, and six catalog arrangements at the same pin. See `docs/scrollable-area-port.md`.
- Layout follows `Layout/Layout.tsx`, the Layout spec, and its seven catalog templates at the same pin. See `docs/layout-port.md`.
- LayoutPanel follows `Layout/{LayoutPanel.tsx,LayoutAreaContext.ts}`, the shared scroll-region contract, and its Navigation and Showcase templates at the same pin. Resize-controller integration remains separate. See `docs/layout-panel-port.md`.
- LayoutContent follows `Layout/{LayoutContent.tsx,LayoutSlotsContext.ts}`, the shared `useScrollRegionProps` and overflow observation contracts, and its Basic and Showcase templates at the same pin. See `docs/layout-content-port.md`.
- LayoutHeader and LayoutFooter follow `Layout/{LayoutHeader.tsx,LayoutFooter.tsx,LayoutDividerContext.ts}`, the shared container padding and block-bleed contracts, and their respective CLI templates at the same pin. See `docs/layout-header-port.md` and `docs/layout-footer-port.md`.
- Section follows `Section/Section.tsx`, the shared Layout padding contract, its specification, and all three Section CLI templates. See `docs/section-port.md`.
- Center follows `Center/Center.tsx`, its specification, and all three Center CLI templates at the same pin. See `docs/center-port.md`.
- Grid follows `Grid/Grid.tsx`, its authoritative specification and five CLI templates at the same pin. GridSpan follows `Grid/GridSpan.tsx`, its specification, and both GridSpan CLI templates. The Grid resize example uses a native range input; no public ResizeHandle port is claimed. See `docs/grid-port.md`.
- The showcase's named AspectRatio, Blockquote, Grid, Stack, Badge, Button, ButtonGroup, Card, ClickableCard, SelectableCard, Divider, IconButton, Kbd, Link, ProgressBar, Skeleton, Spinner, and StatusDot patterns are adapted from their directories under `packages/cli/assets/templates/blocks/components/` at the same revision.

Upstream license:

```text
MIT License

Copyright (c) 2026 Meta Platforms, Inc.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

Inline Code (`src/code/`, its CSS, example and source-projection fixture) additionally
follows `packages/core/src/Code/` and the four CLI `components/Code/` templates
at the same pinned revision. Shared typography tokens remain theme-owned.

## Heroicons (showcase glyphs only)

- `examples/showcase/button_group.rs` includes the clipboard-document, scissors, clipboard, and chevron-down outline SVG paths used by the original named showcase.
- `examples/showcase/center.rs` includes the bold, italic, underline, list-bullet, link, photo, and inbox outline glyphs.
- `examples/showcase/icon_button.rs` also includes the original floating template's Plus outline path.
- `examples/showcase/section.rs` includes the original Check solid glyph.
- `examples/showcase/visually_hidden.rs` includes the original Arrow Down Tray, Share, Trash, Speaker Wave, Arrow Up, and Arrow Down glyphs.
- Source: https://github.com/tailwindlabs/heroicons/tree/v2.2.0/optimized/24
- Copyright (c) Tailwind Labs, Inc.
- MIT license; full text in [assets/licenses/heroicons.txt](assets/licenses/heroicons.txt).
- This copies licensed SVG data; it adds no Rust/npm dependency or network service.

## Lucide / Feather artwork

- `src/indicator/check_indicator.rs` uses the Lucide Check path for the default glyph.
- `src/field_status/component.rs` uses Lucide circle-check, circle-x, and triangle-alert artwork for the Neutral status glyphs.

- `examples/showcase/icon_button.rs` copies Neutral's Search, Copy, Info, Menu, X, MoreHorizontal, and Wrench SVG artwork, as measured from the original named examples.
- `src/toast/pattern.rs` uses the same Neutral X artwork for the private demo dismiss control.
- `examples/showcase/link.rs` registers Neutral's ExternalLink artwork through the Icon-owned global extension seam. The library's fallback remains the original Meta Astryx SVG.
- Source mapping: Meta Astryx's pinned `packages/themes/neutral/src/icons.tsx`; its package declares `lucide-react` `^1.66.0`.
- Lucide: https://github.com/lucide-icons/lucide, copyright (c) 2026 Lucide Icons and Contributors, ISC license.
- Feather-derived icons retain copyright (c) 2013-present Cole Bemis and the MIT license.
- Complete notices: [assets/licenses/lucide.txt](assets/licenses/lucide.txt), including both licenses and the upstream Feather-derived icon list.
- Only SVG artwork is copied from Lucide. No Lucide package dependency or runtime font/network service is added.

## Figtree font (showcase only)

- File: `assets/fonts/Figtree-latin.woff2`.
- Copyright 2022 The Figtree Project Authors (https://github.com/erikdkennedy/figtree).
- License: SIL Open Font License 1.1; complete text in [assets/fonts/OFL.txt](assets/fonts/OFL.txt).
- Source: https://fonts.gstatic.com/s/figtree/v9/_Xms-HUzqDCFdgfMm4S9DaRvzig.woff2, the Latin variable font served by Google Fonts for Figtree.
- Bundled locally for deterministic showcase typography. Button itself inherits the application's font and does not download fonts.
