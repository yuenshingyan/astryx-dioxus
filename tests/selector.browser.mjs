import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8273'});
  const control = "document.querySelector('#selector-examples select')";
  await waitFor(control);
  assert.match(await evaluate(`${control}.labels[0].textContent`), /Fruit/);
  assert.ok(await evaluate(`${control}.validity.valueMissing`));
  const choose = value => evaluate(`${control}.value = ${JSON.stringify(value)}; ${control}.dispatchEvent(new Event('change', {bubbles:true}))`);
  await choose('Banana');
  await waitFor(`${control}.dataset.astryxSelectorValue === 'Banana' && ${control}.value === 'Banana'`);
  await choose('Mango');
  await waitFor(`${control}.value === 'Banana'`);
  assert.deepEqual(await evaluate(`Array.from(new FormData(${control}.form))`), [['fruit', 'Banana']]);
  assert.deepEqual(exceptions, []);
  console.log('Selector: native label, required validity, controlled acceptance and rollback, and form value passed');
});
