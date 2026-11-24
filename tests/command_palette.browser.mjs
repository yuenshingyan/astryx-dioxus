import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8270'});
  const trigger = "document.getElementById('command-palette-trigger')";
  const modal = "document.querySelector('#command-palette-examples dialog:modal')";
  const input = `${modal}?.querySelector('[role=combobox]')`;
  await waitFor(trigger);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  await evaluate(`${trigger}.focus(); ${trigger}.click()`);
  await waitFor(`${input} && document.activeElement === ${input}`);
  const press = async (key, windowsVirtualKeyCode) => {
    for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key, code:key, windowsVirtualKeyCode});
  };
  const current = `document.getElementById((${input}).getAttribute('aria-activedescendant')).textContent`;
  assert.equal(await evaluate(current), 'Home');
  await press('ArrowDown', 40);
  await waitFor(`${current} === 'Settings'`);
  await send('Input.insertText', {text:'HEL'});
  await waitFor(`${current} === 'Help'`);
  await send('Input.insertText', {text:'X'});
  await waitFor(`${modal}?.querySelector('[role=status]')?.textContent === 'No commands found'`);
  assert.equal(await evaluate(`(${input}).hasAttribute('aria-activedescendant')`), false);
  await press('Backspace', 8);
  await waitFor(`${current} === 'Help'`);
  await press('Enter', 13);
  await waitFor(`!${modal} && document.activeElement === ${trigger} && document.querySelector('#command-palette-examples output').textContent === 'help'`);
  const inline = "document.querySelector('#command-palette-examples [role=combobox]')";
  assert.equal(await evaluate(`document.getElementById(${inline}.getAttribute('aria-controls')).querySelectorAll('[role=option]').length`), 5);
  assert.deepEqual(exceptions, []);
  console.log('CommandPalette: inline options, modal focus, filtering, active descendant, empty state, and selection passed');
});
