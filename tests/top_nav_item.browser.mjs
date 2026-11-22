import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8219'});
  const nav = `document.querySelector('#top-nav-item-examples nav')`;
  await waitFor(`${nav}?.querySelector('[aria-current=page]')`);
  assert.equal(await evaluate(`${nav}.querySelector('[aria-current=page]').textContent.trim()`), 'Dashboard');
  const disabled = `${nav}.querySelector('[aria-disabled=true]')`;
  assert.deepEqual(await evaluate(`[${disabled}.hasAttribute('href'), ${disabled}.tabIndex]`), [false, 0]);
  const action = `${nav}.querySelector('button')`;
  await evaluate(`${action}.focus()`);
  await send('Input.dispatchKeyEvent', {type:'keyDown', key:' ', code:'Space', windowsVirtualKeyCode:32, text:' '});
  await send('Input.dispatchKeyEvent', {type:'keyUp', key:' ', code:'Space', windowsVirtualKeyCode:32});
  await waitFor(`document.getElementById('top-nav-item-count').textContent === '1'`);
  assert.ok(await evaluate(`document.activeElement === ${action}`));
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'link' && node.name?.value === 'Archived' && node.properties?.some(property => property.name === 'disabled' && property.value.value === true)));
  assert.deepEqual(exceptions, []);
  console.log('TopNavItem: current and disabled links, keyboard action, and focus passed');
});
