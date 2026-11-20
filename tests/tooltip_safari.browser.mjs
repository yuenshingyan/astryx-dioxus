// Installed macOS Safari, using its built-in WebDriver and the local text_patterns example.
import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {once} from 'node:events';
import {setTimeout as delay} from 'node:timers/promises';

const preview = process.env.TEXT_URL ?? 'http://127.0.0.1:8085';
const port = process.env.SAFARI_DRIVER_PORT ?? '8100';
const endpoint = `http://127.0.0.1:${port}`;
assert.equal(new URL(preview).hostname, '127.0.0.1', 'use the local Text preview');
assert.match(port, /^\d+$/);
assert.equal(await fetch(endpoint, {signal: AbortSignal.timeout(1000)}).then(() => true, () => false), false, 'use an unused WebDriver port');
const driver = spawn('/usr/bin/safaridriver', ['-p', port], {stdio: 'inherit'});
const exited = once(driver, 'exit');
let session;
const command = async (path, body, method = body === undefined ? 'GET' : 'POST') => {
  const response = await fetch(endpoint + path, {
    method, headers: {'Content-Type': 'application/json'}, body: JSON.stringify(body),
    signal: AbortSignal.timeout(20000),
  });
  const {value} = await response.json();
  if (!response.ok) throw Error(`${path}: ${value?.message ?? response.status}`);
  return value;
};
const execute = script => command(`${session}/execute/sync`, {script, args: []});
const waitFor = async expression => {
  const deadline = Date.now() + 10000;
  while (Date.now() < deadline) {
    if (await execute(`return (${expression});`)) return;
    await delay(50);
  }
  throw Error(`Timed out: ${expression}`);
};
const key = value => command(`${session}/actions`, {
  actions: [{type: 'key', id: 'keyboard', actions: [{type: 'keyDown', value}, {type: 'keyUp', value}]}],
});
try {
  await delay(500);
  const {sessionId, capabilities} = await command('/session', {capabilities: {alwaysMatch: {browserName: 'safari'}}});
  assert.ok(sessionId);
  session = `/session/${sessionId}`;
  console.log(`Installed Safari ${capabilities.browserVersion} on macOS ${capabilities['safari:platformVersion']}`);
  await command(`${session}/url`, {url: preview});
  await waitFor('document.getElementById("kb-cut")?.getAttribute("tabindex") === "0"');
  await execute('document.getElementById("kb-before").focus();');
  await key('\uE004'); // Tab
  await waitFor('document.activeElement.id === "kb-cut"');
  await waitFor('document.querySelector("[role=tooltip]:popover-open")?.textContent === "Full truncated text available to keyboard users"');
  assert.equal(await execute(`
    const tip = document.querySelector('[role=tooltip]:popover-open');
    const trigger = document.activeElement;
    const a = trigger.getBoundingClientRect(), b = tip.getBoundingClientRect();
    return (b.bottom <= a.top || b.top >= a.bottom) &&
      b.left >= 0 && b.right <= innerWidth && b.top >= 0 && b.bottom <= innerHeight &&
      trigger.getAttribute('aria-describedby')?.split(/\\s+/).includes(tip.id);
  `), true, 'tooltip is placed clear of the trigger, inside the viewport, and describes the trigger');
  await key('\uE00C'); // Escape
  const tipId = await execute('return document.activeElement.getAttribute("aria-describedby");');
  assert.ok(tipId, 'Escape leaves focus on the described Text');
  assert.equal(await execute('return document.activeElement.id;'), 'kb-cut');
  await waitFor('!document.querySelector("[role=tooltip]:popover-open")');
  await execute('document.getElementById("kb-resize").click();');
  await waitFor(`!document.getElementById(${JSON.stringify(tipId)}) && !document.getElementById('kb-cut').hasAttribute('tabindex') && !document.getElementById('kb-cut').hasAttribute('aria-describedby')`);
  console.log('Installed Safari tooltip placement, keyboard focus, Escape and overflow cleanup: PASS');
} finally {
  try {
    if (session) await command(session, undefined, 'DELETE');
  } finally {
    driver.kill();
    await exited;
  }
}
