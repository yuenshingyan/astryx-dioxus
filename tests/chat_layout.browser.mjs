import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8188'});
  const root = `document.getElementById('chat-layout-main')`;
  const atEnd = `${root}.scrollHeight - ${root}.clientHeight - ${root}.scrollTop <= 1`;
  const pill = `${root}.querySelector('.astryx-chat-layout-scroll-button')`;
  await waitFor(`!!${root} && ${root}.scrollTop > 0 && ${atEnd}`);
  assert.deepEqual(await evaluate(`(() => {
    const panel = document.getElementById('chat-layout-panel');
    const dock = ${root}.querySelector('.astryx-chat-layout__dock');
    return [panel.scrollHeight === panel.clientHeight, getComputedStyle(dock).position, Math.abs(dock.getBoundingClientRect().bottom - ${root}.getBoundingClientRect().bottom) <= 1];
  })()`), [true, 'sticky', true]);
  await evaluate(`${root}.scrollTop = 0`);
  await waitFor(`!${pill}.inert`);
  const add = `${root}.querySelector('.astryx-chat-layout__composer button')`;
  await evaluate(`${add}.click()`);
  await waitFor(`${root}.querySelectorAll('article').length === 13`);
  assert.equal(await evaluate(`${root}.scrollTop`), 0);
  await evaluate(`${pill}.querySelector('button').focus({preventScroll:true})`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', text: '\r', windowsVirtualKeyCode: 13});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`${pill}.inert && ${atEnd}`);
  assert.equal(await evaluate(`document.activeElement.getAttribute('role')`), 'log');
  await evaluate(`${add}.click()`);
  await waitFor(`${root}.querySelectorAll('article').length === 14 && ${atEnd}`);
  assert.deepEqual(exceptions, []);
  console.log('ChatLayout: sticky dock, initial end, reading position, focus handoff and pinned growth passed');
});
