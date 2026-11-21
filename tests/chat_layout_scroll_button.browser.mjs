import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8187'});
  await waitFor(`!!document.getElementById('chat-scroll-probe')`);
  const pill = `document.querySelector('#chat-scroll-probe .astryx-chat-layout-scroll-button')`;
  const button = `${pill}.querySelector('button')`;
  assert.deepEqual(await evaluate(`(() => {
    const collapsed = ${pill};
    const expanded = document.querySelector('#chat-layout-scroll-button-examples [data-expanded]');
    const a = collapsed.getBoundingClientRect();
    const b = expanded.getBoundingClientRect();
    return [a.width, a.height, b.width > a.width, b.height === a.height];
  })()`), [32, 32, true, true]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  for (const name of ['Scroll to bottom', 'New messages', '3 unread replies']) {
    assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === name), name);
  }
  await evaluate(`${button}.focus()`);
  assert.ok(await evaluate(`document.activeElement === ${button}`));
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', text: '\r', windowsVirtualKeyCode: 13});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`${pill}.inert && !${pill}.hasAttribute('data-visible')`);
  await evaluate(`${button}.focus()`);
  await waitFor(`document.activeElement !== ${button}`);
  await send('Emulation.setEmulatedMedia', {features: [{name: 'prefers-reduced-motion', value: 'reduce'}]});
  await waitFor(`getComputedStyle(${pill}).transitionDuration.split(',').every(value => value.trim() === '0s')`);
  await evaluate(`document.getElementById('chat-scroll-show').click()`);
  await waitFor(`!${pill}.inert && getComputedStyle(${pill}).visibility === 'visible'`);
  assert.deepEqual(exceptions, []);
  console.log('ChatLayoutScrollButton: pill sizes, names, keyboard activation, inert exit, and reduced motion passed');
});
