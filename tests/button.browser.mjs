// Dependency-free browser checks. Requires Node.js 22+ and a dedicated Chrome
// instance exposing CDP. See docs/button-port.md for setup commands.
import assert from 'node:assert/strict';
import { writeFile } from 'node:fs/promises';

const cdpUrl = process.env.CDP_URL ?? 'http://127.0.0.1:9222';
const showcaseUrl = process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8080';
const response = await fetch(`${cdpUrl}/json/new?about:blank`, { method: 'PUT' });
assert.ok(response.ok, 'Chrome must expose its debugging endpoint');
const target = await response.json();
const socket = new WebSocket(target.webSocketDebuggerUrl);
await new Promise((resolve, reject) => {
  socket.addEventListener('open', resolve, { once: true });
  socket.addEventListener('error', reject, { once: true });
});
let nextId = 0;
const requests = new Map();
socket.addEventListener('message', ({ data }) => {
  const message = JSON.parse(data);
  const request = requests.get(message.id);
  if (!request) return;
  requests.delete(message.id);
  clearTimeout(request.timer);
  if (message.error) request.reject(new Error(message.error.message));
  else request.resolve(message.result);
});
function send(method, params = {}) {
  return new Promise((resolve, reject) => {
    const id = ++nextId;
    const timer = setTimeout(() => {
      requests.delete(id);
      reject(new Error(`CDP timeout: ${method}`));
    }, 15000);
    requests.set(id, { resolve, reject, timer });
    socket.send(JSON.stringify({ id, method, params }));
  });
}
async function evaluate(expression) {
  const result = await send('Runtime.evaluate', {
    expression, returnByValue: true, awaitPromise: true,
  });
  assert.equal(result.exceptionDetails, undefined, JSON.stringify(result.exceptionDetails));
  return result.result.value;
}
async function waitFor(expression) {
  const deadline = Date.now() + 15000;
  while (Date.now() < deadline) {
    if (await evaluate(expression)) return;
    await new Promise(resolve => setTimeout(resolve, 25));
  }
  assert.fail(`Timed out waiting for: ${expression}`);
}
async function key(key, code, windowsVirtualKeyCode) {
  await send('Input.dispatchKeyEvent', {
    type: 'keyDown', key, code, windowsVirtualKeyCode,
    text: key === 'Enter' ? '\r' : key === ' ' ? ' ' : undefined,
  });
  await send('Input.dispatchKeyEvent', { type: 'keyUp', key, code, windowsVirtualKeyCode });
}
const count = () => evaluate('Number(document.querySelector("#click-count").textContent.match(/\\d+/)[0])');
const click = id => evaluate(`document.getElementById(${JSON.stringify(id)}).click()`);
const focus = id => evaluate(`document.getElementById(${JSON.stringify(id)}).focus()`);

try {
  await send('Page.enable');
  await send('Page.bringToFront');
  await send('Emulation.setDeviceMetricsOverride', { width: 1200, height: 1400, deviceScaleFactor: 1, mobile: false });
  await send('Page.navigate', { url: showcaseUrl });
  await waitFor('Boolean(document.querySelector("#counter"))');

  const point = await evaluate('(() => { const box = document.querySelector("#counter").getBoundingClientRect(); return { x: box.x + box.width / 2, y: box.y + box.height / 2 }; })()');
  await send('Input.dispatchMouseEvent', { type: 'mousePressed', ...point, button: 'left', clickCount: 1 });
  await send('Input.dispatchMouseEvent', { type: 'mouseReleased', ...point, button: 'left', clickCount: 1 });
  await waitFor('document.querySelector("#click-count").textContent === "Click count: 1"');
  await focus('counter');
  await key('Enter', 'Enter', 13);
  await waitFor('document.querySelector("#click-count").textContent === "Click count: 2"');
  await key(' ', 'Space', 32);
  await waitFor('document.querySelector("#click-count").textContent === "Click count: 3"');

  const beforeBlocked = await evaluate('document.querySelector("#ancestor-count").textContent');
  await focus('disabled');
  assert.equal(await evaluate('document.activeElement.id'), 'disabled');
  assert.equal(await evaluate('document.querySelector("#disabled").disabled'), false);
  await click('disabled');
  await key('Enter', 'Enter', 13);
  await key(' ', 'Space', 32);
  await click('loading');
  assert.equal(await count(), 3);
  assert.equal(await evaluate('document.querySelector("#ancestor-count").textContent'), beforeBlocked);

  assert.equal(await evaluate('document.querySelector("#link").tagName'), 'A');
  assert.equal(await evaluate('document.querySelector("#disabled-link").tagName'), 'BUTTON');
  assert.equal(await evaluate('document.querySelector("#disabled-link").hasAttribute("href")'), false);
  assert.equal(await evaluate('document.querySelector("#busy-link").tagName'), 'A');
  await click('busy-link');
  await click('disabled-link');
  assert.equal(await evaluate('location.hash'), '');
  await click('link');
  assert.equal(await evaluate('location.hash'), '#variants-heading');

  await click('disabled-submit');
  await click('busy-submit');
  assert.equal(await evaluate('document.querySelector("#form-count").textContent'), 'Submissions: 0');
  await click('submit');
  await waitFor('document.querySelector("#form-count").textContent === "Submissions: 1"');

  const idleSize = await evaluate('JSON.stringify(document.querySelector("#controlled-loading").getBoundingClientRect().toJSON())');
  await focus('controlled-loading');
  await evaluate('document.querySelector("#toggle-loading").click()');
  await waitFor('document.querySelector("#controlled-loading").getAttribute("aria-busy") === "true"');
  assert.equal(await evaluate('document.activeElement.id'), 'controlled-loading');
  assert.equal(await evaluate('JSON.stringify(document.querySelector("#controlled-loading").getBoundingClientRect().toJSON())'), idleSize);
  await waitFor('document.querySelector("#controlled-loading .astryx-button__status").textContent === "Loading"');
  await click('toggle-loading');
  await waitFor('!document.querySelector("#controlled-loading").hasAttribute("aria-busy")');

  await focus('async-action');
  await evaluate('document.querySelector("#async-action").click(); document.querySelector("#async-action").click()');
  await waitFor('document.querySelector("#async-action").getAttribute("aria-busy") === "true"');
  assert.equal(await evaluate('document.activeElement.id'), 'async-action');
  await waitFor('!document.querySelector("#async-action").hasAttribute("aria-busy")');
  assert.equal(await count(), 4, 'same-tick clicks must start only one non-interruptible action');

  await evaluate('document.querySelector("#interruptible-action").click(); document.querySelector("#interruptible-action").click()');
  await waitFor('document.querySelector("#interruptible-action").getAttribute("aria-busy") === "true"');
  assert.equal(await evaluate('document.querySelector("#interruptible-action").hasAttribute("aria-disabled")'), false);
  await waitFor('!document.querySelector("#interruptible-action").hasAttribute("aria-busy")');
  assert.equal(await count(), 6, 'interruptible actions must accept both clicks');
  await click('prevented-action');
  await new Promise(resolve => setTimeout(resolve, 50));
  assert.equal(await count(), 6, 'preventDefault must prevent click_action');

  const sizes = await evaluate(`Array.from(document.querySelectorAll('#sizes-heading + div .astryx-button')).map(button => button.getBoundingClientRect().height)`);
  assert.deepEqual(sizes, [28, 32, 36]);
  await focus('counter');
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#counter")).outlineColor'), 'rgb(21, 17, 12)', 'catalog focus color must follow its accent override');
  const iconSize = await evaluate('({ width: document.querySelector("#icon-only").getBoundingClientRect().width, height: document.querySelector("#icon-only").getBoundingClientRect().height })');
  assert.equal(iconSize.width, iconSize.height);
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#truncated .astryx-button__label")).textOverflow'), 'ellipsis');
  assert.ok(await evaluate('document.querySelector("#truncated .astryx-button__label").scrollWidth > document.querySelector("#truncated .astryx-button__label").clientWidth'));

  const accessibility = await send('Accessibility.getFullAXTree');
  const namedButton = accessibility.nodes.find(node => node.role?.value === 'button' && node.name?.value === 'Disabled');
  assert.ok(namedButton, 'disabled button must retain an accessible name and button role');
  assert.ok(namedButton.properties.some(property => property.name === 'disabled' && property.value.value === true));
  assert.ok(accessibility.nodes.some(node => node.role?.value === 'button' && node.name?.value === 'Save'), 'loading button must keep its accessible name');

  await send('Emulation.setEmulatedMedia', { features: [{ name: 'prefers-reduced-motion', value: 'reduce' }] });
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#loading .astryx-button__spinner")).animationName'), 'none');
  await send('Emulation.setEmulatedMedia', { features: [{ name: 'forced-colors', value: 'active' }] });
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#counter")).borderTopWidth'), '1px');
  await send('Emulation.setEmulatedMedia', { features: [] });
  if (process.env.SCREENSHOT_PATH) {
    const screenshot = await send('Page.captureScreenshot', { captureBeyondViewport: true });
    await writeFile(process.env.SCREENSHOT_PATH, Buffer.from(screenshot.data, 'base64'));
  }
  console.log('Button browser checks passed: pointer activation, native keyboard activation, disabled/busy guards, propagation, links, forms, loading geometry/focus, async deduplication, interruptible actions, naming, sizing, truncation, reduced motion, and forced colors.');
} finally {
  socket.close();
  await fetch(`${cdpUrl}/json/close/${target.id}`);
}
