# MetadataListItem checkpoint

`src/metadata_list/metadata_list_item.rs` renders adjacent native `<dt>` and `<dd>` elements in an inline description list. The private parent context selects the stacked wrapper; the item does not expose a separate layout override. Native attributes and caller styles reach the inline term or the stacked wrapper. Values can contain interactive content without the item adding keyboard behavior.

The port follows Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, including `MetadataList/MetadataListItem.tsx` (Git blob `629f6364f1d4f9ddbf721e2d9b16d9972588150f`), the item spec, and both catalog templates. Both examples now use the public [MetadataList](metadata-list-port.md) parent.

Prior verification: the five standard Rust checks passed; SSR covered inline and stacked markup and attribute routing. Chrome verified inline term/value alignment, the native AX term, and the focusable value link. The parent browser suite verified stacked DOM markup. That evidence does not establish AX term/definition pairing or spoken announcements.

The pinned styles opt inline terms and both value layouts into `word-break: break-word`; stacked terms inherit the surrounding wrapping rules. The port previously applied `overflow-wrap: anywhere` to all four cells, so a long unbroken stacked label could wrap where upstream leaves it unbroken. The item styles now match the upstream rule placement. The item browser regression checks computed wrapping rules and narrow stacked geometry, but was not run in this follow-up. Raster parity, manual screen-reader verification, and upstream ref-target parity remain open.

```sh
SHOWCASE_URL=http://127.0.0.1:8134 node tests/metadata_list_item.browser.mjs
```
