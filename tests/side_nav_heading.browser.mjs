import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8148'});
  await waitFor(`!!document.getElementById('side-nav-heading-basic')`);
  assert.deepEqual(await evaluate(`(() => {
    const whole = document.getElementById('side-nav-heading-basic');
    const trailing = document.getElementById('side-nav-heading-trailing');
    trailing.querySelector('button').focus();
    return [whole.tagName, whole.getAttribute('href'), whole.offsetHeight >= 32,
      trailing.tagName, trailing.querySelector('a').contains(document.activeElement),
      document.activeElement.textContent, document.querySelectorAll('#side-nav-heading-examples nav').length];
  })()`), ['A', '/', true, 'DIV', false, 'Settings', 3]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'link' && node.name?.value === 'Analytics'));
  assert.deepEqual(exceptions, []);
  console.log('SideNavHeading: linked rows, trailing focus target, geometry, and accessible link name passed');
});
