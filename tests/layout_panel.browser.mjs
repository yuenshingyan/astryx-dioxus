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
  async function ax(id) {
    const {root} = await send('DOM.getDocument');
    const {nodeId} = await send('DOM.querySelector', {nodeId: root.nodeId, selector: `#${id}`});
    const {nodes: [node]} = await send('Accessibility.getPartialAXTree', {nodeId, fetchRelatives: false});
    return [node.role?.value, node.name?.value];
  }
  assert.deepEqual(await ax('layout-panel-scroll'), ['group', 'Scrollable panel example']);
  assert.deepEqual(await ax('layout-panel-divider'), ['complementary', 'Inspector']);
  await evaluate(`document.getElementById('layout-panel-toggle-width').click()`);
  await waitFor(`${panel}.offsetWidth === 0 && !${panel}.hasAttribute('role')`);
  assert.deepEqual(await evaluate(`(() => {
    const node = ${panel}, css = getComputedStyle(node);
    return [css.overflow, css.paddingInlineStart, css.paddingInlineEnd, node.getAttribute('aria-label')];
  })()`), ['clip', '0px', '0px', null]);
  const divided = `document.getElementById('layout-panel-divider')`;
  await waitFor(`${divided}?.classList.contains('astryx-layout-panel--zero-width')`);
  assert.deepEqual(await evaluate(`(() => {
    const node = ${divided}, css = getComputedStyle(node);
    return [node.getBoundingClientRect().width, css.borderInlineStartWidth, css.borderInlineEndWidth,
      css.paddingInlineStart, css.paddingInlineEnd, css.overflow, node.hasAttribute('inert')];
  })()`), [0, '0px', '0px', '0px', '0px', 'clip', false], 'numeric zero with a requested divider');
  assert.deepEqual(await ax('layout-panel-divider'), ['complementary', 'Inspector']);
  await evaluate(`document.getElementById('layout-panel-toggle-width').click()`);
  await waitFor(`${panel}.getAttribute('role') === 'group'`);
  await waitFor(`${divided}.offsetWidth === 160`);

  for (const dir of ['ltr', 'rtl']) {
    const geometry = await evaluate(`(() => {
      document.getElementById('layout-panel-geometry').dir = '${dir}';
      const neighbor = document.getElementById('layout-panel-neighbor');
      const middle = neighbor.parentElement.parentElement;
      const rowWidth = middle.getBoundingClientRect().width;
      const rem = parseFloat(getComputedStyle(document.documentElement).fontSize);
      const read = (part) => {
        const node = document.getElementById('layout-panel-' + part);
        const box = node.getBoundingClientRect(), css = getComputedStyle(node);
        const bleed = node.firstElementChild.getBoundingClientRect();
        return {
          width: box.width,
          inset: [css.paddingInlineStart, css.paddingInlineEnd, css.paddingBlockStart, css.paddingBlockEnd].map(v => parseFloat(v) / rem),
          margins: [css.marginInlineStart, css.marginInlineEnd].map(v => parseFloat(v) / rem),
          physicalBorder: [css.borderLeftWidth, css.borderRightWidth],
          dividerPaint: css.borderInlineStartStyle === 'solid' && css.borderInlineStartColor === getComputedStyle(neighbor).color,
          bleedAligned: Math.abs(bleed.left - box.left - parseFloat(css.borderLeftWidth)) < 0.1 &&
            Math.abs(box.right - bleed.right - parseFloat(css.borderRightWidth)) < 0.1,
        };
      };
      const gap = document.getElementById('layout-panel-gap').getBoundingClientRect();
      const content = neighbor.getBoundingClientRect();
      return {
        rowWidth,
        overlap: ('${dir}' === 'ltr' ? gap.right - content.left : content.right - gap.left) / rem,
        panels: ['gap', 'divider', 'explicit', 'bleed'].map(read),
      };
    })()`);
    // The first row is narrower than its fixed-width panels, even after the shared inset.
    assert.ok(geometry.rowWidth < 112 + 160, `${dir}: panels are under flex pressure`);
    assert.equal(geometry.overlap, 0.5, `${dir}: the start panel overlaps the adjacent content by one inner inset`);
    assert.deepEqual(geometry.panels, [
      {width: 112, inset: [1.5, 0.5, 0.5, 1.5], margins: [0, -0.5], physicalBorder: ['0px', '0px'], dividerPaint: false, bleedAligned: true},
      {width: 160, inset: [0.5, 1.5, 0.5, 1.5], margins: [0, 0], physicalBorder: dir === 'ltr' ? ['1px', '0px'] : ['0px', '1px'], dividerPaint: true, bleedAligned: true},
      {width: 112, inset: [0.375, 0.375, 0.375, 0.375], margins: [0, 0], physicalBorder: ['0px', '0px'], dividerPaint: false, bleedAligned: true},
      {width: 112, inset: [0, 0, 0, 0], margins: [0, 0], physicalBorder: ['0px', '0px'], dividerPaint: false, bleedAligned: true},
    ], `${dir}: automatic and explicit insets, divider, and child bleed`);
  }
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
  console.log('LayoutPanel: arrangements, LTR/RTL geometry, computed divider paint, child bleed, AX naming, and zero-width clipping passed');
});
