# SideNavHeading checkpoint

`src/side_nav/side_nav_heading.rs` ports the expanded, non-menu forms: static heading, whole-heading link, and independent links for the title and surrounding text. Trailing controls sit beside the whole-heading link rather than inside it. Whole-heading and icon links use the shared link renderer and destination validation.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `SideNav/SideNavHeading.tsx` (`f19b5d0bcd2e2bba44e99a508bf07d6709d5fa3b`), the component spec (`4f70cd1d000a00f3424721e94d810235ec66f9ee`), and both catalog templates. The showcase adapts the templates with native navigation framing and a local icon. Menu triggers, collapsed mode, and parent SideNav integration remain pending. The migration checklist stays open.

All five standard Rust checks passed. SSR verified static and linked roots, independent destinations, and trailing-action separation. Chrome verified linked rows, the accessible link name, minimum height, and separate focus for trailing content. The Link and showcase navigation regressions passed. Published screenshot parity and manual assistive technology were not verified.
