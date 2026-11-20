# Third-party notices

## Meta Astryx

The implementations under `src/badge/`, `src/button/`, `src/button_group/`, `src/card/`, `src/clickable_card/`, `src/selectable_card/`, `src/private_attributes/`, `src/layout/`, `src/divider/`, `src/icon/`, `src/icon_button/`, `src/progress_bar/`, `src/motion/`, `src/visually_hidden/`, `src/skeleton/`, `src/spinner/`, `src/status_dot/`, `src/size_context/`, `src/input_group/`, `src/kbd/`, `src/layer/`, `src/link/`, `src/text/`, `src/theme/`, `src/hooks/`, `src/tooltip/`, `src/interactive_role_context/`, `src/i18n/`, and `src/utils/`, together with `assets/button-group-controller.js` and `assets/astryx.css` adapt Meta Astryx's Button, Link/Text typography and navigation, Spinner, icon sizing/lookup, grouping/context contracts, tooltip/press behavior, design tokens, and implemented theme values:

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
- The showcase's named Badge, Button, ButtonGroup, Card, ClickableCard, SelectableCard, Divider, IconButton, Kbd, Link, ProgressBar, Skeleton, Spinner, and StatusDot patterns are adapted from their directories under `packages/cli/assets/templates/blocks/components/` at the same revision.

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

## Heroicons (ButtonGroup and IconButton showcase glyphs only)

- `examples/showcase/button_group.rs` includes the clipboard-document, scissors, clipboard, and chevron-down outline SVG paths used by the original named showcase.
- `examples/showcase/icon_button.rs` also includes the original floating template's Plus outline path.
- Source: https://github.com/tailwindlabs/heroicons/tree/v2.2.0/optimized/24/outline
- Copyright (c) Tailwind Labs, Inc.
- MIT license; full text in [assets/licenses/heroicons.txt](assets/licenses/heroicons.txt).
- This copies licensed SVG data; it adds no Rust/npm dependency or network service.

## Lucide / Feather (IconButton and Link showcase artwork only)

- `examples/showcase/icon_button.rs` copies Neutral's Search, Copy, Info, Menu, X, MoreHorizontal, and Wrench SVG artwork, as measured from the original named examples.
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
