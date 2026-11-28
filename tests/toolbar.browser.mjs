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

  // Home and End should land on the first and last enabled controls, respectively.
  await evaluate(`document.getElementById('toolbar-three').removeAttribute('dir')`);
  for (const [key, code, disabled, start] of [
    ['Home', 36, 'toolbar-edit', 'toolbar-export'],
    ['End', 35, 'toolbar-export', 'toolbar-edit'],
  ]) {
    await evaluate(`{
      document.getElementById('${disabled}').disabled = true;
      document.getElementById('${start}').focus();
    }`);
    assert.equal(await evaluate(`document.activeElement.id`), start);
    await press(key, code);
    await waitFor(`document.activeElement.id === 'toolbar-share'`);
    assert.deepEqual(await evaluate(`Array.from(document.querySelectorAll('#toolbar-three button'))
      .filter(button => !button.disabled && button.tabIndex === 0).map(button => button.id)`), ['toolbar-share']);
    await evaluate(`document.getElementById('${disabled}').disabled = false`);
  }

  const vertical = `document.querySelector('[aria-label="Layout actions"]')`;
  await evaluate(`${vertical}.querySelector('button').focus()`);
  await press('ArrowDown', 40);
  assert.equal(await evaluate(`document.activeElement.textContent.trim()`), 'Distribute');
  assert.deepEqual(exceptions, []);
  console.log('Toolbar: named role, roving tab stop, RTL, Home/End and vertical navigation passed');
});
