import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8261'});
  const root = `document.querySelector('#date-range-input-examples fieldset')`;
  const input = index => `${root}.querySelectorAll('input')[${index}]`;
  const output = `document.getElementById('date-range-value').textContent`;
  const status = `${root}.querySelector('[role=status]').textContent`;
  const change = (index, value) => evaluate(`${input(index)}.value = ${JSON.stringify(value)}; ${input(index)}.dispatchEvent(new Event('change', {bubbles:true}))`);
  await waitFor(`${root} && ${input(1)}`);
  assert.equal(await evaluate(`${root}.querySelector('legend').textContent`), 'Date range');
  assert.deepEqual(await evaluate(`Array.from(${root}.querySelectorAll('input'), node => node.labels[0].textContent)`), ['Start date', 'End date']);
  await change(1, '2026-04-10');
  assert.equal(await evaluate(output), 'No range selected');
  await change(0, '2026-04-15');
  await waitFor(`${status}.includes('allowed bounds')`);
  assert.equal(await evaluate(output), 'No range selected');
  await change(0, '2025-12-31');
  assert.ok(await evaluate(`${input(0)}.validity.rangeUnderflow`));
  await change(0, '2026-04-10');
  await waitFor(`${output} === '2026-04-10 to 2026-04-10' && ${status} === ''`);
  await change(1, '');
  await waitFor(`${output} === 'No range selected'`);
  assert.deepEqual(exceptions, []);
  console.log('DateRangeInput: named endpoints, end-first draft, order and bounds, inclusive range and clearing passed');
});
