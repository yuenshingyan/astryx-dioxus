import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Emulation.setEmulatedMedia', {features: [{name: 'prefers-reduced-motion', value: 'reduce'}]});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8213'});
  const trigger = id => `document.querySelector('#${id} > button')`;
  const panel = t => `document.getElementById(${t}.getAttribute('aria-controls'))`;
  const deployment = trigger('deployment-disclosure');
  const controlled = trigger('controlled-disclosure');
  const disabled = trigger('rollback-disclosure');
  await waitFor(`${deployment}?.getAttribute('aria-expanded') === 'true'`);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  await evaluate(`${panel(deployment)}.querySelector('input').value = 'retained'; ${deployment}.focus()`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13, text: '\r'});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`${panel(deployment)}.hidden`);
  assert.equal(await evaluate(`${deployment}.getAttribute('aria-expanded')`), 'false');
  await evaluate(`${deployment}.click()`);
  await waitFor(`!${panel(deployment)}.hidden`);
  assert.equal(await evaluate(`${panel(deployment)}.querySelector('input').value`), 'retained');
  await evaluate(`${disabled}.click()`);
  assert.deepEqual(await evaluate(`[${disabled}.getAttribute('aria-disabled'), ${disabled}.tabIndex, ${panel(disabled)}.hidden]`), ['true', -1, false]);
  const input = `${panel(controlled)}.querySelector('input')`;
  const close = `${panel(controlled)}.querySelector('button')`;
  // The owner closes the panel even if the focused child consumes focusin.
  await evaluate(`
    ${input}.addEventListener('focusin', event => event.stopPropagation(), {once: true});
    ${controlled}.addEventListener('focus', () => {
      ${controlled}.dataset.wasHiddenOnFocus = String(${panel(controlled)}.hidden);
    }, {once: true});
    ${input}.focus();
  `);
  assert.equal(await evaluate(`document.activeElement === ${input}`), true);
  await evaluate(`${close}.click()`);
  await waitFor(`${panel(controlled)}.hidden && document.activeElement === ${controlled}`);
  assert.equal(await evaluate(`${controlled}.dataset.wasHiddenOnFocus`), 'true');

  // Opening and an owner-set close leave an outside control alone, even if
  // the panel's last focused child consumed the outgoing focus event.
  await evaluate(`${deployment}.focus(); ${controlled}.click()`);
  await waitFor(`!${panel(controlled)}.hidden`);
  assert.equal(await evaluate(`document.activeElement === ${deployment}`), true);
  await evaluate(`
    ${input}.focus();
    ${input}.addEventListener('focusout', event => event.stopPropagation(), {once: true});
    ${deployment}.focus();
  `);
  assert.equal(await evaluate(`document.activeElement === ${deployment}`), true);
  await evaluate(`${close}.click()`);
  await waitFor(`${panel(controlled)}.hidden`);
  await evaluate('new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)))');
  assert.equal(await evaluate(`document.activeElement === ${deployment}`), true);
  const chevron = `${controlled}.querySelector('.astryx-collapsible-chevron')`;
  assert.equal(await evaluate(`getComputedStyle(${chevron}.querySelector('svg')).scale`), '-1 1');
  assert.equal(await evaluate(`getComputedStyle(${chevron}).transitionDuration`), '0s');
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === 'Deployment details'));
  assert.ok(!nodes.some(node => !node.ignored && node.name?.value === 'Deployment reference'));
  assert.deepEqual(exceptions, []);
  console.log('Collapsible: keyboard toggle, retained content, controlled focus return and outside focus preservation, disabled state, and RTL passed');
});
