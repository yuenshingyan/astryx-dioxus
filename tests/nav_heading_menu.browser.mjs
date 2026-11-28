import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8178'});
  const menu = `document.getElementById('heading-menu-products')`;
  await waitFor(`!!${menu}`);
  assert.deepEqual(await evaluate(`[getComputedStyle(${menu}).minWidth, ${menu}.firstElementChild.dataset.size, getComputedStyle(${menu}.firstElementChild).paddingBlockStart]`), ['160px', 'sm', '4px']);
  await evaluate(`${menu}.focus()`);
  const key = async (key, code, windowsVirtualKeyCode) => {
    await send('Input.dispatchKeyEvent', {type: 'keyDown', key, code, windowsVirtualKeyCode});
    await send('Input.dispatchKeyEvent', {type: 'keyUp', key, code, windowsVirtualKeyCode});
  };
  const assertFocusOutline = async (id) => {
    assert.equal(await evaluate(`(() => {
      const row = document.getElementById('${id}');
      const style = getComputedStyle(row);
      return row.matches(':focus-visible') &&
        style.outlineStyle === style.getPropertyValue('--focus-outline-style').trim() &&
        style.outlineWidth === style.getPropertyValue('--focus-outline-width').trim() &&
        parseFloat(style.outlineOffset) === -parseFloat(style.outlineWidth);
    })()`), true, `${id} should show the shared inset keyboard focus outline`);
  };
  await key('ArrowDown', 'ArrowDown', 40);
  await waitFor(`document.activeElement.id === 'heading-menu-dashboard'`);
  await assertFocusOutline('heading-menu-dashboard');
  await key('ArrowUp', 'ArrowUp', 38);
  assert.equal(await evaluate('document.activeElement.id'), 'heading-menu-disabled');
  await assertFocusOutline('heading-menu-disabled');
  assert.equal(await evaluate(`getComputedStyle(document.getElementById('heading-menu-dashboard')).outlineStyle`), 'none');
  await key('Enter', 'Enter', 13);
  assert.equal(await evaluate('location.hash'), '');
  await key('s', 'KeyS', 83);
  assert.equal(await evaluate('document.activeElement.id'), 'heading-menu-settings');
  await key(' ', 'Space', 32);
  await waitFor(`document.getElementById('heading-menu-selection')?.textContent === 'Settings selected'`);
  await key('Home', 'Home', 36);
  assert.equal(await evaluate('document.activeElement.id'), 'heading-menu-dashboard');
  await key('End', 'End', 35);
  assert.equal(await evaluate('document.activeElement.id'), 'heading-menu-disabled');
  assert.deepEqual(exceptions, []);
  console.log('NavHeadingMenu: inherited sizing, navigation, keyboard focus outlines, typeahead, and disabled activation passed');
});
