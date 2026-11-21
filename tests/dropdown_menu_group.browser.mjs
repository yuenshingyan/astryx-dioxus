import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8144'});
  await waitFor(`!!document.getElementById('menu-group-history')`);
  assert.deepEqual(await evaluate(`(() => {
    const root = document.getElementById('menu-group-history');
    const title = document.getElementById(root.getAttribute('aria-labelledby'));
    const css = getComputedStyle(title);
    return [title === root.firstElementChild, title.tabIndex, css.paddingBlockStart, css.paddingInlineStart,
      [...document.querySelectorAll('#dropdown-menu-group-examples [role=group]')].map(group =>
        document.getElementById(group.getAttribute('aria-labelledby')).textContent.trim())];
  })()`), [true, -1, '4px', '8px', ['Version history', 'Add to message', 'Model', 'On completion']]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'group' && node.name?.value === 'Version history'));
  assert.deepEqual(exceptions, []);
  console.log('DropdownMenuGroup: heading association, AX name, and heading spacing passed');
});
