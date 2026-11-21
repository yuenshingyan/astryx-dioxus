import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8139'});
  const avatar = `document.getElementById('avatar-with-status')`;
  const toggle = () => evaluate(`document.getElementById('avatar-status-toggle').click()`);
  await waitFor(`${avatar}?.getAttribute('aria-label') === 'Jane Doe, Online'`);
  assert.equal(await evaluate(`${avatar}.offsetWidth`), 36);
  await toggle();
  await waitFor(`${avatar}.getAttribute('aria-label') === 'Jane Doe, Away'`);
  assert.equal(await evaluate(`${avatar}.offsetWidth`), 128);
  await toggle();
  await waitFor(`${avatar}.getAttribute('aria-label') === 'Jane Doe'`);
  await waitFor(`document.querySelector('#avatar-fallback-image img')?.naturalWidth > 0`);
  await waitFor(`document.querySelector('#avatar-fallback-initials .astryx-avatar-fallback')`);
  await waitFor(`document.querySelector('#avatar-fallback-icon svg')`);
  assert.equal(await evaluate(`getComputedStyle(document.querySelector('#avatar-fallback-initials .astryx-avatar-fallback'),'::before').content.startsWith('"TU"')`), true);
  assert.equal(await evaluate(`document.getElementById('avatar-fallback-icon').getAttribute('role')`), 'presentation');
  await evaluate(`document.getElementById('avatar-action').click()`);
  await waitFor(`document.getElementById('avatar-action-result')`);
  assert.deepEqual(exceptions, []);
  console.log('Avatar: offline fallback chain, wrapped status rename/removal, reactive size, and native activation passed');
});
