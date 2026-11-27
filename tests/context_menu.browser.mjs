import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8268'});
  const area = "document.getElementById('context-menu-demo-area')";
  const trigger = `${area}.parentElement`;
  const menu = "document.querySelector('#context-menu-examples [role=menu]')";
  const open = `${menu}.matches(':popover-open')`;
  const row = n => `${menu}.querySelectorAll('[role=menuitem]')[${n}]`;
  const key = async (key, code, modifiers = 0) => {
    for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key, code:key, windowsVirtualKeyCode:code, modifiers});
  };
  await waitFor(area);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  await evaluate(`${area}.scrollIntoView({block:'center'}); ${trigger}.focus()`);
  await key('F10', 121, 8);
  await waitFor(`${open} && document.activeElement === ${row(0)}`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'menu' && node.name?.value === 'Editing actions'));
  await key('ArrowDown', 40);
  await waitFor(`document.activeElement === ${row(1)}`);
  await key('Enter', 13);
  await waitFor(`!${open} && document.activeElement === ${trigger} && document.querySelector('#context-menu-examples output').textContent === 'Copy'`);
  const [x,y] = await evaluate(`(() => { const r = ${area}.getBoundingClientRect(); return [Math.round(r.left + r.width / 2), Math.round(r.top + r.height / 2)]; })()`);
  for (const type of ['mousePressed', 'mouseReleased']) await send('Input.dispatchMouseEvent', {type, x, y, button:'right', clickCount:1});
  await waitFor(open);
  assert.deepEqual(await evaluate(`(() => { const r = ${menu}.getBoundingClientRect(); return [r.left, r.top]; })()`), [x,y]);
  await evaluate(`${row(2)}.focus()`);
  await key('Enter', 13);
  await waitFor(`${open} && document.activeElement === ${row(2)} && document.querySelector('#context-menu-examples output').textContent === 'Paste'`);
  await key('Escape', 27);
  await waitFor(`!${open} && document.activeElement === ${trigger}`);
  assert.deepEqual(exceptions, []);
  console.log('ContextMenu: named popup, right-click placement, Shift+F10, keyboard selection and focus return passed');
});
