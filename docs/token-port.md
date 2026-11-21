# Token checkpoint

`src/token/` adds static, clickable, link, and link-with-remove tokens. The primary control and Remove are siblings. Disabled controls retain their tab stops, and disabled links have no destination. The component uses shared link rendering, URL validation, text direction, and press feedback. Custom color identifiers are reflected without built-in paint.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Token.tsx` (Git blob `174c7fa8759f0021bf1806df365fae950e029817`), `TokenLink.tsx`, the Token spec, and six catalog templates. Showcase icons are local illustrations. Tokenizer naming-context integration is not ported.

All five standard Rust checks passed. Chrome verified all eleven tone identifiers, primary and nested action separation, disabled focus and activation suppression, and ordinary link delegation. MetadataList and showcase navigation regressions passed. Modified clicks, router adapters, touch, published screenshot parity, and manual assistive technology were not independently verified. The migration checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8136 node tests/token.browser.mjs
```
