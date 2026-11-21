# ChatDictationButton checkpoint

The button renders caller-supplied unsupported, idle, or listening state. While listening, five decorative bars respond to supplied samples and blend toward the error color as volume clips. The caller owns recording, permission handling, and speech recognition. The examples simulate listening without microphone access.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Chat/ChatDictationButton.tsx` (`a5948f176ddebd7ea9b6acfda6db348d1b784e9f`), its spec (`d545e46821dd42b1a86fa0c605cfda95ea63d697`), and both catalog templates. The composer preview uses a native input and the button uses a local microphone glyph. Speech recognition, ChatComposer, shared icon and translation integration remain pending.

All five Rust checks and four Chrome suites passed. SSR covered hidden unsupported state and decorative meter markup. Chrome covered start and stop names, keyboard activation, focus retention, the disabled unsupported state, and reduced motion. Published visual parity and manual assistive technology were not verified. The checklist remains open.
