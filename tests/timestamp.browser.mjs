import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Emulation.setTimezoneOverride', {timezoneId: 'America/Los_Angeles'});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8245'});
  const times = "Array.from(document.querySelectorAll('#timestamp-examples time'))";
  await waitFor(`${times}.length === 4`);
  assert.deepEqual(await evaluate(`${times}.map(node => node.textContent)`), ['2026-03-25 12:00:00', '2026-03-25', '12:00:00', '1774440000']);
  assert.ok(await evaluate(`${times}.every(node => node.dateTime === '2026-03-25T12:00:00Z' && !node.hasAttribute('tabindex'))`));
  await send('Emulation.setTimezoneOverride', {timezoneId: 'Asia/Tokyo'});
  assert.equal(await evaluate(`${times}[0].textContent`), '2026-03-25 12:00:00');
  assert.deepEqual(exceptions, []);
  console.log('Timestamp: absolute UTC formats, native datetime and no extra tab stop passed');
});
