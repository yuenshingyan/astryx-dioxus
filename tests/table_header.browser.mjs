import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8168'});
  await waitFor(`!!document.getElementById('project-table-header')`);
  assert.deepEqual(await evaluate(`(() => {
    const head = document.getElementById('project-table-header');
    const table = head.closest('table');
    return [table.tHead === head, getComputedStyle(head).display, head.rows.length,
      [...head.rows[0].cells].map(cell => [cell.tagName, cell.scope, cell.textContent]),
      table.tBodies[0].rows.length];
  })()`), [true, 'table-header-group', 1, [['TH', 'col', 'Project'], ['TH', 'col', 'Status']], 1]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  const table = nodes.find(node => !node.ignored && node.role?.value === 'table' && node.name?.value === 'Project status');
  assert.ok(table, 'missing named project table');
  const byId = new Map(nodes.map(node => [node.nodeId, node]));
  const descendants = node => (node.childIds ?? []).flatMap(id => {
    const child = byId.get(id);
    return child ? [child, ...descendants(child)] : [];
  });
  assert.deepEqual(descendants(table).filter(node => !node.ignored && node.role?.value === 'columnheader').map(node => node.name.value), ['Project', 'Status']);
  assert.deepEqual(exceptions, []);
  console.log('TableHeader: native table structure and accessible column headers passed');
});
