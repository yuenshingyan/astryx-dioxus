import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8163'});
  const groups = `document.querySelectorAll('#command-palette-group-examples .astryx-command-palette-group')`;
  await waitFor(`${groups}.length === 5`);
  assert.deepEqual(await evaluate(`Array.from(${groups}, group => [getComputedStyle(group).flexDirection,
    getComputedStyle(group).rowGap, group.firstElementChild.getAttribute('aria-hidden'), group.querySelectorAll('button').length])`),
    Array.from({length: 5}, () => ['column', '2px', 'true', 2]));
  const {nodes} = await send('Accessibility.getFullAXTree');
  const byId = new Map(nodes.map(node => [node.nodeId, node]));
  const descendants = node => (node.childIds ?? []).flatMap(id => {
    const child = byId.get(id);
    return child ? [child, ...descendants(child)] : [];
  });
  for (const heading of ['Navigation', 'Actions', 'Recent Files', 'Commands', 'Preferences']) {
    const group = nodes.find(node => !node.ignored && node.role?.value === 'group' && node.name?.value === heading);
    assert.ok(group, `missing accessible group: ${heading}`);
    assert.ok(!descendants(group).some(node => !node.ignored && node.role?.value === 'StaticText' && node.name?.value === heading),
      `group heading repeated in its accessibility subtree: ${heading}`);
  }
  await evaluate(`${groups}[0].querySelector('button').focus()`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', text: '\r', windowsVirtualKeyCode: 13});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`document.getElementById('command-palette-group-selection').textContent === 'Go to Home'`);
  assert.deepEqual(exceptions, []);
  console.log('CommandPaletteGroup: group names, hidden duplicate headings, layout, and native keyboard activation passed');
});
