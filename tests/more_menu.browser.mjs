import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8281'});
  const root = "document.getElementById('more-menu-examples')";
  const trigger = `${root}?.querySelector('[aria-haspopup=menu][aria-label="More options"]')`;
  const disabled = `${root}.querySelector('[aria-haspopup=menu][aria-label="Unavailable options"]')`;
  const panel = `document.getElementById((${trigger}).getAttribute('aria-controls'))`;
  await waitFor(trigger);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  assert.ok(await evaluate(`${trigger}.querySelectorAll('svg circle').length === 3`));
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === 'More options'));
  await evaluate(`${disabled}.click()`);
  assert.equal(await evaluate(`${disabled}.getAttribute('aria-expanded')`), 'false');
  await evaluate(`${trigger}.focus()`);
  const press = async (key, code, text) => {
    await send('Input.dispatchKeyEvent', {type:'keyDown', key, code:key, windowsVirtualKeyCode:code, text});
    await send('Input.dispatchKeyEvent', {type:'keyUp', key, code:key, windowsVirtualKeyCode:code});
  };
  await press('Enter', 13, '\r');
  await waitFor(`${panel}.matches(':popover-open') && document.activeElement.textContent.trim() === 'Edit'`);
  await press('ArrowDown', 40);
  await waitFor("document.activeElement.textContent.trim() === 'Duplicate'");
  await press('Enter', 13, '\r');
  await waitFor(`!${panel}.matches(':popover-open') && document.activeElement === ${trigger} && ${root}.querySelector('output').textContent === 'Duplicate'`);
  assert.deepEqual(exceptions, []);
  console.log('MoreMenu: named icon trigger, disabled guard, keyboard selection and focus return passed');
});
