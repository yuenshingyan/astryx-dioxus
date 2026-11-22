import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8218'});
  const root = `document.getElementById('payment-banner')`;
  const dismiss = `${root}.querySelector('button[aria-label="Dismiss Payment failed."]')`;
  await waitFor(`${root}?.querySelector('[role=alert]')`);
  assert.deepEqual(await evaluate(`Array.from(document.querySelectorAll('#banner-examples [data-status]'), node => node.dataset.status)`), ['info', 'success', 'warning', 'error']);
  assert.equal(await evaluate(`${root}.querySelector('[role=alert] button')`), null);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'alert'));
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === 'Dismiss Payment failed.'));
  await evaluate(`${root}.querySelector('button').click()`);
  await waitFor(`document.getElementById('banner-demo-result').textContent === 'Retry requested'`);
  await evaluate(`${dismiss}.focus()`);
  await send('Input.dispatchKeyEvent', {type:'keyDown', key:'Enter', code:'Enter', windowsVirtualKeyCode:13, text:'\r'});
  await send('Input.dispatchKeyEvent', {type:'keyUp', key:'Enter', code:'Enter', windowsVirtualKeyCode:13});
  await waitFor(`${root} === null`);
  assert.deepEqual(exceptions, []);
  console.log('Banner: status variants, message-only alert, action, and keyboard dismissal passed');
});
