# Drawer modal checkpoint

`src/drawer/component.rs` renders a caller-controlled native modal drawer at the logical start or end edge. It reuses Dialog's dismissal controller and purpose policies. On open, the controller presents the native modal and then attempts to focus the first caller-provided `[data-autofocus]` descendant belonging to that dialog (FR8). Without a marker, native `autofocus` places focus on the content wrapper, which stays out of the tab order. The controller and native focus return on close are unchanged. An explicit close action belongs to the caller's DrawerHeader.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Drawer/Drawer.tsx` (`1d6228bbf8b7d777c6c1cf3cb913f9210f8111bb`), its spec (`a24082647866fd4b179486d0954752f7ee9d8e37`), and the adapted Details showcase (`e2371e2c09e06ec748894da12f5a7e41851ebeed`). The preview uses RTL to exercise logical end placement.

The original modal checkpoint passed five Rust checks and four Chrome suites, covering accessible naming, full-height RTL placement, initial content focus, Escape and header dismissal, and focus restoration. For the focus follow-up, `tests/drawer.rs` checks the server-rendered native fallback and caller marker. `tests/drawer.browser.mjs` now covers both unmarked and caller-marked opens with separate triggers and checks focus return after both dismissal paths; the updated browser suite was not run for this follow-up.

Nonmodal presentation, shared layer ordering, scroll locking, transitions, inset propagation, open-unmount focus recovery, and published visual parity remain open. The checklist remains open.
