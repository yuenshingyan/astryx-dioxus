import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8278'});
  const root = "document.getElementById('toast-examples')";
  const info = `${root}?.querySelector('.astryx-toast-surface__body[role=status]')`;
  const error = `${root}?.querySelector('.astryx-toast-surface__body[role=alert]')`;
  const trigger = "document.getElementById('toast-timed-trigger')";
  await waitFor(`${info}?.textContent === 'Document saved successfully'`);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  await evaluate(`${info}.parentElement.querySelector('[aria-label=Dismiss]').click()`);
  await waitFor(`!${info}`);
  await evaluate(`${trigger}.click()`);
  await waitFor(`${error}?.textContent === 'Could not save document'`);
  const dismiss = `${error}.parentElement.querySelector('[aria-label=Dismiss]')`;
  await evaluate(`${dismiss}.focus()`);
  await new Promise(resolve => setTimeout(resolve, 1750));
  assert.equal(await evaluate(`${error}?.textContent`), 'Could not save document');
  await evaluate(`${trigger}.focus()`);
  await waitFor(`!${error}`);
  assert.deepEqual(exceptions, []);
  console.log('Toast: status and alert markup, manual dismissal, and focus-paused expiry passed');
});
