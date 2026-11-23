import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8241'});
  const open = id => `document.getElementById('${id}-dialog-preview-trigger')`;
  const trigger = `document.getElementById('dialog-preview-trigger')`;
  const modal = `document.querySelector('#dialog-examples dialog:modal')`;
  const escape = async () => {
    await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Escape', code: 'Escape', windowsVirtualKeyCode: 27});
    await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Escape', code: 'Escape', windowsVirtualKeyCode: 27});
  };
  await waitFor(trigger);
  await evaluate(`${trigger}.focus(); ${trigger}.click()`);
  await waitFor(`${modal} && document.activeElement.id === 'dialog-preview-title'`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'dialog' && node.name?.value === 'Modal Title'));
  await escape();
  await waitFor(`!${modal} && document.activeElement === ${trigger}`);
  await evaluate(`${trigger}.click()`);
  await waitFor(modal);
  await send('Input.dispatchMouseEvent', {type: 'mousePressed', x: 4, y: 4, button: 'left', clickCount: 1});
  await send('Input.dispatchMouseEvent', {type: 'mouseReleased', x: 4, y: 4, button: 'left', clickCount: 1});
  await waitFor(`!${modal}`);
  await evaluate(`${open('required')}.focus(); ${open('required')}.click()`);
  await waitFor(`${modal}?.getAttribute('role') === 'alertdialog'`);
  await escape();
  assert.ok(await evaluate(`${modal} !== null`));
  await evaluate(`${modal}.querySelector('button').click()`);
  await waitFor(`!${modal} && document.activeElement === ${open('required')}`);
  assert.deepEqual(exceptions, []);
  console.log('Dialog: title focus, Escape, backdrop dismissal, required actions and restored focus passed');
});
