import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls:['https://*']});
  await send('Page.navigate', {url:process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8231'});
  const editor = `document.querySelector('#chat-composer-token-element-examples [contenteditable=true]')`;
  await waitFor(`${editor}?.querySelectorAll('[data-astryx-token]').length === 2`);
  const tokens = await evaluate(`Array.from(${editor}.querySelectorAll('[data-astryx-token]'), node => [node.dataset.astryxTokenValue, node.textContent, node.isContentEditable, node.tabIndex, getComputedStyle(node).display])`);
  assert.deepEqual(tokens, [['@ada', 'Ada', false, -1, 'inline-flex'], ['#design', 'Design', false, -1, 'inline-flex']]);
  assert.ok(await evaluate(`${editor}.querySelector('[data-astryx-token] strong')?.textContent === 'Design'`));
  assert.deepEqual(exceptions, []);
  console.log('ChatComposerTokenElement: exact values, noneditable wrappers, Badge and custom content passed');
});
