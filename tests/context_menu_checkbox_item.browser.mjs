import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8234'});
  const item = "document.getElementById('context-menu-checkbox-details')";
  await waitFor(`${item}?.getAttribute('aria-checked') === 'false'`);
  assert.deepEqual(await evaluate(`[${item}.getAttribute('role'), ${item}.tabIndex]`), ['menuitemcheckbox', -1]);
  await evaluate(`${item}.click()`);
  await waitFor(`${item}.getAttribute('aria-checked') === 'true'`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  const checkbox = nodes.find(node => !node.ignored && node.role?.value === 'menuitemcheckbox' && node.name?.value === 'Show details');
  assert.equal(checkbox?.properties.find(property => property.name === 'checked')?.value.value, 'true');
  await evaluate(`${item}.click()`);
  await waitFor(`${item}.getAttribute('aria-checked') === 'false'`);
  assert.deepEqual(exceptions, []);
  console.log('ContextMenuCheckboxItem: controlled checked state, accessible name, and menu semantics passed');
});
