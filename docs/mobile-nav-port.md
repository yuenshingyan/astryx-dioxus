# MobileNav checkpoint

`src/mobile_nav/` adds a controlled native modal dialog containing a named navigation landmark. It supports logical start and end placement, caller-owned content, an optional header, and close requests from Escape, the close button, and backdrop clicks. It can read the existing AppShell mobile context when explicit open state is omitted. Native dialog opening and event listeners are installed only after mount.

References at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `MobileNav.tsx` (`63a5465afc8c9fb144d345d718a3730064fd941f`), its spec (`99aabf0a30aa269281755aec049c0e31be8abee8`), and `MobileNavShowcase.tsx` (`6c10edc8e60e3e54095cf1c0d83d6a32790d1b82`). The preview uses local links, a text trigger, and explicit RTL start placement.

All five Rust checks and three Chrome suites passed. The focused suite verified modal and landmark names, initial focus inside the drawer, RTL start placement, Escape and close-button dismissal, and focus restoration. Automatic edge selection, transition and swipe behavior, backdrop-click browser verification, production AppShell integration, and published visual parity remain open.
