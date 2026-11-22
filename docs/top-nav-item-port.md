# TopNavItem checkpoint

The horizontal item supports links, selected and disabled states, icon-only labels, and caller actions. Enabled destinations use the explicit or inherited link renderer. Disabled and rejected destinations remain focusable, non-operable links and bypass the renderer. Actions use native buttons for Enter and Space rather than emulating button behavior on an anchor.

Source at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `TopNav/TopNavItem.tsx` (`29f48cbeaaf29e05042dfb9f81e191ef6a437cb6`), its spec (`7eb6c6f7a497cf1c0547272d59a7cfd3519636a0`), and `TopNavItemShowcase.tsx` (`b25aa0e7a45c09dee3b8f8ac2a1909f1af4fa6be`). The showcase uses a standalone navigation bar with local fragment destinations and adds an action example.

All five Rust checks and three Chrome suites passed. SSR verified that only enabled destinations reach the custom renderer. Chrome verified the current-page marker, the disabled link in the accessibility tree, Space activation, and retained action focus. Drawer mode and close-on-activate, touch press behavior, remaining native attribute forwarding, and published visual parity remain open.
