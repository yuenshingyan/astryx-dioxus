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
  }
  assert.equal(await evaluate('document.querySelector("#v-stack-basic > h5").textContent'), 'Weekly Report');
  console.log('VStack: both original patterns, native heading, child order and measured token gaps passed in LTR and RTL');
});
