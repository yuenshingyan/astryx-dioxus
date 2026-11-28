import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8142'});
  await send('Page.bringToFront');
  const get = id => `document.getElementById('scroll-area-${id}')`;
  await waitFor(`${get('both')}?.dataset.scrollableBlock === 'true' && ${get('both')}.dataset.scrollableInline === 'true'`);
  assert.deepEqual(await evaluate(`['block','inline','both','pass','hold','allow','contain'].map(id => {
    const el = document.getElementById('scroll-area-' + id), css = getComputedStyle(el);
    return [css.overflowX, css.overflowY, css.overscrollBehaviorY, el.getAttribute('tabindex')];
  })`), [
    ['hidden','auto','auto','0'], ['auto','hidden','auto','0'], ['auto','auto','auto','0'],
    ['clip','clip','auto',null], ['hidden','auto','auto',null], ['hidden','auto','auto','0'], ['hidden','auto','contain','0'],
  ]);
  const {viewportType, contentType, supportsScrollState} = await evaluate(`(() => {
    const viewport = ${get('both')};
    const content = viewport.querySelector(':scope > [data-scroll-content]');
    const containerType = el => getComputedStyle(el).getPropertyValue('container-type') || 'normal';
    return {
      viewportType: containerType(viewport),
      contentType: containerType(content),
      supportsScrollState: CSS.supports('container-type', 'scroll-state'),
    };
  })()`);
  assert.equal(viewportType, supportsScrollState ? 'scroll-state' : 'normal');
  assert.equal(contentType, 'normal');
  await evaluate(`${get('both')}.scrollTo(10000, 10000)`);
  await waitFor(`${get('both')}.dataset.scrollBlockEnd === 'true' && ${get('both')}.dataset.scrollInlineEnd === 'true'`);
  await evaluate(`${get('change')}.focus(); document.getElementById('scroll-area-toggle').click()`);
  await waitFor(`${get('change')}.getAttribute('tabindex') === '-1'`);
  assert.equal(await evaluate(`document.activeElement === ${get('change')}`), true);
  await evaluate(`${get('change')}.blur()`);
  await waitFor(`!${get('change')}.hasAttribute('tabindex')`);
  await evaluate(`${get('inline')}.setAttribute('dir', 'rtl'); ${get('inline')}.scrollLeft = -100000`);
  await waitFor(`${get('inline')}.dataset.scrollInlineEnd === 'true' && ${get('inline')}.scrollLeft < 0`);
  await evaluate(`(() => {
    const el = ${get('change')};
    el.style.writingMode = 'vertical-rl';
    el.querySelector('[data-scroll-content] > div').style.width = '480px';
  })()`);
  await waitFor(`${get('change')}.dataset.scrollableBlock === 'true' && getComputedStyle(${get('change')}).overflowX === 'auto'`);
  assert.equal(await evaluate(`getComputedStyle(${get('change')}).overflowY`), 'hidden');
  assert.deepEqual(exceptions, []);
  console.log('ScrollableArea: native axes, edge flags, sticky containment, overscroll CSS, RTL, and focus retention passed');
});
