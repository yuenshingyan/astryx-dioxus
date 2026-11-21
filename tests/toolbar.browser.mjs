import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8193'});
  await waitFor(`document.getElementById('toolbar-share')?.tabIndex === -1`);
  assert.deepEqual(await evaluate(`Array.from(document.querySelectorAll('#toolbar-three button'), b => b.tabIndex)`), [0, -1, -1]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(n => n.role?.value === 'toolbar' && n.name?.value === 'Document actions'));
  const press = async (key, windowsVirtualKeyCode) => {
    for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key, code: key, windowsVirtualKeyCode});
  };
  await evaluate(`document.getElementById('toolbar-edit').focus()`);
  await press('ArrowRight', 39);
  await waitFor(`document.activeElement.id === 'toolbar-share'`);
  await evaluate(`document.getElementById('toolbar-three').dir = 'rtl'`);
  await press('ArrowLeft', 37);
  await waitFor(`document.activeElement.id === 'toolbar-export'`);
  const vertical = `document.querySelector('[aria-label="Layout actions"]')`;
  await evaluate(`${vertical}.querySelector('button').focus()`);
  await press('ArrowDown', 40);
  assert.equal(await evaluate(`document.activeElement.textContent.trim()`), 'Distribute');
  assert.deepEqual(exceptions, []);
  console.log('Toolbar: named role, roving tab stop, RTL and vertical navigation passed');
});
