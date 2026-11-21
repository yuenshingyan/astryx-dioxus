import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8196'});
  const button = `document.querySelector('#info-tip-examples .astryx-info-tip')`;
  const tip = `document.getElementById(${button}.getAttribute('aria-describedby'))`;
  await waitFor(`!!document[Symbol.for('astryx.dioxus.button.runtime')]?.records.get(${button}) && !!${tip}`);
  assert.equal(await evaluate(`document.querySelectorAll('#info-tip-examples button').length`), 6);
  const press = async (key, code, text) => {
    await send('Input.dispatchKeyEvent', {type: 'keyDown', key, windowsVirtualKeyCode: code, ...(text && {text})});
    await send('Input.dispatchKeyEvent', {type: 'keyUp', key, windowsVirtualKeyCode: code});
  };
  await evaluate(`(() => { const before = document.createElement('button'); ${button}.before(before); before.focus(); })()`);
  await press('Tab', 9);
  await waitFor(`${tip}.matches(':popover-open')`);
  assert.equal(await evaluate(`${button}.getAttribute('aria-expanded')`), 'false');
  await press('Enter', 13, '\r');
  await waitFor(`${button}.getAttribute('aria-expanded') === 'true'`);
  assert.ok(await evaluate(`${button}.getAttribute('aria-controls') === ${tip}.id`));
  await press('Escape', 27);
  await waitFor(`!${tip}.matches(':popover-open') && !${button}.hasAttribute('aria-controls')`);
  assert.ok(await evaluate(`document.activeElement === ${button}`));
  await evaluate(`${button}.click()`);
  await waitFor(`${button}.getAttribute('aria-expanded') === 'true'`);
  await evaluate(`${button}.click()`);
  await waitFor(`!${tip}.matches(':popover-open') && ${button}.getAttribute('aria-expanded') === 'false'`);
  assert.deepEqual(exceptions, []);
  console.log('InfoTip: preview, pin, second activation, Escape and retained focus passed');
});
