import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8293'});
  const trigger = `document.querySelector('#dropdown-menu-examples [aria-haspopup="menu"]')`;
  const menu = `document.getElementById(${trigger}.getAttribute('aria-controls'))`;
  const divider = `document.getElementById('dropdown-menu-actions-divider')`;
  const output = `document.querySelector('#dropdown-menu-examples output')`;
  const focused = `document.activeElement.textContent.trim()`;
  const after = `document.getElementById('dropdown-menu-after')`;
  const press = async (key, code, windowsVirtualKeyCode) => {
    for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key, code, windowsVirtualKeyCode});
  };
  await waitFor(trigger);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  await evaluate(`${trigger}.scrollIntoView({block:'center'}); ${trigger}.focus()`);
  await press('ArrowDown', 'ArrowDown', 40);
  await waitFor(`${menu}.matches(':popover-open') && ${focused} === 'Edit'`);
  assert.equal(await evaluate(`${trigger}.getAttribute('aria-expanded')`), 'true');
  assert.ok(await evaluate(`Math.abs(${menu}.getBoundingClientRect().top - ${trigger}.getBoundingClientRect().bottom - 4) < 1`));
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'menu' && node.name?.value === 'Actions'));
  await press('ArrowDown', 'ArrowDown', 40);
  await waitFor(`${focused} === 'Duplicate'`);
  assert.equal(await evaluate(`document.activeElement.getAttribute('aria-disabled')`), 'true');
  await press('Enter', 'Enter', 13);
  assert.ok(await evaluate(`${menu}.matches(':popover-open') && ${output}.textContent === 'No action selected'`));
  await waitFor(`${divider}?.dataset.mounted === 'true'`);
  assert.deepEqual(await evaluate(`(() => {
    const node = ${divider};
    const css = getComputedStyle(node);
    return [node.parentElement === ${menu}, node.getAttribute('role'),
      node.getAttribute('aria-orientation'), node.hasAttribute('tabindex'),
      ['astryx-divider', 'astryx-dropdown-menu-divider', 'actions-divider'].every(name => node.classList.contains(name)),
      css.marginBlockStart, css.marginBlockEnd];
  })()`), [true, 'separator', 'horizontal', false, true, '8px', '8px']);
  await press('ArrowDown', 'ArrowDown', 40);
  await waitFor(`${focused} === 'Delete'`);
  await press('ArrowUp', 'ArrowUp', 38);
  await waitFor(`${focused} === 'Duplicate'`);
  const point = await evaluate(`(() => {
    const {left, top, width, height} = ${divider}.getBoundingClientRect();
    return {x: left + width / 2, y: top + height / 2};
  })()`);
  assert.ok(await evaluate(`${divider}.contains(document.elementFromPoint(${point.x}, ${point.y}))`));
  for (const type of ['mousePressed', 'mouseReleased']) {
    await send('Input.dispatchMouseEvent', {type, ...point, button: 'left', clickCount: 1});
  }
  await waitFor(`${output}.dataset.dividerClicks === '1'`);
  assert.ok(await evaluate(`${menu}.matches(':popover-open') && ${output}.textContent === 'No action selected' && document.activeElement !== ${divider}`));
  await evaluate(`${divider}.previousElementSibling.focus()`);
  await press('End', 'End', 35);
  await waitFor(`${focused} === 'Delete'`);
  await press('e', 'KeyE', 69);
  await waitFor(`${focused} === 'Edit'`);
  await press('Enter', 'Enter', 13);
  const closed = `!${menu}.matches(':popover-open')`;
  await waitFor(`${closed} && document.activeElement === ${trigger} && ${output}.textContent === 'Edit'`);
  await press('ArrowUp', 'ArrowUp', 38);
  await waitFor(`${focused} === 'Delete'`);
  await press('Escape', 'Escape', 27);
  await waitFor(`${closed} && document.activeElement === ${trigger}`);
  await press('ArrowDown', 'ArrowDown', 40);
  await waitFor(`${focused} === 'Edit'`);
  await press('Tab', 'Tab', 9);
  await waitFor(`${closed} && document.activeElement === ${after}`);
  const retainedTrigger = `document.querySelector('#dropdown-menu-keep-open [aria-haspopup=menu]')`;
  const retained = `document.getElementById(${retainedTrigger}.getAttribute('aria-controls'))`;
  await evaluate(`${retainedTrigger}.focus()`);
  await press('ArrowDown', 'ArrowDown', 40);
  await waitFor(`${retained}.matches(':popover-open') && ${focused} === 'Pin preview'`);
  await press('Enter', 'Enter', 13);
  await waitFor(`${retained}.matches(':popover-open') && ${focused} === 'Unpin preview'`);
  await press('ArrowDown', 'ArrowDown', 40);
  await waitFor(`${focused} === 'Done'`);
  await press('Enter', 'Enter', 13);
  await waitFor(`!${retained}.matches(':popover-open') && document.activeElement === ${retainedTrigger}`);
  assert.deepEqual(exceptions, []);
  console.log('DropdownMenu: anchored placement, named menu, keyboard selection, Escape and Tab dismissal passed');
});
