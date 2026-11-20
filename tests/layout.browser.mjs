import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8128'});
  await waitFor(`document.getElementById('layout-test-content')?.getAttribute('role') === 'group'`);
  assert.deepEqual(await evaluate(`(() => {
    const root = document.getElementById('layout-slot-changes');
    const panel = document.getElementById('layout-test-panel');
    const content = document.getElementById('layout-test-content');
    const box = root.getBoundingClientRect();
    content.scrollTop = 30;
    return [panel.getBoundingClientRect().left - box.left, box.right - content.getBoundingClientRect().right,
      parseFloat(getComputedStyle(content).paddingInlineEnd), content.scrollTop];
  })()`), [120, 0, 128, 30]);
  await evaluate(`document.getElementById('layout-toggle-panel').click()`);
  await waitFor(`!document.getElementById('layout-test-panel') && document.getElementById('layout-test-content').classList.contains('astryx-layout-content--open-start')`);
  assert.deepEqual(await evaluate(`(() => {
    const content = document.getElementById('layout-test-content');
    const css = getComputedStyle(content);
    return [content.offsetWidth, css.paddingInlineStart, css.paddingInlineEnd];
  })()`), [600, '128px', '128px']);
  assert.deepEqual(await evaluate(`(() => {
    const layout = document.getElementById('layout-dual');
    layout.dir = 'rtl';
    const start = layout.querySelector('.astryx-layout-panel--start');
    const end = layout.querySelector('.astryx-layout-panel--end');
    return [getComputedStyle(start).borderLeftWidth, getComputedStyle(end).borderRightWidth,
      start.getBoundingClientRect().left > end.getBoundingClientRect().left];
  })()`), ['1px', '1px', true]);
  assert.equal(await evaluate(`['basic','content-only','width','dual','bleed','showcase','sidebar'].every(id => document.querySelector('#layout-' + id + ' .astryx-layout-content'))`), true);
  assert.deepEqual(exceptions, []);
  console.log('Layout: seven arrangements, reactive panel removal, edge scrolling, aligned insets, and RTL panel order passed');
});
