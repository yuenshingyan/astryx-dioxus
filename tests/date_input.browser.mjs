import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8325'});
  const input = "document.querySelector('#date-input-examples input[name=start]')";
  const output = "document.getElementById('date-input-value').textContent";
  await waitFor(input);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  assert.equal(await evaluate(`${input}.labels[0].textContent.trim()`), 'Start date ∙ Required');
  assert.equal(await evaluate(`document.getElementById(${input}.getAttribute('aria-describedby')).textContent`), 'Choose a date in 2024.');
  assert.ok(await evaluate(`${input}.validity.valueMissing`));
  const changeDate = date => evaluate(`${input}.value = ${JSON.stringify(date)}; ${input}.dispatchEvent(new Event('change', {bubbles:true}))`);
  for (const date of ['2024-01-01', '2024-12-31']) {
    await changeDate(date);
    await waitFor(`${output} === ${JSON.stringify(date)}`);
    assert.ok(await evaluate(`${input}.validity.valid`));
  }
  for (const [date, flag] of [['2023-12-31', 'rangeUnderflow'], ['2025-01-01', 'rangeOverflow']]) {
    await changeDate(date);
    // Give a wrongly emitted change time to render before checking the controlled value.
    await evaluate('new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve())))');
    assert.ok(await evaluate(`${input}.validity.${flag}`));
    assert.equal(await evaluate(output), '2024-12-31');
  }
  await changeDate('2024-02-29');
  await waitFor(`${output} === '2024-02-29'`);
  assert.deepEqual(await evaluate(`Array.from(new FormData(${input}.form))`), [['start', '2024-02-29']]);
  const clear = `${input}.form.querySelector('button[aria-label="Clear Start date"]')`;
  assert.equal(await evaluate(`${input}.form.querySelectorAll('.astryx-input-clear-button').length`), 1);
  const toggle = "document.getElementById('date-input-toggle-loading')";
  const spinner = `${input}.parentElement.querySelector('.astryx-spinner[aria-hidden=true]')`;
  await evaluate(`${input}.focus(); ${toggle}.click()`);
  await waitFor(`${input}.getAttribute('aria-busy') === 'true' && ${spinner} && !${clear}`);
  assert.ok(await evaluate(`${input}.readOnly && !${input}.disabled && document.activeElement === ${input}`));
  assert.deepEqual(await evaluate(`Array.from(new FormData(${input}.form))`), [['start', '2024-02-29']]);
  await evaluate(`${input}.value = '2024-03-01'; ${input}.dispatchEvent(new Event('change', {bubbles:true}))`);
  assert.equal(await evaluate(output), '2024-02-29');
  await evaluate(`${input}.value = '2024-02-29'; ${toggle}.click()`);
  await waitFor(`!${input}.hasAttribute('aria-busy') && !${input}.readOnly && !${spinner} && ${clear}`);
  await evaluate(`window.nativeDateEvents = 0; for (const type of ['input','change']) ${input}.addEventListener(type, () => window.nativeDateEvents++); ${clear}.focus()`);
  for (const type of ['keyDown','keyUp']) await send('Input.dispatchKeyEvent', {type, key:'Enter', code:'Enter', windowsVirtualKeyCode:13, ...(type === 'keyDown' ? {text:'\r'} : {})});
  await waitFor(`${output} === 'No date selected' && document.activeElement === ${input} && !${clear}`);
  assert.ok(await evaluate(`${input}.validity.valueMissing && window.nativeDateEvents === 0`));
  const review = "document.querySelector('#date-input-status-example input')";
  assert.ok(await evaluate(`${review}.validity.valid && ${review}.getAttribute('aria-invalid') === 'true'`));
  assert.deepEqual(await evaluate(`${review}.getAttribute('aria-describedby').split(' ').map(id => document.getElementById(id).textContent.trim())`), ['Choose a working day.', 'This date falls on a weekend.']);
  assert.equal(await evaluate(`document.querySelector('#date-input-status-example .astryx-field-status').dataset.variant`), 'detached');
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.name?.value === 'Review date' && node.description?.value.includes('This date falls on a weekend.')));
  assert.deepEqual(exceptions, []);
  console.log('DateInput: label, bounds, leap-day value, clearing and disabled form exclusion passed');
});
