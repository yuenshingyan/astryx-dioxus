# Grid port

Grid renders a native `div` without adding a role, tab stop, or item wrappers. Fixed columns preserve `1fr` minimum-content behavior. Responsive fill and fit use native CSS repeat tracks; a positive cap uses the effective column gap and lets a lone track shrink below its minimum width. Gap steps reuse shared tokens. Row height, axis alignment, sizing precedence, and reflected theme attributes follow the pinned source. Track declarations stay in the component CSS layer so caller classes and media queries can override them.

## Source and verification

Compared against [Meta Astryx `476a315`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e). Twenty retrieved Grid source/spec/test/template and token blobs were checked against that revision's Git tree, including:

| Upstream path | Git blob |
| --- | --- |
| `packages/core/src/Grid/Grid.tsx` | `b9ec203c718fbee3868a5d164574b694989e0785` |
| `packages/core/src/Grid/GridSpan.tsx` | `7c109c52923e91c4e306fe07faf4a260703fc2db` |
| `docs/specs/components/Grid/Grid.spec.md` | `9170217bd04de747a7869cd6491de39638f46396` |

The showcase adapts all five upstream patterns: Showcase, GalleryExample, ResponsiveAutoFit, DashboardLayout, and WithGridSpan. The resize demo uses a native range input instead of the upstream Layout and ResizeHandle; the spanning examples now use the public [GridSpan](grid-span-port.md) component. Neither published screenshot equality nor full parity for those supporting components was verified. The Grid parent checklist remains open pending independent appearance comparison.

SSR tests cover root semantics, DOM order, fixed and responsive declarations, caps, gap overrides, sizing, and invalid numeric inputs. The root composition case also checks that caller class, ARIA, and inline styles survive, while Grid retains ownership of reflected alignment and appends explicit sizing after caller declarations.

The Chrome showcase test covers the five patterns, responsive resizing, full spans, RTL, and caller media CSS precedence. It now includes a separate alignment fixture for the pinned spec's FR5: all four explicit values and browser `normal`, with measured item offsets and sizes in LTR and RTL. It also checks that a caller media rule can override both axes without changing the reflected props. Live execution of the new geometry assertions is pending integrator verification. The test needs only a local showcase preview:

```sh
SHOWCASE_URL=http://127.0.0.1:8112 node tests/grid.browser.mjs
```

The original port passed the five standard Rust checks, including the full Cargo test suite and the WASM showcase build check. Those results predate the alignment fixture. No browser other than Chrome or manual assistive technology was tested for Grid.
