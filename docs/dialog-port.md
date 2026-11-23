# Dialog checkpoint

`src/dialog/component.rs` implements caller-controlled native modal dialogs and an inline preview mode. Info dialogs request dismissal on Escape and backdrop clicks, Form dialogs allow Escape only, and Required dialogs use `alertdialog` and block implicit dismissal. The modal controller focuses the marked DialogHeader title on open when present. Names can reference the caller's title ID.

Pinned sources at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Dialog/Dialog.tsx` (`efec753325d7a669d8092dc1fd1884269471d558`), its spec (`2f82cca6261c0a874de1312eb4feff5c56dd655d`), and the adapted showcase (`0b7169098fc244db1ebd22c10267f3e0722faef7`). The preview makes the composed showcase interactive and adds a required-action example.

All five Rust checks and four Chrome suites passed. The focused Chrome suite verified title focus and naming, Escape, backdrop dismissal, required-action behavior, and native focus restoration. Custom positioning, animated transitions, nested-modal and scroll-lock integration, the remaining catalog arrangements, and published visual parity remain open.
