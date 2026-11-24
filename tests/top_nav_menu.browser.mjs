import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls:['https://*']});
  await send('Page.navigate', {url:process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8265'});
  const trigger = "document.querySelector('#top-nav-menu-examples button[aria-haspopup=menu]')";
  const panel = `document.getElementById(${trigger}.getAttribute('aria-controls'))`;
  const item = index => `${panel}.querySelectorAll('[role=menuitem]')[${index}]`;
  const press = async (key, windowsVirtualKeyCode, text) => {
    await send('Input.dispatchKeyEvent', {type:'keyDown', key, code:key, windowsVirtualKeyCode, text});
    await send('Input.dispatchKeyEvent', {type:'keyUp', key, code:key, windowsVirtualKeyCode});
  };
  await waitFor(trigger);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  await evaluate(`${trigger}.scrollIntoView({block:'center'}); ${trigger}.focus()`);
  await press('Enter', 13, '\r');
  await waitFor(`${panel}.matches(':popover-open') && document.activeElement === ${item(0)}`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'menu' && node.name?.value === 'Tools'));
  await press('ArrowDown', 40);
  await waitFor(`document.activeElement === ${item(1)}`);
  await press('Escape', 27);
  const closed = `!${panel}.matches(':popover-open') && ${trigger}.getAttribute('aria-expanded') === 'false'`;
  await waitFor(`${closed} && document.activeElement === ${trigger}`);
  await press('Enter', 13, '\r');
  await waitFor(`document.activeElement === ${item(0)}`);
  await press('Tab', 9);
  await waitFor(`${closed} && document.activeElement.id === 'tools-search'`);
  await evaluate(`${trigger}.focus(); ${trigger}.click()`);
  await waitFor(`document.activeElement === ${item(0)}`);
  await press('Enter', 13, '\r');
  await waitFor(`${closed} && location.hash === '#tools-analytics'`);
  assert.deepEqual(exceptions, []);
  console.log('TopNavMenu: named popup, focus, arrow navigation, Escape, Tab exit and link activation passed');
});
