# Collapsible checkpoint

The standalone Collapsible supports controlled and uncontrolled disclosure. Its native button owns the expanded state and points to the retained content node. A disabled trigger is perceivable but skipped in the tab order and cannot toggle. A controlled close returns focus to the trigger when focus was inside the content. Leading chevrons mirror in RTL, and reduced motion removes the rotation transition.

Source at Meta Astryx `476a3153417dbf75c565f7df05b0784e94ddd59e`: `Collapsible.tsx` (`d38788c9503c4282eb81d231ace3ed285f42e050`), spec (`584e98f1f2f22d9a498770102610dbac798593c6`), and `useCollapsible.ts` (`d39c2eb9779d2ed9602262568c63d081e5230cd4`). The showcase adapts Without Card and adds a controlled RTL sample. [CollapsibleGroup](collapsible-group-port.md) owns grouped selection. Animated layout, shared touch press behavior, and the remaining catalog arrangements stay open.

SSR covers the trigger-to-panel relationship and retained hidden content. Chrome covers keyboard toggle, retained input state, controlled focus return, disabled behavior, RTL, reduced motion, and accessibility-tree exposure. Published visual parity and spoken output were not verified; the checklist remains open.
