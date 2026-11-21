import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8145'});
  const labels = `document.getElementById('form-layout-horizontal-labels')`;
  const horizontal = `document.getElementById('form-layout-horizontal')`;
  await waitFor(`!!${labels}`);
  const resize = width => send('Emulation.setDeviceMetricsOverride', {width, height: 900, deviceScaleFactor: 1, mobile: false});
  await resize(481);
  assert.deepEqual(await evaluate(`(() => {
    const css = getComputedStyle(${labels});
    return [css.display, css.rowGap, css.columnGap, css.alignItems, ${labels}.closest('form') === null];
  })()`), ['grid', '12px', '16px', 'start', true]);
  await resize(480);
  assert.deepEqual(await evaluate(`(() => {
    const css = getComputedStyle(${labels});
    return [css.display, css.rowGap, css.alignItems, ${labels}.firstElementChild.control === ${labels}.children[1]];
  })()`), ['flex', '16px', 'stretch', true]);
  assert.deepEqual(await evaluate(`(() => {
    const el = ${horizontal};
    const [first, last] = el.children;
    const forward = first.offsetLeft < last.offsetLeft;
    el.dir = 'rtl';
    return [getComputedStyle(el).display, forward, first.offsetLeft > last.offsetLeft, first.offsetWidth === last.offsetWidth];
  })()`), ['grid', true, true, true]);
  assert.deepEqual(exceptions, []);
  console.log('FormLayout: inclusive 480px collapse, logical RTL tracks, spacing, and native label association passed');
});
