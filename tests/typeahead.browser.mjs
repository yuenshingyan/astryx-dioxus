import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8272'});
  const input = "document.querySelector('#typeahead-examples input[role=combobox]')";
  const list = `document.getElementById(${input}.getAttribute('aria-controls'))`;
  const clear = `document.querySelector('#typeahead-examples button[aria-label="Clear Fruit selection"]')`;
  const requests = `document.getElementById('typeahead-clear-requests').textContent`;
  const focused = `document.activeElement === ${input}`;
  const selectBanana = async () => {
    await send('Input.insertText', {text:'Ban'});
    await waitFor(`${input}.getAttribute('aria-expanded') === 'true' && ${list}.querySelectorAll('[role=option]').length === 1`);
    const option = `${list}.querySelector('[role=option]')`;
    assert.equal(await evaluate(`${option}.textContent.trim()`), 'Banana');
    await evaluate(`${option}.click()`);
    await waitFor(`${input}.getAttribute('aria-expanded') === 'false' && ${input}.value === 'Banana' && ${clear}`);
    assert.ok(await evaluate(focused));
  };
  await waitFor(input);
  assert.equal(await evaluate(`${input}.labels[0].textContent.trim()`), 'Fruit');
  assert.equal(await evaluate(`document.getElementById(${input}.getAttribute('aria-describedby')).textContent`), 'Choose a fruit from the results.');
  assert.equal(await evaluate(`${input}.placeholder`), 'Search fruits...');
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  await evaluate(`${input}.focus()`);
  assert.equal(await evaluate(`${input}.getAttribute('aria-expanded')`), 'false');
  assert.equal(await evaluate(`Boolean(${clear})`), false);
  await selectBanana();

  // Clearing a selection also discards its in-progress search draft.
  await evaluate(`${input}.select()`);
  await send('Input.insertText', {text:'Cher'});
  await waitFor(`${input}.value === 'Cher' && ${input}.getAttribute('aria-expanded') === 'true'`);
  const point = await evaluate(`(() => {
    ${clear}.scrollIntoView({block:'center'});
    const {x,y,width,height} = ${clear}.getBoundingClientRect();
    return {x:x+width/2,y:y+height/2};
  })()`);
  await send('Input.dispatchMouseEvent', {type:'mousePressed', ...point, button:'left', clickCount:1});
  assert.ok(await evaluate(focused));
  await send('Input.dispatchMouseEvent', {type:'mouseReleased', ...point, button:'left', clickCount:1});
  await waitFor(`${input}.value === '' && !${clear} && ${focused}`);
  assert.equal(await evaluate(`${input}.getAttribute('aria-expanded')`), 'false');
  assert.equal(await evaluate(requests), 'Clear requests: 1');

  // The caller can decline the same request from keyboard activation.
  await selectBanana();
  await evaluate(`document.getElementById('typeahead-keep-selection').click(); ${clear}.focus()`);
  const space = {key:' ', code:'Space', windowsVirtualKeyCode:32};
  await send('Input.dispatchKeyEvent', {type:'keyDown', ...space});
  await send('Input.dispatchKeyEvent', {type:'keyUp', ...space});
  await waitFor(`${requests} === 'Clear requests: 2' && ${focused}`);
  assert.ok(await evaluate(`${input}.value === 'Banana' && Boolean(${clear})`));
  assert.deepEqual(exceptions, []);
  console.log('Typeahead: Field association, local selection, and caller-controlled pointer and keyboard clearing passed');
});
