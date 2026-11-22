import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8225'});
  const nav = `document.querySelector('#pagination-examples nav[aria-label="Pagination"]')`;
  const compact = `document.querySelector('#pagination-examples nav[aria-label="Compact pagination"]')`;
  const current = `${nav}.querySelector('[aria-current=page]')`;
  await waitFor(`${nav}?.querySelector('[aria-current=page]')?.textContent.trim() === '7'`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'navigation' && node.name?.value === 'Pagination'));
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === 'Go to page 7'));
  await evaluate(`${nav}.querySelector('[aria-label="Go to page 8"]').focus()`);
  await send('Input.dispatchKeyEvent', {type:'keyDown', key:'Enter', code:'Enter', windowsVirtualKeyCode:13, text:'\r'});
  await send('Input.dispatchKeyEvent', {type:'keyUp', key:'Enter', code:'Enter', windowsVirtualKeyCode:13});
  await waitFor(`${current}.textContent.trim() === '8'`);
  assert.ok(await evaluate(`document.activeElement === ${current}`));
  const [previous, next] = await evaluate(`Array.from(${compact}.querySelectorAll('button'), button => button.getAttribute('aria-disabled'))`);
  assert.equal(previous, 'true');
  assert.notEqual(next, 'true');
  await evaluate(`${compact}.querySelector('button').click()`);
  assert.match(await evaluate(`${compact}.textContent`), /Page 1 of 3/);
  await evaluate(`${compact}.querySelector('button:last-child').click()`);
  await waitFor(`${compact}.textContent.includes('Page 2 of 3')`);
  assert.deepEqual(exceptions, []);
  console.log('Pagination: named landmark, current page, keyboard activation, retained focus and bounds passed');
});
