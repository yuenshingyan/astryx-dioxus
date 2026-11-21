import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8167'});
  const toggle = `document.querySelector('[aria-controls="mobile-nav-basic-panel"]')`;
  const panel = `document.getElementById('mobile-nav-basic-panel')`;
  await waitFor(`${toggle}?.getAttribute('aria-expanded') === 'false' && ${panel}.hidden`);
  await evaluate(`${toggle}.focus()`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', text: '\r', windowsVirtualKeyCode: 13});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`${toggle}.getAttribute('aria-expanded') === 'true' && !${panel}.hidden`);
  assert.equal(await evaluate(`document.activeElement === ${toggle}`), true);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === 'Open menu' &&
    node.properties?.some(property => property.name === 'expanded' && property.value?.value === true)));
  await evaluate(`${toggle}.click()`);
  await waitFor(`${toggle}.getAttribute('aria-expanded') === 'false' && ${panel}.hidden`);
  assert.equal(await evaluate(`document.querySelector('[aria-controls="mobile-nav-showcase-panel"]').getAttribute('aria-label')`), 'Open navigation');
  assert.deepEqual(exceptions, []);
  console.log('MobileNavToggle: keyboard activation, expanded state, names, and controlled panel passed');
});
