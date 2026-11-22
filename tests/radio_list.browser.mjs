import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8209'});
  const group = `document.getElementById('size-radio-group')`;
  await waitFor(`${group}?.querySelectorAll('input').length === 3`);
  assert.deepEqual(await evaluate(`['notification-radio-group','size-radio-group'].map(id => {
    const style = getComputedStyle(document.getElementById(id));
    return [style.flexDirection, style.gap];
  })`), [['column','8px'], ['row','20px']]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  const notification = nodes.find(node => !node.ignored && node.role?.value === 'radiogroup' && node.name?.value === 'Notification preference');
  assert.equal(notification?.description?.value, 'Choose one channel for alerts.');
  await evaluate(`${group}.querySelector('input:checked').focus()`);
  for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key: 'ArrowRight', code: 'ArrowRight', windowsVirtualKeyCode: 39});
  await waitFor(`${group}.querySelectorAll('input')[2].checked && document.activeElement === ${group}.querySelectorAll('input')[2]`);
  assert.equal(await evaluate(`${group}.querySelectorAll('input:checked').length`), 1);
  assert.deepEqual(exceptions, []);
  console.log('RadioList: group name and description, layout, and native radio navigation passed');
});
