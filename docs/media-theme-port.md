# MediaTheme checkpoint

`src/theme/media_theme.rs` provides explicit dark and light surfaces, automatic mode, and an off mode. The wrapper uses `display: contents` and stays mounted across mode changes. The browser hook measures the parent, composites translucent CSS backgrounds over the first opaque ancestor, and keeps ambient colors when they meet the upstream selection threshold. Image and gradient backdrops use the configured fallback; the hook does not sample image pixels. The server renders the configured fallback for automatic mode.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `theme/MediaTheme.tsx` (`2a702ea52be10b9ef44dd0ae41ed67a356b75b4e`), `hooks/useAutoMediaMode.ts` (`2cdb6fb4c662959252f9ef30bc5f0c68e15256b1`), `onMediaTokens.ts`, `mediaSide.ts`, and three catalog templates. The showcase uses their pinned offline images; the media overlay omits the source icon. Theme generation and component-specific on-media theme overrides are outside this checkpoint. The private Toast measurement path is unchanged.

All five standard Rust checks and the MediaTheme and showcase navigation Chrome suites passed. Chrome verified explicit tokens, automatic ambient suppression, gradient fallback, offline images, and preservation of child identity. Color measurement uses clamped sRGB canvas values, and the selection threshold is not an accessibility certification. Published screenshot parity, other browsers, and manual assistive technology were not verified. The checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8147 node tests/media_theme.browser.mjs
```
