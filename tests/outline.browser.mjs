import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8224'});
  const nav = `document.querySelector('#outline-examples nav')`;
  const tokens = `${nav}.querySelector('a[href="#outline-tokens"]')`;
  await waitFor(`${nav}?.querySelector('[aria-current="location"]')`);
  assert.equal(await evaluate(`${nav}.querySelector('[aria-current]').textContent`), 'Theming');
  assert.ok(await evaluate(`parseFloat(getComputedStyle(${tokens}).paddingInlineStart) > parseFloat(getComputedStyle(${nav}.querySelector('a')).paddingInlineStart)`));
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'navigation' && node.name?.value === 'Table of contents'));
  await evaluate(`${tokens}.focus()`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13, text: '\r'});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`location.hash === '#outline-tokens' && document.querySelector(':target')?.id === 'outline-tokens'`);
  assert.deepEqual(exceptions, []);
  console.log('Outline: named navigation, current location, indentation, and keyboard fragment navigation passed');
});
