import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8230'});
  const nav = "document.getElementById('workspace-side-nav')";
  await waitFor(`${nav}?.querySelector('[aria-current=page]')`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'navigation' && node.name?.value === 'Workspace navigation'));
  const geometry = await evaluate(`(() => {
    const rail = ${nav};
    const [top, scroll, bottom] = [...rail.children];
    const before = [top.getBoundingClientRect().y, bottom.getBoundingClientRect().y];
    scroll.scrollTop = scroll.scrollHeight;
    return {width: rail.clientWidth, offset: scroll.scrollTop, fixed: before.every((y, i) => y === [top, bottom][i].getBoundingClientRect().y), size: bottom.querySelector('button').dataset.size};
  })()`);
  assert.equal(geometry.width, 260);
  assert.ok(geometry.offset > 0 && geometry.fixed);
  assert.equal(geometry.size, 'sm');
  assert.deepEqual(exceptions, []);
  console.log('SideNav: named landmark, independent middle scroll and footer sizing passed');
});
