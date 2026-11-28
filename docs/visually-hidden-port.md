# VisuallyHidden port

`VisuallyHidden` clips content without adding `aria-hidden`, a role, or live-region behavior. It defaults to a `span`; `as_element` can render a block `div`, a heading, or another caller-owned root. Forward the supplied attributes and children unchanged. Use a block `div` and caller-provided `aria-live` for announcements. Use conditional rendering or `hidden` to hide content from everyone; `inert` and `aria-hidden` also pass through. Don't put interactive controls inside the clipped root: clipping and `pointer-events: none` do not remove keyboard focusability.

## Pinned source and dependencies

The [upstream pin](https://github.com/meta/astryx/commit/476a3153417dbf75c565f7df05b0784e94ddd59e) resolves to tree `a56f6eeb32d5e27d874a247c40b2bcd1fb960ab6`. Compared the supplied source and spec, English doc, export, test, Storybook story, all four templates (TSX and metadata), and their direct Astryx sources against that tree's Git blob hashes. `VisuallyHidden.tsx` is `dcbe3f5019e5bf94adfd0df35f8d7e2f81c8c6d1`; `VisuallyHidden.spec.md` is `aac13991da90e8277963ffc820ae7f87595fff83`.

The primitive uses a fixed, token-free clip block; it has no design variants or theme dependency. All 13 declarations match the pinned StyleX source after normalizing property names, spacing, and numeric pixel lengths. This is a source comparison, not a rendering comparison. `BaseProps` supplies the native passthrough contract. Dioxus uses `GlobalAttributes`, including mount and event callbacks, rather than a React ref prop. Raw `xstyle` is ignored for source compatibility without introducing a typed StyleX prop. Both raw `style` and Dioxus's individual CSS properties (which use the `style` namespace) are discarded before applying the clip, along with caller classes. This also preserves the clipping used by ProgressBar and AvatarGroup; neither calls the primitive with style overrides.

| Pinned template | Port and evidence |
| --- | --- |
| Structural Heading | Hidden native `h2` names the same three build cards. SSR verifies a custom heading root without a wrapper; Chrome AX finds `Build status`. |
| Live Region | Same Move task action and three columns. Chrome verifies the text update on the same polite `div`; this does not verify that AT speaks the update. |
| Supplementary Context | Same two stat cards, decorative arrows, and hidden trend phrases. Chrome AX finds both phrases. |
| Showcase | Same Card/Stack/Text/Icon/IconButton composition. The three IconButtons own their labels, as in the source; the hidden polite `div` carries the action summary. Metadata aliases were compared with the template, and all six SVG glyphs across this and the stats template were checked against Heroicons 2.2.0, using the tarball integrity from the pinned lockfile. Chrome AX finds all three button names. Don't rely on a polite region with only initial text to speak on mount. |

The templates use existing Card, Badge, Button, IconButton, Text, Icon and Layout stack ports; no new dependency or cascade change is needed. A separate filter example demonstrates an actual `VisuallyHidden` child naming a native icon-only button (the named template's IconButton intentionally doesn't take children) and a persistent polite result count. It leaves the original four patterns intact.

## Verification and completion blockers

- [PR #241](https://github.com/yuenshingyan/astryx-dioxus/pull/241) (`5f3a872`) ran full CI and passed all eleven integrated Chrome suites, including `tests/visually_hidden.browser.mjs`. The focused suite verifies computed clipping and pointer/selection styles, Chrome AX names and content (including the filter name supplied by its hidden child), and both polite messages updating on their original `div` nodes. No production or showcase behavior changed in this follow-up.
- Six focused tests passed offline through the assigned Cargo slot: five SSR cases and a Dioxus mutation test that verifies `mounted` and `click` listeners register on the caller-rendered root. Native mount/event **delivery** still needs an integrator-run browser check on that actual root; the Rust test only verifies registration. Formatting, CSS bundle verification, browser-script syntax, and diff checks passed. No browser, preview, or full CI rerun was started in this worktree.
- **Manual AT:** The content-pattern and Live Region items remain open. On a real screen reader, verify that tabbing to the filter speaks “Show passing builds only”; keyboard-activate Move task and the filter, and confirm that “Task moved to In progress” and “1 build result” are spoken without focus leaving the activated control. The Chrome DOM/CSS/AX checks do not establish speech, focus retention during keyboard activation, or raster parity.
- **Browser comparison:** Coordinate with the integrator to verify clipping and AX exposure in a non-Chromium engine and compare captures of all four pinned recipes at wide and narrow widths (including wrapping and the complete Speaker Wave glyph). The local pinned manifest inventories only the default story's Neutral light/dark baselines, not captures of the four recipes.
- **Shared dependencies:** The recipes use migrated slices of Text, Icon, and the Layout/Stack family (HStack and VStack); their checklist parents are still open. Their standalone migration and any recipe differences attributable to them belong to those owners, not VisuallyHidden.
- **Approval:** The public-component-API INV6 owner must record the styling-free exception with an AST-071 known-gap entry linked from FR3, or approve composition of root styling inputs under OQ1. Until then the port preserves the released fixed clip behavior without promising React ref/StyleX type parity.

```sh
SHOWCASE_URL=http://127.0.0.1:8119 node tests/visually_hidden.browser.mjs
```
