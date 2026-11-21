import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Emulation.setEmulatedMedia', {features: [{name: 'prefers-reduced-motion', value: 'reduce'}]});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8152'});
  const button = `document.getElementById('collapse-workspace')`;
  await waitFor(`${button}?.getAttribute('aria-label') === 'Collapse sidebar'`);
  const direction = () => evaluate(`(() => {
    const path = ${button}.querySelector('svg path');
    const matrix = path.getScreenCTM();
    const tip = path.getPointAtLength(path.getTotalLength() / 2).matrixTransform(matrix);
    const tail = path.getPointAtLength(0).matrixTransform(matrix);
    return tip.x < tail.x ? 'left' : 'right';
  })()`);
  assert.equal(await direction(), 'left', 'LTR collapse points left');
  await evaluate(`document.getElementById('collapse-cancel').click(); ${button}.click()`);
  assert.equal(await evaluate(`${button}.getAttribute('aria-label')`), 'Collapse sidebar');
  await evaluate(`document.getElementById('collapse-cancel').click(); ${button}.focus()`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', text: '\r', windowsVirtualKeyCode: 13});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`${button}.getAttribute('aria-label') === 'Expand sidebar'`);
  assert.deepEqual(await evaluate(`(() => {
    const button = ${button};
    const mirror = button.querySelector('.astryx-side-nav-collapse-button__mirror');
    const chevron = getComputedStyle(mirror.firstElementChild);
    return [button === document.activeElement, getComputedStyle(mirror).transform,
      chevron.transform, chevron.transitionDuration,
      document.getElementById('collapse-basic').getAttribute('aria-label')];
  })()`), [true, 'none', 'matrix(-1, 0, 0, -1, 0, 0)', '0s', 'Collapse sidebar']);
  assert.equal(await direction(), 'right', 'LTR expand points right');
  await evaluate(`${button}.closest('nav').dir = 'rtl'`);
  assert.equal(await direction(), 'left', 'RTL expand points left');
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === 'Expand sidebar'));
  await evaluate(`${button}.click()`);
  await waitFor(`${button}.getAttribute('aria-label') === 'Collapse sidebar'`);
  assert.equal(await direction(), 'right', 'RTL collapse points right');
  assert.deepEqual(exceptions, []);
  console.log('SideNavCollapseButton: keyboard toggle, cancellation, state names, RTL transform, and reduced motion passed');
});
