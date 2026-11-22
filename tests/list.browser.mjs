import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8200'});
  await waitFor(`document.querySelectorAll('#steps-list > li').length === 3`);
  assert.deepEqual(await evaluate(`(() => {
    const steps = document.getElementById('steps-list');
    const settings = document.getElementById('settings-list');
    return [steps.tagName, steps.start, steps.dataset.density, settings.tagName,
      getComputedStyle(settings).rowGap, getComputedStyle(settings).overflowX];
  })()`), ['OL', 3, 'compact', 'UL', '2px', 'clip']);
  const {nodes} = await send('Accessibility.getFullAXTree');
  for (const name of ['Ordered steps', 'Settings']) {
    assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'list' && node.name?.value === name), name);
  }
  assert.deepEqual(exceptions, []);
  console.log('List: named native ordered and unordered lists, start, and spacing passed');
});
