# Toast: standalone surface checkpoint

`src/toast/component.rs` renders a standalone notification with a body, optional trailing actions, and a dismiss button. The caller removes the toast after a manual or automatic dismissal request. The browser timer pauses on hover and focus and is disposed on unmount. Info bodies use `status`; error bodies use `alert`. The private Stack Toast viewport is unchanged.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Toast/Toast.tsx` (`be220993f46f1964127cf7439a88b208359b9a74`), its spec (`adeefca06b9bcac13e6c0c25f71e56c87f89af25`), and the showcase (`5e43480ee7dfb7d217117f10b7cf7ef358f387aa`). The adapted showcase keeps the saved-document message and uses an inline timed error example instead of the toast host.

The CSS bundle check, all five Rust checks, and three Chrome suites passed. The focused suite checked live-region markup, manual dismissal, and timer pause and resume on focus and blur. It did not verify spoken announcements. Host and queue integration, swipe dismissal, transitions, media theme and visual parity, and hover-pause browser evidence remain open. The checklist remains open.
