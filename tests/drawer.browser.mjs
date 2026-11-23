import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8243'});
  const trigger = "document.getElementById('drawer-preview-trigger')";
  const drawer = "document.querySelector('#drawer-examples dialog:modal')";
  await waitFor(trigger);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  await evaluate(`${trigger}.focus(); ${trigger}.click()`);
  await waitFor(`${drawer} && document.activeElement === ${drawer}.querySelector('.astryx-drawer__content')`);
  assert.deepEqual(await evaluate(`(() => { const rect = ${drawer}.getBoundingClientRect(); return [rect.left, rect.top, rect.width, rect.height === innerHeight]; })()`), [0, 0, 360, true]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'dialog' && node.name?.value === 'Details'));
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Escape', code: 'Escape', windowsVirtualKeyCode: 27});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Escape', code: 'Escape', windowsVirtualKeyCode: 27});
  await waitFor(`!${drawer} && document.activeElement === ${trigger}`);
  await evaluate(`${trigger}.click()`);
  await waitFor(drawer);
  await evaluate(`${drawer}.querySelector('button[aria-label="Close"]').click()`);
  await waitFor(`!${drawer} && document.activeElement === ${trigger}`);
  assert.deepEqual(exceptions, []);
  console.log('Drawer: RTL edge, modal name, Escape, close action and restored focus passed');
});
