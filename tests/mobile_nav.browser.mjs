import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8239'});
  const dialog = `document.getElementById('mobile-nav-preview')`;
  const trigger = `document.getElementById('open-mobile-nav-preview')`;
  await waitFor(`${trigger} && ${dialog} && !${dialog}.open`);
  await evaluate(`${trigger}.focus(); ${trigger}.click()`);
  await waitFor(`${dialog}.matches(':modal')`);
  assert.ok(await evaluate(`${dialog}.contains(document.activeElement)`));
  assert.ok(await evaluate(`Math.abs(${dialog}.firstElementChild.getBoundingClientRect().right - ${dialog}.getBoundingClientRect().right) < 1`));
  const {nodes} = await send('Accessibility.getFullAXTree');
  for (const role of ['dialog', 'navigation']) {
    assert.ok(nodes.some(node => !node.ignored && node.role?.value === role && node.name?.value === 'Workspace navigation'));
  }
  await send('Input.dispatchKeyEvent', {type:'keyDown', key:'Escape', code:'Escape', windowsVirtualKeyCode:27});
  await send('Input.dispatchKeyEvent', {type:'keyUp', key:'Escape', code:'Escape', windowsVirtualKeyCode:27});
  await waitFor(`!${dialog}.open && document.activeElement === ${trigger}`);
  await evaluate(`${trigger}.click()`);
  await waitFor(`${dialog}.matches(':modal')`);
  await evaluate(`${dialog}.querySelector('button').click()`);
  await waitFor(`!${dialog}.open && document.activeElement === ${trigger}`);
  assert.deepEqual(exceptions, []);
  console.log('MobileNav: modal and landmark names, RTL placement, Escape, close action, and restored focus passed');
});
