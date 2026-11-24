import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8274'});
  const select = "document.querySelector('#multi-selector-examples select')";
  const output = "document.querySelector('#multi-selector-examples output').textContent";
  const values = `Array.from(${select}.selectedOptions, option => option.value)`;
  await waitFor(select);
  assert.ok(await evaluate(`${select}.multiple && ${select}.validity.valueMissing`));
  assert.match(await evaluate(`${select}.labels[0].textContent`), /Columns/);
  const choose = choices => evaluate(`(() => {
    for (const option of ${select}.options) option.selected = ${JSON.stringify(choices)}.includes(option.value);
    ${select}.dispatchEvent(new Event('change', {bubbles:true}));
  })()`);
  await choose(['Role']);
  await waitFor(`${output} === 'Role'`);
  await choose(['Name', 'Role']);
  await waitFor(`${output} === 'Role, Name'`);
  await choose(['Created']);
  await waitFor(`JSON.stringify(${values}) === '["Name","Role"]'`);
  assert.deepEqual(await evaluate(`new FormData(${select}.form).getAll('columns')`), ['Name', 'Role']);
  assert.deepEqual(exceptions, []);
  console.log('MultiSelector: native label, required state, selection order, rollback and repeated form values passed');
});
