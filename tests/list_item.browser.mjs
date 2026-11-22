import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8203'});
  await waitFor(`document.querySelectorAll('#list-item-features > li').length === 2`);
  assert.deepEqual(await evaluate(`(() => {
    const [first, last] = document.querySelectorAll('#list-item-features > li');
    const number = document.querySelector('#steps-list .astryx-list-item__marker');
    return [getComputedStyle(first).paddingInlineStart, getComputedStyle(first).paddingBlockStart,
      getComputedStyle(first).borderBlockEndWidth, getComputedStyle(last).borderBlockEndWidth,
      first.querySelector('[data-kind]').dataset.kind, number.dataset.kind,
      getComputedStyle(number, '::before').content, getComputedStyle(number.parentElement).counterIncrement];
  })()`), ['0px', '12px', '1px', '0px', 'circle', 'decimal', 'counter(astryx-list) "."', 'astryx-list 1']);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'listitem'));
  assert.deepEqual(exceptions, []);
  console.log('ListItem: native row, inherited flush and spacious layout, dividers and marker styles passed');
});
