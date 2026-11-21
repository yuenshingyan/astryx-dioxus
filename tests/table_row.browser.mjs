import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8173'});
  await waitFor(`!!document.getElementById('project-hours-row')`);
  assert.deepEqual(await evaluate(`(() => {
    const row = document.getElementById('project-hours-row');
    const table = row.closest('table');
    return [[...table.rows].map(item => [item.parentElement.tagName, item.className, item.rowIndex]),
      getComputedStyle(row).display, [...row.cells].map(cell => cell.textContent), row.hasAttribute('role')];
  })()`), [[['THEAD', 'astryx-table-row', 0], ['TBODY', 'astryx-table-row', 1],
    ['TBODY', 'astryx-table-row', 2], ['TFOOT', 'astryx-table-row', 3]], 'table-row', ['Documentation', '8'], false]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  const byId = new Map(nodes.map(node => [node.nodeId, node]));
  const header = nodes.find(node => !node.ignored && node.role?.value === 'rowheader' && node.name?.value === 'Documentation');
  assert.equal(byId.get(header?.parentId)?.role?.value, 'row');
  assert.deepEqual(exceptions, []);
  console.log('TableRow: native rows in all sections and accessible row semantics passed');
});
