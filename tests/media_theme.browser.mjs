import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8147'});
  const root = `document.querySelector('#media-theme-probe > .astryx-media-theme')`;
  await waitFor(`${root}?.dataset.astryxMedia === 'dark' && [...document.querySelectorAll('#media-theme-examples img')].every(img => img.naturalWidth > 0)`);
  assert.deepEqual(await evaluate(`[...document.querySelectorAll('#media-theme-examples .astryx-media-theme')].map(node => [getComputedStyle(node).display, getComputedStyle(node).colorScheme, getComputedStyle(node).color])`), [
    ['contents', 'dark', 'rgb(255, 255, 255)'], ['contents', 'dark', 'rgb(255, 255, 255)'],
    ['contents', 'light', 'rgb(17, 17, 17)'], ['contents', 'dark', 'rgb(255, 255, 255)'],
  ]);
  await evaluate(`window.retained = document.getElementById('media-theme-retained'); retained.value = 'Keep this'`);
  await evaluate(`document.getElementById('media-theme-cycle').click()`);
  await waitFor(`!${root}.hasAttribute('data-astryx-media')`);
  await evaluate(`document.getElementById('media-theme-cycle').click()`);
  await waitFor(`${root}.dataset.astryxMedia === 'dark'`);
  await evaluate(`document.getElementById('media-theme-cycle').click()`);
  await waitFor(`${root}.dataset.astryxMedia === 'light'`);
  await evaluate(`document.getElementById('media-theme-cycle').click()`);
  await waitFor(`!${root}.hasAttribute('data-astryx-media')`);
  assert.equal(await evaluate(`document.getElementById('media-theme-retained') === retained && retained.value === 'Keep this'`), true);
  assert.deepEqual(exceptions, []);
  console.log('MediaTheme: offline images, explicit tokens, automatic contrast, fallback, and retained child identity passed');
});
