import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8214'});
  const buttons = id => `Array.from(document.querySelectorAll('#${id} .astryx-collapsible-trigger'))`;
  const single = buttons('single-accordion');
  const multiple = buttons('multiple-accordion');
  const states = selector => `${selector}.map(button => button.getAttribute('aria-expanded')).join(',')`;
  await waitFor(`${states(single)} === 'true,false,false'`);
  assert.equal(await evaluate(states(multiple)), 'true,false,false');
  await evaluate(`${single}[1].focus()`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13, text: '\r'});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`${states(single)} === 'false,true,false'`);
  assert.ok(await evaluate(`document.activeElement === ${single}[1]`));
  await evaluate(`${single}[1].click(); ${multiple}[1].click()`);
  await waitFor(`${states(single)} === 'false,false,false' && ${states(multiple)} === 'true,true,false'`);
  assert.deepEqual(await evaluate(`${single}.slice(0, 2).map(button => getComputedStyle(button.parentElement).borderBlockStartWidth)`), ['0px', '1px']);
  assert.ok(await evaluate(`${multiple}.every(button => button.parentElement.parentElement.id === 'multiple-accordion')`));
  assert.deepEqual(exceptions, []);
  console.log('CollapsibleGroup: single and multiple selection, keyboard focus, dividers, and wrapperless layout passed');
});
