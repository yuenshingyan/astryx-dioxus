import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8201'});
  await waitFor(`!!document.getElementById('item-inline-preview')`);
  assert.deepEqual(await evaluate(`(() => {
    const row = document.getElementById('item-inline-preview');
    const label = row.querySelector('.astryx-item__label');
    const description = row.querySelector('.astryx-item__description');
    const end = row.querySelector('.astryx-item__end');
    const result = [row.hasAttribute('tabindex'), getComputedStyle(row).paddingBlockStart,
      label.scrollWidth === label.clientWidth, description.scrollWidth > description.clientWidth,
      getComputedStyle(description).textOverflow, end.textContent];
    row.dir = 'rtl';
    result.push(end.getBoundingClientRect().left < label.getBoundingClientRect().left);
    return result;
  })()`), [false, '4px', true, true, 'ellipsis', 'Draft', true]);
  assert.deepEqual(exceptions, []);
  console.log('Item: static row, compact padding, description ellipsis, and RTL slot order passed');
});
