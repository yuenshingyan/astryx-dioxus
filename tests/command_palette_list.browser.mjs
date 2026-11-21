import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8165'});
  await waitFor(`document.querySelectorAll('#command-list-grouped [role=option]').length === 6`);
  assert.deepEqual(await evaluate(`(() => {
    const list = document.getElementById('command-list-grouped');
    const style = getComputedStyle(list);
    list.scrollTop = list.scrollHeight;
    return [style.flexDirection, style.rowGap, style.paddingTop, style.overflowY,
      list.scrollTop > 0, list.querySelectorAll('[role=group]').length];
  })()`), ['column', '2px', '4px', 'auto', true, 2]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  for (const name of ['Commands', 'Grouped commands']) {
    assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'listbox' && node.name?.value === name));
  }
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'option' && node.name?.value === 'Help (unavailable)' &&
    node.properties?.some(property => property.name === 'disabled' && property.value?.value === true)));
  assert.deepEqual(exceptions, []);
  console.log('CommandPaletteList: listbox names, grouped options, disabled option semantics, and scrolling passed');
});
