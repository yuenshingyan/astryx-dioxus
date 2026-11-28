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
  const escape = async () => {
    for (const type of ['keyDown', 'keyUp']) {
      await send('Input.dispatchKeyEvent', {type, key:'Escape', code:'Escape', windowsVirtualKeyCode:27});
    }
  };
  await escape();
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
  const controlled = "document.querySelector('#controlled-hover-card-example .astryx-hover-card')";
  const controlledTrigger = "document.getElementById('controlled-hover-card-trigger')";
  const show = "document.getElementById('controlled-hover-card-show')";
  const close = "document.getElementById('controlled-hover-card-close')";
  const requests = "document.getElementById('controlled-hover-card-requests').textContent";
  const allowDelays = () => new Promise(resolve => setTimeout(resolve, 400));
  await evaluate(`${controlledTrigger}.scrollIntoView({block:'center'})`);
  await moveTo(controlledTrigger);
  await allowDelays();
  assert.ok(await evaluate(`!${controlled}.matches(':popover-open')`));
  assert.equal(await evaluate(requests), 'Visibility requests: 0');

  await evaluate(`${show}.click()`);
  await waitFor(`${controlled}.matches(':popover-open')`);
  await evaluate(`${close}.focus()`);
  await escape();
  await waitFor(`${requests} === 'Visibility requests: 1 (close)'`);
  assert.ok(await evaluate(`${controlled}.matches(':popover-open') && document.activeElement === ${close}`));

  await send('Input.dispatchMouseEvent', {type:'mouseMoved', x:1, y:1});
  await evaluate(`${show}.focus()`);
  await allowDelays();
  assert.ok(await evaluate(`${controlled}.matches(':popover-open')`));
  assert.equal(await evaluate(requests), 'Visibility requests: 1 (close)');

  await evaluate(`${close}.focus(); ${close}.click()`);
  await waitFor(`!${controlled}.matches(':popover-open') && document.activeElement === ${controlledTrigger}`);
  await evaluate(`${controlledTrigger}.blur(); ${controlledTrigger}.focus()`);
  await allowDelays();
  assert.ok(await evaluate(`!${controlled}.matches(':popover-open')`));
  assert.equal(await evaluate(requests), 'Visibility requests: 1 (close)');
  assert.deepEqual(exceptions, []);
  console.log('HoverCard: uncontrolled desktop behavior and caller-controlled visibility passed');
});
