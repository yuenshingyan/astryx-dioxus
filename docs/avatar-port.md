# Avatar checkpoint

Avatar renders a static image semantic, native button, or provider-backed link. It resolves the named and numeric size scales, clips only the face, and falls back from the primary source to the secondary source, initials, or the person glyph. Its status slot supplies reactive size context and collects labels from nested AvatarStatusDot components after mount. `status_label` also supports explicit server-rendered names and custom status elements. Caller ARIA naming takes precedence.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Avatar.tsx` (Git blob `8107a024b0c74703c988546e264e725360c95b99`), `Avatar.spec.md` (`6c262f6305d146e4b32304f7b9626db43877a9f4`), and the catalog templates. Seven Avatar arrangements and both AvatarStatusDot arrangements are available. The image examples use pinned offline portraits. [AvatarGroup](avatar-group-port.md) now supplies group geometry; the HoverCard user-card example remains pending, and the interactive example uses inline feedback rather than Toast.

All five standard Rust checks passed. Chrome verified the offline fallback chain, wrapped status-label updates and removal, reactive sizing, and button activation. The standalone AvatarStatusDot and showcase navigation regressions also passed.

FR2 follow-up: an empty `src` or `fallback_src` is treated as absent, matching the pinned source. Nonempty sources are still used as supplied (no trimming), with failure tracked per source. The focused SSR regression checks the immediate fallback photo and default icon without waiting for an image error; it does not exercise browser loading or retry.

## Status name gate

Both supplied blobs match the pinned Git tree. FR5 reads a directly passed element's `label` during render, before any child report. In Dioxus 0.7.10, Avatar receives an `Element`; the direct component's `VComponent::props` is crate-private and the public mounted-scope accessors are too late for first-render naming. The existing report runs in an effect and schedules an Avatar rerender, so it does not establish upstream PR1's before-paint/no-rerender behavior.

**Decision needed:** accept explicit `status_label` as the first-render/SSR contract, or approve a way to pass label metadata at slot construction (or wait for supported Dioxus prop access). With current boundaries, even a directly supplied AvatarStatusDot only reports after mount. Custom status components must supply `status_label` or contain an AvatarStatusDot.

`tests/avatar.rs` adds root-markup assertions for explicit SSR status and caller naming, plus VirtualDom coverage for direct and wrapped mount, rename, blank label, removal and remount. The focused run was blocked compiling the library by `No space left on device`; these new assertions are not verified. They check rendered attributes, not a browser accessibility tree or speech.

The initials implementation selects Unicode scalars, not full grapheme clusters. Full tooltip parity, published screenshots, other browsers, and manual assistive technology remain unverified. The migration checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8139 node tests/avatar.browser.mjs
```
