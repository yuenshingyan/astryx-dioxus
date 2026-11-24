import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8266'});
  const trigger = "document.getElementById('hover-card-trigger')";
  const card = "document.querySelector('#hover-card-examples .astryx-hover-card')";
  await waitFor(trigger);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  await evaluate(`${trigger}.scrollIntoView({block:'center'}); ${trigger}.focus()`);
  await waitFor(`${card}.matches(':popover-open')`);
  assert.equal(await evaluate(`${card}.getAttribute('role')`), 'group');
  assert.match(await evaluate(`${card}.textContent`), /Jane Doe.*Software Engineer/);
  for (const type of ['keyDown', 'keyUp']) {
    await send('Input.dispatchKeyEvent', {type, key:'Escape', code:'Escape', windowsVirtualKeyCode:27});
  }
  await waitFor(`!${card}.matches(':popover-open') && document.activeElement === ${trigger}`);
  await evaluate(`${trigger}.blur()`);
  const moveTo = async selector => {
    const [x,y] = await evaluate(`(() => { const r = ${selector}.getBoundingClientRect(); return [r.left + r.width / 2, r.top + r.height / 2]; })()`);
    await send('Input.dispatchMouseEvent', {type:'mouseMoved', x, y});
  };
  await send('Input.dispatchMouseEvent', {type:'mouseMoved', x:1, y:1});
  await moveTo(trigger);
  await waitFor(`${card}.matches(':popover-open')`);
  await moveTo(card);
  await new Promise(resolve => setTimeout(resolve, 350));
  assert.ok(await evaluate(`${card}.matches(':popover-open')`));
  await send('Input.dispatchMouseEvent', {type:'mouseMoved', x:1, y:1});
  await waitFor(`!${card}.matches(':popover-open')`);
  assert.deepEqual(exceptions, []);
  console.log('HoverCard: keyboard focus, Escape, mouse entry, hover transfer and dismissal passed');
});
