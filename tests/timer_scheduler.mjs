import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {runInNewContext} from 'node:vm';

const source = await readFile(new URL('../src/timer/timer.js', import.meta.url), 'utf8');
let now = 5_000_000;
let receive;
let scheduled;
const text = {nodeType: 3, data: '0s'};
const node = {isConnected: true, firstChild: text, dateTime: 'PT0S'};
const run = runInNewContext(`(async () => { ${source} })()`, {
  dioxus: {recv: () => new Promise(resolve => { receive = resolve; })},
  document: {querySelector: () => node}, CSS: {escape: value => value},
  Node: {TEXT_NODE: 3}, Date: {now: () => now},
  setTimeout: (tick, delay) => { scheduled = {tick, delay}; return 1; },
  clearTimeout: () => { scheduled = undefined; },
});
const send = async value => { receive(value); await new Promise(setImmediate); };
const check = (label, duration, delay) => {
  assert.deepEqual([text.data, node.dateTime, scheduled.delay], [label, duration, delay]);
};
const tick = advance => { now += advance; scheduled.tick(); };
await send(['timer', [now - 59_500, false]]);
check('59s', 'PT59S', 500);
tick(500);
check('1m 00s', 'PT60S', 1000);
await send([now - 3_599_750, false]);
check('59m 59s', 'PT3599S', 250);
tick(250);
check('1h 00m', 'PT3600S', 60_000);
tick(125_000); // Delayed browser timer catches up from the wall clock.
check('1h 02m', 'PT3720S', 55_000);
await send([now - 3_725_000, true]);
check('1:02:05', 'PT3725S', 1000);
await send([now + 9_000_000_000, true]);
check('0:00', 'PT0S', 2_147_483_647);
await send(false);
await run;
assert.equal(scheduled, undefined);
console.log('Timer scheduler: display boundaries, delayed tick, future clamp, and cancellation passed');
