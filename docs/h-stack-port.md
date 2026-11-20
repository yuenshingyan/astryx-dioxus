# HStack port

`HStack` delegates to `Stack` with horizontal direction and adds no wrapper, CSS, or semantics. `h_align` and `v_align` override `justify` and `align`, respectively. The main and cross axes use the existing closed Stack alignment types. Sizing, logical padding, wrapping, overflow, native attributes, callbacks, and caller-rendered roots are forwarded unchanged.

The port follows [Meta Astryx `476a315`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e), including `HStack/HStack.tsx` (Git blob `b62fc29a8e45cd64c4a2098ff704a8e1996e6fd4`). Eight HStack source, test, and template blobs were verified against the pinned Git tree. The Basic and Showcase patterns use the existing Badge and Text components; vertical framing uses Stack, not a public VStack port.

SSR tests verify equivalence with horizontal Stack, alignment precedence, and native root passthrough. Chrome checks both patterns for alignment, token gaps, and logical ordering in LTR and RTL. This does not verify published screenshot parity, other browsers, or manual assistive-technology behavior. The migration checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8116 node tests/h_stack.browser.mjs
```
