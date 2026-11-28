# VisuallyHidden port

`VisuallyHidden` clips content without adding `aria-hidden`, a role, or live-region behavior. It defaults to a `span`; `as_element` can render a block `div`, a heading, or another caller-owned root. Forward the supplied attributes and children unchanged. Use a block `div` and caller-provided `aria-live` for announcements. Use conditional rendering or `hidden` to hide content from everyone; `inert` and `aria-hidden` also pass through. Don't put interactive controls inside the clipped root: clipping and `pointer-events: none` do not remove keyboard focusability.

## Pinned source and dependencies

The [upstream pin](https://github.com/meta/astryx/commit/476a3153417dbf75c565f7df05b0784e94ddd59e) resolves to tree `a56f6eeb32d5e27d874a247c40b2bcd1fb960ab6`. Compared the supplied source and spec, English doc, export, test, Storybook story, all four templates (TSX and metadata), and their direct Astryx sources against that tree's Git blob hashes. `VisuallyHidden.tsx` is `dcbe3f5019e5bf94adfd0df35f8d7e2f81c8c6d1`; `VisuallyHidden.spec.md` is `aac13991da90e8277963ffc820ae7f87595fff83`.

The primitive uses a fixed, token-free clip block; it has no design variants or theme dependency. `BaseProps` supplies the native passthrough contract. Dioxus uses `GlobalAttributes`, including mount and event callbacks, rather than a React ref prop. Raw `xstyle` is ignored for source compatibility without introducing a typed StyleX prop. Both raw `style` and Dioxus's individual CSS properties (which use the `style` namespace) are discarded before applying the clip, along with caller classes. This also preserves the clipping used by ProgressBar and AvatarGroup; neither calls the primitive with style overrides.

| Pinned template | Port and evidence |
| --- | --- |
| Structural Heading | Hidden native `h2` names the same three build cards. SSR verifies a custom heading root without a wrapper; prior Chrome AX evidence finds `Build status`. |
| Live Region | Same Move task action and three columns. Prior Chrome evidence verifies the text update on the same polite `div`; this does not verify that AT speaks the update. |
| Supplementary Context | Same two stat cards, decorative arrows, and hidden trend phrases. Prior Chrome AX evidence finds both phrases. |
| Showcase | Same Card/Stack/Text/Icon/IconButton composition. The three IconButtons own their labels, as in the source; the hidden polite `div` carries the action summary. Metadata aliases were compared with the template, and all six SVG glyphs across this and the stats template were checked against Heroicons 2.2.0, using the tarball integrity from the pinned lockfile. Prior Chrome AX evidence finds all three button names. Don't rely on a polite region with only initial text to speak on mount. |

The templates use existing Card, Badge, Button, IconButton, Text, Icon and Layout stack ports; no new dependency or cascade change is needed. A separate filter example demonstrates an actual `VisuallyHidden` child naming a native icon-only button (the named template's IconButton intentionally doesn't take children) and a persistent polite result count. It leaves the original four patterns intact.

## Verification and completion blockers

- Five focused SSR tests passed via the assigned Cargo slot: default clip and no implicit semantics, ignored raw and namespaced styles, native opt-out attributes, wrapper-free custom heading, and showcase label/region markup. The final Speaker Wave SVG path data was checked directly against the pinned package after the SSR run.
- The Chrome results referenced above are from the existing port, not a new run. The expanded browser regression checks computed clipping and pointer/selection styles, AX naming from the filter's hidden child, and a result count update on the same mounted node. Run it against an integrated showcase build; browser/preview and the full Rust, Clippy and WASM checks were not run in this worktree.
- The content-pattern and Live Region items stay open pending that browser run and a manual screen-reader pass: confirm the hidden child is spoken as the button name, and that activating Move task and toggling the filter each speaks the changed polite message without shifting focus. The structural/template checks are not a claim of screenshot parity. Compare all four rendered recipes with the pinned originals (including responsive wrapping and the complete Speaker Wave glyph); verify clipping and AX exposure in a non-Chromium engine. Native mount/event delivery through a custom root has not been browser-tested here.
- Upstream spec FR3/OQ1 remains an owner decision: recorded styling-free exception versus composition of root styling inputs. This port preserves the released fixed clip behavior; it does not resolve OQ1 or promise React ref/StyleX type parity.

```sh
SHOWCASE_URL=http://127.0.0.1:8119 node tests/visually_hidden.browser.mjs
```
