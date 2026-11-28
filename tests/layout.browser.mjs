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
  assert.deepEqual(await evaluate(`(() => {
    const root = document.getElementById('layout-arbitrary-content').getBoundingClientRect();
    const lane = document.getElementById('layout-arbitrary-lane').closest('.astryx-layout__middle').getBoundingClientRect();
    return [lane.left - root.left, root.right - lane.right, lane.width];
  })()`), [120, 120, 360], 'LayoutContent in a side panel must not uncap arbitrary main content');
  assert.deepEqual(await evaluate(`['layout-string-zero', 'layout-numeric-zero'].map(id => {
    const content = document.getElementById(id).querySelector('.astryx-layout-content');
    const css = getComputedStyle(content);
    return [content.offsetWidth, css.paddingInlineStart, css.paddingInlineEnd];
  })`), [[600, '300px', '300px'], [600, '300px', '300px']]);

  const heights = await evaluate(`['layout-fill-probe', 'layout-auto-probe'].map(id => {
    const layout = document.getElementById(id);
    const css = getComputedStyle(layout);
    const box = layout.getBoundingClientRect();
    const surface = layout.closest('.astryx-section').getBoundingClientRect();
    const content = layout.querySelector('.astryx-layout-content');
    const inner = getComputedStyle(layout.firstElementChild);
    return {
      margins: [css.marginInlineStart, css.marginInlineEnd, css.marginBlockStart, css.marginBlockEnd],
      offsets: [box.left - surface.left, surface.right - box.right, box.top - surface.top],
      publication: ['inline-start', 'inline-end', 'block-start', 'block-end'].map(edge =>
        inner.getPropertyValue('--container-padding-' + edge).trim()),
      flush: getComputedStyle(content).paddingBlockStart,
      height: box.height,
      scrolls: content.scrollHeight > content.clientHeight,
    };
  })`);
  for (const state of heights) {
    assert.deepEqual(state.margins, ['-8px', '-12px', '-16px', '-24px']);
    assert.deepEqual(state.offsets, [0, 0, 0]);
    assert.deepEqual(state.publication, ['0px', '0px', '0px', '0px']);
    assert.equal(state.flush, '0px', 'a dividerless adjacent header consumes the gap');
  }
  assert.equal(heights[0].height, 180);
  assert.equal(heights[0].scrolls, true);
  assert.ok(heights[1].height > 300);
  assert.equal(heights[1].scrolls, false);
  assert.deepEqual(await evaluate(`(() => {
    const outer = document.getElementById('layout-outer-inset').firstElementChild;
    const nested = document.querySelector('#layout-nested-inset .astryx-layout-header__inner');
    return [getComputedStyle(outer).paddingInlineStart, getComputedStyle(nested).paddingInlineStart,
      getComputedStyle(nested).maxWidth];
  })()`), ['0px', '24px', 'none']);

  const {root} = await send('DOM.getDocument');
  const {nodeId} = await send('DOM.querySelector', {nodeId: root.nodeId, selector: '#layout-nested-inset'});
  const {nodes: [nested]} = await send('Accessibility.getPartialAXTree', {nodeId, fetchRelatives: false});
  assert.equal(nested.role?.value, 'region');
  assert.equal(nested.name?.value, 'Nested workspace');
  assert.deepEqual(exceptions, []);
  console.log('Layout: arrangements, slot changes, geometry/insets and caller-owned Chrome AX landmark passed');
});
