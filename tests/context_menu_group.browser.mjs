import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8233'});
  const menu = `document.querySelector('#context-menu-group-examples [role=menu]')`;
  await waitFor(`${menu}?.querySelectorAll('[role=menuitem]').length === 3`);
  assert.deepEqual(await evaluate(`Array.from(${menu}.children, node => node.getAttribute('role'))`), ['group', 'separator', 'group', 'group']);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'group' && node.name?.value === 'Organize'));
  assert.deepEqual(exceptions, []);
  console.log('ContextMenuGroup: accessible group name, shared menu rows and divider passed');
});
