import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8222'});
  const group = `document.querySelector('#grouped-tool-calls button')`;
  const detail = `document.querySelector('#single-tool-call button')`;
  const panel = `document.getElementById(${group}.getAttribute('aria-controls'))`;
  await waitFor(`${group}?.getAttribute('aria-expanded') === 'false'`);
  assert.ok(await evaluate(`${panel}.hidden && ${group}.textContent.includes('Running')`));
  await evaluate(`${group}.focus()`);
  await send('Input.dispatchKeyEvent', {type:'keyDown', key:' ', code:'Space', windowsVirtualKeyCode:32, text:' '});
  await send('Input.dispatchKeyEvent', {type:'keyUp', key:' ', code:'Space', windowsVirtualKeyCode:32});
  await waitFor(`!${panel}.hidden`);
  assert.match(await evaluate(`${panel}.textContent`), /\+0/);
  await evaluate(`${detail}.click()`);
  await waitFor(`document.getElementById(${detail}.getAttribute('aria-controls'))?.textContent === 'Date formatter source'`);
  await evaluate(`${detail}.click()`);
  await waitFor(`!${detail}.hasAttribute('aria-controls')`);
  assert.deepEqual(exceptions, []);
  console.log('ChatToolCalls: latest-call preview, keyboard disclosure, metadata and result detail passed');
});
