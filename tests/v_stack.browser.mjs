import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8117'});
  await waitFor('!!document.getElementById("v-stack-gap-6")');
  for (const dir of ['ltr', 'rtl']) {
    const rows = await evaluate(`['basic','gap-2','gap-4','gap-6'].map(id => {
      const root = document.getElementById('v-stack-'+id);
      root.dir = '${dir}';
      const style = getComputedStyle(root);
      const children = [...root.children];
      const boxes = children.map(el => el.getBoundingClientRect());
      return [style.display, style.flexDirection, style.rowGap,
        children.map(el => el.textContent.trim()),
        boxes.every((box,i) => !i || Math.abs(box.top-boxes[i-1].bottom-parseFloat(style.rowGap)) < 1/64)];
    })`);
    assert.deepEqual(rows[0], ['flex', 'column', '12px', ['Weekly Report', 'VStack arranges its children in a vertical column.', 'The gap prop controls the spacing between each item.'], true]);
    for (const [index, gap] of ['8px','16px','24px'].entries()) {
      assert.deepEqual(rows[index+1], ['flex','column',gap,['Step 1','Step 2','Step 3'],true]);
    }
    const compositions = await evaluate(`['playground','with-items'].map(id => {
      const root = document.getElementById('v-stack-'+id);
      root.dir = '${dir}';
      const style = getComputedStyle(root);
      const children = [...root.children];
      const boxes = children.map(el => el.getBoundingClientRect());
      return {
        layout: [style.display, style.flexDirection, style.rowGap],
        labels: children.map(el => el.textContent.trim()),
        isSpaced: boxes.every((box, i) => !i || Math.abs(box.top - boxes[i-1].bottom - parseFloat(style.rowGap)) < 1/64),
        isExpectedChild: children.every(el => {
          if (id !== 'playground') return el.classList.contains('astryx-item');
          const card = getComputedStyle(el);
          return el.classList.contains('astryx-card') && ['Top','Right','Bottom','Left'].every(edge =>
            Math.abs(parseFloat(card['padding'+edge]) + parseFloat(card['border'+edge+'Width']) - 12) < 1/64);
        }),
      };
    })`);
    for (const [index, labels] of [['Item 1', 'Item 2', 'Item 3'], ['Item 1', 'Item 2']].entries()) {
      assert.deepEqual(compositions[index], {
        layout: ['flex', 'column', '8px'], labels, isSpaced: true, isExpectedChild: true,
      });
    }
  }
  assert.equal(await evaluate('document.querySelector("#v-stack-basic > h5").textContent'), 'Weekly Report');
  console.log('VStack: templates, Card and Item compositions, native heading, child order and measured token gaps passed in LTR and RTL');
});
