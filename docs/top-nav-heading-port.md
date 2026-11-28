# TopNavHeading checkpoint

`src/top_nav/top_nav_heading.rs` ports the non-menu forms: static and linked logo-only identity, static text, whole-heading links, and independent destinations. Like the pinned source, product identity text does not create a page heading. Whole-heading and logo links use the shared link renderer and destination validation. Empty hrefs and destinations on omitted supporting lines do not create links or split the product link. A blank `logo_label` does not replace the product-name fallback on independent logo links or the link content's name in logo-only mode.

The grid keeps the logo at its natural width, truncates plain text, and lets independent links wrap. Unlike the pinned TopNavHeading, the whole-heading link puts trailing content in a sibling slot, matching SideNavHeading and avoiding nested interactive controls. Use passive artwork in a linked logo slot; move controls to the trailing slot. A logo-only link with decorative artwork needs a caller-supplied name.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `TopNav/TopNavHeading.tsx` (`cade0cf0f6a9c200097e3e9b364ab47e9cd93296`), the component spec (`6258af6ce95df71723c37f1250a212cb911e1800`), and both catalog templates. The showcase adapts those templates with native navigation framing and uses [NavIcon](nav-icon-port.md) for the wrapped logos. The migration checklist stays open.

At the earlier checkpoint, all five standard Rust checks and the SideNavHeading, Link, and showcase navigation browser regressions passed. Chrome AX showed the named logo link; separate layout/interaction assertions checked logo track sizing, narrow RTL truncation, wrapping independent links, and trailing focus. This was not raster or spoken-output evidence.

The focused SSR suite now also checks the native link boundaries, non-heading identity inside TopNav, label markup, renderer precedence, and rejected destinations. The updated Chrome suite adds DOM and AX assertions and exercises Tab and Enter on the product link and trailing button. It was syntax-checked only; the integrator still needs to run it. No new paint/geometry measurements, published raster comparison, or manual screen-reader speech checks were made. The named template checkboxes remain open pending comparison with the originals.

Menu modes (FR4–FR11 and the menu branches of FR1–FR3) need a shared Popover host that can anchor to the row without wrapping the menu in a dialog, a NavHeadingMenu selection-to-close context that preserves a single menu role, and the shared hover hand-off and slide-to-select behaviors. The current Popover hardcodes the dialog role, and NavHeadingMenu is an inline menu with no host close contract. These are shared-owner dependencies; adding a parallel local runtime or a new public `menu` prop is not part of this pass. Crowded-bar behavior in the actual TopNav slot still needs integrator verification.

```sh
SHOWCASE_URL=http://127.0.0.1:8149 node tests/top_nav_heading.browser.mjs
```
