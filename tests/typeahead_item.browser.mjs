import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8175'});
  await waitFor(`!!document.getElementById('typeahead-item-narrow')`);
  assert.deepEqual(await evaluate(`(() => {
    const item = document.getElementById('typeahead-item-narrow');
    const [icon, content] = item.children;
    return [document.querySelectorAll('#typeahead-item-basic .astryx-typeahead-item').length,
      document.querySelectorAll('#typeahead-item-people .astryx-avatar').length,
      getComputedStyle(item).opacity, item.tabIndex, item.getAttribute('role'),
      icon.getBoundingClientRect().left > content.getBoundingClientRect().left,
      [...content.children].every(line => line.scrollWidth > line.clientWidth && getComputedStyle(line).textOverflow === 'ellipsis')];
  })()`), [3, 5, '0.5', -1, null, true, true]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  for (const name of ['Alice Johnson', 'Product management and planning']) {
    assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'StaticText' && node.name?.value === name), name);
  }
  assert.deepEqual(exceptions, []);
  console.log('TypeaheadItem: result layout, RTL artwork placement, dimming, and accessible text passed');
});
