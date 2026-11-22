import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8202'});
  await waitFor(`document.querySelectorAll('#selector-option-examples .astryx-selector-option').length === 10`);
  assert.deepEqual(await evaluate(`(() => {
    const [plain, stacked, inline] = document.querySelectorAll('#selector-option-descriptions > .astryx-selector-option');
    const css = getComputedStyle(inline);
    const desc = inline.querySelector('.astryx-item__description');
    const label = inline.querySelector('.astryx-item__label');
    return [css.paddingBlockStart, css.paddingInlineEnd, css.borderRadius, css.minWidth,
      inline.hasAttribute('tabindex'), stacked.offsetHeight > plain.offsetHeight,
      label.clientWidth === label.scrollWidth, desc.clientWidth < desc.scrollWidth,
      getComputedStyle(desc).textOverflow, inline.querySelector('.astryx-item__end').textContent];
  })()`), ['0px', '0px', '0px', '0px', false, true, true, true, 'ellipsis', 'Default']);
  assert.deepEqual(exceptions, []);
  console.log('SelectorOption: embedded Item geometry, stacked and inline text, and trailing content passed');
});
