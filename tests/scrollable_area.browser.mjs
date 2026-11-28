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
  const rail = await evaluate(`(() => {
    const viewport = ${get('inline')};
    const content = viewport.querySelector(':scope > [data-scroll-content]');
    const card = document.getElementById('scroll-area-rail-card');
    const heading = document.getElementById('scroll-area-rail-heading');
    const first = content.querySelector('.astryx-card');
    const outer = card.getBoundingClientRect();
    const bounds = viewport.getBoundingClientRect();
    const cardStyle = getComputedStyle(card);
    const contentStyle = getComputedStyle(content);
    return {
      leftGap: bounds.left - outer.left - parseFloat(cardStyle.borderLeftWidth),
      rightGap: outer.right - bounds.right - parseFloat(cardStyle.borderRightWidth),
      headingDelta: first.getBoundingClientRect().left - heading.getBoundingClientRect().left,
      border: parseFloat(cardStyle.borderLeftWidth),
      firstWidth: first.getBoundingClientRect().width,
      blockStart: contentStyle.paddingBlockStart,
      blockEnd: contentStyle.paddingBlockEnd,
      inlineEnd: contentStyle.paddingInlineEnd,
      overflow: viewport.scrollWidth - viewport.clientWidth,
    };
  })()`);
  assert.ok(Math.abs(rail.leftGap) < 1 && Math.abs(rail.rightGap) < 1, JSON.stringify(rail));
  // The Rust Card subtracts its border when publishing insets; rail content uses the full spacing step.
  assert.ok(Math.abs(rail.headingDelta) <= rail.border + 0.5, JSON.stringify(rail));
  assert.equal(rail.firstWidth, 170);
  assert.ok(parseFloat(rail.blockStart) > 0 && parseFloat(rail.blockStart) < parseFloat(rail.blockEnd));
  assert.equal(rail.blockEnd, rail.inlineEnd);
  assert.ok(rail.overflow > 0);
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
