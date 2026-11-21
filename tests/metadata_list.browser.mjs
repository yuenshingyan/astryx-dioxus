import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8135'});
  await send('Page.bringToFront');
  const root = "document.getElementById('metadata-list-collapsible')";
  const button = `${root}.querySelector('button')`;
  await waitFor(`${root}?.querySelector('button')`);
  assert.deepEqual(await evaluate(`(() => {
    const dl = ${root}.querySelector('dl');
    window.firstTerm = dl.querySelector('dt');
    const toggle = ${button};
    toggle.focus();
    return [dl.querySelectorAll('dt').length, toggle.getAttribute('aria-controls') === dl.id,
      toggle.getAttribute('aria-expanded'), toggle.textContent, document.activeElement === toggle];
  })()`), [3, true, 'false', 'Show more', true]);
  const pressEnter = async () => {
    await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13, text: '\r'});
    await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  };
  await pressEnter();
  await waitFor(`${button}.getAttribute('aria-expanded') === 'true'`);
  assert.deepEqual(await evaluate(`[${root}.querySelectorAll('dt').length, ${button}.textContent, ${root}.querySelector('dt') === firstTerm, document.activeElement === ${button}]`), [6, 'Show less', true, true]);
  await pressEnter();
  await waitFor(`${button}.getAttribute('aria-expanded') === 'false'`);
  assert.equal(await evaluate(`${root}.querySelectorAll('dt').length`), 3);
  assert.deepEqual(await evaluate(`['horizontal','multi'].map(id => {
    const list = document.querySelector('#metadata-list-' + id + ' dl');
    return [getComputedStyle(list).display, [...list.children].every(item => item.matches('div') && item.firstElementChild.matches('dt') && item.lastElementChild.matches('dd')), list.children.length];
  })`), [['flex', true, 4], ['grid', true, 6]]);
  assert.deepEqual(exceptions, []);
  console.log('MetadataList: keyboard disclosure, focus and item identity, horizontal and stacked grid markup passed');
});
