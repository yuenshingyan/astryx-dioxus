import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8206'});
  await waitFor(`!!document.getElementById('menu-delete-project')`);
  const result = `document.getElementById('project-menu-result').textContent`;
  await evaluate(`document.getElementById('menu-edit-project').click()`);
  await waitFor(`${result} === 'Edit requested'`);
  await evaluate(`document.getElementById('menu-archive-project').click()`);
  assert.equal(await evaluate(result), 'Edit requested');
  await evaluate(`document.getElementById('menu-delete-project').click()`);
  await waitFor(`${result} === 'Delete requested'`);
  assert.deepEqual(await evaluate(`(() => {
    const row = document.getElementById('menu-delete-project');
    return [row.tabIndex, getComputedStyle(row).color === getComputedStyle(row.querySelector('.astryx-item__label')).color];
  })()`), [-1, true]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'menuitem' && node.name?.value === 'Delete project'));
  assert.deepEqual(exceptions, []);
  console.log('DropdownMenuItem: action and disabled behavior, destructive foreground and menu semantics passed');
});
