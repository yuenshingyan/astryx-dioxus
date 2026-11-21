import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8151'});
  await waitFor(`!!document.getElementById('side-nav-section-probe')`);
  const assertName = async () => {
    const {nodes} = await send('Accessibility.getFullAXTree');
    assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'group' && node.name?.value === 'Workspace'));
  };
  await assertName();
  assert.deepEqual(await evaluate(`(() => {
    const group = document.getElementById('side-nav-section-probe');
    const header = group.firstElementChild;
    const [title, end] = header.children;
    const [first, second] = group.lastElementChild.children;
    window.sectionLink = first;
    window.sectionName = group.getAttribute('aria-labelledby');
    return [end.getBoundingClientRect().right < title.getBoundingClientRect().left,
      second.getBoundingClientRect().top - first.getBoundingClientRect().bottom,
      document.querySelectorAll('#side-nav-section-examples nav [role=group]').length,
      new Set([...document.querySelectorAll('#side-nav-section-examples [role=group]')].map(node => node.getAttribute('aria-labelledby'))).size];
  })()`), [true, 2, 4, 5]);
  await evaluate(`document.getElementById('side-nav-section-toggle').click()`);
  await waitFor(`getComputedStyle(document.getElementById('side-nav-section-probe').firstElementChild).position === 'absolute'`);
  assert.equal(await evaluate(`(() => {
    const group = document.getElementById('side-nav-section-probe');
    return group.getAttribute('aria-labelledby') === window.sectionName &&
      !!document.getElementById(window.sectionName) && group.querySelector('a') === window.sectionLink;
  })()`), true);
  await assertName();
  assert.deepEqual(exceptions, []);
  console.log('SideNavSection: group name before and after clipping, RTL slot placement, spacing, and item identity passed');
});
