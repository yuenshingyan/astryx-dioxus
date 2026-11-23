import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8235'});
  const recent = `document.getElementById('context-menu-sort-recent')`;
  const name = `document.getElementById('context-menu-sort-name')`;
  await waitFor(`${recent}?.getAttribute('aria-checked') === 'true'`);
  assert.equal(await evaluate(`${name}.getAttribute('aria-checked')`), 'false');
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'group' && node.name?.value === 'Sort documents'));
  await evaluate(`${name}.click()`);
  await waitFor(`${name}.getAttribute('aria-checked') === 'true' && ${recent}.getAttribute('aria-checked') === 'false'`);
  const owner = `document.getElementById('context-menu-sort-owner')`;
  assert.deepEqual(await evaluate(`[${owner}.getAttribute('role'), ${owner}.getAttribute('aria-disabled'), ${owner}.tabIndex]`), ['menuitemradio', 'true', -1]);
  await evaluate(`${owner}.click()`);
  assert.equal(await evaluate(`${owner}.getAttribute('aria-checked')`), 'false');
  assert.equal(await evaluate(`${name}.getAttribute('aria-checked')`), 'true');
  assert.deepEqual(exceptions, []);
  console.log('ContextMenuRadioGroup: accessible group name and exclusive selection passed');
});
