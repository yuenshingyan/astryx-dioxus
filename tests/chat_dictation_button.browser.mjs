import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8189'});
  const button = `document.querySelector('#dictation-basic button')`;
  await waitFor(`!!${button}`);
  assert.equal(await evaluate(`${button}.getAttribute('aria-label')`), 'Start dictation');
  assert.equal(await evaluate(`document.querySelector('#dictation-unsupported button').getAttribute('aria-disabled')`), 'true');
  await evaluate(`${button}.focus()`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', text: '\r', windowsVirtualKeyCode: 13});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`${button}.getAttribute('aria-label') === 'Stop dictation'`);
  const meter = `document.querySelector('#dictation-basic .astryx-chat-dictation-button__meter')`;
  assert.deepEqual(await evaluate(`[
    ${meter}.children.length, ${meter}.getAttribute('aria-hidden'), getComputedStyle(${meter}).height,
    ${button} === document.activeElement, !!${button}.querySelector('svg'),
  ]`), [5, 'true', '14px', true, false]);
  const clipping = await evaluate(`Number.parseFloat(${meter}.style.getPropertyValue('--_dictation-clip'))`);
  assert.ok(Math.abs(clipping - 50) < 0.00001, `Expected 50% within f32 rounding, got ${clipping}%`);
  await send('Emulation.setEmulatedMedia', {features: [{name: 'prefers-reduced-motion', value: 'reduce'}]});
  await waitFor(`getComputedStyle(${meter}.firstElementChild).transitionDuration.split(',').every(s => s.trim() === '0s')`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === 'Stop dictation'));
  await evaluate(`${button}.click()`);
  await waitFor(`!${meter} && ${button}.getAttribute('aria-label') === 'Start dictation'`);
  assert.deepEqual(exceptions, []);
  console.log('ChatDictationButton: names, keyboard toggle, focus retention, decorative meter and reduced motion passed');
});
