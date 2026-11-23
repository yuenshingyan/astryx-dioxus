import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls:['https://*']});
  await send('Emulation.setEmulatedMedia', {features: [{name:'prefers-color-scheme', value:'light'}]});
  await send('Page.navigate', {url:process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8238'});
  const card = name => `document.querySelector('#theme-examples [data-astryx-theme="${name}"] .astryx-card')`;
  const color = node => `getComputedStyle(${node}).color`;
  const warm = card('warm-docs');
  const forest = card('forest-docs');
  await waitFor(`${warm} && ${color(warm)} === 'rgb(59, 47, 21)'`);
  assert.equal(await evaluate(color(forest)), 'rgb(5, 46, 22)');
  const rootMode = await evaluate(`document.documentElement.getAttribute('data-theme')`);
  assert.equal(await evaluate(`getComputedStyle(document.getElementById('theme-system-in-dark')).colorScheme`), 'light dark');
  await evaluate(`document.querySelector('#theme-examples > button').click()`);
  await waitFor(`${color(warm)} === 'rgb(254, 243, 199)' && ${color(forest)} === 'rgb(220, 252, 231)'`);
  assert.equal(await evaluate(`document.documentElement.getAttribute('data-theme')`), rootMode);
  assert.deepEqual(exceptions, []);
  console.log('Theme: scoped palette, reactive scheme and nested system reset passed');
});
