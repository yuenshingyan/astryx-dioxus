# MetadataListItem checkpoint

`src/metadata_list/metadata_list_item.rs` renders adjacent native `<dt>` and `<dd>` elements in an inline description list. The private parent context selects the stacked wrapper; the item does not expose a separate layout override. Native attributes and caller styles reach the inline term or the stacked wrapper. Values can contain interactive content without the item adding keyboard behavior.

The port follows Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, including `MetadataList/MetadataListItem.tsx` (Git blob `629f6364f1d4f9ddbf721e2d9b16d9972588150f`), the item spec, and both catalog templates. The examples use native description lists with demo grid styling until the parent MetadataList is ported.

All five standard Rust checks passed. SSR tests covered inline and stacked markup and attribute routing. Chrome verified both inline examples, term/value alignment, the native accessibility-tree term, and the focusable value link. Showcase navigation also passed. Stacked browser behavior, published screenshot parity, and manual screen-reader testing remain open.

```sh
SHOWCASE_URL=http://127.0.0.1:8134 node tests/metadata_list_item.browser.mjs
```
