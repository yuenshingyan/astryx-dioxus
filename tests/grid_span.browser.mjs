import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8113'});
  await waitFor('!!document.getElementById("grid-span-columns")');
  assert.deepEqual(await evaluate(`['showcase','columns'].map(id=>document.getElementById('grid-span-'+id).children.length)`), [6,4]);
  for (const dir of ['ltr','rtl']) {
    const result = await evaluate(`(() => {
      const grid = document.getElementById('grid-span-showcase');
      grid.dir = '${dir}';
      grid.style.gridAutoRows = '80px';
      const [wide, single, tall] = grid.children;
      const box = e => e.getBoundingClientRect();
      const css = getComputedStyle(wide);
      return {
        wide: box(wide).width, single: box(single).width, tall: box(tall).height,
        startsFirst: ${dir === 'rtl' ? 'box(wide).right === box(grid).right' : 'box(wide).left === box(grid).left'},
        native: wide.tagName === 'DIV' && !wide.hasAttribute('role') && !wide.hasAttribute('tabindex'),
        innerTrack: css.display === 'grid' && css.minWidth === '0px' && css.gridTemplateColumns === box(wide).width + 'px',
      };
    })()`);
    assert.deepEqual(result, {wide: 297, single: 91, tall: 172, startsFirst: true, native: true, innerTrack: true}, dir);
  }
  assert.equal(await evaluate(`(() => {
    const grid = document.getElementById('grid-with-span');
    const span = grid.lastElementChild;
    return span.classList.contains('astryx-grid-span') && span.getBoundingClientRect().width === grid.getBoundingClientRect().width;
  })()`), true, 'full span covers all explicit tracks');
  console.log('GridSpan: both patterns, numeric/full spans, row heights, RTL and native wrapper passed');
});
