import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8174'});
  const table = `document.getElementById('quarterly-table')`;
  await waitFor(`${table}?.closest('.astryx-table-scroll-wrapper').tabIndex === 0`);
  assert.deepEqual(await evaluate(`(() => {
    const table = ${table};
    const viewport = table.closest('.astryx-table-scroll-wrapper');
    const first = getComputedStyle(table.tBodies[0].rows[0].cells[0]);
    const last = getComputedStyle(table.tBodies[0].rows[4].cells[4]);
    viewport.scrollLeft = 60;
    return [viewport.scrollLeft > 0, table.rows.length, first.paddingBlockStart, first.paddingInlineStart,
      first.borderBlockEndWidth, first.borderInlineEndWidth, last.borderBlockEndWidth, last.borderInlineEndWidth,
      document.querySelector('#table-examples .astryx-table-scroll-wrapper').tabIndex];
  })()`), [true, 6, '4px', '8px', '1px', '1px', '0px', '0px', -1]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  for (const role of ['group', 'table']) {
    assert.ok(nodes.some(node => !node.ignored && node.role?.value === role && node.name?.value === 'Grid dividers'));
  }
  assert.deepEqual(exceptions, []);
  console.log('Table: native sections, compact grid geometry, named overflow and focus gating passed');
});
