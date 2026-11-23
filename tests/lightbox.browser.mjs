import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8254'});
  const trigger = "document.getElementById('lightbox-trigger')";
  const panel = "document.querySelector('#lightbox-examples dialog:modal')";
  const close = `${panel}.querySelector('button')`;
  const image = `${panel}.querySelector('img')`;
  await waitFor(trigger);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  await evaluate(`${trigger}.focus(); ${trigger}.click()`);
  await waitFor(`${panel} && ${image}?.naturalWidth > 0 && document.activeElement === ${close}`);
  assert.deepEqual(await evaluate(`(() => { const r = ${panel}.getBoundingClientRect(); return [r.left,r.top,r.width,r.height]; })()`), await evaluate('[0,0,innerWidth,innerHeight]'));
  const {nodes} = await send('Accessibility.getFullAXTree');
  for (const [role, name] of [['dialog', 'Media viewer'], ['image', 'Coastal shoreline with ocean waves'], ['button', 'Close']]) {
    assert.ok(nodes.some(node => !node.ignored && node.role?.value === role && node.name?.value === name), name);
  }
  assert.match(await evaluate(`${panel}.querySelector('figcaption').textContent`), /scenic coastline/);
  for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key:'Escape', code:'Escape', windowsVirtualKeyCode:27});
  const closed = `!${panel} && document.activeElement === ${trigger}`;
  await waitFor(closed);
  await evaluate(`${trigger}.click()`);
  await waitFor(panel);
  await evaluate(`${close}.click()`);
  await waitFor(closed);
  assert.deepEqual(exceptions, []);
  console.log('Lightbox: fullscreen geometry, image and caption, dismissal and focus return passed');
});
