# SideNavSection checkpoint

`src/side_nav/side_nav_section.rs` renders a group named by its title, with an optional subtitle and inline-end content. The explicit hidden state uses VisuallyHidden and preserves the group name and item DOM. As in the pinned source, clipping the header does not remove its content from the tab order; do not put focusable controls in `end_content` when the header is hidden. The upstream spec records this as an open accessibility issue.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `SideNav/SideNavSection.tsx` (`bc4d6457da45ebc804bac261ecb48571dbfe9e6b`), its spec (`7645ff30c5edc7b861ce161859e0bb6d93d5eaac`), and both catalog templates. The showcase preserves their grouping and link labels with native navigation and links in place of the unported SideNav and SideNavItem. Rail-driven collapse behavior remains pending.

All five standard Rust checks passed. SSR verified naming and root attribute precedence. Chrome verified names before and after clipping in the accessibility tree, unique title references, RTL end placement, item spacing, and retained item DOM. SideNavHeading and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified. The migration checklist remains open.
