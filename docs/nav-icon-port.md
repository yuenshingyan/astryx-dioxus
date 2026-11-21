# NavIcon checkpoint

`src/nav_icon/component.rs` renders one `span.astryx-nav-icon` with a centered, caller-owned glyph. It uses the shared medium element size, accent background, and on-accent foreground. Classes, inline style, and native span attributes go on the root. NavIcon adds no role, accessible name, or focus stop.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `NavIcon/NavIcon.tsx` (`44ffb9e723495fa0d78377ebbef3847f266783b3`), its spec (`f7afeb7e8e42478dbb51825642a827b31199f698`), shared tokens, and both catalog templates. The examples reproduce the search, calendar, and wrench arrangements using pinned fallback artwork. TopNavHeading now uses NavIcon in place of its demo-only wrapper. PageNavHeader is not ported.

All five standard Rust checks passed. SSR verified root attribute forwarding and caller-owned icon semantics. Chrome verified all five marks' size, centering, accent paint in light and dark, lack of a focus stop, and TopNavHeading spacing and link naming. The TopNavHeading and showcase navigation regressions passed. Published screenshot parity and manual assistive technology were not verified; the migration checklist stays open.

```sh
SHOWCASE_URL=http://127.0.0.1:8150 node tests/nav_icon.browser.mjs
```
