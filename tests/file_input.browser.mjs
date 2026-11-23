import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8248'});
  const selector = '#file-input-examples input[name=attachment]';
  const input = `document.querySelector('${selector}')`;
  await waitFor(input);
  assert.equal(await evaluate(`${input}.labels[0].textContent.trim()`), 'Upload file ∙ Required');
  assert.ok(await evaluate(`${input}.validity.valueMissing && ${input}.accept === '.txt'`));
  assert.equal(await evaluate(`document.getElementById(${input}.getAttribute('aria-describedby')).textContent`), 'Choose a local file. This demo does not upload it.');
  const {root} = await send('DOM.getDocument');
  const {nodeId} = await send('DOM.querySelector', {nodeId: root.nodeId, selector});
  await send('DOM.setFileInputFiles', {nodeId, files: [fileURLToPath(new URL('./fixtures/file-input/sample.txt', import.meta.url))]});
  const count = "document.getElementById('file-input-count').textContent";
  await waitFor(`${count} === '1 selected'`);
  assert.ok(await evaluate(`${input}.validity.valid && new FormData(${input}.form).get('attachment').name === 'sample.txt'`));
  assert.ok(await evaluate(`${input}.form.elements.attachments.multiple && !new FormData(${input}.form).has('unavailable')`));
  await evaluate(`${input}.form.reset()`);
  assert.ok(await evaluate(`${input}.files.length === 0 && ${input}.validity.valueMissing`));
  assert.deepEqual(exceptions, []);
  console.log('FileInput: associated label and description, required validity, selection and form reset, and form behavior passed');
});
