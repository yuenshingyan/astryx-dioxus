import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8262'});
  const field = "document.querySelector('#chat-composer-input-examples textarea')";
  const output = "document.querySelector('#chat-composer-input-examples output').textContent";
  await waitFor(field);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'textbox' && node.name?.value === 'Message'));
  const enter = async (modifiers = 0) => {
    await send('Input.dispatchKeyEvent', {type:'keyDown', key:'Enter', code:'Enter', windowsVirtualKeyCode:13, text:'\r', modifiers});
    await send('Input.dispatchKeyEvent', {type:'keyUp', key:'Enter', code:'Enter', windowsVirtualKeyCode:13, modifiers});
  };
  await evaluate(`${field}.focus()`);
  await send('Input.insertText', {text:'   '});
  await enter();
  assert.equal(await evaluate(output), '');
  await send('Input.insertText', {text:'First'});
  await enter(8);
  await send('Input.insertText', {text:'Second  '});
  await waitFor(`${field}.value === '   First\\nSecond  '`);
  await evaluate(`${field}.dispatchEvent(new KeyboardEvent('keydown', {key:'Enter', isComposing:true, bubbles:true, cancelable:true}))`);
  assert.equal(await evaluate(output), '');
  await enter();
  await waitFor(`${output} === 'First\\nSecond' && ${field}.value === ''`);
  assert.equal(await evaluate(`document.activeElement === ${field}`), true);
  assert.deepEqual(exceptions, []);
  console.log('ChatComposerInput: textbox name, blank guard, Shift+Enter, composition guard and submit/clear passed');
});
