# VStack port

`VStack` delegates to `Stack` with vertical direction and adds no wrapper, styles, or semantics. `v_align` takes precedence over `justify` on the main axis; `h_align` takes precedence over `align` on the cross axis. Sizing, logical padding, wrapping, overflow, native attributes and events, and caller-rendered roots pass through to Stack.

The port follows [Meta Astryx `476a315`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e). Seven VStack source, test, and template blobs were verified against the pinned tree, including `VStack/VStack.tsx` (Git blob `026c3912cf2706dab6feecd84fc996e3557fcb40`). The showcase includes the Basic and Showcase templates using existing Heading, Text, Badge, and HStack components.

The pinned tree has no standalone VStack spec; the owning [Stack spec](https://github.com/meta/astryx/blob/476a3153417dbf75c565f7df05b0784e94ddd59e/docs/specs/components/Stack/Stack.spec.md) supplies FR1–FR5, FR7 and AR1. FR5 keeps the four sizing declarations physical (`width`, `height`, `max-width`, `min-height`); FR3 makes padding logical. VStack should not translate those sizes to `inline-size` or `block-size`. AR2's overflow naming is for StackItem, not for VStack.

The showcase also includes the previously missing family playground (gap 2, three Cards with padding 3) and the TSX usage example (gap 2, two Items). The latter supplies labels to satisfy Rust Item's API. The five focused SSR tests passed, covering these compositions, the default `div`, a caller-named native `nav`, logical padding precedence including zero, and sizing declarations after caller style. They do not prove computed geometry, native mounted-element identity, or announcements.

In the original port pass, Chrome verified the two named templates, native heading markup, child order, and measured token gaps in LTR and RTL. For the new compositions, the browser suite below adds direct-child, applied Card inset (padding plus border), and measured gap checks. The integrator still needs to run these checks and verify mounted-root identity/native interactions. Published screenshot comparisons, other browsers, and manual assistive-technology behavior remain open.

```sh
SHOWCASE_URL=http://127.0.0.1:8117 node tests/v_stack.browser.mjs
```
