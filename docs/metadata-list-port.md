# MetadataList checkpoint

`src/metadata_list/component.rs` owns the native description list, optional title, column layout, and Show more/less button. Its private context controls item stacking and resets at a layer boundary. Horizontal orientation ignores vertical column, label, and collapse settings. Visible items retain their identity when the list expands or collapses; the button keeps keyboard focus and points to the list with `aria-controls`.

The disclosure counts item components, including conditional items and keyed loops. Following the pinned disclosure behavior (FR4), adding or removing `max_num_of_items` on a vertical list updates the visible prefix without remounting entries that remain visible. The same keyed entries are used with and without the limit. The native toggle and `MetadataListItem` markup are unchanged. Caller-authored static RSX fragments containing multiple roots are preserved as a group rather than split. Fixed column counts use a nonzero integer, and callers can localize both disclosure labels.

The port follows Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, including `MetadataList.tsx` (Git blob `a3f85e1742b52bfdcd61306c59eb37d7635a55a7`), `MetadataList.spec.md` (`c281e6e66e78e4936ea0a2854e9c0e521557133f`), and five catalog arrangements. The multi-column example substitutes Badge for the unported Token. Both MetadataListItem examples now use the public parent.

At the initial checkpoint, all five standard Rust checks passed. Chrome verified keyboard disclosure, focus retention, item identity, horizontal and stacked markup, and the existing item, Layout, and showcase navigation suites. This follow-up adds a focused Rust regression test for retained item mounts and toggle visibility as the limit is added, removed, or equals the item count. Browser focus during live limit changes was not retested. Published screenshot parity, other browsers, and manual assistive technology remain unverified. The checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8135 node tests/metadata_list.browser.mjs
```
