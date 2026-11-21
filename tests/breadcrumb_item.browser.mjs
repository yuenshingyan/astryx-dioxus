import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8137'});
  await send('Page.bringToFront');
  await waitFor(`document.querySelector('#breadcrumb-auto [aria-current="page"]')?.textContent === 'Details'`);
  assert.equal(await evaluate(`document.querySelector('#breadcrumb-auto [aria-current]').tagName`), 'A');
  await evaluate(`document.getElementById('breadcrumb-toggle').click()`);
  await waitFor(`document.querySelector('#breadcrumb-auto [aria-current="page"]')?.textContent === 'Home'`);
  await evaluate(`document.getElementById('breadcrumb-toggle').click()`);
  await waitFor(`document.querySelector('#breadcrumb-auto [aria-current="page"]')?.textContent === 'Details'`);
  assert.equal(await evaluate(`document.querySelectorAll('#breadcrumb-auto [aria-current]').length`), 1);
  assert.deepEqual(await evaluate(`(() => {
    const nav = document.querySelector('[aria-label="Settings breadcrumbs"]');
    nav.dir = 'rtl';
    const separators = [...nav.querySelectorAll('.astryx-breadcrumb-item__separator')];
    return [nav.querySelector('[aria-current]').tagName, getComputedStyle(separators[0]).display,
      separators[1].getAttribute('aria-hidden'), getComputedStyle(separators[1].firstElementChild).transform];
  })()`), ['SPAN', 'none', 'true', 'matrix(-1, 0, 0, 1, 0, 0)']);
  await evaluate(`document.querySelector('#breadcrumb-projects button').focus()`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13, text: '\r'});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`document.getElementById('breadcrumb-action-result').textContent === 'Projects'`);
  assert.equal(await evaluate(`document.querySelectorAll('#breadcrumb-item-examples nav').length`), 5);
  assert.deepEqual(exceptions, []);
  console.log('BreadcrumbItem: both arrangements, native activation, explicit and dynamic current page, RTL separators passed');
});
