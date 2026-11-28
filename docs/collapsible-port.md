# Collapsible checkpoint

The standalone Collapsible supports controlled and uncontrolled disclosure. Its native button owns the expanded state and points to the retained content node. A disabled trigger is perceivable but skipped in the tab order and cannot toggle. Leading chevrons mirror in RTL, and reduced motion removes the rotation transition.

For the pinned spec’s AR4, a web close now reads the panel’s actual focus before applying `hidden`, rather than inferring it from bubbling events. After the close commits, focus returns to the trigger only if it was inside when the panel closed and hasn’t moved to another control. Opening does not move focus. Renderers without a browser DOM retain the existing event-based recovery.

Source at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Collapsible.tsx` (`d38788c9503c4282eb81d231ace3ed285f42e050`), spec (`584e98f1f2f22d9a498770102610dbac798593c6`), and `useCollapsible.ts` (`d39c2eb9779d2ed9602262568c63d081e5230cd4`). The showcase adapts Without Card and adds a controlled RTL sample. [CollapsibleGroup](collapsible-group-port.md) owns grouped selection. Animated layout, shared touch press behavior, and the remaining catalog arrangements stay open.

SSR covers the trigger-to-panel relationship and retained hidden content. The existing Chrome suite covers keyboard toggle, retained input state, controlled focus return, disabled behavior, RTL, reduced motion, and accessibility-tree exposure. It now includes regression cases for owner-set close when a child stops `focusin` or `focusout` from bubbling, and for preserving outside focus on open. The new cases were not run in a browser for this follow-up. Published visual parity and spoken output remain unverified; the checklist remains open.
