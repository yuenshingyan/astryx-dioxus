# RadioIndicator port

`RadioIndicator` is the decorative visual for a caller-owned selection, not a form control. Both states retain the circle; checked adds a dot unless the caller supplies replacement content. The root owns `aria-hidden` and the reflected size and state, and forwards native attributes, styles, and callbacks. Keep role, name, focus behavior, and focusable content on the owner, not in the hidden subtree.

## Pinned authority and inventory

Compared Git blob hashes against the complete Meta Astryx tree at `476a3153417dbf75c565f7df05b0784e94ddd59e`. There is no standalone RadioIndicator spec, doc, or CLI template, and no Indicator-family CLI template. The RadioList and RadioListItem templates are examples of owning controls, not additional RadioIndicator variants. The authoritative examples are the three inline family recipes and the checked `md` snippet in the TSX.

- `packages/core/src/Indicator/`: `RadioIndicator.tsx` (`eb6bc709`), `Indicator.doc.mjs` (`baeea1c0`), `Indicator.test.tsx` (`bf7bb29a`), `RadioIndicator.circle.source-build.test.mjs` (`5fe4d257`), `types.ts` (`17ae1ae6`), `indicator.markers.stylex.ts` (`9fa548da`), `indicatorRegistry.ts` (`7bffb620`), `indicatorRegistry.test.tsx` (`96d3e7f2`), and `useIndicator.ts` (`bed35a07`). The family spec `docs/specs/components/Indicator/Indicator.spec.md` (`6eb48ff4`), chiefly FR3–FR6 and AR2, covers RadioIndicator.
- Shared dependencies: `utils/isRenderable.ts` (`2238a00a`), `mergeProps.ts` (`3750bb89`), `themeProps.ts` (`b93fb205`), `BaseProps.ts` (`02db0724`), `theme/tokens.stylex.ts` (`ec6ecd40`), `theme/defineTheme.ts` (`61452e61`), and `hooks/useIndicatorFocusRing.tsx` (`fbc2fb0a`). Checked the owning `RadioListItem` (`5a012a65`) and `DropdownMenuRadioItem` (`0820105b`) to distinguish host behavior from the decorative primitive.

## Audit and remaining evidence

- **State and chrome:** The enum requires checked or unchecked with no partial state. SSR verifies a persistent empty unchecked span and the canonical checked dot. The circle and dot both use literal `50%`, as in the source, not `--radius-full`. The checked-medium source example now renders in the showcase and is covered by SSR.
- **Geometry and paint:** Source and CSS agree on 20/8px and 24/10px geometry, shared border, surface, accent and on-accent aliases, and visual-only disabled opacity (`0.5`) with muted border/unchecked fill. Under a marked hover-capable ancestor, checked fill still tints even if disabled, but the disabled border remains muted; the port now matches that source distinction. The upstream radio has no separate `:active` paint. Enabled RadioListItem sets the upstream marker; Rust owning controls do not currently set the equivalent marker. Their integration is outside this component block. The prior Chrome evidence covered baseline geometry, replacement, scoped hover, reduced motion and forced colors. Re-run for the changed disabled hover branch, default checked-medium geometry, and the radius-token stress case. A rendered comparison against the pin is still needed (light/dark, enabled/disabled, and hover).
- **Replacement:** `None` retains the state mark; `Some(Element)` deliberately replaces it even when the element or container renders nothing. SSR covers explicit empty content, an empty element, nonempty text, and `0`. This is not full React `isRenderable` parity: the existing API has no separate raw boolean/nullish slot, and an explicitly supplied empty `Element` (including one made from an empty string) is still an intentional replacement. The shared Rust `is_renderable` walks virtual fragments and would reject some deliberately empty replacements. A change to the public empty-replacement contract would need an API decision; callers should use `None` for absent content.
- **Decoration:** SSR verifies the protected `aria-hidden` and no component-generated role, input or tab stop. The wide native attribute API still lets callers forward focus-affecting attributes; it was not narrowed. Browser and manual AT verification with a real host are still needed: only the host participates in the tab order and announces its name, disabled and selection state once; its focus ring remains visible, including under forced colors. Verify that the selected dot stays distinguishable and reduced motion disables transitions.
- **Restyle an indicator (the common path):** The showcase uses scoped author CSS on `astryx-radio-indicator` and `astryx-radio-indicator-dot`, including a checked+disabled override. Browser assertions are ready for actual computed border, dot radius and override paint; they have not been run on this change.
- **Replace an indicator with your own component:** The pinned recipe installs a token-styled **checkbox** and uses shallow `isRenderable` to prioritize pending content. This is not a third radio state. Rust has no equivalent theme indicator registry or host resolution; the upstream recipe is documented here, not claimed as a port.
- **Use radio visuals for single selection:** In the pin, `defineTheme({indicators: {check: RadioIndicator}})` makes existing check slots draw both checked dots and unchecked circles. Replacing `radio` itself is a separate key; CSS styling is not either replacement. Rust lacks both name-based replacements and the host resolution path (e.g. Selector) needed to demonstrate them. Mounting RadioIndicator directly does not verify this recipe.

## Verification

Four focused RadioIndicator SSR tests passed via the assigned Cargo slot. Formatting, bundle generation/verification, and the browser script syntax check passed. The browser suite, visual comparison, manual AT, full Rust suite, Clippy, and the standalone WASM showcase build were not run for this change. No browser or preview was started.

Integrator, against a separately started showcase:

```sh
SHOWCASE_URL=http://127.0.0.1:8121 node tests/radio_indicator.browser.mjs
```
