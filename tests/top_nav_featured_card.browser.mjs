import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8153'});
  await waitFor(`document.querySelector('#featured-card-rtl img')?.naturalWidth > 0`);
  assert.deepEqual(await evaluate(`(() => {
    const card = document.querySelector('#featured-card-rtl');
    const img = card.querySelector('img');
    const link = card.querySelector('a');
    const [label, arrow] = link.children;
    const ltr = document.querySelector('#release a').lastElementChild;
    return [img.offsetHeight, img.offsetWidth === card.clientWidth, getComputedStyle(img).objectFit,
      img.alt, img.getAttribute('aria-hidden'), getComputedStyle(ltr).transform,
      getComputedStyle(arrow).transform, arrow.getBoundingClientRect().right <= label.getBoundingClientRect().left];
  })()`), [140, true, 'cover', '', 'true', 'none', 'matrix(-1, 0, 0, 1, 0, 0)', true]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  for (const name of ['Explore the release', 'اقرأ المزيد']) {
    assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'link' && node.name?.value === name));
  }
  await evaluate(`document.querySelector('#featured-card-rtl a').click()`);
  await waitFor(`location.hash === '#release'`);
  assert.deepEqual(exceptions, []);
  console.log('Featured card: offline image, decorative semantics, direction, link names, and navigation passed');
});
