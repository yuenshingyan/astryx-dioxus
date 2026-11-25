import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8286'});
  const section = "document.getElementById('bottom-sheet-switcher-examples')";
  const dialogs = `${section}.querySelectorAll('dialog:modal')`;
  const active = `${section}.querySelector('dialog:modal')`;
  const click = async (label, root = section) => evaluate(`[...${root}.querySelectorAll('button')].find(node => node.textContent.trim() === ${JSON.stringify(label)}).click()`);
  const expectSheet = async label => {
    await waitFor(`${dialogs}.length === 1 && ${active}.getAttribute('aria-label') === ${JSON.stringify(label)}`);
    assert.ok(await evaluate(`${active}.contains(document.activeElement)`));
  };
  await waitFor(`${section}?.querySelectorAll('dialog').length === 2`);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  assert.equal(await evaluate(`${dialogs}.length`), 0);
  await click('Review settings');
  await expectSheet('Review notification settings');
  await click('Continue', active);
  await expectSheet('Confirm settings');
  await click('Back', active);
  await expectSheet('Review notification settings');
  for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key:'Escape', code:'Escape', windowsVirtualKeyCode:27});
  await waitFor(`${dialogs}.length === 0`);
  await click('Review settings');
  await expectSheet('Review notification settings');
  await click('Continue', active);
  await expectSheet('Confirm settings');
  await click('Save', active);
  await waitFor(`${dialogs}.length === 0 && ${section}.querySelector('output').textContent === 'Settings saved for this demo'`);
  assert.deepEqual(exceptions, []);
  console.log('BottomSheetSwitcher: forward and back, active modal focus, Escape and Save passed');
});
