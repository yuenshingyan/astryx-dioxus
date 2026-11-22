import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8220'});
  const nav = `document.querySelector('#side-nav-item-examples nav')`;
  const disabled = `${nav}.querySelector('button[aria-disabled=true]')`;
  const action = `${nav}.querySelector('button:not([aria-disabled])')`;
  await waitFor(`${nav}?.querySelector('a[aria-current=page]')`);
  assert.match(await evaluate(`${nav}.querySelector('a[aria-current=page]').textContent`), /Inbox.*12/);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === 'Settings' && node.properties?.some(property => property.name === 'disabled' && property.value.value === true)));
  await evaluate(`${disabled}.focus(); ${disabled}.click()`);
  assert.equal(await evaluate('document.activeElement.textContent.trim()'), 'Settings');
  assert.equal(await evaluate(`document.getElementById('side-nav-item-count').textContent`), '0');
  await evaluate(`${action}.focus()`);
  await send('Input.dispatchKeyEvent', {type:'keyDown', key:' ', code:'Space', windowsVirtualKeyCode:32, text:' '});
  await send('Input.dispatchKeyEvent', {type:'keyUp', key:' ', code:'Space', windowsVirtualKeyCode:32});
  await waitFor(`document.getElementById('side-nav-item-count').textContent === '1'`);
  assert.ok(await evaluate(`document.activeElement === ${action}`));
  assert.deepEqual(exceptions, []);
  console.log('SideNavItem: current link, trailing badge, disabled action, and keyboard focus passed');
});
