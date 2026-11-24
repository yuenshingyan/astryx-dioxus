import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8275'});
  const trigger = "document.querySelector('#complex-selector-examples [role=combobox]')";
  const panel = `document.getElementById(${trigger}.getAttribute('aria-controls'))`;
  const focused = "document.activeElement.getAttribute('aria-label')";
  const open = `${panel}.matches(':popover-open')`;
  const press = async (key, windowsVirtualKeyCode, text) => {
    await send('Input.dispatchKeyEvent', {type:'keyDown', key, code:key, windowsVirtualKeyCode, text});
    await send('Input.dispatchKeyEvent', {type:'keyUp', key, code:key, windowsVirtualKeyCode});
  };
  await waitFor(trigger);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  await evaluate(`${trigger}.scrollIntoView({block:'center'}); ${trigger}.focus()`);
  await press('ArrowDown', 40);
  await waitFor(`${open} && ${focused} === 'Apple, Juicy'`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'dialog' && node.name?.value === 'Fruit blend'));
  await evaluate(`${panel}.querySelector('[aria-label="Pear, Peak"]').focus()`);
  await press('Enter', 13, '\r');
  const closed = `!${open} && document.activeElement === ${trigger}`;
  await waitFor(`${closed} && ${trigger}.textContent.includes('Pear · Peak')`);
  await press('ArrowDown', 40);
  await waitFor(`${open} && ${focused} === 'Pear, Peak'`);
  await press('Escape', 27);
  await waitFor(closed);
  assert.deepEqual(exceptions, []);
  console.log('ComplexSelector: dialog name, caller-owned value and focus placement/return passed');
});
