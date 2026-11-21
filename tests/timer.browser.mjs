import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8132'});
  const timer = `document.getElementById('timer-reactive')`;
  await waitFor(`${timer}?.textContent === '1h 02m'`);
  assert.deepEqual(await evaluate(`(() => {
    window.ownedTime = ${timer};
    window.ownedText = ownedTime.firstChild;
    return [ownedTime.tagName, ownedTime.dateTime, getComputedStyle(ownedTime).fontVariantNumeric,
      document.querySelectorAll('#timer-examples time').length, ownedTime.closest('[aria-live]') === null];
  })()`), ['TIME', 'PT3720S', 'tabular-nums', 8, true]);
  await evaluate(`document.getElementById('timer-toggle-format').click()`);
  await waitFor(`/^1:02:\\d{2}$/.test(${timer}.textContent)`);
  const duration = await evaluate(`${timer}.dateTime`);
  await waitFor(`${timer}.dateTime !== '${duration}'`);
  assert.equal(await evaluate(`${timer} === ownedTime && ownedTime.firstChild === ownedText`), true);
  await evaluate(`window.lastDuration = ownedTime.dateTime; document.getElementById('timer-toggle-mount').click()`);
  await waitFor(`${timer} === null`);
  await waitFor(`Number(document.getElementById('timer-inline').dateTime.slice(2, -1)) >= 2`);
  assert.equal(await evaluate(`ownedTime.dateTime === lastDuration`), true);
  assert.deepEqual(exceptions, []);
  console.log('Timer: original arrangements, native duration, stable node identity, format changes, ticking and unmount passed');
});
