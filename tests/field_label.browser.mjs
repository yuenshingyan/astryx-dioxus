import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8146'});
  await waitFor(`!!document.getElementById('field-label-basic-phone')`);
  await evaluate(`document.getElementById('field-label-basic-phone-description').click()`);
  assert.equal(await evaluate(`document.activeElement.id`), 'field-label-basic-phone');
  await evaluate(`document.querySelector('#field-label-digest-description a').click()`);
  assert.equal(await evaluate(`document.getElementById('field-label-digest').checked`), false);
  await evaluate(`document.getElementById('field-label-digest-description').click()`);
  assert.equal(await evaluate(`document.getElementById('field-label-digest').checked`), true);
  assert.deepEqual(await evaluate(`(() => {
    const label = document.querySelector('label[for="field-label-search"]');
    const group = document.getElementById('field-label-notifications');
    return [getComputedStyle(label.parentElement).display, getComputedStyle(label).position,
      label.control.id, label.textContent.trim(), group.tagName, group.textContent.includes('Optional')];
  })()`), ['contents', 'absolute', 'field-label-search', 'Search', 'SPAN', true]);
  const help = `document.querySelector('label[for="field-label-api-key"] + button')`;
  const hiddenHelp = `document.querySelector('.astryx-field-label__row:has(> label[for="field-label-search"]) > .astryx-info-tip')`;
  await waitFor(`!!document[Symbol.for('astryx.dioxus.button.runtime')]?.records.get(${help})`);
  assert.equal(await evaluate(`${help}.getAttribute('aria-label')`), 'More information about API key');
  await evaluate(`${help}.click()`);
  await waitFor(`${help}.getAttribute('aria-expanded') === 'true'`);
  assert.notEqual(await evaluate(`document.activeElement.id`), 'field-label-api-key');
  assert.equal(await evaluate(`getComputedStyle(${hiddenHelp}).position`), 'absolute');
  await evaluate(`(() => { const before = document.createElement('button'); ${hiddenHelp}.before(before); before.focus(); })()`);
  for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key: 'Tab', code: 'Tab', windowsVirtualKeyCode: 9});
  await waitFor(`document.activeElement === ${hiddenHelp} && getComputedStyle(${hiddenHelp}).position === 'relative'`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.name?.value === 'Search' && node.description?.value === 'Search the catalog'));
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'group' && node.name?.value.includes('Notifications')));
  assert.deepEqual(exceptions, []);
  console.log('FieldLabel: native association, hidden name and description, optionality, safe description activation, and independent help passed');
});
