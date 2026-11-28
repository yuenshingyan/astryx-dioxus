# MobileNav checkpoint

`src/mobile_nav/` adds a controlled native modal dialog containing a named navigation landmark. It supports logical start and end placement, caller-owned content, an optional header, and close requests from Escape, the close button, and backdrop clicks. It can read the existing AppShell mobile context when explicit open state is omitted. Native dialog opening and event listeners are installed only after mount.

References at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `MobileNav.tsx` (`63a5465afc8c9fb144d345d718a3730064fd941f`), its spec (`99aabf0a30aa269281755aec049c0e31be8abee8`), and `MobileNavShowcase.tsx` (`6c10edc8e60e3e54095cf1c0d83d6a32790d1b82`). The preview uses local links, a text trigger, and explicit RTL start placement.

At the initial checkpoint, all five Rust checks and three Chrome suites passed. The focused browser suite covered modal and landmark names, initial focus inside the drawer, RTL start placement, Escape and close-button dismissal, and focus return.

The MobileNav controller now records the opener before `showModal()` and explicitly restores focus after native close (including teardown), if the opener is still connected and the browser has not already restored it. It also ignores `cancel` events from descendants, such as a file input, while keeping native dialog close requests controlled. This follows the pinned source's focus-return behavior (FR6/AR2) and the dialog-origin part of its close-request handling (FR4). `node tests/mobile_nav_controller.mjs` exercises those paths without a browser, including a declined close, reopening from another opener, and a removed opener.

Native browser focus behavior was not re-verified for this change. Shared dismissal ordering, Tab wrapping, and scroll lock were not changed. Automatic edge selection, the 85vw width cap, transition and touch behavior, backdrop-click browser verification, production AppShell integration, and published visual parity remain open. The controller tests do not establish raster or speech parity.
