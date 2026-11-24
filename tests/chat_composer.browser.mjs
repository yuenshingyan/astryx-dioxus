import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8263'});
  const root = "document.getElementById('chat-composer-examples')";
  const editor = `${root}?.querySelector('textarea')`;
  const sendButton = `${root}?.querySelector('button[aria-label=Send]')`;
  const stop = `${root}?.querySelector('button[aria-label=Stop]')`;
  const output = `${root}.querySelector('output').textContent`;
  await waitFor(editor);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  const type = async text => {
    await evaluate(`${editor}.focus()`);
    await send('Input.insertText', {text});
  };
  await type('   ');
  assert.equal(await evaluate(`${sendButton}.getAttribute('aria-disabled')`), 'true');
  await type(' First ');
  await waitFor(`${sendButton}.getAttribute('aria-disabled') !== 'true'`);
  await evaluate(`${sendButton}.click()`);
  await waitFor(`${stop} && ${editor}.disabled && ${editor}.value === '' && ${output} === 'First'`);
  await evaluate(`${stop}.click()`);
  await waitFor(`${sendButton} && !${editor}.disabled`);
  await type('Second');
  await send('Input.dispatchKeyEvent', {type:'keyDown', key:'Enter', code:'Enter', windowsVirtualKeyCode:13, text:'\r'});
  await send('Input.dispatchKeyEvent', {type:'keyUp', key:'Enter', code:'Enter', windowsVirtualKeyCode:13});
  await waitFor(`${stop} && ${editor}.value === '' && ${output} === 'Second'`);
  assert.deepEqual(exceptions, []);
  console.log('ChatComposer: blank guard, button and keyboard send, draft clearing, disabled editor and Stop passed');
});
