import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8302'});
  const form = "document.querySelector('#date-time-input-examples form')";
  const date = `${form}.querySelector('input[type=date]')`;
  const time = `${form}.querySelector('input[type=time]')`;
  const output = "document.getElementById('date-time-input-value').textContent";
  const change = (input, value) => evaluate(`${input}.value = ${JSON.stringify(value)}; ${input}.dispatchEvent(new Event('change', {bubbles:true}))`);
  await waitFor(`${form} && ${date} && ${time}`);
  assert.equal(await evaluate(`${date}.labels[0].textContent`), 'Meeting time');
  assert.equal(await evaluate(`${time}.getAttribute('aria-label')`), 'Time');
  await change(time, '14:25');
  assert.equal(await evaluate(output), 'No date and time selected');
  await change(date, '2024-02-29');
  await waitFor(`${output} === '2024-02-29T14:25'`);
  assert.deepEqual(await evaluate(`Array.from(new FormData(${form}))`), [['meeting', '2024-02-29T14:25']]);
  await change(time, '16:30');
  await waitFor(`${output} === '2024-02-29T16:30'`);
  await change(date, '');
  await waitFor(`${output} === 'No date and time selected' && ${time}.value === ''`);
  await change(date, '2024-03-01');
  await waitFor(`${output}.startsWith('2024-03-01T')`);
  assert.match(await evaluate(output), /^2024-03-01T\d{2}:\d{2}$/);
  const example = "document.getElementById('date-time-input-status-example')";
  const descriptions = ['Choose an available review slot.', 'This review slot is already booked.'];
  for (const type of ['date', 'time']) {
    const segment = `${example}.querySelector('input[type=${type}]')`;
    assert.ok(await evaluate(`${segment}.validity.valid && ${segment}.getAttribute('aria-invalid') === 'true'`));
    assert.deepEqual(await evaluate(`${segment}.getAttribute('aria-describedby').split(' ').map(id => document.getElementById(id).textContent.trim())`), descriptions);
  }
  assert.equal(await evaluate(`${example}.querySelectorAll('.astryx-field-status').length`), 1);
  assert.equal(await evaluate(`document.getElementById('review-slot-status').dataset.variant`), 'detached');
  const {nodes} = await send('Accessibility.getFullAXTree');
  for (const name of ['Review date', 'Review time']) {
    assert.ok(nodes.some(node => !node.ignored && node.name?.value === name && node.description?.value.includes(descriptions[1])));
  }
  assert.deepEqual(exceptions, []);
  console.log('DateTimeInput: native names, time-first draft, combined form value, clearing and date-first default passed');
});
