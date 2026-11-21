# TopNavMegaMenuItem desktop checkpoint

`src/top_nav/top_nav_mega_menu_item.rs` ports the standalone desktop tile. It renders an anchor for a destination, a native non-submit button for an action, or a static div. Link tiles use the explicit or provider renderer and share navigation URL validation. The tile supplies its focus outline and native CSS hover and active feedback; the upstream press-feedback lifecycle, drawer rendering, panel dismissal, and focus return are not ported here.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `TopNav/TopNavMegaMenuItem.tsx` (`72754c0fad0d22767b639dc0b26139546122e938`), its spec (`fbce818a453f2b513a713292a6565fd65b15aca6`), and both catalog templates. The showcase adapts both standalone arrangements with inline artwork.

All five standard Rust checks passed. SSR verified the three native roots and tabindex forwarding. Chrome verified keyboard activation, native navigation, icon box geometry, and accessible names. Featured Card and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified; the checklist remains open.
