# AlertDialog checkpoint

`src/alert_dialog/` composes the native Dialog and shared Button components. The modal associates the visible title and consequence text with its `alertdialog` role and focuses Cancel on opening. Escape and Cancel request dismissal; backdrop clicks do not. Confirm invokes the caller's action without closing. The inline preview uses a group instead of claiming modal semantics.

Pinned sources at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `AlertDialog/AlertDialog.tsx` (`1908a17c45312f923759b2f2b59c2be2f770d091`), its spec (`23746408f57df6d6f3950709c9e5ef3fd03d69e4`), and Delete Confirmation (`d8ff8dedfc161cea88815795de1959cc3813d740`). The sample uses caller-controlled state and logs a request instead of deleting data.

All five Rust checks and four Chrome suites passed. Chrome verified the alert name and description, initial Cancel focus, blocked backdrop dismissal, caller-owned action, Escape, and focus restoration. Narrow touch layout, transitions, imperative integration, the async catalog example, and published visual parity remain open.
