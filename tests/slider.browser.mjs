import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8247'});
  const form = `document.querySelector('#slider-examples form')`;
  const ranges = `(${form}.querySelectorAll('input[type=range]'))`;
  await waitFor(`${form} && ${ranges}.length === 4`);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  const press = async (key, windowsVirtualKeyCode) => {
    for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key, code: key, windowsVirtualKeyCode});
  };
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'slider' && node.name?.value === 'Volume'));
  await evaluate(`${ranges}[0].focus()`);
  await press('ArrowRight', 39);
  await waitFor(`${ranges}[0].dataset.value === '51'`);
  await evaluate(`${form}.dir = 'rtl'`);
  await press('ArrowLeft', 37);
  await waitFor(`${ranges}[0].dataset.value === '52'`);
  await evaluate(`${ranges}[1].focus()`);
  await press('ArrowUp', 38);
  await waitFor(`${ranges}[1].dataset.value === '30'`);
  await evaluate(`${ranges}[2].focus()`);
  await press('End', 35);
  await waitFor(`${ranges}[2].value === '30'`);
  assert.deepEqual(await evaluate(`Array.from(new FormData(${form}))`), [['volume', '52']]);
  assert.ok(await evaluate(`${ranges}[3].disabled`));
  assert.deepEqual(exceptions, []);
  console.log('Slider: accessible name, LTR/RTL and vertical keys, controlled rollback and disabled form exclusion passed');
});
