import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8170'});
  await waitFor(`!!document.getElementById('project-table-body')`);
  assert.deepEqual(await evaluate(`(() => {
    const body = document.getElementById('project-table-body');
    const table = body.closest('table');
    return [table.tBodies.length, table.tBodies[0] === body, getComputedStyle(body).display,
      [...table.children].map(child => child.tagName), [...body.rows].map(row => row.rowIndex)];
  })()`), [1, true, 'table-row-group', ['CAPTION', 'THEAD', 'TBODY', 'TFOOT'], [1, 2]]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  const byId = new Map(nodes.map(node => [node.nodeId, node]));
  const table = nodes.find(node => !node.ignored && node.role?.value === 'table' && node.name?.value === 'Project hours');
  assert.ok(table, 'missing project hours table');
  for (const name of ['Documentation', 'Review']) {
    assert.ok(nodes.some(node => {
      if (node.ignored || node.role?.value !== 'rowheader' || node.name?.value !== name) return false;
      for (let parent = byId.get(node.parentId); parent; parent = byId.get(parent.parentId)) {
        if (parent === table) return true;
      }
      return false;
    }), `missing row header: ${name}`);
  }
  assert.deepEqual(exceptions, []);
  console.log('TableBody: native section order and accessible body row headers passed');
});
