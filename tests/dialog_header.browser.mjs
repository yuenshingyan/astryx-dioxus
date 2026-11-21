import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8180'});
  await waitFor(`!!document.getElementById('dialog-header-open')`);
  assert.deepEqual(await evaluate(`Array.from(document.querySelectorAll('#dialog-header-examples .astryx-dialog-header h2'), node => [node.textContent, node.tabIndex, node.hasAttribute('data-autofocus'), node === document.activeElement])`), [
    ['Invite teammates', -1, false, false],
    ['Edit Profile', -1, false, false],
  ]);
  await evaluate(`document.getElementById('dialog-header-open').click()`);
  await waitFor(`document.activeElement.id === 'dialog-header-test-title'`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'dialog' && node.name?.value === 'Details'));
  await evaluate(`document.querySelector('#dialog-header-examples [role="dialog"] button[aria-label="Close"]').click()`);
  await waitFor(`!document.getElementById('dialog-header-test-title')`);
  assert.deepEqual(exceptions, []);
  console.log('DialogHeader: inline focus suppression, title focus, dialog naming, and close passed');
});
