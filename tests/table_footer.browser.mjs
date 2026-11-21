import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8169'});
  await waitFor(`!!document.getElementById('project-table-footer')`);
  assert.deepEqual(await evaluate(`(() => {
    const footer = document.getElementById('project-table-footer');
    const table = footer.closest('table');
    return [table.tFoot === footer, getComputedStyle(footer).display, footer.rows[0].rowIndex,
      [...footer.rows[0].cells].map(cell => [cell.tagName, cell.textContent])];
  })()`), [true, 'table-footer-group', 3, [['TH', 'Total'], ['TD', '12']]]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  const byId = new Map(nodes.map(node => [node.nodeId, node]));
  const table = nodes.find(node => !node.ignored && node.role?.value === 'table' && node.name?.value === 'Project hours');
  assert.ok(table, 'missing named project hours table');
  function insideTable(node) {
    for (let parent = byId.get(node.parentId); parent; parent = byId.get(parent.parentId)) {
      if (parent === table) return true;
    }
    return false;
  }
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'rowheader' && node.name?.value === 'Total' && insideTable(node)));
  assert.deepEqual(exceptions, []);
  console.log('TableFooter: native row order and accessible summary heading passed');
});
