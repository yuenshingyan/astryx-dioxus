import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8284'});
  const frame = "document.getElementById('overflow-list-frame')";
  const names = `Array.from(${frame}?.querySelectorAll('.astryx-overflow-list__row button') ?? [], node => node.textContent.trim())`;
  const exact = labels => `JSON.stringify(${names}) === ${JSON.stringify(JSON.stringify(labels))}`;
  await waitFor(exact(['Edit', 'Duplicate', 'Share', 'Archive', 'Delete']));
  await evaluate(`${frame}.style.width = '170px'`);
  await waitFor(`${names}.at(-1)?.startsWith('+')`);
  const collapsed = await evaluate(names);
  assert.ok(collapsed.length < 5);
  assert.deepEqual(collapsed.slice(0, -1), ['Edit', 'Duplicate', 'Share', 'Archive', 'Delete'].slice(0, collapsed.length - 1));
  assert.equal(collapsed.at(-1), `+${6 - collapsed.length} more`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === collapsed.at(-1)));
  assert.equal(nodes.filter(node => !node.ignored && node.role?.value === 'button' && node.name?.value === collapsed.at(-1)).length, 1);
  assert.ok(await evaluate(`(() => { const row = ${frame}.querySelector('.astryx-overflow-list__row'); return row.lastElementChild.getBoundingClientRect().right <= row.getBoundingClientRect().right + 1; })()`));
  assert.ok(await evaluate(`${frame}.querySelector('[aria-hidden=true][inert]') !== null`));
  await evaluate(`${frame}.style.width = '400px'`);
  await waitFor(exact(['Edit', 'Duplicate', 'Share', 'Archive', 'Delete']));
  assert.deepEqual(exceptions, []);
  console.log('OverflowList: shrink and grow-back, suffix count, inert measurement and accessible indicator passed');
});
