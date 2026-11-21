# ChatLayout checkpoint

`src/chat/chat_layout.rs` provides an internally scrolling transcript and an in-flow sticky composer dock. Density selects message width, dock spacing, and blur geometry. The scroll action can be default, hidden, or custom. The default action jumps to the end and transfers focus to the transcript log before the pill becomes inert. The private stream-scroll controller follows content growth while pinned and leaves the reading position alone when scrolled up.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Chat/ChatLayout.tsx` (`b2c2b3255e9242d34e2a201d8f958ca7a867150e`), its spec (`6e5aeb02a7777a1bdf8dc4c2ca1bec725891a13d`), and both catalog templates. The examples use a native input instead of ChatComposer. External scroll containers, fixed dock and software keyboard handling, transcript ref registration, new-message labeling, and upstream spring scrolling remain open.

All five standard Rust checks and four Chrome suites passed. Tests covered empty and custom-action markup, short and overflowing layouts, sticky dock position, initial end positioning, reading-position retention, keyboard focus handoff, and pinned growth. Published visual parity and manual assistive technology were not verified. The checklist remains open.
