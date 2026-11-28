import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8320'});
  const all = `document.getElementById('checkbox-select-all')`;
  const blocked = `document.getElementById('checkbox-disabled')`;
  await waitFor(`${all}?.indeterminate`);
  assert.ok(await evaluate(`${blocked}.form === null && !${blocked}.hasAttribute('name')`));
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  const space = async () => {
    for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key: ' ', code: 'Space', windowsVirtualKeyCode: 32});
  };
  await send('Emulation.setEmulatedMedia', {features: [{name: 'prefers-reduced-motion', value: 'reduce'}]});
  assert.ok(await evaluate('matchMedia("(hover: hover)").matches'));
  const moveMouse = (x, y) => send('Input.dispatchMouseEvent', {type: 'mouseMoved', x, y});
  const checkLabelHover = async (input, shouldTint) => {
    const label = `${input}.labels[0]`;
    const row = `${input}.closest('.astryx-checkbox-input__row')`;
    const fill = () => evaluate(`getComputedStyle(${input}.nextElementSibling).backgroundColor`);
    await evaluate(`${label}.scrollIntoView({block: 'center'})`);
    await moveMouse(0, 0);
    await waitFor(`!${row}.matches(':hover')`);
    const baseline = await fill();
    const {x, y} = await evaluate(`(() => {
      const r = ${label}.getBoundingClientRect();
      return {x: r.x + r.width / 2, y: r.y + r.height / 2};
    })()`);
    await moveMouse(x, y);
    // Chromium also matches :hover on the associated input when its label is hit.
    await waitFor(`${label}.matches(':hover') && ${label}.contains(document.elementFromPoint(${x}, ${y}))`);
    assert.equal((await fill()) !== baseline, shouldTint, input);
  };
  await checkLabelHover(`document.getElementById('checkbox-unchecked')`, true);
  await checkLabelHover(blocked, false);
  await evaluate(`${blocked}.focus(); ${blocked}.click(); ${blocked}.labels[0].click()`);
  await space();
  assert.ok(await evaluate(`document.activeElement === ${blocked} && !${blocked}.checked`));
  await evaluate(`${all}.focus()`);
  await space();
  await waitFor(`${all}.checked && !${all}.indeterminate`);
  assert.equal(await evaluate(`document.querySelectorAll('#checkbox-input-examples input:checked').length`), 6);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => node.role?.value === 'checkbox' && node.name?.value === 'Select all notifications'));
  await evaluate(`document.getElementById('checkbox-unchecked').click()`);
  await waitFor(`!document.getElementById('checkbox-unchecked').checked`);
  const loading = "document.getElementById('checkbox-loading')";
  const toggle = "document.getElementById('checkbox-toggle-loading')";
  await evaluate(`${toggle}.click()`);
  await waitFor(`${loading}.getAttribute('aria-busy') === 'true'`);
  assert.ok(await evaluate(`${loading}.nextElementSibling.querySelector('.astryx-spinner[aria-hidden=true]') !== null`));
  await checkLabelHover(loading, true);
  await evaluate(`${loading}.focus(); ${loading}.click(); ${loading}.labels[0].click()`);
  await space();
  assert.ok(await evaluate(`!${loading}.checked && ${loading}.dataset.checked === 'false' && document.activeElement === ${loading}`));
  await evaluate(`${toggle}.click()`);
  await waitFor(`!${loading}.hasAttribute('aria-busy') && !${loading}.nextElementSibling.querySelector('.astryx-spinner')`);
  await space();
  await waitFor(`${loading}.checked && ${loading}.dataset.checked === 'true'`);
  assert.deepEqual(exceptions, []);
  console.log('CheckboxInput: label hover, native mixed state, loading and disabled activation guards passed');
});
