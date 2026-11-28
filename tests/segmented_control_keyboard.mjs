import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {runInNewContext} from 'node:vm';

const source = await readFile(new URL('../src/segmented_control/segmented_control.js', import.meta.url), 'utf8');
const document = new EventTarget();
let direction = 'ltr';
let focused;
const clicks = [];
const root = {querySelectorAll: () => items};
const items = ['grid', 'chart', 'list', 'table'].map(value => ({
  tabIndex: -1,
  closest: selector => selector === '[role=radiogroup]' ? root : null,
  matches: selector => selector === '[aria-checked=true]' ? value === 'grid' : value === 'chart',
  focus() { focused = this; },
  click() { clicks.push(value); },
}));
const [grid, , list] = items;
document.getElementById = id => id === 'segments' ? root : null;
let receive;
const run = runInNewContext(`(async () => { ${source} })()`, {
  document,
  dioxus: {recv: () => new Promise(resolve => { receive = resolve; })},
  getComputedStyle: () => ({direction}),
  MutationObserver: class { observe() {} disconnect() {} },
});
receive('segments');
await new Promise(setImmediate);

const press = key => {
  const event = Object.assign(new Event('keydown', {cancelable: true}), {key});
  Object.defineProperty(event, 'target', {value: focused});
  document.dispatchEvent(event);
  return event.defaultPrevented;
};

for (const dir of ['ltr', 'rtl']) {
  direction = dir;
  grid.focus();
  for (const key of ['ArrowUp', 'ArrowDown']) {
    assert.equal(press(key), false, `${dir} ${key} must leave the default action available`);
    assert.equal(focused, grid, `${dir} ${key} must not move focus`);
    assert.deepEqual(clicks, [], `${dir} ${key} must not request selection`);
  }

  const forward = dir === 'rtl' ? 'ArrowLeft' : 'ArrowRight';
  assert.equal(press(forward), true);
  assert.equal(focused, list, `${dir} ${forward} must skip the disabled segment`);
  assert.deepEqual(clicks.splice(0), ['list']);
  assert.deepEqual(items.map(item => item.tabIndex), [0, -1, -1, -1], 'the caller has not changed selection');
}

receive();
await run;
console.log('SegmentedControl keyboard: vertical arrows pass through, horizontal arrows preserve LTR/RTL navigation');
