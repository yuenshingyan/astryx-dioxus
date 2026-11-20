# Center port

Center renders a single native `div` without adding a role, name, or tab stop. It supports block and inline flex, both-axis or single-axis centering, validated sizing, and logical padding. Each edge resolves from its own prop, then its axis prop, then uniform padding; zero is explicit. That padding is local and does not publish container padding. Caller classes and inline style can override layout, while Center owns `data-axis`.

The port follows [Meta Astryx `476a315`](https://github.com/meta/astryx/tree/476a3153417dbf75c565f7df05b0784e94ddd59e). Eleven Center source, specification, test, and template blobs were checked against that revision's Git tree. `Center.tsx` is blob `4e2c74dac91ee1e66e6edfd540936f0b5da35358`; the specification is `29bd13dfa191340f6111664ea524b28b33656579`.

The released implementation's single-axis names follow the logical flex axes. In vertical writing, `Horizontal` therefore centers vertically, and `Vertical` centers horizontally. This preserves the behavior documented as a gap in FR2 rather than silently changing it. All three templates are represented in the showcase, using existing Stack, Card, Icon, IconButton, Text, and Heading components.

All five standard Rust checks passed. The Chrome suite compared the three patterns against source-derived flex declarations and verified formatting control names and the released axis behavior. The showcase navigation check also passed. The browser comparison does not execute upstream React/StyleX or compare published screenshots. Other browsers and manual assistive technology were not tested; the Center parent checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8114 node tests/center.browser.mjs
```
