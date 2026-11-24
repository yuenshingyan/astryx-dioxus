import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8259'});
  const group = name => `document.querySelector('#segmented-control-examples [aria-label="${name}"][role=radiogroup]')`;
  const option = (name, value) => `${group(name)}.querySelector('[data-value="${value}"]')`;
  const checked = (name, value) => `${option(name, value)}.getAttribute('aria-checked') === 'true'`;
  const press = async (key, windowsVirtualKeyCode) => {
    for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key, code:key, windowsVirtualKeyCode});
  };
  await waitFor(group('View mode'));
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'radio' && node.name?.value === 'Timeline'));
  await evaluate(`${option('View mode', 'grid')}.focus()`);
  await press('ArrowRight', 39);
  await waitFor(checked('View mode', 'list'));
  await press('End', 35);
  await waitFor(checked('View mode', 'table'));
  assert.equal(await evaluate(`${group('View mode')}.querySelectorAll('[tabindex="0"]').length`), 1);
  await evaluate(`${option('Board views', 'board')}.focus()`);
  await press('ArrowLeft', 37);
  await waitFor(checked('Board views', 'timeline'));
  assert.equal(await evaluate(`${option('Board views', 'chart')}.getAttribute('aria-disabled')`), 'true');
  await evaluate(`${option('Board views', 'chart')}.click(); ${option('Board views', 'list')}.click()`);
  assert.ok(await evaluate(checked('Board views', 'timeline')));
  assert.deepEqual(exceptions, []);
  console.log('SegmentedControl: radio names, roving stop, RTL, disabled skipping, and caller cancellation passed');
});
