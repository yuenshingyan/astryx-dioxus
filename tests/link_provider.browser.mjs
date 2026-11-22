import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8212'});
  const link = `document.querySelector('#link-provider-examples a')`;
  const result = `document.getElementById('link-provider-result')`;
  await waitFor(`${link}?.getAttribute('href') === '/dashboard'`);
  const initialUrl = await evaluate('location.href');
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'link' && node.name?.value === 'Go to dashboard'));
  await evaluate(`${link}.focus()`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13, text: '\r'});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`${result}.textContent.includes('/dashboard')`);
  assert.equal(await evaluate('location.href'), initialUrl);
  assert.ok(await evaluate(`document.activeElement === ${link}`));
  assert.deepEqual(exceptions, []);
  console.log('LinkProvider: router interception, link semantics, and retained keyboard focus passed');
});
