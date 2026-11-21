import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8155'});
  const probe = `document.getElementById('toggle-width-probe')`;
  await waitFor(`${probe}?.getAttribute('aria-pressed') === 'false'`);
  const before = await evaluate(`${probe}.getBoundingClientRect().width`);
  await evaluate(`${probe}.focus()`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', text: '\r', windowsVirtualKeyCode: 13});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`${probe}.getAttribute('aria-pressed') === 'true'`);
  assert.equal(await evaluate(`${probe}.getBoundingClientRect().width`), before);
  assert.equal(await evaluate(`${probe} === document.activeElement`), true);
  const favorite = `document.querySelector('#toggle-button-examples .astryx-toggle-button')`;
  await evaluate(`${favorite}.click()`);
  await waitFor(`${favorite}.getAttribute('aria-pressed') === 'true' && ${favorite}.textContent.includes('★')`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === 'Visible' && node.properties?.some(prop => prop.name === 'pressed' && prop.value.value === 'true')));
  await send('Emulation.setEmulatedMedia', {features: [{name: 'forced-colors', value: 'active'}]});
  await waitFor(`(() => {
    const sample = document.createElement('span');
    sample.style.cssText = 'forced-color-adjust:none;background-color:Highlight;color:HighlightText';
    document.body.append(sample);
    const expected = getComputedStyle(sample);
    const actual = getComputedStyle(${probe});
    const matches = expected.backgroundColor === actual.backgroundColor && expected.color === actual.color;
    sample.remove();
    return matches;
  })()`);
  assert.deepEqual(exceptions, []);
  console.log('ToggleButton: native keyboard activation, reserved width, icon swap, AX pressed state, and forced colors passed');
});
