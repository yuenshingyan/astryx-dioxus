import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Emulation.setEmulatedMedia', {features: [{name: 'prefers-reduced-motion', value: 'reduce'}]});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8176'});
  await waitFor(`document.querySelectorAll('#overlay-examples img').length === 4 && [...document.querySelectorAll('#overlay-examples img')].every(img => img.naturalWidth > 0)`);
  const scrim = id => `document.querySelector('#overlay-${id} > .astryx-overlay-scrim')`;
  assert.deepEqual(await evaluate(`(() => {
    const fill = ${scrim('showcase')};
    const strip = ${scrim('strip')};
    return [fill.offsetHeight === fill.parentElement.clientHeight,
      strip.offsetHeight < strip.parentElement.clientHeight,
      strip.getBoundingClientRect().bottom === strip.parentElement.getBoundingClientRect().bottom,
      fill.querySelector('[data-astryx-media]').dataset.astryxMedia,
      getComputedStyle(fill).transitionDuration];
  })()`), [true, true, true, 'dark', '0s']);
  assert.equal(await evaluate(`getComputedStyle(${scrim('hover')}).visibility`), 'hidden');
  await evaluate(`document.getElementById('overlay-hover').focus()`);
  await waitFor(`getComputedStyle(${scrim('hover')}).visibility === 'visible'`);
  assert.equal(await evaluate(`${scrim('controlled')}.inert`), true);
  await evaluate(`document.getElementById('overlay-control').click()`);
  await waitFor(`!${scrim('controlled')}.inert && getComputedStyle(${scrim('controlled')}).visibility === 'visible'`);
  await evaluate(`document.getElementById('overlay-control').click()`);
  await waitFor(`${scrim('controlled')}.inert && getComputedStyle(${scrim('controlled')}).visibility === 'hidden'`);
  assert.deepEqual(exceptions, []);
  console.log('Overlay: offline images, fill and strip geometry, focus reveal, and controlled visibility passed');
});
