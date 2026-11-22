import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8227'});
  const overview = "document.getElementById('overview-tab')";
  const activity = "document.getElementById('activity-tab')";
  await waitFor(`${overview}?.tabIndex === 0`);
  await evaluate(`${overview}.focus()`);
  const key = async (name, code, text) => {
    await send('Input.dispatchKeyEvent', {type: 'keyDown', key: name, code: name, windowsVirtualKeyCode: code, text});
    await send('Input.dispatchKeyEvent', {type: 'keyUp', key: name, code: name, windowsVirtualKeyCode: code});
  };
  await key('ArrowLeft', 37);
  await waitFor(`document.activeElement === ${activity}`);
  assert.equal(await evaluate(`${overview}.getAttribute('aria-selected')`), 'true');
  assert.deepEqual(await evaluate(`[${overview}.tabIndex, ${activity}.tabIndex]`), [-1, 0]);
  await key('Enter', 13, '\r');
  await waitFor(`${activity}.getAttribute('aria-selected') === 'true' && !document.getElementById('activity-panel').hidden`);
  assert.ok(await evaluate(`document.getElementById('overview-panel').hidden`));
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'tablist' && node.name?.value === 'Project views'));
  assert.deepEqual(exceptions, []);
  console.log('TabList: single tab stop, RTL arrows, disabled skip, manual activation and panel association passed');
});
