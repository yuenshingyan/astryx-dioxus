import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8226'});
  const item = value => `document.querySelector('#tab-examples [data-tab-value="${value}"]')`;
  await waitFor(`${item('inbox')}?.getAttribute('aria-current') === 'true'`);
  await evaluate(`${item('archive')}.focus()`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13, text: '\r'});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`${item('archive')}.getAttribute('aria-current') === 'true'`);
  assert.ok(await evaluate(`document.activeElement === ${item('archive')}`));
  await evaluate(`${item('drafts')}.click()`);
  assert.equal(await evaluate(`${item('archive')}.getAttribute('aria-current')`), 'true');
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === 'Drafts' && node.properties.some(p => p.name === 'disabled' && p.value.value)));
  assert.deepEqual(exceptions, []);
  console.log('Tab: caller-controlled selection, keyboard activation, focus and disabled state passed');
});
