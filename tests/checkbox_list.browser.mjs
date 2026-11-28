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
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  const key = async (key, code, windowsVirtualKeyCode) => {
    for (const type of ['keyDown', 'keyUp']) {
      await send('Input.dispatchKeyEvent', {type, key, code, windowsVirtualKeyCode});
    }
  };
  const tab = () => key('Tab', 'Tab', 9);
  const space = () => key(' ', 'Space', 32);
  const input = `${group}.querySelectorAll('input')[1]`;
  await evaluate(`${group}.querySelector('input').focus()`);
  await tab();
  assert.ok(await evaluate(`document.activeElement === ${input}`));
  await space();
  await waitFor(`${input}.dataset.checked === 'true'`);
  assert.ok(await evaluate(`${input} === document.activeElement && ${group}.querySelectorAll(':checked').length === 2`));
  const states = `document.getElementById('checkbox-list-item-states')`;
  await waitFor(`${states}.querySelector('input').indeterminate`);
  const updated = await send('Accessibility.getFullAXTree');
  const checkbox = name => updated.nodes.find(node => !node.ignored && node.role?.value === 'checkbox' && node.name?.value === name);
  const property = (node, name) => node?.properties?.find(prop => prop.name === name)?.value?.value;
  assert.equal(checkbox('Team access')?.description?.value, 'Some teams have access');
  assert.equal(property(checkbox('Team access'), 'checked'), 'mixed');
  assert.equal(checkbox('Guest access')?.description?.value, 'Managed by your administrator');
  assert.equal(property(checkbox('Guest access'), 'disabled'), true);
  assert.equal(property(checkbox('Guest access'), 'checked'), 'true');
  assert.ok(checkbox('Pro plan Recommended option'));

  const guest = `${states}.querySelectorAll('input')[1]`;
  const guestRow = `${guest}.closest('li')`;
  await evaluate(`${states}.querySelector('input').focus()`);
  await tab();
  assert.ok(await evaluate(`document.activeElement === ${guest} && ${guest}.matches(':focus-visible') && !${guest}.disabled`));
  // Computed paint only: the existing indicator owns the outline, not the row.
  assert.ok(await evaluate(`getComputedStyle(${guest}.nextElementSibling).outlineStyle !== 'none' && getComputedStyle(${guestRow}).outlineStyle === 'none'`));
  assert.ok(await evaluate(`(() => {
    const disabledColor = getComputedStyle(${guest}.labels[0]).color;
    return [...${guestRow}.querySelectorAll('.astryx-item__label, .astryx-item__description')]
      .every(slot => getComputedStyle(slot).color === disabledColor);
  })()`));
  assert.equal(await evaluate(`${guestRow}.hasAttribute('data-selected')`), false);
  assert.equal(await evaluate(`${input}.closest('li').dataset.selected`), 'selected');
  await space();
  assert.ok(await evaluate(`document.activeElement === ${guest} && ${guest}.checked`));
  const {x, y} = await evaluate(`(() => {
    ${guest}.scrollIntoView({block: 'center'});
    const r = ${guest}.getBoundingClientRect();
    return {x: r.x + r.width / 2, y: r.y + r.height / 2};
  })()`);
  assert.ok(await evaluate(`document.elementFromPoint(${x}, ${y}) === ${guest}`));
  for (const type of ['mousePressed', 'mouseReleased']) {
    await send('Input.dispatchMouseEvent', {type, x, y, button: 'left', clickCount: 1});
  }
  assert.ok(await evaluate(`document.activeElement === ${guest} && ${guest}.checked`));
  assert.match(await evaluate(`document.getElementById('checkbox-list-disabled-requests').textContent`), /: 0$/);
  await tab();
  assert.ok(await evaluate(`document.activeElement === ${states}.querySelectorAll('input')[2]`));
  assert.equal(await evaluate(`${states}.querySelectorAll('li[tabindex]').length`), 0);
  assert.deepEqual(exceptions, []);
  console.log('CheckboxList: DOM/AX naming and state, native Tab/Space and pointer activation guard, computed focus/label styles passed');
});
