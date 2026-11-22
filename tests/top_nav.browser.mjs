import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8229'});
  const main = "document.getElementById('main-navigation-preview')";
  const balanced = "document.getElementById('balanced-navigation-preview')";
  await waitFor(`${main}?.querySelector('[aria-current=page]')`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'navigation' && node.name?.value === 'Main navigation'));
  const layout = await evaluate(`(() => {
    const nav = ${balanced}; nav.style.width = '720px';
    const [leading, center, end] = [...nav.children].map(node => node.getBoundingClientRect());
    const rect = nav.getBoundingClientRect();
    return [leading.x > end.x, Math.abs(center.x + center.width / 2 - (rect.x + rect.width / 2))];
  })()`);
  assert.ok(layout[0], 'leading slot is on the right in RTL');
  assert.ok(layout[1] <= 1, 'center slot stays centered with room for all slots');
  const wraps = await evaluate(`(() => {
    const nav = ${main}; nav.style.width = '300px';
    return nav.lastElementChild.getBoundingClientRect().top > nav.firstElementChild.getBoundingClientRect().top;
  })()`);
  assert.ok(wraps, 'end slot wraps below leading content when constrained');
  assert.deepEqual(exceptions, []);
  console.log('TopNav: named landmark, balanced RTL center, and narrow slot wrapping passed');
});
