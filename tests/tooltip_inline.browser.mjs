import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8195'});
  const trigger = `document.querySelector('#tooltip-examples .astryx-tooltip-text')`;
  const tip = `document.getElementById(${trigger}.getAttribute('aria-describedby'))`;
  await waitFor(`!!${trigger}`);
  assert.equal(await evaluate(`${tip}.matches(':popover-open')`), false);
  const press = async (key, code) => {
    for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key, code: key, windowsVirtualKeyCode: code});
  };
  await evaluate(`(() => { const button = document.createElement('button'); ${trigger}.before(button); button.focus(); })()`);
  await press('Tab', 9);
  await waitFor(`${tip}.matches(':popover-open')`);
  assert.equal(await evaluate(`${tip}.textContent`), 'Your data is encrypted and never shared');
  assert.ok(await evaluate(`document.activeElement === ${trigger}`));
  await press('Escape', 27);
  await waitFor(`!${tip}.matches(':popover-open')`);
  assert.ok(await evaluate(`document.activeElement === ${trigger}`));
  const hover = await evaluate(`(() => {
    const span = document.querySelectorAll('#tooltip-examples .astryx-tooltip-text')[1];
    span.scrollIntoView({block: 'center', behavior: 'instant'});
    const {left, top, width, height} = span.getBoundingClientRect();
    return {x: left + width / 2, y: top + height / 2};
  })()`);
  await send('Input.dispatchMouseEvent', {type: 'mouseMoved', ...hover});
  await waitFor(`Array.from(document.querySelectorAll('#tooltip-examples [role="tooltip"]')).some(node => node.textContent === 'Standard 30-day agreement' && node.matches(':popover-open'))`);
  assert.deepEqual(exceptions, []);
  console.log('Tooltip inline: pointer and Tab reveal, Escape dismissal, and retained focus passed');
});
