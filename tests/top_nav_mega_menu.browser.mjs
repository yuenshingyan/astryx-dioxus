import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8264'});
  const trigger = "document.querySelector('#top-nav-mega-menu-examples button[aria-controls]')";
  const panel = `document.getElementById(${trigger}.getAttribute('aria-controls'))`;
  const key = async (key, windowsVirtualKeyCode, text) => {
    await send('Input.dispatchKeyEvent', {type:'keyDown', key, code:key, windowsVirtualKeyCode, text});
    await send('Input.dispatchKeyEvent', {type:'keyUp', key, code:key, windowsVirtualKeyCode});
  };
  await waitFor(trigger);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  await evaluate(`${trigger}.scrollIntoView({block:'center'}); ${trigger}.focus()`);
  await key('Enter', 13, '\r');
  await waitFor(`${panel}.matches(':popover-open') && ${trigger}.getAttribute('aria-expanded') === 'true'`);
  assert.equal(await evaluate(`${panel}.querySelectorAll('a[href]').length`), 5);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'group' && node.name?.value === 'Products'));
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'link' && node.name?.value.startsWith('Read the changelog')));
  await key('Tab', 9);
  await waitFor(`document.activeElement === ${panel}.querySelector('a')`);
  await key('Escape', 27);
  const closed = `!${panel}.matches(':popover-open') && ${trigger}.getAttribute('aria-expanded') === 'false'`;
  await waitFor(`${closed} && document.activeElement === ${trigger}`);
  await evaluate(`${trigger}.click()`);
  await waitFor(`${panel}.matches(':popover-open')`);
  await evaluate("document.getElementById('mega-deploy').click()");
  await waitFor(closed);
  assert.deepEqual(exceptions, []);
  console.log('TopNavMegaMenu: disclosure, named link group, keyboard traversal, Escape focus return, and outside dismissal passed');
});
