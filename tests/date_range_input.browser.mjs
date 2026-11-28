import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8328'});
  const root = `document.querySelector('#date-range-input-examples fieldset')`;
  const input = index => `${root}.querySelectorAll('input')[${index}]`;
  const output = `document.getElementById('date-range-value').textContent`;
  const status = `${root}.querySelector(':scope > [role=status]').textContent`;
  const change = (index, value) => evaluate(`${input(index)}.value = ${JSON.stringify(value)}; ${input(index)}.dispatchEvent(new Event('change', {bubbles:true}))`);
  await waitFor(`${root} && ${input(1)}`);
  assert.equal(await evaluate(`${root}.querySelector('legend').textContent`), 'Date range');
  assert.deepEqual(await evaluate(`Array.from(${root}.querySelectorAll('input'), node => node.labels[0].textContent)`), ['Start date', 'End date']);
  const clear = `${root}.querySelector('button[aria-label="Clear Date range"]')`;
  assert.equal(await evaluate(clear), null);
  await change(1, '2026-04-10');
  assert.equal(await evaluate(clear), null);
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
  await change(0, '2026-04-10');
  await change(1, '2026-04-16');
  await waitFor(clear);
  await change(1, '2026-04-17');
  await waitFor(`${status} === 'Choose no more than seven days.'`);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  const toggle = "document.getElementById('date-range-toggle-loading')";
  const endpoints = `${root}.querySelectorAll('input[type=date]')`;
  await evaluate(`${input(0)}.focus(); ${toggle}.click()`);
  await waitFor(`Array.from(${endpoints}).every(el => el.readOnly && el.getAttribute('aria-busy') === 'true') && !${clear}`);
  assert.ok(await evaluate(`document.activeElement === ${input(0)} && !${root}.disabled`));
  assert.equal(await evaluate(`${root}.querySelectorAll('.astryx-spinner[aria-hidden=true]').length`), 2);
  await change(1, '2026-04-15');
  assert.equal(await evaluate(output), '2026-04-10 to 2026-04-16');
  assert.equal(await evaluate(status), 'Choose no more than seven days.');
  await evaluate(`${input(1)}.value = '2026-04-17'; ${toggle}.click()`);
  await waitFor(`Array.from(${endpoints}).every(el => !el.readOnly && !el.hasAttribute('aria-busy')) && ${clear} && !${root}.querySelector('.astryx-spinner')`);
  await evaluate(`window.clearInputEvents = 0; for (const el of ${root}.querySelectorAll('input')) for (const type of ['input','change']) el.addEventListener(type, () => window.clearInputEvents++); ${clear}.focus()`);
  for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key:'Enter', code:'Enter', windowsVirtualKeyCode:13, ...(type === 'keyDown' ? {text:'\r'} : {})});
  await waitFor(`${output} === 'No range selected' && ${input(0)}.value === '' && ${input(1)}.value === '' && ${status} === '' && !${clear} && document.activeElement === ${input(0)}`);
  assert.equal(await evaluate('window.clearInputEvents'), 0);
  assert.deepEqual(exceptions, []);
  console.log('DateRangeInput: named endpoints, end-first draft, order and bounds, inclusive range and clearing passed');
});
