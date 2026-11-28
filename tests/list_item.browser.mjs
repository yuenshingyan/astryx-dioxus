import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8203'});
  await waitFor(`document.querySelectorAll('#list-item-features > li').length === 2`);
  assert.deepEqual(await evaluate(`(() => {
    const [first, last] = document.querySelectorAll('#list-item-features > li');
    const number = document.querySelector('#steps-list .astryx-list-item__marker');
    return [getComputedStyle(first).paddingInlineStart, getComputedStyle(first).paddingBlockStart,
      getComputedStyle(first).borderBlockEndWidth, getComputedStyle(last).borderBlockEndWidth,
      first.querySelector('[data-kind]').dataset.kind, number.dataset.kind,
      getComputedStyle(number, '::before').content, getComputedStyle(number.parentElement).counterIncrement];
  })()`), ['0px', '12px', '1px', '0px', 'circle', 'decimal', 'counter(astryx-list) "."', 'astryx-list 1']);
  const counters = await evaluate(`(() => {
    const list = document.querySelector('#steps-list');
    const row = list.querySelector('li');
    const marker = row.querySelector('.astryx-list-item__marker');
    const label = row.querySelector('.astryx-item__label');
    const originalListStyle = list.style.cssText;
    const originalRowStyle = row.style.cssText;
    const markerWidth = () => marker.getBoundingClientRect().width;
    try {
      return ['ltr', 'rtl'].map(direction => {
        list.style.direction = direction;
        row.style.removeProperty('--_item-inset-inline');
        row.style.removeProperty('counter-increment');
        const short = markerWidth();
        row.style.counterIncrement = 'astryx-list 123456789';
        const hanging = markerWidth();
        const inset = parseFloat(getComputedStyle(row).paddingInlineStart);
        const markerStyle = getComputedStyle(marker);
        const labelStyle = getComputedStyle(label);
        const usesBodyMetrics = markerStyle.fontSize === labelStyle.fontSize &&
          markerStyle.lineHeight === labelStyle.lineHeight;
        const whiteSpace = getComputedStyle(marker, '::before').whiteSpace;
        row.style.setProperty('--_item-inset-inline', '0px');
        return {direction, short, hanging, flush: markerWidth(), inset, usesBodyMetrics, whiteSpace};
      });
    } finally {
      list.style.cssText = originalListStyle;
      row.style.cssText = originalRowStyle;
    }
  })()`);
  for (const {direction, short, hanging, flush, inset, usesBodyMetrics, whiteSpace} of counters) {
    assert.ok(usesBodyMetrics, `${direction}: counter should use label typography metrics`);
    assert.equal(whiteSpace, 'nowrap', direction);
    assert.ok(hanging > short && inset > 0, `${direction}: expected a long counter and a nonzero inset`);
    assert.ok(Math.abs(flush - hanging - inset) < 0.1, `${direction}: long counter should use the full inset`);
  }
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'listitem'));
  assert.deepEqual(exceptions, []);
  console.log('ListItem: native row, inherited layout and dividers, and LTR/RTL counter hang passed');
});
