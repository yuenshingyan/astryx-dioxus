import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8143'});
  const divider = `document.getElementById('dropdown-menu-divider-demo')`;
  const trigger = `document.querySelector('#dropdown-menu-divider-example [aria-haspopup="menu"]')`;
  const menu = `${divider}.closest('[role="menu"]')`;
  const focused = `document.activeElement.textContent.trim()`;
  const arrow = async (key, windowsVirtualKeyCode) => {
    for (const type of ['keyDown', 'keyUp']) {
      await send('Input.dispatchKeyEvent', {type, key, code: key, windowsVirtualKeyCode});
    }
  };
  await waitFor(`!!${divider} && !!${trigger}`);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  await evaluate(`${trigger}.focus()`);
  await arrow('ArrowDown', 40);
  await waitFor(`${menu}.matches(':popover-open') && ${focused} === 'Edit'`);
  await arrow('ArrowDown', 40);
  await waitFor(`${focused} === 'Delete'`);
  await arrow('ArrowUp', 38);
  await waitFor(`${focused} === 'Edit'`);
  assert.ok(await evaluate(`${divider}.focus(); document.activeElement !== ${divider}`));
  assert.deepEqual(await evaluate(`(() => {
    const node = ${divider};
    const css = getComputedStyle(node);
    const rule = node.querySelector('.astryx-divider__line').getBoundingClientRect();
    return [node.getAttribute('role'), node.getAttribute('aria-orientation'), node.getAttribute('tabindex'),
      css.marginBlockStart, css.marginBlockEnd, css.getPropertyValue('--spacing-1').trim(),
      rule.height === 1, rule.width > 0 && rule.width === node.getBoundingClientRect().width];
  })()`), ['separator', 'horizontal', null, '4px', '4px', '0.25rem', true, true]);
  const {root} = await send('DOM.getDocument');
  const {nodeId} = await send('DOM.querySelector', {nodeId: root.nodeId, selector: '#dropdown-menu-divider-demo'});
  const {nodes} = await send('Accessibility.getPartialAXTree', {nodeId, fetchRelatives: false});
  assert.equal(nodes[0].role.value, 'separator');
  assert.equal(nodes[0].name?.value ?? '', '');
  assert.deepEqual(exceptions, []);
  console.log('DropdownMenuDivider: separator semantics, block spacing, line geometry and arrow traversal passed');
});
