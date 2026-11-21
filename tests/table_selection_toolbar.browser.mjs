import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8194'});
  const root = `document.getElementById('table-selection-toolbar-examples')`;
  const toolbar = `document.getElementById('table-bulk-actions')`;
  await waitFor(`${root}?.querySelectorAll('input').length === 3`);
  assert.equal(await evaluate(`!!${toolbar}`), false);
  await evaluate(`${root}.querySelectorAll('input')[0].click(); ${root}.querySelectorAll('input')[2].click()`);
  await waitFor(`${toolbar}?.textContent.includes('2 selected')`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(n => n.role?.value === 'toolbar' && n.name?.value === 'Bulk actions'));
  await evaluate(`${toolbar}.querySelector('button:last-child').focus()`);
  for (const type of ['keyDown', 'keyUp']) {
    await send('Input.dispatchKeyEvent', {type, key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13, ...(type === 'keyDown' && {text: '\r'})});
  }
  await waitFor(`!${toolbar} && ${root}.querySelectorAll('input:checked').length === 0`);
  assert.deepEqual(exceptions, []);
  console.log('TableSelectionToolbar: visible count, accessible name and keyboard clear passed');
});
