import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8161'});
  await waitFor(`!!document.getElementById('input-group-probe')`);
  await evaluate(`document.querySelector('#input-group-examples input[aria-label="URL"]').focus()`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Tab', code: 'Tab', windowsVirtualKeyCode: 9});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Tab', code: 'Tab', windowsVirtualKeyCode: 9});
  await waitFor(`document.activeElement.id === 'input-group-addon'`);
  assert.deepEqual(await evaluate(`(() => {
    const row = document.getElementById('input-group-probe');
    const button = document.getElementById('input-group-addon');
    const style = getComputedStyle(button);
    return [row.dataset.size, row.dataset.status, row.offsetHeight, button.offsetHeight,
      style.borderStartStartRadius, style.borderStartEndRadius, style.zIndex,
      ...row.getAttribute('aria-describedby').split(' ').map(id => document.getElementById(id).textContent.trim())];
  })()`), ['sm', 'warning', 28, 28, '0px', '0px', '2', 'Search your workspace', 'Results may be incomplete']);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'group' && node.name?.value === 'Search' && node.description?.value.includes('Results may be incomplete')));
  assert.deepEqual(exceptions, []);
  console.log('InputGroup: name, descriptions, row height, and direct addon focus stacking passed');
});
