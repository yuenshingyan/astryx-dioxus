import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8154'});
  await waitFor(`!!document.querySelector('#mega-item-action')`);
  assert.deepEqual(await evaluate(`(() => {
    const basic = document.querySelector('#mega-item-basic');
    const showcase = document.querySelector('#mega-item-showcase');
    const icon = showcase.querySelector('.astryx-top-nav-mega-menu-item__icon');
    const action = document.querySelector('#mega-item-action');
    action.focus();
    return [basic.querySelectorAll('a').length, showcase.querySelectorAll('a').length,
      icon.offsetWidth, icon.offsetHeight, action.type, action === document.activeElement,
      document.querySelector('#mega-item-static').tabIndex];
  })()`), [2, 4, 40, 40, 'button', true, -1]);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', text: '\r', windowsVirtualKeyCode: 13});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`document.querySelector('#mega-item-static').textContent.trim() === 'Updates: 1'`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === 'Refresh'));
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'link' && node.name?.value.includes('Edge Functions')));
  await evaluate(`document.querySelector('#mega-item-basic a').click()`);
  await waitFor(`location.hash === '#edge'`);
  assert.deepEqual(exceptions, []);
  console.log('Mega-menu item: link, keyboard action, static tile, icon geometry, and accessible names passed');
});
