import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8172'});
  await waitFor(`!!document.getElementById('project-docs-row')`);
  assert.deepEqual(await evaluate(`['project-hours-column', 'project-docs-row'].map(id => {
    const cell = document.getElementById(id);
    return [cell.tagName, cell.scope, cell.className, cell.hasAttribute('role')];
  })`), [['TH', 'col', 'astryx-table-header-cell', false], ['TH', 'row', 'astryx-table-header-cell', false]]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  const byId = new Map(nodes.map(node => [node.nodeId, node]));
  for (const [role, name] of [['columnheader', 'Hours'], ['rowheader', 'Documentation']]) {
    assert.ok(nodes.some(node => {
      if (node.ignored || node.role?.value !== role || node.name?.value !== name) return false;
      for (let parent = byId.get(node.parentId); parent; parent = byId.get(parent.parentId)) {
        if (parent.role?.value === 'table' && parent.name?.value === 'Project hours') return true;
      }
      return false;
    }), `missing ${role}: ${name}`);
  }
  assert.deepEqual(exceptions, []);
  console.log('TableHeaderCell: native scopes and accessible row and column headers passed');
});
