import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8140'});
  await send('Page.bringToFront');
  await waitFor(`!!document.getElementById('avatar-overflow-action')`);
  assert.deepEqual(await evaluate(`['zero','large','action'].map(id => {
    const node = document.getElementById('avatar-overflow-' + id);
    return [node.tagName, node.textContent.trim(), node.getAttribute('aria-label'), node.offsetHeight,
      id === 'large' ? node.offsetWidth > 40 : node.offsetWidth === 40];
  })`), [
    ['SPAN', '+0', '0 more', 40, true],
    ['SPAN', '+4912', '4,912 more', 40, true],
    ['BUTTON', '+3', '3 more', 40, true],
  ]);
  assert.equal(await evaluate(`document.querySelectorAll('#avatar-group-overflow-examples [aria-label="12 more"]').length`), 2);
  await evaluate(`document.getElementById('avatar-overflow-action').focus()`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13, text: '\r'});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`document.getElementById('avatar-overflow-activations').textContent === '1'`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === '3 more'));
  assert.deepEqual(exceptions, []);
  console.log('AvatarGroupOverflow: counts, pill growth, custom names, and keyboard activation passed');
});
