import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8158'});
  await waitFor(`document.querySelectorAll('#input-group-text-examples [data-astryx-input-group-item]').length === 7`);
  assert.deepEqual(await evaluate(`(() => {
    const [prefix, input, suffix] = document.getElementById('input-group-text-rtl').children;
    const a = getComputedStyle(prefix);
    const b = getComputedStyle(suffix);
    input.focus();
    return [a.direction, getComputedStyle(prefix.firstElementChild).direction,
      a.borderTopRightRadius, a.borderTopLeftRadius, b.borderTopRightRadius, b.borderTopLeftRadius,
      b.marginInlineStart, prefix.getBoundingClientRect().left >= input.getBoundingClientRect().right,
      prefix.tabIndex, document.activeElement === input];
  })()`), ['rtl', 'ltr', '10px', '0px', '0px', '10px', '-1px', true, -1, true]);
  assert.deepEqual(exceptions, []);
  console.log('InputGroupText: RTL corners and overlap, LTR prefix, and native input focus passed');
});
