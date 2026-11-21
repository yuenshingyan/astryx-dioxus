# DrawerHeader checkpoint

`src/drawer/drawer_header.rs` composes LayoutHeader, a native level-two Heading, optional subtitle and action slots, and an icon-only ghost Button when a close callback is provided. The button requests `false`; the surrounding surface owns its name, dismissal, and focus. End-slot compensation uses logical edges and defaults to both axes when a close button is present.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Drawer/DrawerHeader.tsx` (`2486f5db193f3aa07ad739625c086d456a519413`), its spec (`b4d7b0faff26116d608c7a8967c298a6f52491d3`), and three catalog templates. The showcase adapts them with inline Layout panels. The Drawer container and translation-provider integration remain pending; applications can supply a localized close label.

All five standard Rust checks passed. SSR verified slot markup, close-button presence and naming, and default compensation. Chrome verified the heading and close button in the accessibility tree, keyboard close, divider reflection, and RTL compensation. LayoutHeader and showcase navigation regressions passed. Published visual parity and manual assistive technology were not verified. The migration checklist remains open.
