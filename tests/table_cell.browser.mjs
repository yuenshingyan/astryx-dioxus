import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8171'});
  await waitFor(`!!document.getElementById('project-hours-cell')`);
  assert.deepEqual(await evaluate(`(() => {
    const cell = document.getElementById('project-hours-cell');
    return [cell.tagName, getComputedStyle(cell).display, cell.cellIndex, cell.hasAttribute('role'),
      cell.headers.split(' ').map(id => document.getElementById(id).textContent)];
  })()`), ['TD', 'table-cell', 1, false, ['Documentation', 'Hours']]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  const byId = new Map(nodes.map(node => [node.nodeId, node]));
  assert.ok(nodes.some(node => {
    if (node.ignored || node.role?.value !== 'cell' || node.name?.value !== '8') return false;
    for (let parent = byId.get(node.parentId); parent; parent = byId.get(parent.parentId)) {
      if (parent.role?.value === 'table' && parent.name?.value === 'Project hours') return true;
    }
    return false;
  }), 'the hours value is exposed as a native cell in the project table');
  assert.deepEqual(exceptions, []);
  console.log('TableCell: native cell, header references, and accessibility tree passed');
});
