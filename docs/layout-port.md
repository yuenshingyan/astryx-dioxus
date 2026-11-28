# Layout checkpoint

`src/layout/component.rs` composes five logical slots with the public Layout regions, or with arbitrary caller elements. The outer root owns mount callbacks, native attributes, caller styles, and `data-height`. The inner stack resets inherited Layout-specific sizing and outer insets. Slot presence and divider defaults update without remounting content; a layer boundary clears inherited defaults. Conditional empty nodes and fragments count as absent. A caller component that itself renders nothing is still a supplied slot, as its output is not inspected.

`content_width` keeps a single LayoutContent scrollport at the edge with zero or one side panel when the width can participate in alignment arithmetic. Both panels, arbitrary content, intrinsic widths, percentages, and bare variables use the constrained composition where applicable. A LayoutContent inside a side panel cannot qualify the middle row for open-edge alignment. CSS `"0"` is lowered to `0px` like numeric zero so it can be subtracted from a length. LayoutPanel receives its logical side from Layout rather than from example CSS.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Layout.tsx` (Git blob `73bea724b5de6749e38adf488c8d85bb2b738095`), `Layout.spec.md` (`c4fa592a3c4b22c9e148363eb661eaa1dfbf06bb`), and seven JSX templates with their catalog descriptors. The showcase adapts Lists with native static lists. Panel resize, sliding collapse, and focus handoff still depend on `useResizable` and LayoutPanel integration; the existing ResizeHandle is a standalone keyboard-only separator. The complete upstream overlay CSS reset contract has not been ported or verified across overlay components. The migration checklist stays open.

At the previous checkpoint all five standard Rust checks passed. Chrome verified seven showcase arrangements, logical RTL panel order, conditional removal, edge scrolling and inset alignment. The four Layout region suites, StackItem, and showcase navigation also passed.

This follow-up's five focused SSR tests verify slot composition and native semantics, root metadata, divider context isolation, and width-path selection; they do not measure layout. The native showcase target also compiled. The new browser probes cover the constrained arbitrary-content lane, string and numeric zero, asymmetric inset escape, fill versus auto, adjacent-header flush, and nested padding/width reset. The caller-owned landmark has a separate Chrome AX assertion. These probes await the integrator's browser run; there is no new Chrome DOM/AX or computed geometry/paint evidence, no published raster comparison, and no manual screen-reader speech verification.

AppShell already applies Layout's CSS reset at its root. Overlay-wide inset isolation still needs the layer surface owners to apply and verify the full reset at their DOM roots; `LayerContentBoundary` resets Rust contexts without creating a DOM node. The named AppShell composition and template appearance are not signed off by SSR or arrangement-presence checks.

```sh
SHOWCASE_URL=http://127.0.0.1:8128 node tests/layout.browser.mjs
```
