import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8217'});
  const input = id => `document.getElementById('${id}')`;
  const dark = input('dark-mode-switch');
  const premium = input('premium-switch');
  const managed = input('managed-switch');
  await waitFor(`${dark}?.role === 'switch'`);
  const center = `${dark}.parentElement.querySelector('.astryx-switch-thumb').getBoundingClientRect().x`;
  const off = await evaluate(center);
  await evaluate(`${dark}.focus()`);
  await send('Input.dispatchKeyEvent', {type:'keyDown', key:' ', code:'Space', windowsVirtualKeyCode:32, text:' '});
  await send('Input.dispatchKeyEvent', {type:'keyUp', key:' ', code:'Space', windowsVirtualKeyCode:32});
  await waitFor(`${dark}.checked && ${dark}.dataset.checked === 'true'`);
  assert.ok(await evaluate(center) < off, 'RTL on thumb moves left');
  const {nodes} = await send('Accessibility.getFullAXTree');
  const darkNode = nodes.find(node => !node.ignored && node.role?.value === 'switch' && node.name?.value === 'Dark mode');
  assert.equal(darkNode?.description?.value, 'Switch to a darker color scheme.');
  assert.equal(darkNode?.properties.find(p => p.name === 'checked')?.value.value, 'true');
  await evaluate(`${premium}.focus(); ${premium}.click(); ${managed}.click()`);
  await waitFor(`!${managed}.checked`);
  assert.deepEqual(await evaluate(`[${premium}.checked, document.activeElement === ${premium}, Array.from(new FormData(document.getElementById('switch-form')))]`), [true, true, []]);
  assert.deepEqual(exceptions, []);
  console.log('Switch: keyboard toggle, controlled rollback, disabled form behavior, AX, and RTL passed');
});
