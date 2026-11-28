# NavIcon checkpoint

`src/nav_icon/component.rs` renders one `span.astryx-nav-icon` with a centered, caller-owned glyph. It uses the shared medium element size, accent background, and on-accent foreground. Classes, inline style, and native span attributes go on the root. NavIcon adds no role, accessible name, or focus stop.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `NavIcon/NavIcon.tsx` (`44ffb9e723495fa0d78377ebbef3847f266783b3`), its spec (`f7afeb7e8e42478dbb51825642a827b31199f698`), shared tokens, and both catalog templates. The examples reproduce the search, calendar, and wrench arrangements using pinned fallback artwork. TopNavHeading now uses NavIcon in place of its demo-only wrapper. PageNavHeader is not ported.

The pinned source and FR1–FR3/AR1 specify one neutral wrapper and one theming target (`nav-icon`, represented by `.astryx-nav-icon` here). Link activation, current/disabled state, and navigation grouping are host behavior, not NavIcon defaults. In a heading with visible text, make redundant artwork decorative. For a logo-only link, supply meaningful artwork or a name on the link; NavIcon supplies neither. Dioxus accepts `dioxus::events::onmounted` through the `attributes` vector rather than a React `ref`; use `class` and `style` rather than a literal StyleX `xstyle` prop.

- **SSR / VDOM:** `tests/nav_icon.rs` covers caller attributes and artwork semantics, the mount listener targeting the root, and both decorative and meaningful artwork inside TopNavHeading links. The decorative case keeps caller-supplied `aria-current` on the link. These assert markup and listener placement, not a computed name, live mount callback, or keyboard behavior.
- **DOM / AX (prior Chrome checkpoint):** The example roots reported `tabIndex = -1`, and Chrome's AX tree exposed the text-bearing TopNavHeading link with the expected name. The logo-only case has SSR evidence only.
- **Computed CSS / geometry (prior Chrome checkpoint):** All five marks matched the medium size, centering, and accent tokens in light and dark; the TopNavHeading spacing check also passed. The pinned style declarations match the port by source inspection. Neither is raster evidence.
- **Open:** PageNavHeader composition is blocked on that unported host. Published raster parity and spoken output with assistive technology have not been verified. Live mounting and styling precedence were not exercised in this follow-up.

```sh
SHOWCASE_URL=http://127.0.0.1:8150 node tests/nav_icon.browser.mjs
```
