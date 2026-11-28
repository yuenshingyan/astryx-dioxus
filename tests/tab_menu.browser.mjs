import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8267'});
  const trigger = "document.querySelector('#tab-menu-examples [aria-haspopup=menu]')";
  const panel = `document.getElementById(${trigger}.getAttribute('aria-controls'))`;
  const radio = index => `${panel}.querySelectorAll('[role=menuitemradio]')[${index}]`;
  const press = async (key, windowsVirtualKeyCode, text) => {
    await send('Input.dispatchKeyEvent', {type:'keyDown', key, code:key, windowsVirtualKeyCode, text});
    await send('Input.dispatchKeyEvent', {type:'keyUp', key, code:key, windowsVirtualKeyCode});
  };
  await waitFor(trigger);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  assert.equal(await evaluate(`${trigger}.textContent.trim()`), 'Settings⌄');
  assert.equal(await evaluate(`${trigger}.tabIndex`), 0);
  await evaluate(`${trigger}.scrollIntoView({block:'center'}); ${trigger}.focus()`);
  const open = `${panel}.matches(':popover-open')`;
  const closed = `!${open} && ${trigger}.getAttribute('aria-expanded') === 'false'`;
  await press('Enter', 13, '\r');
  await waitFor(`${open} && document.activeElement === ${radio(0)}`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'menu' && node.name?.value === 'More'));
  for (const name of ['Settings', 'Integrations', 'Billing']) {
    assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'menuitemradio' && node.name?.value === name));
  }
  assert.equal(await evaluate(`${panel}.firstElementChild.textContent`), 'More');
  assert.equal(await evaluate(`${radio(0)}.getAttribute('aria-checked')`), 'true');
  await press('b', 66, 'b');
  await waitFor(`document.activeElement === ${radio(2)}`);
  await press('Home', 36);
  await waitFor(`document.activeElement === ${radio(0)}`);
  await press('ArrowDown', 40);
  await waitFor(`document.activeElement === ${radio(1)}`);
  await press('Enter', 13, '\r');
  await waitFor(`${closed} && document.activeElement === ${trigger} && ${trigger}.textContent.includes('Integrations')`);
  await press('Enter', 13, '\r');
  await waitFor(`${open} && document.activeElement === ${radio(0)}`);
  assert.equal(await evaluate(`${radio(1)}.getAttribute('aria-checked')`), 'true');
  await press('Escape', 27);
  await waitFor(`${closed} && document.activeElement === ${trigger}`);
  await press('Enter', 13, '\r');
  await waitFor(`${open} && document.activeElement === ${radio(0)}`);
  await press('Tab', 9);
  await waitFor(`${closed} && document.activeElement.id === 'tab-menu-after'`);
  assert.deepEqual(exceptions, []);
  console.log('TabMenu: selected label, accessible radio popup, arrow selection, Escape and Tab exit passed');
});
