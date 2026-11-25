import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8285'});
  const grid = "document.querySelector('#calendar-examples [role=grid]')";
  const day = n => `${grid}.querySelector('[data-day="${n}"]')`;
  const key = async (key, windowsVirtualKeyCode) => {
    for (const type of ['keyDown', 'keyUp']) {
      await send('Input.dispatchKeyEvent', {type, key, code: key, windowsVirtualKeyCode,
        ...(key === 'Enter' && type === 'keyDown' ? {text: '\r', unmodifiedText: '\r'} : {})});
    }
  };
  await waitFor(grid);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'grid' && node.name?.value === 'April 2026'));
  assert.equal(await evaluate(`${day(15)}.closest('td').getAttribute('aria-selected')`), 'true');
  assert.equal(await evaluate(`${day(1)}.closest('td').cellIndex`), 3);
  await evaluate(`${day(15)}.focus()`);
  await key('ArrowRight', 39);
  await waitFor(`document.activeElement === ${day(16)} && ${day(16)}.tabIndex === 0`);
  await key('Enter', 13);
  await waitFor(`${day(16)}.closest('td').getAttribute('aria-selected') === 'true'`);
  assert.equal(await evaluate(`${day(15)}.closest('td').getAttribute('aria-selected')`), 'false');
  await evaluate(`${grid}.dir = 'rtl'`);
  await key('ArrowLeft', 37);
  await waitFor(`document.activeElement === ${day(17)}`);
  const next = "document.querySelector('#calendar-examples [aria-label=\"Next month\"]')";
  const previous = "document.querySelector('#calendar-examples [aria-label=\"Previous month\"]')";
  assert.ok(await evaluate(`${previous}.getAttribute('aria-disabled') === 'true' && ${day(9)}.disabled && !${day(10)}.disabled`));
  await evaluate(`${next}.focus(); ${next}.click()`);
  await waitFor(`${day(1)}.getAttribute('aria-label') === 'Friday, May 1, 2026'`);
  assert.equal(await evaluate(`${grid}.querySelectorAll('button[tabindex="0"]').length`), 1);
  assert.ok(await evaluate(`${next}.getAttribute('aria-disabled') === 'true' && !${previous}.hasAttribute('aria-disabled') && !${day(20)}.disabled && ${day(21)}.disabled`));
  assert.equal(await evaluate(`document.activeElement === ${next}`), true);
  await evaluate(`${next}.click()`);
  assert.equal(await evaluate(`${day(1)}.getAttribute('aria-label')`), 'Friday, May 1, 2026');
  assert.deepEqual(exceptions, []);
  console.log('Calendar: named grid, Gregorian placement, selection, RTL keys, month navigation and roving tab stop passed');
});
