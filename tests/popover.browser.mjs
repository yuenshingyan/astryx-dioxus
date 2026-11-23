import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls:['https://*']});
  await send('Page.navigate', {url:process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8244'});
  const trigger = `document.querySelector('#popover-examples [aria-haspopup=dialog]')`;
  const panel = `document.getElementById(${trigger}.getAttribute('aria-controls'))`;
  await waitFor(`${trigger}?.getAttribute('aria-expanded') === 'false'`);
  await evaluate(`${trigger}.scrollIntoView({block:'center'}); ${trigger}.focus(); ${trigger}.click()`);
  await waitFor(`${panel}.matches(':popover-open')`);
  assert.equal(await evaluate(`${trigger}.getAttribute('aria-expanded')`), 'true');
  assert.ok(await evaluate(`Math.abs(${panel}.getBoundingClientRect().top - ${trigger}.getBoundingClientRect().bottom - 4) < 1`));
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'dialog' && node.name?.value === 'Settings'));
  await evaluate(`${panel}.querySelector('button').focus()`);
  await send('Input.dispatchKeyEvent', {type:'keyDown', key:'Escape', code:'Escape', windowsVirtualKeyCode:27});
  await send('Input.dispatchKeyEvent', {type:'keyUp', key:'Escape', code:'Escape', windowsVirtualKeyCode:27});
  await waitFor(`!${panel}.matches(':popover-open') && document.activeElement === ${trigger}`);
  await evaluate(`${trigger}.click()`);
  await waitFor(`${panel}.matches(':popover-open')`);
  await evaluate(`document.getElementById('popover-preview-outside').click()`);
  await waitFor(`!${panel}.matches(':popover-open')`);
  assert.deepEqual(exceptions, []);
  console.log('Popover: anchor placement, accessible name, Escape, outside click and focus return passed');
});
