# Drawer modal checkpoint

`src/drawer/component.rs` renders a caller-controlled native modal drawer at the logical start or end edge. It reuses Dialog's dismissal controller and purpose policies. Focus starts on the drawer content, and an explicit close action belongs to the caller's DrawerHeader.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Drawer/Drawer.tsx` (`1d6228bbf8b7d777c6c1cf3cb913f9210f8111bb`), its spec (`a24082647866fd4b179486d0954752f7ee9d8e37`), and the adapted Details showcase (`e2371e2c09e06ec748894da12f5a7e41851ebeed`). The preview uses RTL to exercise logical end placement.

All five Rust checks and four Chrome suites passed. The focused suite verified the accessible dialog name, full-height RTL placement, initial content focus, Escape and header dismissal, and focus restoration. Nonmodal presentation, shared layer ordering, scroll locking, transitions, inset propagation, and published visual parity remain open. The checklist remains open.
