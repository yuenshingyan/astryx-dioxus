# Section port

Section groups settings, forms, sidebars, and other page regions; Card is for discrete objects. Section's outer `div` owns sizing, caller `class`/`style` (the native CSS counterpart to StyleX), native attributes, and event listeners. Use `onmounted` to access the wrapper instead of a React ref. The inner `div` owns the `astryx-section` theme target, reflected variant, padding, and dividers. Neither box adds a role, name, landmark, or tab stop. Custom variants still reflect their name without a built-in fill.

Explicit padding resolves edge → axis → uniform. Without explicit values, the theme edge/axis precedes the nearest Section's explicit uniform padding, then the theme shorthand, then step 4. Edge and axis overrides publish the applied inset but do not replace the inherited uniform default. A nested wrapper escapes the parent's inline insets and only escapes a top/bottom inset at the corresponding child boundary. Overlay roots stop Section padding inheritance. Ordinary nested Layout and an AppShell layout root are distinct upstream: Layout's inner wrapper clears the bleed variables, whereas AppShell applies Layout's full reset at the shell root. The Rust AppShell does not yet apply that reset; see the outstanding integration below.

Sizing uses pixel numbers or `SizeValue` CSS strings. CSS-wide keywords apply directly to the dimension; using them as private variable values would instead change the variable's cascade. Caller inline style follows generated declarations. The upstream `minHeight`-only limitation is preserved: the wrapper can grow without stretching the inner paint.

## Source comparison

Compared to Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`. `Section.tsx` (`65d38e593153e4da761b5e3fe851371311bc9dfd`) and `Section.spec.md` (`23b0528c530c753b156ea92da79d71a2ae35bd7e`) match the pinned Git tree. The same blob check covered `Section.doc.mjs`, all three CLI templates and their metadata, Section tests and stories, shared `BaseProps`, sizing and theme helpers/tokens, Layout's `container`, padding and boundary resets, AppShell's root and its tests, and the templates' Stack, Heading, Text, Icon and Button sources. The local CheckIcon path matches `@heroicons/react` 2.2.0 from the pin, verified against the lockfile integrity hash without installing it.

| Detailed item | Evidence or remaining check |
| --- | --- |
| Built-in variants | Source and markup agree on the three variants. In integrated Chrome, the showcase's computed default and muted backgrounds match the corresponding local theme tokens under both color schemes, and the transparent variant computes to transparent. This is not raster or upstream paint evidence. |
| Divider edges | SSR checks each side independently and checks duplicate suppression. The stylesheet uses the shared border token, physical top/bottom, and logical start/end. Integrated Chrome verifies bottom borders and RTL inline-start. |
| Padding | SSR checks the full spacing scale, zero, precedence and publication for both axes, and that edge-only overrides don't propagate as uniform values. Integrated Chrome verifies step 4, inherited 24px uniform padding, 6px theme edge precedence, and restoration of the 16px default at the overlay boundary. The AppShell boundary remains open. |
| Sizing and page composition | SSR checks wrapper ownership, pixel/CSS serialization, CSS-wide keywords, caller inline style, and children. With Dividers and the centered settings example show heading-plus-Stack composition. Integrated Chrome measures the inner box at 125px after setting height directly on the wrapper, not through the sizing prop. CSS-wide sizing and the retained `minHeight`-only geometry still need renderer checks. |
| Center a Section with a maximum width | The showcase uses the pinned example's 1280px cap and both explicit logical auto margins, using `style` instead of StyleX. Integrated Chrome measures 1280px with 80px on each side in a 1440px host. |
| SectionVariants | Source-matched structure and copy; integrated Chrome verifies the three reflected variants, 20px insets, and computed theme-token mapping. Comparison of rendered pixels is pending. |
| SectionWashHighlight | Source-matched pricing content, spacing, Icon and Button. Integrated Chrome verifies the default and muted regions each span the example's width, with 16px and 24px insets respectively. |
| SectionWithDividers | Source-matched three settings groups and two bottom borders. SSR verifies the groups and modifiers; integrated Chrome verifies the reflected default variant, 20px insets, and 1px/1px/0px bottom borders. |

PR #241's full CI and integrated Chrome run passed, including `tests/section.browser.mjs`. No browser or preview was started for this follow-up. Formatting, CSS bundle verification and browser-script syntax passed locally. The six focused SSR tests passed in the prior audit; the local rerun through the assigned Cargo slot stopped during dependency compilation with `No space left on device`.

## Remaining parent evidence

- **Shared Layout/AppShell boundary (FR3):** Layout owns the FR8 reset and AppShell must apply it at its own root (FR18), not on the inner Layout or in Section. Verify that the shell does not use its enclosing Section's padding for bleed and that a default Section inside a shell nested in `Section { padding: SpacingStep::Six }` uses its own theme/default inset, not 24px.
- **Appearance (FR2):** The two frozen `core-section--default__neutral-{light,dark}` PNGs in `/tmp/section-476a315` were revalidated against the pinned Git blobs and manifest SHA-256 values (1024×768, Chromium 149, Linux arm64). They cover the Default Storybook story only: a 300px Section with the pinned paragraph, inside a surface decorator with spacing-6 padding. Compare a matching Rust capture, not the variants showcase, against these images; separate upstream render evidence is needed for muted and transparent. No Rust raster comparison or cross-engine verification was performed.
- **Renderer (FR1/FR5/AR1):** Check actual Section sizing props (including a CSS-wide value and `minHeight` without `height`), verify that `onmounted` resolves to the outer wrapper and caller event handlers are attached there, and verify native Tab order across the children. Coordinate browser work with the integrator.
- **Manual AT (AR1):** With a screen reader, verify that the default Section adds no landmark or focus stop, a caller-named region is exposed exactly once, and child control navigation is unchanged. SSR markup is not evidence of screen-reader output.

```sh
SHOWCASE_URL=http://127.0.0.1:8118 node tests/section.browser.mjs
```
