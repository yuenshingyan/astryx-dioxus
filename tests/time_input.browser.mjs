import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8301'});
  const input = "document.querySelector('#time-input-examples input[name=time]')";
  const output = "document.getElementById('time-input-value').textContent";
  await waitFor(input);
  assert.equal(await evaluate(`${input}.labels[0].textContent`), 'Time');
  assert.equal(await evaluate(`document.getElementById(${input}.getAttribute('aria-describedby')).textContent`), 'Choose a time from 09:00 to 17:00.');
  assert.ok(await evaluate(`${input}.value = '08:59'; ${input}.validity.rangeUnderflow`));
  assert.ok(await evaluate(`${input}.value = '17:01'; ${input}.validity.rangeOverflow`));
  assert.ok(await evaluate(`${input}.value = '12:00:01'; ${input}.validity.stepMismatch`));
  await evaluate(`${input}.value = '12:34'; ${input}.dispatchEvent(new Event('change', {bubbles:true}))`);
  await waitFor(`${output} === '12:34'`);
  assert.deepEqual(await evaluate(`Array.from(new FormData(${input}.form))`), [['time', '12:34'], ['precise', '12:34:56']]);
  await evaluate(`${input}.value = ''; ${input}.dispatchEvent(new Event('change', {bubbles:true}))`);
  await waitFor(`${output} === 'No time selected'`);
  const review = "document.querySelector('#time-input-status-example input')";
  assert.ok(await evaluate(`${review}.validity.valid && ${review}.getAttribute('aria-invalid') === 'true'`));
  assert.deepEqual(await evaluate(`${review}.getAttribute('aria-describedby').split(' ').map(id => document.getElementById(id).textContent.trim())`), ['Choose a time outside the lunch break.', 'This time overlaps the lunch break.']);
  assert.equal(await evaluate(`document.querySelector('#time-input-status-example .astryx-field-status').dataset.variant`), 'detached');
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.name?.value === 'Review time' && node.description?.value.includes('This time overlaps the lunch break.')));
  assert.deepEqual(exceptions, []);
  console.log('TimeInput: label, description, bounds, minute/second precision and form behavior passed');
});
