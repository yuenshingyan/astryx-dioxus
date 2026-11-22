import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8207'});
  const group = `document.getElementById('notification-preferences')`;
  await waitFor(`${group}?.querySelectorAll('ul > li').length === 3`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  const ax = nodes.find(node => !node.ignored && node.role?.value === 'group' && node.name?.value === 'Notification preferences');
  assert.ok(ax);
  assert.match(ax.description?.value ?? '', /Choose how you would like to be notified.*SMS availability varies by region/);
  const input = `${group}.querySelectorAll('input')[1]`;
  await evaluate(`${input}.focus()`);
  for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key: ' ', code: 'Space', windowsVirtualKeyCode: 32});
  await waitFor(`${input}.dataset.checked === 'true'`);
  assert.ok(await evaluate(`${input} === document.activeElement && ${group}.querySelectorAll(':checked').length === 2`));
  const states = `document.getElementById('checkbox-list-item-states')`;
  await waitFor(`${states}.querySelector('input').indeterminate`);
  const updated = await send('Accessibility.getFullAXTree');
  assert.ok(updated.nodes.some(node => !node.ignored && node.role?.value === 'checkbox' && node.name?.value === 'Team access' && node.description?.value === 'Some teams have access'));
  const guest = `${states}.querySelectorAll('input')[1]`;
  await evaluate(`${guest}.click()`);
  assert.ok(await evaluate(`!${guest}.checked && ${guest}.getAttribute('aria-disabled') === 'true'`));
  assert.equal(await evaluate(`${states}.querySelectorAll('li[tabindex]').length`), 0);
  assert.deepEqual(exceptions, []);
  console.log('CheckboxList: named group, descriptions, native list, and keyboard selection passed');
});
