import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8221'});
  const input = `document.getElementById('command-query')`;
  await waitFor(`${input}?.getAttribute('aria-label') === 'Search commands'`);
  assert.ok(await evaluate(`document.activeElement !== ${input}`));
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'textbox' && node.name?.value === 'Search commands'));
  await evaluate(`${input}.focus()`);
  await send('Input.insertText', {text: 'Open project'});
  await waitFor(`document.getElementById('command-query-value').textContent === 'Open project'`);
  assert.ok(await evaluate(`document.activeElement === ${input} && !${input}.hasAttribute('aria-expanded')`));
  assert.deepEqual(exceptions, []);
  console.log('CommandPaletteInput: input name, editing, retained focus, and standalone textbox semantics passed');
});
