import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Emulation.setEmulatedMedia', {features: [{name: 'prefers-reduced-motion', value: 'reduce'}]});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8228'});
  const root = `document.getElementById('chat-composer-drawer-examples')`;
  const button = `${root}.querySelector('button[aria-controls]')`;
  const region = `document.getElementById(${button}.getAttribute('aria-controls'))`;
  await waitFor(`${root}?.querySelector('button[aria-expanded="true"]')`);
  await evaluate(`${button}.focus()`);
  const space = async () => {
    await send('Input.dispatchKeyEvent', {type:'keyDown', key:' ', code:'Space', windowsVirtualKeyCode:32, text:' '});
    await send('Input.dispatchKeyEvent', {type:'keyUp', key:' ', code:'Space', windowsVirtualKeyCode:32});
  };
  await space();
  await waitFor(`${region}.inert && ${button}.getAttribute('aria-label') === 'Expand Files'`);
  assert.ok(await evaluate(`${region}.textContent.includes('design-spec.pdf')`));
  assert.equal(await evaluate(`getComputedStyle(${region}).transitionDuration`), '0s');
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(!nodes.some(node => !node.ignored && node.name?.value === 'Review files'));
  await space();
  await waitFor(`!${region}.inert && document.activeElement === ${button}`);
  assert.equal(await evaluate(`${root}.querySelectorAll('button[aria-controls]').length`), 1);
  assert.ok(await evaluate(`${root}.textContent.includes('notes.txt')`));
  assert.deepEqual(exceptions, []);
  console.log('ChatComposerDrawer: keyboard disclosure, retained inert content and reduced motion passed');
});
