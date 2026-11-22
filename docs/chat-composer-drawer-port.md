# ChatComposerDrawer checkpoint

`src/chat/chat_composer_drawer.rs` renders inline composer content. A count, including zero, enables a native disclosure with a stable content reference. Without a count, content stays open and there is no toggle. Collapsed content stays mounted but inert; visual summaries are inert and do not name the trigger. Reduced motion removes the collapse and fade transitions.

Pinned Meta Astryx sources at `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Chat/ChatComposerDrawer.tsx` (`991a4c165b8bc8b2f6e9559f7b441b9b8b7ffdf7`), its spec (`4daf9fece6301bdcab4d68e24dce02ff38ed5238`), and the adapted Collapsible arrangement (`a6fc835eb9ec4a81dcc28757c5855c4d0215fec2`). The standalone preview uses static file tokens and adds a focusable control to verify inertness.

All five Rust checks passed. Chrome verified keyboard disclosure, retained inert content, reduced motion, and showcase navigation. The existing ChatLayout regression suite passed and timed out on separate runs of both the preceding preview and this checkpoint; its thresholds remain unchanged. Production composer integration, remaining catalog arrangements, controlled-close focus behavior, and visual parity remain open.
