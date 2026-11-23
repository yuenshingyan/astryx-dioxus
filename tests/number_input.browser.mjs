import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8249'});
  const input = "document.querySelector('#number-input-examples input[name=quantity]')";
  const output = "document.getElementById('number-input-value').textContent";
  await waitFor(input);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'spinbutton' && node.name?.value === 'Quantity'));
  assert.equal(await evaluate(`document.getElementById(${input}.getAttribute('aria-describedby')).textContent`), 'Enter a whole number from 0 to 10.');
  await evaluate(`${input}.focus()`);
  for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key:'ArrowUp', code:'ArrowUp', windowsVirtualKeyCode:38});
  await evaluate(`${input}.blur()`);
  await waitFor(`${output} === '1'`);
  assert.deepEqual(await evaluate(`Array.from(new FormData(${input}.form))`), [['quantity', '1'], ['fixed', '3']]);
  assert.ok(await evaluate(`${input}.form.elements.fixed.readOnly && ${input}.form.elements.unavailable.disabled`));
  assert.ok(await evaluate(`${input}.value = '11'; ${input}.validity.rangeOverflow`));
  assert.ok(await evaluate(`${input}.value = '1.5'; ${input}.validity.stepMismatch`));
  await evaluate(`${input}.value = ''; ${input}.dispatchEvent(new Event('change', {bubbles:true}))`);
  await waitFor(`${output} === 'Empty'`);
  assert.deepEqual(exceptions, []);
  console.log('NumberInput: accessible name and description, keyboard step, bounds, clearing, and form behavior passed');
});
