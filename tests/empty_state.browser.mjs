import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8133'});
  await waitFor(`document.readyState === 'complete' && !!document.getElementById('empty-state-dynamic')`);
  const live = `document.querySelector('[data-astryx-live-region="polite"]')`;
  await waitFor(`${live}?.textContent === ''`);
  const root = `document.getElementById('empty-state-dynamic')`;
  const click = () => evaluate(`document.getElementById('empty-state-toggle').click()`);
  assert.deepEqual(await evaluate(`(() => {
    const compact = document.getElementById('empty-state-compact');
    const regular = ${root};
    return [regular.querySelector('h4').textContent, getComputedStyle(compact.querySelector('.astryx-empty-state__actions')).flexDirection,
      getComputedStyle(regular.querySelector('.astryx-empty-state__actions')).flexDirection,
      parseFloat(getComputedStyle(compact).paddingBlockStart) < parseFloat(getComputedStyle(regular).paddingBlockStart),
      compact.firstElementChild.getAttribute('aria-hidden'), compact.hasAttribute('aria-live')];
  })()`), ['Nothing to review', 'column', 'row', true, 'true', false]);
  await click();
  await waitFor(`${live}.textContent === 'No matching results Try another query.'`);
  // Exercise separate announcement turns, not the writer's same-frame batching.
  await waitFor(`${live}.textContent === ''`);
  await click();
  await waitFor(`${live}.textContent === 'No matching results Try fewer filters.'`);
  await waitFor(`${live}.textContent === ''`);
  await click();
  await waitFor(`${root} === null`);
  await click();
  await waitFor(`${live}.textContent === 'Nothing to review Try another query.'`);
  assert.deepEqual(exceptions, []);
  console.log('EmptyState: semantic heading, compact layout, decorative artwork, and title/description live-region writes passed');
});
