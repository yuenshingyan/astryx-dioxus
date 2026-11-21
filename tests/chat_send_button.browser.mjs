import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8182'});
  await waitFor(`!!document.getElementById('chat-send-action')`);
  assert.deepEqual(await evaluate(`Array.from(document.querySelectorAll('#chat-send-states button'), b => [b.getAttribute('aria-label'), b.getAttribute('aria-disabled') === 'true', b.dataset.variant, b.dataset.size])`), [
    ['Send', true, 'primary', 'md'], ['Send', false, 'primary', 'sm'], ['Stop', false, 'secondary', 'md'],
  ]);
  const action = `document.getElementById('chat-send-action')`;
  await evaluate(`${action}.focus()`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', text: '\r', windowsVirtualKeyCode: 13});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`${action}.getAttribute('aria-label') === 'Stop'`);
  assert.ok(await evaluate(`document.activeElement === ${action} && !${action}.hasAttribute('aria-disabled') && ${action}.querySelector('svg rect') !== null`));
  assert.equal(await evaluate(`document.getElementById('chat-send-result').textContent`), 'Send action ran first');
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === 'Stop'));
  await evaluate(`${action}.click()`);
  await waitFor(`${action}.getAttribute('aria-disabled') === 'true' && ${action}.getAttribute('aria-label') === 'Send'`);
  await evaluate(`${action}.click()`);
  assert.equal(await evaluate(`document.getElementById('chat-send-result').textContent`), 'Stop action ran first');
  assert.equal(await evaluate(`${action}.getAttribute('aria-label')`), 'Send');
  assert.deepEqual(exceptions, []);
  console.log('ChatSendButton: readiness, send/stop transition, keyboard activation and callback order passed');
});
