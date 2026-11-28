# HStack port

`HStack` delegates to `Stack` with horizontal direction and adds no wrapper, CSS, or semantics. `h_align` and `v_align` override `justify` and `align`, respectively. The main and cross axes use the existing closed Stack alignment types. Sizing, logical padding, wrapping, overflow, native attributes, callbacks, and caller-rendered roots are forwarded unchanged.

The port follows [Meta Astryx `476a315`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e). `HStack.tsx` (Git blob `b62fc29a8e45cd64c4a2098ff704a8e1996e6fd4`), its unit tests, both templates, and the [Stack family spec](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/docs/specs/components/Stack/Stack.spec.md) were checked against the pinned tree. There is no standalone HStack spec at that pin. Padding and flow are logical; FR5 and the source use physical CSS `width`, `height`, `max-width`, and `min-height` for the sizing props.

Basic uses the existing Badge components. Showcase now uses the public `VStack` for the outer frame and the three label/row groups, as in the template. Its outer frame has gap 6, width `100%`, and the inline `max-width: 400px` cap. The separate native-root probe is not part of either catalog pattern.

Four focused SSR tests pass: horizontal Stack equivalence and alignment precedence, default div without implicit semantics or spacing reflection, the caller's labeled nav with direct links, and declaration order for all four explicit dimensions over caller inline styles. The showcase also compiles for the native test target. Shared Stack and styles were not changed.

The Chrome suite checks LTR/RTL row order, alignment and token gaps. It now also checks the Showcase frame and the caller-owned nav's actual mounted node, Tab navigation, and Enter activation with a bubbling click callback. These live assertions were not run in this worktree. Matching upstream rendering (including raster comparison), other engines, and manual assistive-technology verification remain open; SSR and DOM checks do not establish those outcomes.

```sh
SHOWCASE_URL=http://127.0.0.1:8116 node tests/h_stack.browser.mjs
```
