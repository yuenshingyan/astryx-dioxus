import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {setImmediate} from 'node:timers/promises';
import {runInNewContext} from 'node:vm';

const script = readFileSync(new URL('../src/radio_list/sync_radios.js', import.meta.url), 'utf8');
const form = new EventTarget();
const group = new EventTarget();
const radio = (owner, controlled, checked) => ({
  matches: () => controlled,
  closest: () => owner,
  dataset: {radioChecked: String(checked)},
  checked,
});
const selected = radio(group, true, true);
const unselected = radio(group, true, false);
const unmanaged = radio(group, false, false);
const nested = radio(new EventTarget(), true, false);
group.closest = selector => selector === 'form' ? form : null;
group.querySelectorAll = () => [selected, unselected, nested];

let dispose;
const untilDisposed = new Promise(resolve => { dispose = resolve; });
let receiveCount = 0;
const timers = new Set();
const flush = () => {
  for (const run of timers) {
    timers.delete(run);
    run();
  }
};
const done = runInNewContext(`(async () => { ${script} })()`, {
  dioxus: {recv: () => receiveCount++ === 0 ? Promise.resolve('group-id') : untilDisposed},
  CSS: {escape: value => value},
  document: {querySelector: () => group},
  MutationObserver: class { observe() {} disconnect() {} },
  setTimeout: run => { timers.add(run); return run; },
  clearTimeout: run => timers.delete(run),
});
await setImmediate();

assert.equal(form.dispatchEvent(new Event('reset', {cancelable: true})), true);
// Stand in for the native default action, which runs after reset dispatch.
selected.checked = false;
unselected.checked = true;
unmanaged.checked = true;
nested.checked = true;
assert.equal(selected.checked, false, 'does not run during reset dispatch');
flush();
assert.deepEqual([selected.checked, unselected.checked, unmanaged.checked, nested.checked], [true, false, true, true]);

form.dispatchEvent(new Event('reset'));
assert.equal(timers.size, 1);
dispose();
await done;
assert.equal(timers.size, 0, 'unmount cancels queued reconciliation');
form.dispatchEvent(new Event('reset'));
assert.equal(timers.size, 0, 'unmount removes the reset listener');
console.log('RadioList: deferred reset reconciliation, ownership boundaries, and cleanup passed');
