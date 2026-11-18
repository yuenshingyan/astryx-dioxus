# Third-party notices

## Meta Astryx

The implementations under `src/button/`, `src/button_group/`, `src/icon/`, `src/icon_button/`, `src/spinner/`, `src/size_context/`, `src/input_group/`, `src/layer/`, and `src/link/`, together with `assets/button-controller.js`, `assets/button-group-controller.js`, and `assets/astryx.css` adapt Meta Astryx's Button, Spinner, icon sizing, grouping/context contracts, tooltip/press behavior, design tokens, and Button-specific theme values:

- Repository: https://github.com/meta/astryx
- Reference revision: `476a3153417dbf75c565f7df05b0784e94ddd59e`
- Sources: `packages/core/src/Button/Button.tsx`, `packages/core/src/Badge/Badge.tsx` (contrast-audit mappings), `packages/core/src/Spinner/{Spinner.tsx,spinnerRotor.ts}`, `packages/core/src/Icon/IconSize.stylex.ts`, `packages/core/src/Tooltip/useTooltip.tsx`, `packages/core/src/utils/{pressFeedback.ts,pressGesture.ts}`, `packages/core/src/theme/tokens.stylex.ts`, `packages/themes/neutral/src/{neutralTheme.ts,neutralPaletteRefs.generated.ts}`, and `apps/docsite/src/themes/astryxTheme.ts`.
- ButtonGroup additionally follows `packages/core/src/ButtonGroup/{ButtonGroup.tsx,ButtonGroupContext.ts}`, `packages/core/src/hooks/{useListFocus.ts,useGroupPressTracking.ts}`, `packages/core/src/IconButton/IconButton.tsx`, and `packages/core/src/Layer/layerScopedContext.tsx` at the same revision.
- The showcase's named Button, ButtonGroup, and IconButton patterns are adapted from their directories under `packages/cli/assets/templates/blocks/components/` at the same revision.

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

## Heroicons (ButtonGroup and IconButton showcase glyphs only)

- `examples/showcase/button_group.rs` includes the clipboard-document, scissors, clipboard, and chevron-down outline SVG paths used by the original named showcase.
- `examples/showcase/icon_button.rs` also includes the original floating template's Plus outline path.
- Source: https://github.com/tailwindlabs/heroicons/tree/v2.2.0/optimized/24/outline
- Copyright (c) Tailwind Labs, Inc.
- MIT license; full text in [assets/licenses/heroicons.txt](assets/licenses/heroicons.txt).
- This copies licensed SVG data; it adds no Rust/npm dependency or network service.

## Lucide / Feather (IconButton showcase artwork only)

- `examples/showcase/icon_button.rs` copies Neutral's Search, Copy, Info, Menu, X, MoreHorizontal, and Wrench SVG artwork, as measured from the original named examples.
- Source mapping: Meta Astryx's pinned `packages/themes/neutral/src/icons.tsx`; its package declares `lucide-react` `^1.66.0`.
- Lucide: https://github.com/lucide-icons/lucide, copyright (c) 2026 Lucide Icons and Contributors, ISC license.
- Feather-derived icons retain copyright (c) 2013-present Cole Bemis and the MIT license.
- Complete notices: [assets/licenses/lucide.txt](assets/licenses/lucide.txt), including both licenses and the upstream Feather-derived icon list.
- Only SVG data is copied. No Lucide dependency, icon-registry API, or runtime font/network service is added.

## Figtree font (showcase only)

- File: `assets/fonts/Figtree-latin.woff2`.
- Copyright 2022 The Figtree Project Authors (https://github.com/erikdkennedy/figtree).
- License: SIL Open Font License 1.1; complete text in [assets/fonts/OFL.txt](assets/fonts/OFL.txt).
- Source: https://fonts.gstatic.com/s/figtree/v9/_Xms-HUzqDCFdgfMm4S9DaRvzig.woff2, the Latin variable font served by Google Fonts for Figtree.
- Bundled locally for deterministic showcase typography. Button itself inherits the application's font and does not download fonts.
