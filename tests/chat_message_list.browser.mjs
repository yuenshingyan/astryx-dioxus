import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8186'});
  await waitFor(`!!document.getElementById('chat-list-top')`);
  const geometry = await evaluate(`(() => {
    const top = document.getElementById('chat-list-top');
    const bottom = document.getElementById('chat-list-bottom');
    const y = root => {
      const r = document.createRange();
      r.selectNode(root.firstElementChild.lastChild);
      return r.getBoundingClientRect().top - root.getBoundingClientRect().top;
    };
    const insets = [...document.querySelectorAll('#chat-message-list-examples [data-density][role="log"]')].slice(0,3)
      .map(node => [getComputedStyle(node.firstElementChild).paddingBlockStart, getComputedStyle(node.firstElementChild).gap]);
    return {restsLower: y(bottom) > y(top) + 40, insets, gap: getComputedStyle(top.firstElementChild).gap, tabIndex: top.tabIndex};
  })()`);
  assert.deepEqual(geometry, {restsLower: true, insets: [['16px','16px'], ['8px','8px'], ['24px','24px']], gap: '4px', tabIndex: 0});
  await evaluate(`document.getElementById('chat-list-toggle').click()`);
  await waitFor(`document.getElementById('chat-list-top').getAttribute('aria-busy') === 'true'`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'log' && node.properties?.some(p => p.name === 'live' && p.value?.value === 'polite')));
  await evaluate(`document.getElementById('chat-list-toggle').click()`);
  await waitFor(`!document.getElementById('chat-list-top').hasAttribute('aria-busy')`);
  assert.ok(await evaluate(`document.getElementById('chat-list-empty').textContent.includes('No messages yet')`));
  assert.deepEqual(exceptions, []);
  console.log('ChatMessageList: log semantics, busy transitions, alignment, density and empty content passed');
});
