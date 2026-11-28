import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8308'});
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
  assert.deepEqual(await evaluate(`Array.from(${root}.querySelectorAll('input'), el => [el.getAttribute('aria-invalid'), document.getElementById(el.getAttribute('aria-describedby'))?.textContent, el.validity.valid])`), [
    ['true', 'Enter dates in order and within the allowed bounds.', true],
    ['true', 'Enter dates in order and within the allowed bounds.', true],
  ]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  for (const name of ['Start date', 'End date']) {
    assert.ok(nodes.some(node => !node.ignored && node.name?.value === name && node.description?.value.includes('allowed bounds')));
  }
  assert.equal(await evaluate(output), 'No range selected');
  await change(0, '2025-12-31');
  assert.ok(await evaluate(`${input(0)}.validity.rangeUnderflow`));
  await change(0, '2026-04-10');
  await waitFor(`${output} === '2026-04-10 to 2026-04-10' && ${status} === ''`);
  assert.ok(await evaluate(`Array.from(${root}.querySelectorAll('input')).every(el => !el.hasAttribute('aria-invalid') && !el.hasAttribute('aria-describedby'))`));
  await change(1, '2026-04-17');
  await waitFor(`${status} === 'Choose no more than seven days.'`);
  assert.equal(await evaluate(output), '2026-04-10 to 2026-04-10');
  assert.ok(await evaluate(`Array.from(${root}.querySelectorAll('input')).every(el => el.getAttribute('aria-invalid') === 'true' && el.validity.valid)`));
  await change(1, '2026-04-16');
  await waitFor(`${output} === '2026-04-10 to 2026-04-16' && ${status} === ''`);
  await change(1, '');
  await waitFor(`${output} === 'No range selected'`);
  assert.deepEqual(exceptions, []);
  console.log('DateRangeInput: named endpoints, end-first draft, order and bounds, inclusive range and clearing passed');
});
