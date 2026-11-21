import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8156'});
  const button = key => `document.getElementById('toggle-${key}')`;
  const pressed = key => `${button(key)}.getAttribute('aria-pressed')`;
  await waitFor(`${button('grid')}?.getAttribute('aria-pressed') === 'true'`);
  await evaluate(`${button('list')}.focus()`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', text: '\r', windowsVirtualKeyCode: 13});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`${pressed('list')} === 'true' && ${pressed('grid')} === 'false'`);
  assert.equal(await evaluate(`document.activeElement === ${button('list')}`), true);
  await evaluate(`${button('list')}.click()`);
  await waitFor(`${pressed('list')} === 'false'`);
  await evaluate(`${button('italic')}.click()`);
  await waitFor(`${pressed('bold')} === 'true' && ${pressed('italic')} === 'true'`);
  await evaluate(`${button('bold')}.click()`);
  await waitFor(`${pressed('bold')} === 'false' && ${pressed('italic')} === 'true'`);
  assert.deepEqual(await evaluate(`[
    getComputedStyle(document.getElementById('toggle-view')).flexWrap,
    ${button('grid')}.dataset.size, ${button('table')}.dataset.size,
    getComputedStyle(document.getElementById('toggle-disabled-group')).flexDirection,
    [...document.querySelectorAll('#toggle-disabled-group button')].map(node => node.getAttribute('aria-disabled'))
  ]`), ['wrap', 'sm', 'md', 'column', ['true', 'true']]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'group' && node.name?.value === 'View mode'));
  assert.deepEqual(exceptions, []);
  console.log('ToggleButtonGroup: single deselection, multi-select, focus, sizing, and disabled inheritance passed');
});
