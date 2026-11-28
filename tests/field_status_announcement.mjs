import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {test} from 'node:test';
import {runInNewContext} from 'node:vm';

const source = readFileSync(new URL('../src/field_status/use_field_status_announcement.js', import.meta.url), 'utf8');
const ancestor = (attributes, parentElement = null) => ({
  parentElement,
  getAttribute: name => attributes[name] ?? null,
});

async function route(root, ...updates) {
  const marker = 'field-status-1';
  const input = [marker, ...updates, false];
  const writes = [];
  let pending;
  await runInNewContext(`(async () => {${source}\n})()`, {
    document: {querySelector(selector) {
      assert.equal(selector, `[data-astryx-field-status="${marker}"]`);
      return root;
    }},
    dioxus: {recv() {
      let next = input.shift();
      if (typeof next === 'function') {
        next();
        next = input.shift();
      }
      if (next === false) pending?.(); // Flush at the simulated frame before unmount.
      return next;
    }},
    installAnnounce: () => ({createAnnouncer: () => ({
      announce(element, message, isEligible, priority) {
        assert.equal(element, root);
        if (isEligible()) pending = () => {
          if (isEligible()) writes.push([message, priority]);
        };
      },
      withdrawPending() { pending = undefined; },
      dispose() { pending = undefined; },
    })}),
  });
  return writes;
}

test('live ancestors own FieldStatus text, including through a non-live wrapper', async () => {
  for (const attributes of [{role: 'log'}, {role: 'status'}, {role: 'alert'}, {'aria-live': 'polite'}, {'aria-live': 'assertive'}]) {
    const root = {isConnected: true, parentElement: ancestor({}, ancestor(attributes))};
    assert.deepEqual(await route(root, ['Enter a name', true]), [], JSON.stringify(attributes));
  }
});

test('outside a live region, the shared channel still uses the status urgency', async () => {
  const root = {isConnected: true, parentElement: ancestor({role: 'log', 'aria-live': 'off'})};
  for (const [message, isError, priority] of [['Enter a name', true, 'assertive'], ['Saved', false, 'polite']]) {
    assert.deepEqual(await route(root, [message, isError]), [[message, priority]]);
  }
  assert.deepEqual(await route(root, ['Enter a name', true], ['', true]), []);
});

test('a newly live ancestor invalidates a queued shared write', async () => {
  const attributes = {};
  const root = {isConnected: true, parentElement: ancestor(attributes)};
  const writes = await route(root, ['Enter a name', true], () => { attributes.role = 'log'; });
  assert.deepEqual(writes, []);
});
