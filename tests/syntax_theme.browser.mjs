import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8192'});
  const roots = '#syntax-theme-examples [data-astryx-syntax-theme]';
  await waitFor(`document.querySelectorAll('${roots}').length === 3`);
  assert.deepEqual(await evaluate(`Array.from(document.querySelectorAll('${roots}'), node => [
    node.dataset.astryxSyntaxTheme, getComputedStyle(node.querySelector('pre')).backgroundColor,
    getComputedStyle(node.querySelector('code span')).color,
  ])`), [
    ['one-dark-pro', 'rgb(40, 44, 52)', 'rgb(198, 120, 221)'],
    ['dracula', 'rgb(40, 42, 54)', 'rgb(255, 121, 198)'],
    ['github-light', 'rgb(255, 255, 255)', 'rgb(207, 34, 46)'],
  ]);
  assert.deepEqual(exceptions, []);
  console.log('SyntaxTheme: scoped palette colors passed');
});
