# ChatMessageBubble checkpoint

ChatMessageBubble inherits sender and density from ChatMessage, with assistant and balanced defaults when standalone. It renders a single painting surface plus optional sibling name and metadata slots. Filled and ghost variants share inline padding, and grouped corners follow the sender's logical side. An explicit validated width replaces the default cap.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Chat/ChatMessageBubble.tsx` (`b240230fdbfe24fdcb943e6c8753f4954bea27ef`), its spec (`3cf27ef1059e92e770638f1f7bc63f8ba20207a2`), and six catalog templates. Previews use static transcript wrappers, shortened copy, and substitute artifact content. ChatMessage examples now use the real bubble. ChatMessageList density integration remains pending.

At the initial checkpoint, all five standard Rust checks and four Chrome suites passed. Focused checks covered sibling markup, standalone defaults, RTL grouped corners, name-row avatar offset, inherited densities, and full-width ghost geometry.

Reflow follow-up (pinned spec AR3): the bubble-owned name and metadata wrappers now wrap unbroken text in a narrow transcript. The name row uses the upstream spacing token as a minimum rather than a fixed height, so wrapped text can grow without changing the usual short row. This is a port-side reflow adaptation; padding, slot gaps, bubble width, and ChatMessage avatar placement rules are unchanged. The showcase and browser regression cover both slots at 320 CSS px. The new browser assertions were not run in this follow-up.

Published visual parity, manual assistive technology, and browser verification of the new reflow case remain open. The migration checklist remains open.
