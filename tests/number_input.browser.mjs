import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8323'});
  const input = "document.querySelector('#number-input-examples input[name=quantity]')";
  const output = "document.getElementById('number-input-value').textContent";
  await waitFor(input);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'spinbutton' && node.name?.value === 'Quantity'));
  assert.equal(await evaluate(`document.getElementById(${input}.getAttribute('aria-describedby')).textContent`), 'Enter a whole number from 0 to 10.');
  const budget = nodes.find(node => !node.ignored && node.role?.value === 'spinbutton' && node.name?.value === 'Budget');
  assert.match(budget?.description?.value ?? '', /Enter the proposed budget\..*Must be a positive amount/);
  assert.deepEqual(await evaluate(`Array.from(document.querySelectorAll('#number-input-statuses input'), el => {
    const message = el.closest('.astryx-field').querySelector('.astryx-field-status');
    const isDescribed = el.getAttribute('aria-describedby').split(' ').includes(message.id);
    return [el.getAttribute('aria-invalid'), el.checkValidity(), isDescribed && message.textContent.trim(), message.dataset.variant];
  })`), [
    ['true', true, 'Must be a positive amount', 'attached'],
    [null, true, 'Exceeds typical team size', 'detached'],
    [null, true, 'On track', 'attached'],
  ]);
  const completion = nodes.find(node => !node.ignored && node.role?.value === 'spinbutton' && node.name?.value === 'Completion');
  assert.match(completion?.description?.value ?? '', /On track.*%/);
  const entry = "document.querySelector('#number-input-statuses .astryx-number-input__entry[data-units]')";
  assert.deepEqual(await evaluate(`[${entry}.querySelector('input').valueAsNumber, ${entry}.querySelector('.astryx-number-input__units').textContent]`), [25, '%']);
  const isSuffixOnRight = `${entry}.querySelector('input').getBoundingClientRect().right <= ${entry}.querySelector('span').getBoundingClientRect().left`;
  assert.ok(await evaluate(isSuffixOnRight));
  await evaluate(`${entry}.dir = 'rtl'`);
  assert.ok(await evaluate(`${entry}.querySelector('span').getBoundingClientRect().right <= ${entry}.querySelector('input').getBoundingClientRect().left`));
  const clear = `${input}.form.querySelector('button[aria-label="Clear Quantity"]')`;
  assert.equal(await evaluate(`${input}.form.querySelectorAll('.astryx-input-clear-button').length`), 1);
  assert.equal(await evaluate(`${input}.valueAsNumber`), 0);
  await evaluate(`window.clearEvents = 0; for (const type of ['input','change']) ${input}.addEventListener(type, () => window.clearEvents++); ${clear}.focus()`);
  for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key:'Enter', code:'Enter', windowsVirtualKeyCode:13, ...(type === 'keyDown' ? {text:'\r'} : {})});
  await waitFor(`${output} === 'Empty' && ${input}.value === '' && !${clear} && document.activeElement === ${input}`);
  assert.equal(await evaluate('window.clearEvents'), 0);
  await evaluate(`${input}.value = '0'; ${input}.dispatchEvent(new Event('change', {bubbles:true}))`);
  await waitFor(`${output} === '0' && ${clear}`);
  await evaluate(`${input}.focus()`);
  for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key:'ArrowUp', code:'ArrowUp', windowsVirtualKeyCode:38});
  await evaluate(`${input}.blur()`);
  await waitFor(`${output} === '1'`);
  await evaluate(`${input}.focus()`);
  // CDP modifier bits: Alt (1), Ctrl (2), Meta (4), Shift (8).
  for (const modifiers of [1, 2, 4, 8]) {
    for (const [key, windowsVirtualKeyCode] of [['ArrowUp', 38], ['ArrowDown', 40]]) {
      for (const type of ['keyDown', 'keyUp']) {
        await send('Input.dispatchKeyEvent', {type, key, code: key, windowsVirtualKeyCode, modifiers});
      }
      assert.equal(await evaluate(`${input}.valueAsNumber`), 1, `${key} with modifier ${modifiers}`);
    }
  }
  await evaluate(`${input}.blur()`);
  assert.equal(await evaluate(output), '1');
  assert.deepEqual(await evaluate(`Array.from(new FormData(${input}.form))`), [['quantity', '1'], ['fixed', '3']]);
  assert.ok(await evaluate(`${input}.form.elements.fixed.readOnly && ${input}.form.elements.unavailable.disabled`));
  assert.ok(await evaluate(`${input}.value = '11'; ${input}.validity.rangeOverflow`));
  assert.ok(await evaluate(`${input}.value = '1.5'; ${input}.validity.stepMismatch`));
  await evaluate(`${input}.value = ''; ${input}.dispatchEvent(new Event('change', {bubbles:true}))`);
  await waitFor(`${output} === 'Empty'`);
  assert.deepEqual(exceptions, []);
  console.log('NumberInput: accessible name and description, keyboard step, bounds, clearing, and form behavior passed');
});
