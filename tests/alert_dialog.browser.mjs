import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8242'});
  const trigger = "document.getElementById('alert-preview-trigger')";
  const modal = "document.querySelector('#alert-dialog-examples dialog:modal')";
  await waitFor(trigger);
  await evaluate(`${trigger}.focus(); ${trigger}.click()`);
  await waitFor(`${modal} && document.activeElement.textContent.trim() === 'Cancel'`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  const alert = nodes.find(node => !node.ignored && node.role?.value === 'alertdialog' && node.name?.value === 'Delete item?');
  assert.match(alert?.description?.value ?? '', /cannot be undone/);
  await send('Input.dispatchMouseEvent', {type:'mousePressed', x:4, y:4, button:'left', clickCount:1});
  await send('Input.dispatchMouseEvent', {type:'mouseReleased', x:4, y:4, button:'left', clickCount:1});
  assert.ok(await evaluate(`${modal} !== null`));
  await evaluate(`${modal}.querySelector('button:last-child').click()`);
  await waitFor(`document.getElementById('alert-preview-action').textContent === 'Requests: 1'`);
  assert.ok(await evaluate(`${modal} !== null`));
  await send('Input.dispatchKeyEvent', {type:'keyDown', key:'Escape', code:'Escape', windowsVirtualKeyCode:27});
  await send('Input.dispatchKeyEvent', {type:'keyUp', key:'Escape', code:'Escape', windowsVirtualKeyCode:27});
  await waitFor(`!${modal} && document.activeElement === ${trigger}`);
  assert.deepEqual(exceptions, []);
  console.log('AlertDialog: accessible message, Cancel focus, blocked backdrop, caller-owned action, and Escape passed');
});
