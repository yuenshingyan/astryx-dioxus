import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8232'});
  const rows = `document.querySelectorAll('#context-menu-item-examples [role=menuitem]')`;
  await waitFor(`${rows}.length === 3`);
  assert.ok(await evaluate(`${rows}[0].textContent.includes('Modify this item')`));
  assert.equal(await evaluate(`${rows}[2].tabIndex`), -1);
  await evaluate(`${rows}[2].click()`);
  await waitFor(`document.getElementById('context-menu-item-result').textContent === 'Delete requested'`);
  assert.deepEqual(exceptions, []);
  console.log('ContextMenuItem: shared action rows, secondary copy and activation passed');
});
