import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8177'});
  await waitFor(`!!document.getElementById('heading-menu-dashboard')`);
  assert.deepEqual(await evaluate(`(() => {
    const link = document.getElementById('heading-menu-dashboard');
    const disabled = document.getElementById('heading-menu-disabled');
    return [link.tagName, link.tabIndex, link.draggable, disabled.tagName,
      disabled.hasAttribute('href'), disabled.getAttribute('aria-disabled')];
  })()`), ['A', -1, false, 'A', false, 'true']);
  await evaluate(`document.getElementById('heading-menu-dashboard').click()`);
  await waitFor(`location.hash === '#nav-heading-menu-item-examples'`);
  await evaluate(`document.getElementById('heading-menu-disabled').click()`);
  assert.equal(await evaluate('location.hash'), '#nav-heading-menu-item-examples');
  await evaluate(`document.getElementById('heading-menu-settings').click()`);
  await waitFor(`document.getElementById('heading-menu-selection')?.textContent === 'Settings selected'`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'menuitem' && node.name?.value === 'Dashboard'));
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'menuitem' && node.name?.value === 'Unavailable' && node.properties?.some(property => property.name === 'disabled' && property.value?.value === true)));
  assert.deepEqual(exceptions, []);
  console.log('NavHeadingMenuItem: link navigation, action selection, and disabled semantics passed');
});
