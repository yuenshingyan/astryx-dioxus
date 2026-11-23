import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8251'});
  const input = "document.querySelector('#date-input-examples input[name=start]')";
  const output = "document.getElementById('date-input-value').textContent";
  await waitFor(input);
  assert.equal(await evaluate(`${input}.labels[0].textContent.trim()`), 'Start date ∙ Required');
  assert.equal(await evaluate(`document.getElementById(${input}.getAttribute('aria-describedby')).textContent`), 'Choose a date in 2024.');
  assert.ok(await evaluate(`${input}.validity.valueMissing`));
  assert.ok(await evaluate(`${input}.value = '2023-12-31'; ${input}.validity.rangeUnderflow`));
  assert.ok(await evaluate(`${input}.value = '2025-01-01'; ${input}.validity.rangeOverflow`));
  await evaluate(`${input}.value = '2024-02-29'; ${input}.dispatchEvent(new Event('change', {bubbles:true}))`);
  await waitFor(`${output} === '2024-02-29'`);
  assert.deepEqual(await evaluate(`Array.from(new FormData(${input}.form))`), [['start', '2024-02-29']]);
  await evaluate(`${input}.value = ''; ${input}.dispatchEvent(new Event('change', {bubbles:true}))`);
  await waitFor(`${output} === 'No date selected'`);
  assert.deepEqual(exceptions, []);
  console.log('DateInput: label, bounds, leap-day value, clearing and disabled form exclusion passed');
});
