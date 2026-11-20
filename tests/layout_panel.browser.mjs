import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8127'});
  const panel = `document.getElementById('layout-panel-scroll')`;
  await waitFor(`${panel}?.getAttribute('role') === 'group'`);
  assert.deepEqual(await evaluate(`(() => {
    const node = ${panel};
    node.scrollTop = 30;
    return [node.scrollTop, node.offsetWidth, node.getAttribute('aria-label'), node.hasAttribute('tabindex')];
  })()`), [30, 140, 'Scrollable panel example', false]);
  await evaluate(`document.getElementById('layout-panel-toggle-width').click()`);
  await waitFor(`${panel}.offsetWidth === 0 && !${panel}.hasAttribute('role')`);
  assert.deepEqual(await evaluate(`(() => {
    const node = ${panel}, css = getComputedStyle(node);
    return [css.overflow, css.paddingInlineStart, css.paddingInlineEnd, node.getAttribute('aria-label')];
  })()`), ['clip', '0px', '0px', null]);
  await evaluate(`document.getElementById('layout-panel-toggle-width').click()`);
  await waitFor(`${panel}.getAttribute('role') === 'group'`);
  assert.deepEqual(await evaluate(`['navigation','showcase'].map(example => {
    const node = document.getElementById('layout-panel-' + example);
    node.dir = 'rtl';
    node.style.setProperty('--layout-padding-own-outer-x', '20px');
    node.style.setProperty('--layout-padding-inner-x', '8px');
    const css = getComputedStyle(node);
    return [node.getAttribute('role'), node.offsetWidth, css.borderLeftWidth, css.borderRightWidth, css.paddingRight, css.paddingLeft,
      [...node.querySelectorAll('li')].map(item => item.textContent.trim())];
  })`), [
    ['navigation', 140, '1px', '0px', '20px', '8px', ['Overview','Analytics','Settings']],
    ['navigation', 140, '1px', '0px', '20px', '8px', ['Overview','Analytics','Reports','Settings']],
  ]);
  assert.deepEqual(exceptions, []);
  console.log('LayoutPanel: both arrangements, RTL start-slot inset and divider, scrolling, naming, and zero-width clipping passed');
});
