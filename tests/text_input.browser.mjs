import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8215'});
  const input = `document.getElementById('name-input')`;
  await waitFor(`${input}?.labels.length === 1`);
  await evaluate(`${input}.focus()`);
  await send('Input.insertText', {text: 'Sam'});
  await waitFor(`document.getElementById('name-input-preview').textContent === 'Sam'`);
  assert.equal(await evaluate(`${input}.value`), 'Sam');
  assert.ok(await evaluate(`document.activeElement === ${input}`));
  assert.equal(await evaluate(`document.querySelector('#text-input-examples input[type=password]').autocomplete`), 'new-password');
  assert.deepEqual(await evaluate(`Array.from(new FormData(document.getElementById('text-input-form')))`), [['reference', 'Reference']]);
  await evaluate(`document.getElementById('disabled-text-input').focus()`);
  await send('Input.insertText', {text: 'blocked'});
  assert.deepEqual(await evaluate(`[document.activeElement.id, document.activeElement.value, document.activeElement.getAttribute('aria-disabled')]`), ['disabled-text-input', 'Unavailable', 'true']);
  const {nodes} = await send('Accessibility.getFullAXTree');
  const email = nodes.find(node => !node.ignored && node.role?.value === 'textbox' && node.name?.value === 'Email field');
  assert.match(email?.description?.value ?? '', /Use a work email.*Enter a valid email address/);
  assert.deepEqual(exceptions, []);
  console.log('TextInput: editing, accessible labeling and validation, and disabled form behavior passed');
});
