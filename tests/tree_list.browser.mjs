import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8291'});
  const tree = "document.querySelector('#tree-list-examples [role=tree]')";
  const row = label => `${tree}.querySelector('[aria-label="${label}"]')`;
  const isFocused = label => `document.activeElement === ${row(label)}`;
  const city = label => `document.querySelector('[aria-label="Cities"] [aria-label="${label}"]')`;
  const key = async (key, windowsVirtualKeyCode) => {
    for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key, code:key, windowsVirtualKeyCode});
  };
  await waitFor(tree);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'tree' && node.name?.value === 'Project files'));
  assert.ok(!nodes.some(node => !node.ignored && node.role?.value === 'treeitem' && node.name?.value === 'Button.tsx'));
  // Tab starts on the first enabled city and on the selected nested project row.
  for (const [heading, stop] of [['h3', city('El Paso')], ['h2', row('App.tsx')]]) {
    await waitFor(`${stop}?.tabIndex === 0`);
    assert.equal(await evaluate(`${stop}.closest('[role=tree]').querySelectorAll('[tabindex="0"]').length`), 1);
    await evaluate(`(() => {
      const heading = document.querySelector('#tree-list-examples ${heading}');
      heading.tabIndex = -1;
      heading.focus();
    })()`);
    await key('Tab', 9);
    await waitFor(`document.activeElement === ${stop}`);
  }
  await key('Home', 36);
  await waitFor(isFocused('src'));
  assert.equal(await evaluate(`${row('src')}.tabIndex`), 0);
  assert.equal(await evaluate(`${row('App.tsx')}.tabIndex`), -1);
  await key('ArrowRight', 39);
  await waitFor(isFocused('components'));
  await key('ArrowRight', 39);
  await waitFor(`${row('components')}.getAttribute('aria-expanded') === 'true'`);
  await key('ArrowDown', 40);
  await waitFor(isFocused('Button.tsx'));
  await key('Enter', 13);
  await waitFor(`${row('Button.tsx')}.getAttribute('aria-selected') === 'true'`);
  await evaluate(`${tree}.dir = 'rtl'`);
  await key('ArrowRight', 39);
  await waitFor(isFocused('components'));
  await key('ArrowRight', 39);
  await waitFor(`${row('components')}.getAttribute('aria-expanded') === 'false'`);
  assert.equal(await evaluate(`${row('Button.tsx')}.getClientRects().length`), 0);
  assert.equal(await evaluate(`${tree}.querySelectorAll('[tabindex="0"]').length`), 1);
  await key('End', 35);
  await waitFor(isFocused('README.md'));
  await key('b', 66); // Button.tsx is hidden by its collapsed parent.
  assert.ok(await evaluate(isFocused('README.md')));
  const focusCity = label => waitFor(`document.activeElement === ${city(label)}`);
  await evaluate(`${city('El Paso')}.focus()`);
  // The key buffer is isolated per tree: the earlier 'b' cannot pollute this match.
  await key('e', 69);
  await focusCity('Évora');
  await key('e', 69);
  await focusCity('El Paso'); // Disabled Essen does not participate in cycling.
  await key('é', 69);
  await focusCity('Évora');
  // Verify that a different prefix is accepted after the documented reset interval.
  await new Promise(resolve => setTimeout(resolve, 550));
  await key('o', 79);
  await key('d', 68);
  await focusCity('Odense');
  assert.deepEqual(exceptions, []);
  console.log('TreeList: named tree, initial Tab entry, visible-only traversal, branch expansion, RTL and selection passed');
});
