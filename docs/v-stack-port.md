# VStack port

`VStack` delegates to `Stack` with vertical direction and adds no wrapper, styles, or semantics. `v_align` takes precedence over `justify` on the main axis; `h_align` takes precedence over `align` on the cross axis. Sizing, logical padding, wrapping, overflow, native attributes and events, and caller-rendered roots pass through to Stack.

The port follows [Meta Astryx `476a315`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e). Seven VStack source, test, and template blobs were verified against the pinned tree, including `VStack/VStack.tsx` (Git blob `026c3912cf2706dab6feecd84fc996e3557fcb40`). The showcase includes the Basic and Showcase templates using existing Heading, Text, Badge, and HStack components.

All five standard Rust checks passed. SSR tests compare VStack with an explicitly configured vertical Stack and verify alignment precedence and native root passthrough. Chrome verified both templates, native heading semantics, child order, and measured token gaps in LTR and RTL; HStack, Stack, and showcase navigation regression tests also passed. Published screenshot parity, other browsers, and manual assistive-technology behavior were not tested. The migration checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8117 node tests/v_stack.browser.mjs
```
