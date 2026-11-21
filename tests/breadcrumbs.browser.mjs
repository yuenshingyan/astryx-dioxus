import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8138'});
  const root = `document.getElementById('breadcrumbs-reactive')`;
  await waitFor(`${root}?.querySelector('a[aria-current="page"]')`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'navigation' && node.name?.value === 'Reactive breadcrumb'));
  assert.deepEqual(await evaluate(`(() => {
    const trail = document.getElementById('breadcrumbs-default');
    return [trail.tagName, trail.getAttribute('aria-label'), trail.firstElementChild.tagName,
      getComputedStyle(trail.firstElementChild).listStyleType];
  })()`), ['NAV', 'Breadcrumb', 'OL', 'none']);
  assert.deepEqual(await evaluate(`['Chevron', 'Arrow', 'Dot'].map(name => {
    const trail = document.querySelector('nav[aria-label="Breadcrumb, ' + name + ' separator"]');
    return [...trail.querySelectorAll('.astryx-breadcrumb-item__separator')].map(node => node.textContent.trim());
  })`), ['›', '→', '·'].map(mark => [mark, mark, mark]));
  assert.ok(await evaluate(`(() => {
    const trail = document.getElementById('breadcrumbs-deep');
    trail.style.width = '140px';
    return new Set([...trail.querySelectorAll('li')].map(item => item.getBoundingClientRect().top)).size > 1;
  })()`));
  await evaluate(`window.lastCrumb = ${root}.querySelector('a[aria-current]'); document.getElementById('breadcrumbs-toggle').click()`);
  await waitFor(`${root}.querySelector('li')?.dataset.variant === 'supporting'`);
  assert.deepEqual(await evaluate(`[${root}.dataset.variant, ${root}.querySelector('li:nth-child(2) .astryx-breadcrumb-item__separator').textContent.trim(), ${root}.querySelector('a[aria-current]') === lastCrumb]`), ['supporting', '·', true]);
  assert.deepEqual(exceptions, []);
  console.log('Breadcrumbs: landmark name, repeated separators, narrow wrapping, reactive styling, and current link identity passed');
});
