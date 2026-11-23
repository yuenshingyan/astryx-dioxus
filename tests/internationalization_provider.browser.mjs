import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8237'});
  const nav = `document.querySelector('#internationalization-examples nav')`;
  const toggle = `document.querySelector('#internationalization-examples > button')`;
  await waitFor(`${nav}?.textContent.includes('Próxima página')`);
  await evaluate(`${nav}.lastElementChild.click()`);
  await waitFor(`${nav}.querySelector('[aria-current=page]')?.textContent.trim() === '3'`);
  await evaluate(`${toggle}.click()`);
  await waitFor(`${nav}.textContent.includes('Next page') && ${nav}.textContent.includes('Previous page')`);
  assert.equal(await evaluate(`${nav}.querySelector('[aria-current=page]').textContent.trim()`), '3');
  assert.deepEqual(exceptions, []);
  console.log('InternationalizationProvider: parent locale fallback and reactive language switching passed');
});
