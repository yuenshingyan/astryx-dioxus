import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8158'});
  await waitFor(`document.querySelectorAll('#input-group-text-examples [data-astryx-input-group-item]').length === 9`);
  assert.deepEqual(await evaluate(`(() => {
    const [prefix, input, suffix] = document.getElementById('input-group-text-rtl').children;
    const a = getComputedStyle(prefix);
    const b = getComputedStyle(suffix);
    input.focus();
    return [a.direction, getComputedStyle(suffix.firstElementChild).direction,
      a.borderTopRightRadius, a.borderTopLeftRadius, b.borderTopRightRadius, b.borderTopLeftRadius,
      b.marginInlineStart, prefix.getBoundingClientRect().left >= input.getBoundingClientRect().right,
      prefix.tabIndex, document.activeElement === input];
  })()`), ['rtl', 'ltr', '10px', '0px', '0px', '10px', '-1px', true, -1, true]);
  assert.deepEqual(await evaluate(`(() => {
    const row = document.getElementById('input-group-text-rtl-url');
    const [prefix, input, suffix] = row.children;
    return [getComputedStyle(row.parentElement).direction, getComputedStyle(input).direction,
      prefix.getBoundingClientRect().left < input.getBoundingClientRect().left,
      input.getBoundingClientRect().left < suffix.getBoundingClientRect().left,
      prefix.textContent + input.placeholder + suffix.textContent];
  })()`), ['rtl', 'ltr', true, true, 'https://example.com']);
  assert.deepEqual(exceptions, []);
  console.log('InputGroupText: RTL corners and overlap, LTR prefix, and native input focus passed');
});
