import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8120'});
  await waitFor('!!document.getElementById("check-slot")');
  assert.equal(await evaluate('document.getElementById("check-absent") === null'), true);
  assert.equal(await evaluate('!!document.querySelector("#check-mark.astryx-icon.example-check svg")'), true);
  for (const scheme of ['light', 'dark']) {
    await evaluate(`document.getElementById('check-indicator-examples').style.colorScheme = ${JSON.stringify(scheme)}`);
    const roots = await evaluate(`['check-mark','check-disabled','check-slot'].map(id => {
      const root = document.getElementById(id);
      const box = root.getBoundingClientRect();
      return [box.width, box.height, root.getAttribute('aria-hidden'), getComputedStyle(root).color];
    })`);
    const enabled = scheme === 'light' ? 'rgb(27, 27, 27)' : 'rgb(241, 241, 241)';
    const disabled = scheme === 'light' ? 'rgb(145, 145, 145)' : 'rgb(82, 82, 82)';
    assert.deepEqual(roots, [[16, 16, 'true', enabled], [16, 16, 'true', disabled], [16, 16, 'true', enabled]]);
  }
  console.log('CheckIndicator: decorative root, absent unchecked mark, replacement size and Neutral colors passed');
});
