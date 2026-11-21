# TopNavHeading checkpoint

`src/top_nav/top_nav_heading.rs` ports the non-menu forms: static and linked logo-only headings, static text, whole-heading links, and independent destinations. The grid keeps the logo at its natural width, truncates plain text, and lets independent links wrap. Whole-heading and logo links use the shared link renderer and destination validation. Unlike the pinned TopNavHeading, the whole-heading link puts trailing content in a sibling slot, matching SideNavHeading and avoiding nested interactive controls.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `TopNav/TopNavHeading.tsx` (`cade0cf0f6a9c200097e3e9b364ab47e9cd93296`), the component spec (`6258af6ce95df71723c37f1250a212cb911e1800`), and both catalog templates. The showcase adapts those templates with native navigation framing and local artwork in place of NavIcon. TopNav and NavIcon integration and the menu trigger remain pending. The migration checklist stays open.

All five standard Rust checks passed. SSR verified named logo-only links, independent destinations, and separation of trailing controls. Chrome verified logo track sizing and accessible name, narrow RTL truncation, wrapping independent links, and trailing focus. SideNavHeading, Link, and showcase navigation regressions passed. Published screenshot parity and manual assistive technology were not verified.

```sh
SHOWCASE_URL=http://127.0.0.1:8149 node tests/top_nav_heading.browser.mjs
```
