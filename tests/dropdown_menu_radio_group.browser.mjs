import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8191'});
  await waitFor(`!!document.getElementById('sort-options')`);
  assert.deepEqual(await evaluate(`(() => {
    const group = document.getElementById('sort-options');
    const css = getComputedStyle(group);
    return [group.getAttribute('role'), group.getAttribute('aria-label'), group.hasAttribute('tabindex'), css.flexDirection, css.rowGap];
  })()`), ['group', 'Sort by', false, 'column', '2px']);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'group' && node.name?.value === 'Sort options'));
  await evaluate(`document.querySelector('#sort-options input[value="oldest"]').click()`);
  await waitFor(`document.querySelector('#sort-options input:checked').value === 'oldest'`);
  assert.deepEqual(exceptions, []);
  console.log('DropdownMenuRadioGroup: accessible group name, spacing, and native preview selection passed');
});
