import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8184'});
  await waitFor(`document.querySelectorAll('#chat-message-examples article').length === 12`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  for (const label of ['Agent', 'Message from user', 'Message from system']) {
    assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'article' && node.name?.value === label), label);
  }
  const geometry = await evaluate(`(() => {
    const agent = document.getElementById('chat-message-agent');
    const avatar = agent.firstElementChild;
    const column = agent.lastElementChild;
    const user = document.getElementById('chat-message-user');
    const body = user.querySelector('.astryx-chat-message__body');
    const endAligned = body.lastElementChild.getBoundingClientRect().right === body.getBoundingClientRect().right;
    user.dir = 'rtl';
    return [column.getBoundingClientRect().left - avatar.getBoundingClientRect().right,
      endAligned, body.lastElementChild.getBoundingClientRect().left === body.getBoundingClientRect().left,
      getComputedStyle(body).rowGap];
  })()`);
  assert.deepEqual(geometry, [12, true, true, '4px']);
  const row = `document.querySelector('#chat-message-reactive .astryx-chat-message-metadata')`;
  assert.equal(await evaluate(`${row}.dataset.user`), 'true');
  await evaluate(`document.getElementById('chat-message-toggle').click()`);
  await waitFor(`!${row}.hasAttribute('data-user')`);
  assert.deepEqual(exceptions, []);
  console.log('ChatMessage: article names, logical alignment, density, and reactive sender inheritance passed');
});
