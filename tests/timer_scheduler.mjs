import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {test} from 'node:test';
import {runInNewContext} from 'node:vm';

const source = await readFile(new URL('../src/timer/timer.js', import.meta.url), 'utf8');

function startBridge({hasNode = true} = {}) {
  const mountedAt = 5_000_000;
  let now = mountedAt;
  let receive;
  let closed = false;
  let timeoutId = 0;
  const timers = new Map();
  const writes = [];
  let label = '0s';
  let duration = 'PT0S';
  const text = {
    nodeType: 3,
    get data() { return label; },
    set data(value) { label = value; writes.push(value); },
  };
  const node = {
    isConnected: true,
    firstChild: text,
    get dateTime() { return duration; },
    set dateTime(value) { duration = value; writes.push(value); },
  };
  // Reproduce the web evaluator's channel close after normal completion.
  const run = runInNewContext(`(async () => { ${source}\ndioxus.close(); })()`, {
    dioxus: {
      recv: () => new Promise(resolve => { receive = resolve; }),
      close: () => { closed = true; },
    },
    document: {querySelector: () => hasNode ? node : null},
    CSS: {escape: value => value},
    Node: {TEXT_NODE: 3},
    Date: {now: () => now},
    setTimeout: (tick, delay) => {
      const id = ++timeoutId;
      timers.set(id, {tick, due: now + delay});
      return id;
    },
    clearTimeout: id => { timers.delete(id); },
  });
  return {
    mountedAt, run, timers, writes,
    get closed() { return closed; },
    get now() { return now; },
    advance: ms => { now += ms; },
    send: async value => { receive(value); await new Promise(setImmediate); },
    check: (expectedLabel, expectedDuration, delay) => {
      assert.equal(timers.size, 1);
      assert.deepEqual([label, duration, [...timers.values()][0].due - now], [
        expectedLabel, expectedDuration, delay,
      ]);
    },
    tick: advance => {
      assert.equal(timers.size, 1);
      now += advance;
      const [id, {tick, due}] = [...timers.entries()][0];
      assert.ok(now >= due, 'timeout fired before its deadline');
      timers.delete(id);
      tick();
    },
  };
}

async function stop(bridge) {
  await bridge.send(false);
  await bridge.run;
  assert.equal(bridge.timers.size, 0);
  assert.equal(bridge.closed, true);
}

test('display boundaries, delayed ticks, and future timeout clamp', async () => {
  const bridge = startBridge();
  await bridge.send(['timer', [bridge.now - 59_500, false]]);
  bridge.check('59s', 'PT59S', 500);
  bridge.tick(500);
  bridge.check('1m 00s', 'PT60S', 1000);
  await bridge.send([bridge.now - 3_599_750, false]);
  bridge.check('59m 59s', 'PT3599S', 250);
  bridge.tick(250);
  bridge.check('1h 00m', 'PT3600S', 60_000);
  bridge.tick(125_000);
  bridge.check('1h 02m', 'PT3720S', 55_000);
  await bridge.send([bridge.now - 3_725_000, true]);
  bridge.check('1:02:05', 'PT3725S', 1000);
  bridge.tick(1000);
  bridge.check('1:02:06', 'PT3726S', 1000);
  await bridge.send([bridge.now + 9_000_000_000, true]);
  bridge.check('0:00', 'PT0S', 2_147_483_647);
  await stop(bridge);
});

test('unchanged config retains the timeout; genuine changes retain the mount origin', async () => {
  const bridge = startBridge();
  await bridge.send(['timer', [null, false]]);
  bridge.check('0s', 'PT0S', 1000);
  const [initialTimeout] = bridge.timers.keys();
  bridge.advance(250);
  await bridge.send([null, false]);
  bridge.check('0s', 'PT0S', 750);
  assert.deepEqual([...bridge.timers.keys()], [initialTimeout]);
  assert.deepEqual(bridge.writes, []);

  await bridge.send([bridge.now + 2750, false]);
  bridge.check('0s', 'PT0S', 3750);
  assert.deepEqual(bridge.writes, []);
  bridge.tick(3750);
  bridge.check('1s', 'PT1S', 1000);

  await bridge.send([null, true]);
  bridge.check('0:04', 'PT4S', 1000);
  const [clockTimeout] = bridge.timers.keys();
  await bridge.send([bridge.mountedAt, true]);
  assert.deepEqual([...bridge.timers.keys()], [clockTimeout]);
  await stop(bridge);
});

test('a missing mounted node leaves no timeout and closes the evaluator channel', async () => {
  const bridge = startBridge({hasNode: false});
  await bridge.send(['missing', [null, false]]);
  await bridge.run;
  assert.equal(bridge.timers.size, 0);
  assert.equal(bridge.closed, true);
});
