import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8143'});
  const divider = `document.getElementById('dropdown-menu-divider-demo')`;
  await waitFor(`!!${divider}`);
  assert.deepEqual(await evaluate(`(() => {
    const node = ${divider};
    const css = getComputedStyle(node);
    return [node.getAttribute('role'), node.getAttribute('aria-orientation'), node.tabIndex,
      css.marginBlockStart, css.marginBlockEnd, css.getPropertyValue('--spacing-1').trim(),
      node.querySelector('.astryx-divider__line').getBoundingClientRect().width > 0];
  })()`), ['separator', 'horizontal', -1, '4px', '4px', '0.25rem', true]);
  const {root} = await send('DOM.getDocument');
  const {nodeId} = await send('DOM.querySelector', {nodeId: root.nodeId, selector: '#dropdown-menu-divider-demo'});
  const {nodes} = await send('Accessibility.getPartialAXTree', {nodeId, fetchRelatives: false});
  assert.equal(nodes[0].role.value, 'separator');
  assert.deepEqual(exceptions, []);
  console.log('DropdownMenuDivider: native separator semantics, block spacing and line geometry passed');
});
