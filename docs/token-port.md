# Token checkpoint

`src/token/` adds static, clickable, link, and link-with-remove tokens. The primary control and Remove are siblings. Disabled controls retain their tab stops, and disabled links have no destination. The component uses shared link rendering, URL validation, text direction, and press feedback. Custom color identifiers are reflected without built-in paint.

Source: Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`, `Token.tsx` (Git blob `174c7fa8759f0021bf1806df365fae950e029817`), `TokenLink.tsx`, the Token spec, and six catalog templates. Showcase icons are local illustrations. Tokenizer naming-context integration is not ported.

At the initial checkpoint, all five standard Rust checks passed. Chrome verified all eleven tone identifiers, primary and nested action separation, disabled focus and activation suppression, and ordinary link delegation. MetadataList and showcase navigation regressions passed. Modified clicks, router adapters, touch, published screenshot parity, and manual assistive technology were not independently verified. The migration checklist remains open.

```sh
SHOWCASE_URL=http://127.0.0.1:8136 node tests/token.browser.mjs
```

## Follow-up: disabled destination in a button trigger context

The supplied `Token.tsx` and `Token.spec.md` were verified byte-for-byte against the pinned revision above (`Token.spec.md` Git blob `66dda03dd84f56365aa4f72c1438381adbdf25e1`). Under ORD1, a disabled destination yields to an inherited button role even without an explicit click callback. Token now renders its inner button with `aria-disabled="true"`, without removing its tab stop; an enabled destination still wins, and a disabled destination with no button role still renders as a focusable link without `href`.

`tests/token.rs` covers the rendered enabled and disabled structures. This follow-up does not verify browser focus, activation, or assistive technology; the checklist stays open.
