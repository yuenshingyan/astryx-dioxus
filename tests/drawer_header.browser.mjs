import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8179'});
  await waitFor(`!!document.getElementById('drawer-header-preview')`);
  assert.deepEqual(await evaluate(`(() => {
    const header = document.getElementById('drawer-header-preview');
    const slot = header.querySelector('.astryx-drawer-header-end-content');
    const style = getComputedStyle(slot);
    return [header.dataset.divider, header.querySelector('h2').textContent, style.marginLeft, style.marginRight, style.marginTop];
  })()`), ['true', 'Host details', '-8px', '0px', '-8px']);
  await evaluate(`Array.from(document.querySelectorAll('#drawer-header-examples button')).find(button => button.textContent.trim() === 'Open host details').click()`);
  const header = `Array.from(document.querySelectorAll('#drawer-header-examples .astryx-drawer-header')).find(node => node.querySelector('h2').textContent === 'web-prod-04')`;
  await waitFor(`!!(${header})`);
  assert.ok(await evaluate(`(${header}).textContent.includes('Healthy') && (${header}).textContent.includes('us-east-1')`));
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'heading' && node.name?.value === 'web-prod-04'));
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === 'Close'));
  await evaluate(`(${header}).querySelector('button[aria-label="Close"]').focus()`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', text: '\r', windowsVirtualKeyCode: 13});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`!(${header})`);
  assert.deepEqual(exceptions, []);
  console.log('DrawerHeader: native heading, close action, divider, and logical RTL compensation passed');
});
