import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8276'});
  const input = "document.querySelector('#tokenizer-examples [role=combobox]')";
  const list = `document.getElementById(${input}.getAttribute('aria-controls'))`;
  const values = `JSON.stringify(new FormData(${input}.form).getAll('tags'))`;
  const press = async (key, windowsVirtualKeyCode, text) => {
    await send('Input.dispatchKeyEvent', {type:'keyDown', key, code:key, windowsVirtualKeyCode, text});
    await send('Input.dispatchKeyEvent', {type:'keyUp', key, code:key, windowsVirtualKeyCode});
  };
  await waitFor(input);
  assert.equal(await evaluate(`${input}.labels[0].textContent.trim()`), 'Tags');
  assert.equal(await evaluate(values), '["1","2"]');
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  await evaluate(`${input}.focus()`);
  await waitFor(`${input}.getAttribute('aria-expanded') === 'true'`);
  assert.equal(await evaluate(`${list}.querySelectorAll('[role=option]').length`), 4);
  await send('Input.insertText', {text:'Mark'});
  await waitFor(`${list}.querySelectorAll('[role=option]').length === 1`);
  await press('ArrowDown', 40);
  await press('Enter', 13, '\r');
  await waitFor(`${values} === '["1","2","3"]' && ${input}.value === ''`);
  const remove = "document.querySelector('#tokenizer-examples [aria-label=\"Remove Design\"]')";
  await waitFor(remove);
  await evaluate(`${remove}.focus()`);
  await press('Enter', 13, '\r');
  await waitFor(`${values} === '["2","3"]' && document.activeElement === ${input}`);
  assert.deepEqual(exceptions, []);
  console.log('Tokenizer: labeled input, filtered choices, add/remove, repeated form values, and focus return passed');
});
