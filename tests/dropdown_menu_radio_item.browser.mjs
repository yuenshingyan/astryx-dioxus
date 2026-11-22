import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8204'});
  const rows = `Array.from(document.querySelectorAll('[aria-label="Sort reports"] [role="menuitemradio"]'))`;
  await waitFor(`${rows}.length === 3`);
  const state = `${rows}.map(row => [row.getAttribute('aria-checked'), row.tabIndex, row.getAttribute('aria-disabled')])`;
  assert.deepEqual(await evaluate(state), [['true', -1, null], ['false', -1, null], ['false', -1, 'true']]);
  await evaluate(`document.getElementById('sort-oldest').click()`);
  await waitFor(`${rows}[1].getAttribute('aria-checked') === 'true'`);
  await evaluate(`document.getElementById('sort-name').click()`);
  assert.deepEqual(await evaluate(state), [['false', -1, null], ['true', -1, null], ['false', -1, 'true']]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'menuitemradio' && node.name?.value.includes('Oldest first')));
  assert.deepEqual(exceptions, []);
  console.log('DropdownMenuRadioItem: controlled selection, disabled activation, and menuitemradio semantics passed');
});
