import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8272'});
  const input = "document.querySelector('#typeahead-examples input[role=combobox]')";
  const list = `document.getElementById(${input}.getAttribute('aria-controls'))`;
  await waitFor(input);
  assert.equal(await evaluate(`${input}.labels[0].textContent.trim()`), 'Fruit');
  assert.equal(await evaluate(`document.getElementById(${input}.getAttribute('aria-describedby')).textContent`), 'Choose a fruit from the results.');
  assert.equal(await evaluate(`${input}.placeholder`), 'Search fruits...');
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  await evaluate(`${input}.focus()`);
  assert.equal(await evaluate(`${input}.getAttribute('aria-expanded')`), 'false');
  await send('Input.insertText', {text:'Ban'});
  await waitFor(`${input}.getAttribute('aria-expanded') === 'true' && ${list}.querySelectorAll('[role=option]').length === 1`);
  const option = `${list}.querySelector('[role=option]')`;
  assert.equal(await evaluate(`${option}.textContent.trim()`), 'Banana');
  await evaluate(`${option}.click()`);
  await waitFor(`${input}.getAttribute('aria-expanded') === 'false' && ${input}.value === 'Banana'`);
  assert.equal(await evaluate(`document.activeElement === ${input}`), true);
  assert.deepEqual(exceptions, []);
  console.log('Typeahead: field label, description, local filtering, selection and focus retention passed');
});
