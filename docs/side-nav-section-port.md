# SideNavSection checkpoint

`src/side_nav/side_nav_section.rs` renders a group named by its title, with an optional subtitle and inline-end content. The explicit hidden state uses VisuallyHidden and preserves the group name and item DOM. As in the pinned source, clipping the header does not remove its content from the tab order; do not put focusable controls in `end_content` when the header is hidden. The upstream spec records this as an open accessibility issue.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `SideNav/SideNavSection.tsx` (`bc4d6457da45ebc804bac261ecb48571dbfe9e6b`), its spec (`7645ff30c5edc7b861ce161859e0bb6d93d5eaac`), and both catalog templates used in the original port. The Basic and Showcase examples now use the existing `SideNav` and `SideNavItem` instead of stand-in landmarks and links. They retain the original port's labels and local destinations. The separate header-visibility probe still uses plain links to isolate section behavior.

- **SSR markup:** `tests/side_nav_section.rs` checks two titled groups per example, distinct title references, a single current-page link per rail, and a disabled item outside the clipped header with no destination. It also covers root label precedence. This does not test keyboard activation or tab order.
- **Earlier browser DOM/AX evidence:** Chrome reported the group name before and after clipping and checked unique title references and retained item DOM. The earlier layout probe measured RTL end placement and item spacing and checked computed absolute positioning for the clipped header. No browser checks were rerun for the new host composition.
- **Raster and spoken evidence:** No published raster comparison or manual screen-reader speech verification.

The content checklist row is supported. Full catalog parity remains open. The pinned source also hides headers via `useSideNavCollapse`; the existing `SideNav` has no collapse context to consume. Hidden interactive `end_content` remains subject to the upstream spec's open decision (OQ1).
