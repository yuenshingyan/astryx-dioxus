import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8164'});
  await waitFor(`document.querySelectorAll('#command-footer-default kbd').length === 4`);
  for (const dir of ['ltr', 'rtl']) {
    assert.deepEqual(await evaluate(`(() => {
      const footer = document.querySelector('#command-footer-default');
      const custom = document.querySelector('#command-footer-custom');
      footer.dir = '${dir}';
      footer.style.width = '160px';
      const hints = [...footer.children];
      const boxes = hints.map(hint => hint.getBoundingClientRect());
      return [getComputedStyle(footer).flexWrap, new Set(boxes.map(box => box.top)).size > 1,
        footer.scrollWidth === footer.clientWidth, hints.map(hint => hint.querySelectorAll('kbd').length),
        footer.getAttribute('role'), footer.tabIndex, custom.querySelectorAll('kbd').length,
        custom.textContent.includes('Tip: Press ⌘K')];
    })()`), ['wrap', true, true, [2, 1, 1], null, -1, 0, true]);
  }
  assert.deepEqual(exceptions, []);
  console.log('CommandPaletteFooter: default key groups, custom content, and narrow LTR/RTL wrapping passed');
});
