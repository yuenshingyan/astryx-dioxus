// Dependency-free browser checks. Requires Node.js 22+ and a dedicated Chrome
// instance exposing CDP. See docs/button-port.md for setup commands.
import assert from 'node:assert/strict';
import { readFile, writeFile } from 'node:fs/promises';

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
const browserErrors = [];
socket.addEventListener('message', ({ data }) => {
  const message = JSON.parse(data);
  if (message.method === 'Runtime.exceptionThrown') {
    browserErrors.push(message.params.exceptionDetails.exception?.description ?? message.params.exceptionDetails.text);
  }
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
  await send('Runtime.enable');
  await send('Page.bringToFront');
  await send('Emulation.setDeviceMetricsOverride', { width: 1200, height: 1400, deviceScaleFactor: 1, mobile: false });
  await send('Page.navigate', { url: showcaseUrl });
  await waitFor('Boolean(document.querySelector("#counter"))');
  await evaluate('document.fonts.ready.then(() => true)');
  await evaluate('document.querySelector("#counter").scrollIntoView({block:"center"})');

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
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#loading .astryx-spinner__rotor")).animationDuration'), '300s');
  await send('Emulation.setEmulatedMedia', { features: [{ name: 'forced-colors', value: 'active' }] });
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#counter")).borderTopWidth'), '1px');
  await send('Emulation.setEmulatedMedia', { features: [] });
  const reference = JSON.parse(await readFile(new URL('./fixtures/button-neutral-reference.json', import.meta.url), 'utf8'));
  for (const mode of ['light', 'dark']) {
    await evaluate(`document.querySelector('#documented-patterns').style.colorScheme = '${mode}'`);
    await waitFor(`getComputedStyle(document.querySelector('#neutral-primary')).backgroundColor === ${JSON.stringify(reference.measurements[mode][0].background)}`);
    for (const expected of reference.measurements[mode]) {
      const actual = await evaluate(`(() => { const button=document.querySelector('#neutral-${expected.variant}'); const css=getComputedStyle(button); return {label:button.textContent.trim(),variant:button.dataset.variant,size:button.dataset.size,background:css.backgroundColor,color:css.color,radius:css.borderRadius,fontSize:css.fontSize,lineHeight:css.lineHeight,fontWeight:css.fontWeight,width:button.getBoundingClientRect().width,height:button.getBoundingClientRect().height,paddingInlineStart:css.paddingInlineStart,paddingInlineEnd:css.paddingInlineEnd,gap:css.gap}; })()`);
      const { width: actualWidth, ...actualStyle } = actual;
      const { width: expectedWidth, ...expectedStyle } = expected;
      assert.deepEqual(actualStyle, expectedStyle, `${mode} ${expected.variant} must match the measured upstream Neutral example`);
      assert.ok(Math.abs(actualWidth - expectedWidth) <= 1 / 32, `${mode} ${expected.variant} width differs by more than two Chromium layout subpixels`);
    }
  }
  await evaluate('document.querySelector("#documented-patterns").style.colorScheme = "light"');
  await waitFor('Boolean(document[Symbol.for("astryx.dioxus.button.runtime")])');
  await focus('tooltip');
  await waitFor('Boolean(document.querySelector("#tooltip + [role=tooltip]:popover-open"))');
  assert.equal(await evaluate('document.querySelector("#tooltip + [role=tooltip]").textContent'), 'Save your changes');
  assert.ok(await evaluate('document.querySelector("#tooltip").getAttribute("aria-describedby").includes(document.querySelector("#tooltip + [role=tooltip]").id)'));
  await key('Escape', 'Escape', 27);
  assert.equal(await evaluate('Boolean(document.querySelector("#tooltip + [role=tooltip]:popover-open"))'), false);
  await focus('tooltip-same');
  await waitFor('Boolean(document.querySelector("#tooltip-same + [role=tooltip]:popover-open"))');
  assert.equal(await evaluate('document.querySelector("#tooltip-same").hasAttribute("aria-describedby")'), false, 'repeated name must not become a duplicate description');
  await focus('tooltip-description');
  await waitFor('Boolean(document.querySelector("#tooltip-description + [role=tooltip]:popover-open"))');
  assert.ok(await evaluate('document.querySelector("#tooltip-description").getAttribute("aria-describedby").startsWith("button-extra-description ")'));
  await focus('tooltip-disabled');
  await waitFor('Boolean(document.querySelector("#tooltip-disabled + [role=tooltip]:popover-open"))');
  await key('Escape', 'Escape', 27);

  const hoverPoint = await evaluate('(() => { const button=document.querySelector("#tooltip"); button.scrollIntoView({block:"center"}); const box=button.getBoundingClientRect(); return {x:box.x+box.width/2,y:box.y+box.height/2}; })()');
  await send('Input.dispatchMouseEvent', { type: 'mouseMoved', ...hoverPoint });
  await waitFor('Boolean(document.querySelector("#tooltip + [role=tooltip]:popover-open"))');
  const tipPoint = await evaluate('(() => { const box=document.querySelector("#tooltip + [role=tooltip]").getBoundingClientRect(); return {x:box.x+box.width/2,y:box.y+box.height/2}; })()');
  await send('Input.dispatchMouseEvent', { type: 'mouseMoved', ...tipPoint });
  await new Promise(resolve => setTimeout(resolve, 200));
  assert.ok(await evaluate('Boolean(document.querySelector("#tooltip + [role=tooltip]:popover-open"))'), 'tooltip must remain hoverable');
  await send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: 0, y: 0 });
  await waitFor('!document.querySelector("#tooltip + [role=tooltip]:popover-open")');
  const cutPoint = await evaluate('(() => { const button=document.querySelector("#truncated"); button.scrollIntoView({block:"center"}); const box=button.getBoundingClientRect(); return {x:box.x+box.width/2,y:box.y+box.height/2}; })()');
  await send('Input.dispatchMouseEvent', { type: 'mouseMoved', ...cutPoint });
  await new Promise(resolve => setTimeout(resolve, 250));
  assert.equal(await evaluate('document.querySelectorAll(".astryx-button-tooltip:popover-open").length'), 0, 'hover must not open the automatic cut-label tooltip');
  await focus('counter');
  await key('Tab', 'Tab', 9);
  await focus('truncated');
  await waitFor('Array.from(document.querySelectorAll(".astryx-button-tooltip:popover-open")).some(tip => tip.textContent === "A long label that must truncate")');
  assert.equal(await evaluate('document.querySelector("#truncated").hasAttribute("aria-describedby")'), false);
  await key('Escape', 'Escape', 27);
  await focus('tooltip-precedence');
  await waitFor('Array.from(document.querySelectorAll(".astryx-button-tooltip:popover-open")).some(tip => tip.textContent === "Caller tooltip wins")');
  assert.equal(await evaluate('document.querySelectorAll(".astryx-button-tooltip:popover-open").length'), 1);
  await focus('counter');

  assert.equal(await evaluate('document.querySelector("#inherited-size").dataset.size'), 'sm');
  await click('provider-toggle');
  await waitFor('document.querySelector("#inherited-size").dataset.size === "lg"');
  assert.equal(await evaluate('document.querySelector("#explicit-size").dataset.size'), 'md');
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#default-icon .astryx-icon")).width'), '16px');
  assert.equal(await evaluate('document.querySelector("#default-icon").hasAttribute("aria-describedby")'), false, 'decorative icons must not duplicate the label in the tooltip description');
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#large-icon .astryx-icon")).width'), '20px');
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#explicit-icon .astryx-icon")).width'), '24px');
  assert.equal(await evaluate('document.querySelector("#input-addon").getAttribute("aria-disabled")'), 'true');
  assert.equal(await evaluate('document.querySelector("#input-addon").dataset.size'), 'sm');
  assert.equal(await evaluate('document.querySelector("#group-a").dataset.elevation'), 'none');
  assert.equal(await evaluate('document.querySelector("#group-disabled").getAttribute("aria-disabled")'), 'true');
  await waitFor('document.querySelector("#group-a").tabIndex === 0 && document.querySelector("#group-b").tabIndex === -1 && document.querySelector("#group-c").tabIndex === -1');
  await focus('group-a');
  await key('ArrowRight', 'ArrowRight', 39);
  assert.equal(await evaluate('document.activeElement.id'), 'group-b');
  await key('End', 'End', 35);
  assert.equal(await evaluate('document.activeElement.id'), 'group-c');
  await key('Home', 'Home', 36);
  assert.equal(await evaluate('document.activeElement.id'), 'group-a');
  await focus('vertical-a');
  await key('ArrowDown', 'ArrowDown', 40);
  assert.equal(await evaluate('document.activeElement.id'), 'vertical-b');
  assert.equal(await evaluate('document.querySelector("#custom-link").dataset.router'), 'custom');
  assert.equal(await evaluate('document.querySelector("#explicit-link").dataset.router'), 'custom');
  assert.equal(await evaluate('document.querySelector("#custom-link-disabled").tagName'), 'BUTTON');
  assert.equal(await evaluate('document.querySelector("#explicit-overrides-provider").dataset.router'), 'alternate');
  await click('router-toggle');
  await waitFor('document.querySelector("#custom-link").dataset.router === "alternate"');
  await focus('focus-swap');
  await click('focus-swap-toggle');
  await waitFor('document.querySelector("#focus-swap").tagName === "BUTTON"');
  await waitFor('document.activeElement.id === "focus-swap"');
  await click('focus-swap-toggle');
  await waitFor('document.querySelector("#focus-swap").tagName === "A"');
  await waitFor('document.activeElement.id === "focus-swap"');
  await waitFor('Boolean(document.querySelector("#focus-swap + [role=tooltip]:popover-open"))');
  await focus('callback-button');
  await waitFor('document.querySelector("#callback-count").textContent === "Callbacks: 1"');
  await key('Escape', 'Escape', 27);
  await waitFor('document.querySelector("#callback-count").textContent === "Callbacks: 2"');
  await focus('custom-link');
  await waitFor('document.querySelector("#callback-count").textContent === "Callbacks: 12"');

  // Scope cleanup must remove DOM controllers/layers and cancel Button's task.
  await focus('unmount-action');
  await waitFor('Boolean(document.querySelector("#unmount-action + [role=tooltip]:popover-open"))');
  const unmountedController = await evaluate('document.querySelector("#unmount-action").dataset.astryxButtonId');
  await click('unmount-action');
  await waitFor('document.querySelector("#unmount-action").getAttribute("aria-busy") === "true"');
  await click('unmount-toggle');
  await waitFor('!document.querySelector("#unmount-action")');
  await waitFor(`![...document[Symbol.for('astryx.dioxus.button.runtime')].records.values()].some(record => record.id === ${JSON.stringify(unmountedController)})`);
  await new Promise(resolve => setTimeout(resolve, 900));
  assert.equal(await evaluate('document.querySelector("#completed-tasks").textContent'), 'Completed tasks: 0');
  assert.equal(await evaluate(`Boolean(document.getElementById(${JSON.stringify(unmountedController + '-tooltip')}))`), false);

  await send('Emulation.setDeviceMetricsOverride', { width: 390, height: 844, deviceScaleFactor: 1, mobile: true });
  await send('Emulation.setTouchEmulationEnabled', { enabled: true, maxTouchPoints: 1 });
  const targetPoint = async (id, offsetY = 0) => {
    await evaluate(`document.getElementById(${JSON.stringify(id)}).scrollIntoView({block:'center'})`);
    return evaluate(`(() => { const box = document.getElementById(${JSON.stringify(id)}).getBoundingClientRect(); return {x:box.x + box.width / 2,y:${offsetY} ? box.y + ${offsetY} : box.y + box.height / 2}; })()`);
  };
  const touch = (type, point) => send('Input.dispatchTouchEvent', { type, touchPoints: point ? [{ ...point, id: 1 }] : [] });
  const standalone = await targetPoint('touch-button');
  await touch('touchStart', standalone);
  await waitFor('document.querySelector("#touch-button").dataset.astryxPress === "on"');
  await touch('touchEnd');
  await waitFor('document.querySelector("#touch-count").textContent === "Touches: 1"');
  await waitFor('!document.querySelector("#touch-button").hasAttribute("data-astryx-press")');
  assert.equal(await evaluate('document.querySelectorAll(".astryx-button-tooltip:popover-open").length'), 0, 'touch must not open a cut-label tooltip');
  const band = await targetPoint('touch-button', -5);
  await touch('touchStart', band);
  await touch('touchEnd');
  await waitFor('document.querySelector("#touch-count").textContent === "Touches: 2"');

  const cutTouch = await targetPoint('truncated');
  await touch('touchStart', cutTouch);
  await touch('touchEnd');
  assert.equal(await evaluate('document.querySelectorAll(".astryx-button-tooltip:popover-open").length'), 0, 'tapping a cut label must activate without opening a tooltip');
  const disabledHint = await targetPoint('tooltip-disabled');
  await touch('touchStart', disabledHint);
  await touch('touchEnd');
  await waitFor('Boolean(document.querySelector("#tooltip-disabled + [role=tooltip]:popover-open"))');
  await touch('touchStart', {x:2,y:2});
  await touch('touchEnd');
  await waitFor('!document.querySelector("#tooltip-disabled + [role=tooltip]:popover-open")');

  const groupStart = await targetPoint('group-a');
  const groupEnd = await evaluate('(() => { const box = document.querySelector("#group-b").getBoundingClientRect(); return {x:box.x+box.width/2,y:box.y+box.height/2}; })()');
  await touch('touchStart', groupStart);
  await touch('touchMove', groupEnd);
  await waitFor('document.querySelector("#group-b").dataset.astryxPress === "on"');
  await touch('touchEnd');
  await waitFor('document.querySelector("#group-value").textContent === "Group action: Second"');
  const scroller = await targetPoint('scroll-touch-button');
  await touch('touchStart', scroller);
  assert.equal(await evaluate('document.querySelector("#scroll-touch-button").hasAttribute("data-astryx-press")'), false, 'scrollable hosts delay finger feedback');
  await evaluate('document.querySelector("#touch-scroller").scrollTop = 30');
  await touch('touchEnd');
  await new Promise(resolve => setTimeout(resolve, 250));
  assert.equal(await evaluate('document.querySelector("#scroll-touch-button").hasAttribute("data-astryx-press")'), false);
  assert.equal(await evaluate('document.querySelector("#touch-count").textContent'), 'Touches: 2', 'scroll must cancel activation');
  await focus('scroll-touch-button');
  await key('Enter', 'Enter', 13);
  await waitFor('document.querySelector("#touch-count").textContent === "Touches: 3"');
  const repeatPoint = await targetPoint('touch-button');
  await touch('touchStart', repeatPoint);
  await touch('touchEnd');
  await waitFor('document.querySelector("#touch-count").textContent === "Touches: 4"');
  await new Promise(resolve => setTimeout(resolve, 50));
  await touch('touchStart', repeatPoint);
  await new Promise(resolve => setTimeout(resolve, 200));
  assert.equal(await evaluate('document.querySelector("#touch-button").dataset.astryxPress'), 'on', 'old fade timer must not erase a new held press');
  await touch('touchEnd');
  await waitFor('document.querySelector("#touch-count").textContent === "Touches: 5"');
  await evaluate('(() => { const style=document.createElement("style"); style.id="caller-pseudo-style"; style.textContent="#touch-button::before {background-color:rgb(1,2,3)}"; document.head.append(style); })()');
  const decoratedPoint = await targetPoint('touch-button');
  await touch('touchStart', decoratedPoint);
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#touch-button"),"::before").backgroundColor'), 'rgb(1, 2, 3)', 'caller pseudo-element styling must win over the press layer');
  await touch('touchEnd');
  await waitFor('document.querySelector("#touch-count").textContent === "Touches: 6"');
  await evaluate('document.querySelector("#caller-pseudo-style").remove()');
  const popupPoint = await targetPoint('popup-button');
  await touch('touchStart', popupPoint);
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#popup-button")).transform'), 'none', 'open popup anchors must not scale');
  await touch('touchEnd');
  const occludedPoint = await targetPoint('touch-button');
  await evaluate('(() => { const dialog=document.createElement("dialog"); dialog.id="test-modal"; dialog.style.cssText="inset:0;margin:0;width:100vw;height:100vh;max-width:none;max-height:none;padding:0"; dialog.textContent="Modal blocks background touch targets"; document.body.append(dialog); dialog.showModal(); })()');
  await touch('touchStart', occludedPoint);
  await touch('touchEnd');
  assert.equal(await evaluate('document.querySelector("#touch-count").textContent'), 'Touches: 6', 'expanded touch targets must not activate through a modal');
  await evaluate('document.querySelector("#test-modal").close(); document.querySelector("#test-modal").remove()');

  await send('Emulation.setTouchEmulationEnabled', { enabled: false });
  await send('Emulation.setDeviceMetricsOverride', { width: 1200, height: 2200, deviceScaleFactor: 1, mobile: false });
  await evaluate('window.scrollTo(0,0)');
  if (process.env.SCREENSHOT_PATH) {
    const screenshot = await send('Page.captureScreenshot', { captureBeyondViewport: true });
    await writeFile(process.env.SCREENSHOT_PATH, Buffer.from(screenshot.data, 'base64'));
  }
  assert.deepEqual(browserErrors, [], 'showcase must not produce uncaught browser exceptions');
  console.log('Button browser checks passed: upstream light/dark visual metrics; keyboard/pointer/touch activation; disabled/busy/form guards; async deduplication and unmount cancellation; tooltip naming, truncation, dismissal and cleanup; inherited/icon sizes; connected-group focus/touch; router adapters; focus retention; reduced motion and forced colors.');
} finally {
  socket.close();
  await fetch(`${cdpUrl}/json/close/${target.id}`);
}
