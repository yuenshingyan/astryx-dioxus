import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8181'});
  await waitFor(`document.querySelectorAll('#chat-system-message-examples .astryx-chat-system-message').length === 20`);
  const measured = await evaluate(`(() => {
    const section = document.querySelector('#chat-system-message-examples');
    const rows = [...section.querySelectorAll('.astryx-chat-system-message')];
    const row = rows.find(r => r.querySelector('svg'));
    const icon = row.querySelector('svg');
    const before = icon.getBoundingClientRect().width;
    const wideHeight = row.getBoundingClientRect().height;
    row.style.width = '125px';
    row.dir = 'rtl';
    const box = row.getBoundingClientRect();
    return {
      passive: rows.every(r => !r.hasAttribute('role') && !r.hasAttribute('aria-live') && !r.hasAttribute('tabindex')),
      dates: rows.filter(r => r.dataset.variant === 'divider' && r.querySelector('[role="separator"]')).length,
      wrapped: box.height > wideHeight && row.scrollWidth === row.clientWidth,
      sameIcon: before > 0 && before === icon.getBoundingClientRect().width,
      rtlIcon: icon.getBoundingClientRect().right === row.firstElementChild.getBoundingClientRect().right,
    };
  })()`);
  assert.deepEqual(measured, {passive: true, dates: 7, wrapped: true, sameIcon: true, rtlIcon: true});
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'separator' && node.name?.value === 'Today'));
  assert.deepEqual(exceptions, []);
  console.log('ChatSystemMessage: four arrangements, named separators, passive rows and narrow RTL wrapping passed');
});
