import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor}) => {
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8223'});
  const handle = label => `document.querySelector('[aria-label="${label}"][role=separator]')`;
  const width = handle('Preview width');
  const height = handle('Details height');
  const key = async (name, code, modifiers = 0) => {
    await send('Input.dispatchKeyEvent', {type: 'keyDown', key: name, code: name, windowsVirtualKeyCode: code, modifiers});
    await send('Input.dispatchKeyEvent', {type: 'keyUp', key: name, code: name, windowsVirtualKeyCode: code, modifiers});
  };
  await waitFor(`${width}?.getAttribute('aria-valuenow') === '180'`);
  await evaluate(`${width}.focus()`);
  await key('ArrowRight', 39);
  await waitFor(`${width}.getAttribute('aria-valuenow') === '190'`);
  await key('ArrowRight', 39, 8);
  await waitFor(`${width}.getAttribute('aria-valuenow') === '240'`);
  await key('End', 35);
  await waitFor(`document.getElementById('resize-preview-pane').getBoundingClientRect().width === 280`);
  await evaluate(`${height}.focus()`);
  await key('ArrowDown', 40);
  await waitFor(`${height}.getAttribute('aria-valuenow') === '90'`);
  await key('Home', 36);
  await waitFor(`${height}.getAttribute('aria-valuenow') === '50'`);
  assert.ok(await evaluate(`document.activeElement === ${height}`));
  console.log('ResizeHandle: keyboard steps, bounds, reversed axis, pane geometry and focus passed');
});
