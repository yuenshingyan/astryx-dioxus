# SideNavItem checkpoint

The expanded-rail leaf supports links and actions, shared link-renderer resolution, selected icons, a current-page marker, and non-interactive trailing content such as a badge. Disabled items remain focusable buttons without invoking caller actions. Rejected destinations bypass the custom renderer and remain non-operable.

Sources at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `SideNav/SideNavItem.tsx` (`6e6692da2eb4d598b1e07c54f221d4f3db850beb`), its spec (`b23ade6adefcd7ddd1b1b0822e3ad9ff5e2ca2e7`), and `SideNavItemShowcase.tsx` (`ff8853ed7edf00ada8aa725800bf5c9b4aa7121c`). The adapted showcase uses local fragment links, omits the original icons, and adds an action example.

All five Rust checks and four Chrome suites passed. SSR verified selected-icon substitution, renderer selection, and disabled markup. Chrome verified the current link, trailing badge, disabled action, and Space activation with retained focus. Nested rows, separate row actions, collapsed rail behavior, touch press feedback, and published visual parity remain open.
