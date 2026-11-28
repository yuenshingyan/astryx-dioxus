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
  await evaluate(`(() => {
    const [small, medium, large] = ${group}.querySelectorAll('input');
    small.defaultChecked = false;
    medium.defaultChecked = true;
    large.defaultChecked = false;
    ${group}.querySelector('input:checked').focus();
  })()`);
  for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key: 'ArrowRight', code: 'ArrowRight', windowsVirtualKeyCode: 39});
  await waitFor(`${group}.querySelectorAll('input')[2].checked && document.activeElement === ${group}.querySelectorAll('input')[2]`);
  assert.equal(await evaluate(`${group}.querySelectorAll('input:checked').length`), 1);
  await waitFor(`${group}.querySelectorAll('input')[2].dataset.radioChecked === 'true'`);
  const updated = await send('Accessibility.getFullAXTree');
  assert.ok(updated.nodes.some(node => !node.ignored && node.role?.value === 'radio' && node.name?.value === 'Large' && node.description?.value === 'Extra room'));
  await evaluate(`${group}.querySelector('input:disabled').click()`);
  assert.equal(await evaluate(`${group}.querySelector('input:checked').value`), 'Large');
  assert.equal(await evaluate(`(() => {
    const group = ${group};
    const form = group.closest('form');
    group.dataset.resetChanges = '0';
    group.addEventListener('change', () => group.dataset.resetChanges++);
    form.reset();
    return new FormData(form).get('radio-preview-size');
  })()`), 'Medium', 'native reset ran before reconciliation');
  await waitFor(`${group}.querySelector('input:checked')?.value === 'Large'`);
  assert.deepEqual(await evaluate(`(() => {
    const group = ${group};
    const selected = group.querySelector('input:checked');
    return [new FormData(group.closest('form')).get('radio-preview-size'), document.activeElement === selected, group.dataset.resetChanges];
  })()`), ['Large', true, '0']);
  assert.deepEqual(exceptions, []);
  console.log('RadioList: group semantics, layout, native navigation, and controlled form reset passed');
});
