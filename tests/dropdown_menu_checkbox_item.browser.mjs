import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8205'});
  const rows = `Array.from(document.querySelectorAll('#dropdown-menu-checkbox-item-examples [role="menuitemcheckbox"]'))`;
  await waitFor(`${rows}.length === 3`);
  const state = `${rows}.map(row => [row.getAttribute('aria-checked'), row.tabIndex, row.getAttribute('aria-disabled')])`;
  assert.deepEqual(await evaluate(state), [['true', -1, null], ['false', -1, null], ['false', -1, 'true']]);
  await evaluate(`document.getElementById('menu-checkbox-labels').click()`);
  await waitFor(`${rows}[1].getAttribute('aria-checked') === 'true'`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'menuitemcheckbox' && node.name?.value === 'Show labels'));
  await evaluate(`document.getElementById('menu-checkbox-grid').click()`);
  await waitFor(`${rows}[0].getAttribute('aria-checked') === 'false'`);
  await evaluate(`document.getElementById('menu-checkbox-guides').click()`);
  assert.deepEqual(await evaluate(state), [['false', -1, null], ['true', -1, null], ['false', -1, 'true']]);
  assert.deepEqual(exceptions, []);
  console.log('DropdownMenuCheckboxItem: independent toggles, disabled activation and menu semantics passed');
});
