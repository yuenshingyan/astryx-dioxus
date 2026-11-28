# Section port

Section groups settings, forms, sidebars, and other page regions; Card is for discrete objects. Section's outer `div` owns sizing, caller `class`/`style` (the native CSS counterpart to StyleX), native attributes, and event listeners. Use `onmounted` to access the wrapper instead of a React ref. The inner `div` owns the `astryx-section` theme target, reflected variant, padding, and dividers. Neither box adds a role, name, landmark, or tab stop. Custom variants still reflect their name without a built-in fill.

Explicit padding resolves edge → axis → uniform. Without explicit values, the theme edge/axis precedes the nearest Section's explicit uniform padding, then the theme shorthand, then step 4. Edge and axis overrides publish the applied inset but do not replace the inherited uniform default. A nested wrapper escapes the parent's inline insets and only escapes a top/bottom inset at the corresponding child boundary. Overlay roots stop Section padding inheritance. Ordinary nested Layout and an AppShell layout root are distinct upstream: Layout's inner wrapper clears the bleed variables, whereas AppShell applies Layout's full reset at the shell root. The Rust AppShell does not yet apply that reset; see the outstanding integration below.

Sizing uses pixel numbers or `SizeValue` CSS strings. CSS-wide keywords apply directly to the dimension; using them as private variable values would instead change the variable's cascade. Caller inline style follows generated declarations. The upstream `minHeight`-only limitation is preserved: the wrapper can grow without stretching the inner paint.

## Source comparison

Compared to Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`. `Section.tsx` (`65d38e593153e4da761b5e3fe851371311bc9dfd`) and `Section.spec.md` (`23b0528c530c753b156ea92da79d71a2ae35bd7e`) match the pinned Git tree. The same blob check covered `Section.doc.mjs`, all three CLI templates and their metadata, Section tests and stories, shared `BaseProps`, sizing and theme helpers/tokens, Layout's `container`, padding and boundary resets, AppShell's root and its tests, and the templates' Stack, Heading, Text, Icon and Button sources. The local CheckIcon path matches `@heroicons/react` 2.2.0 from the pin, verified against the lockfile integrity hash without installing it.

| Detailed item | Evidence or remaining check |
| --- | --- |
| Built-in variants | Source and markup agree on the default and three variants; the stylesheet maps to surface, muted, and transparent. Actual light/dark paint comparison is pending. |
| Divider edges | SSR checks each side independently and checks duplicate suppression. The stylesheet uses the shared border token, physical top/bottom, and logical start/end. Prior Chrome evidence covers bottom borders and RTL inline-start. |
| Padding | SSR checks the full spacing scale, zero, precedence and publication for both axes, and that edge-only overrides don't propagate as uniform values. CSS consumes the published insets and falls back to spacing 4. Prior Chrome evidence covers step 4, inherited uniform padding, theme edge precedence, and overlay reset. The separate AppShell boundary remains open for the parent audit. |
| Sizing and page composition | SSR checks wrapper ownership, pixel/CSS serialization, CSS-wide keywords, caller inline style, and children. With Dividers and the centered settings example show heading-plus-Stack composition. Browser evidence is still needed for CSS-wide sizing and the retained `minHeight`-only geometry. |
| Center a Section with a maximum width | The showcase uses the pinned example's 1280px cap and both explicit logical auto margins, using `style` instead of StyleX. SSR checks the declarations. Centered geometry is pending. |
| SectionVariants | Source-matched structure and copy, verified in SSR and historical Chrome variant/inset metrics. Actual theme paint is pending. |
| SectionWashHighlight | Source-matched pricing content, spacing, Icon and Button. The muted callout's full-width geometry is pending. |
| SectionWithDividers | Source-matched three settings groups and two bottom borders. SSR verifies the groups and modifiers; prior Chrome evidence verifies default surfaces by variant, 20px inset, and 1px/1px/0px bottom borders. |

The six focused SSR tests (`cargo test --locked --test section`, through the assigned Cargo slot), formatting, and bundle verification passed. The browser test was syntax-checked only.

## Remaining parent evidence

- Integrate Layout's FR8 reset at the AppShell root (FR18), outside Section ownership. A Section inside an AppShell nested under `Section { padding: SpacingStep::Six }` should use its own theme/default inset, not inherit 24px, and the shell must not use the outer Section inset for bleed.
- Run the updated browser regression against the showcase and compare rendered appearance with the originals. The upstream manifest's two `core-section--default__neutral-{light,dark}` Storybook PNGs match their pinned blobs and manifest SHA-256 values, but have not been compared with a current Rust render. The prior Chrome tests compared source-derived metrics, not screenshots. Other engines are unverified.
- Verify native `onmounted`/event targeting and keyboard traversal in the renderer, and manually verify that a default Section adds no landmark or focus stop, a caller-named region is exposed once, and child control navigation is unchanged. SSR verifies the markup but not browser or screen-reader behavior.

```sh
SHOWCASE_URL=http://127.0.0.1:8118 node tests/section.browser.mjs
```
