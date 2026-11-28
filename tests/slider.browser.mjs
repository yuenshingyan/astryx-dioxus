import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8298'});
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
  const marks = index => `Array.from(${ranges}[${index}].parentElement.querySelectorAll('.astryx-slider__mark'))`;
  assert.deepEqual(await evaluate(`${marks(0)}.map(el => el.textContent)`), ['0', '25', '50', '75', '100']);
  assert.ok(await evaluate(`${marks(0)}[0].parentElement.getAttribute('aria-hidden') === 'true'`));
  assert.ok(await evaluate(`${marks(0)}[0].getBoundingClientRect().x < ${marks(0)}[4].getBoundingClientRect().x`));
  assert.ok(await evaluate(`${marks(1)}[0].getBoundingClientRect().y > ${marks(1)}[1].getBoundingClientRect().y`));
  assert.deepEqual(await evaluate(`${marks(0)}.filter(el => el.hasAttribute('data-filled')).map(el => el.textContent)`), ['0', '25', '50']);
  const caption = `${ranges}[0].closest('.astryx-slider').querySelector('.astryx-slider__value')`;
  assert.equal(await evaluate(`${caption}.textContent`), '50%');
  assert.equal(await evaluate(`${ranges}[0].getAttribute('aria-valuetext')`), '50%');
  assert.equal(await evaluate(`${ranges}[1].getAttribute('aria-valuetext')`), null);
  await evaluate(`${ranges}[0].focus()`);
  await press('ArrowRight', 39);
  await waitFor(`${ranges}[0].dataset.value === '51' && ${caption}.textContent === '51%' && ${ranges}[0].getAttribute('aria-valuetext') === '51%'`);
  await evaluate(`${form}.dir = 'rtl'`);
  assert.ok(await evaluate(`${marks(0)}[0].getBoundingClientRect().x > ${marks(0)}[4].getBoundingClientRect().x`));
  await press('ArrowLeft', 37);
  await waitFor(`${ranges}[0].dataset.value === '52'`);
  await press('PageUp', 33);
  await waitFor(`${caption}.textContent === '62%' && ${ranges}[0].getAttribute('aria-valuetext') === '62%'`);
  assert.deepEqual(await evaluate(`Array.from(new FormData(${form}))`), [['volume', '62']]);
  await press('PageDown', 34);
  await waitFor(`${ranges}[0].dataset.value === '52'`);
  await evaluate(`${ranges}[1].focus()`);
  await press('ArrowUp', 38);
  await waitFor(`${ranges}[1].dataset.value === '30'`);
  await press('PageUp', 33);
  await waitFor(`${ranges}[1].dataset.value === '80'`);
  await press('PageUp', 33);
  await waitFor(`${ranges}[1].dataset.value === '100'`);
  await press('PageDown', 34);
  await waitFor(`${ranges}[1].dataset.value === '50'`);
  await evaluate(`(() => {
    const input = ${ranges}[2];
    window.fixedVolumePageEvents = [];
    for (const type of ['input', 'change']) {
      input.addEventListener(type, () => window.fixedVolumePageEvents.push([type, input.value]));
    }
    input.focus();
  })()`);
  await press('PageUp', 33);
  await waitFor(`window.fixedVolumePageEvents.length >= 2 && ${ranges}[2].value === '30'`);
  assert.deepEqual(await evaluate('window.fixedVolumePageEvents'), [['input', '40'], ['change', '40']]);
  await press('End', 35);
  await waitFor(`${ranges}[2].value === '30'`);
  assert.deepEqual(await evaluate(`Array.from(new FormData(${form}))`), [['volume', '52']]);
  assert.ok(await evaluate(`${ranges}[3].disabled`));
  await evaluate(`${ranges}[0].focus()`);
  await press('Home', 36);
  await waitFor(`${ranges}[0].dataset.value === '0' && ${marks(0)}.filter(el => el.hasAttribute('data-filled')).length === 1`);
  assert.deepEqual(exceptions, []);
  console.log('Slider: accessible name, arrows and ten-step Page keys, controlled rollback and disabled form exclusion passed');
});
