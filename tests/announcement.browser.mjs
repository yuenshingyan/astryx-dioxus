// Internal announcement bridge contracts. Node 22+, Chrome CDP; no npm packages.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

const endpoint = process.env.CDP_URL ?? 'http://127.0.0.1:9222';
const target = await (await fetch(`${endpoint}/json/new?about:blank`, { method: 'PUT' })).json();
const socket = new WebSocket(target.webSocketDebuggerUrl);
await new Promise((resolve, reject) => {
  socket.addEventListener('open', resolve, { once: true });
  socket.addEventListener('error', reject, { once: true });
});
let serial = 0;
const pending = new Map();
socket.addEventListener('message', ({ data }) => {
  const message = JSON.parse(data);
  const request = pending.get(message.id);
  if (!request) return;
  pending.delete(message.id);
  clearTimeout(request.timer);
  if (message.error) request.reject(new Error(JSON.stringify(message.error)));
  else request.resolve(message.result);
});
const send = (method, params = {}) => new Promise((resolve, reject) => {
  const id = ++serial;
  const timer = setTimeout(() => { pending.delete(id); reject(new Error(`CDP timeout: ${method}`)); }, 10000);
  pending.set(id, { resolve, reject, timer });
  socket.send(JSON.stringify({ id, method, params }));
});
const evaluate = async expression => {
  const result = await send('Runtime.evaluate', { expression, returnByValue: true, awaitPromise: true });
  assert.equal(result.exceptionDetails, undefined, JSON.stringify(result.exceptionDetails));
  return result.result.value;
};
const writer = await readFile(new URL('../src/hooks/use_announce/use_announce.js', import.meta.url), 'utf8');
const reader = await readFile(new URL('../src/hooks/use_announce_rendered_text/use_announce_rendered_text.js', import.meta.url), 'utf8');

async function contracts(source) {
  const check = (condition, name) => { if (!condition) throw new Error(name); };
  const frames = new Map(), timers = new Map();
  let serial = 0, readyState = 'complete';
  Object.defineProperty(document, 'readyState', { configurable: true, get: () => readyState });
  window.requestAnimationFrame = callback => { const id = ++serial; frames.set(id, callback); return id; };
  window.cancelAnimationFrame = id => frames.delete(id);
  window.setTimeout = (callback, ms) => { check(ms === 2000, 'only the channel clear timer exists'); const id = ++serial; timers.set(id, callback); return id; };
  window.clearTimeout = id => timers.delete(id);
  const frame = () => { const callbacks = [...frames.values()]; frames.clear(); callbacks.forEach(callback => callback(0)); };
  const clear = () => { const callbacks = [...timers.values()]; timers.clear(); callbacks.forEach(callback => callback()); };
  const bodyRegion = () => document.querySelector('body > [data-astryx-live-region="polite"]');
  const text = () => bodyRegion()?.textContent ?? '';
  const root = (id, html = '', parent = document.body) => {
    const element = document.createElement('div');
    element.id = id;
    element.style.cssText = 'min-width:20px;min-height:20px';
    element.innerHTML = html;
    parent.appendChild(element);
    return element;
  };
  const AsyncFunction = Object.getPrototypeOf(async function () {}).constructor;
  const mount = async (element, fallback = 'Loading', isActive = true) => {
    let resolve, revision = 0;
    const queue = [];
    const dioxus = { recv: () => queue.length ? Promise.resolve(queue.shift()) : new Promise(next => { resolve = next; }) };
    const task = new AsyncFunction('dioxus', source)(dioxus);
    const post = async value => {
      if (resolve) { const next = resolve; resolve = null; next(value); } else queue.push(value);
      await Promise.resolve(); await Promise.resolve();
    };
    const render = (name = fallback, active = isActive, sameRevision = false) => post([
      `#${element.id}`, active, name, sameRevision ? revision : ++revision,
    ]);
    await render();
    return { render, drop: async () => { await post('dispose'); await task; } };
  };
  const settle = () => { frame(); frame(); };
  const initial = root('initial');
  const a = await mount(initial, 'Starting');
  const persistent = bodyRegion();
  check(persistent && text() === '', 'region mounts empty');
  check(persistent.getAttribute('aria-live') === 'polite' && persistent.getAttribute('aria-atomic') === 'true', 'polite atomic channel');
  check(initial.getAttribute('aria-live') === null && initial.getAttribute('role') === null, 'owner has no live semantics');
  await a.render('Starting', true, true);
  check(frames.size === 1, 'mount/effect duplicate wire revision shares one read/write');
  settle();
  check(text() === 'Starting', 'post-load fallback');
  clear();
  await a.render();
  check(!frames.size && text() === '', 'same render text dedupes');
  await a.render('Saving');
  settle();
  check(text() === 'Saving' && bodyRegion() === persistent, 'fallback change and persistent region');
  clear();
  await a.render('Saving', false);
  await a.render('Saving', true);
  settle();
  check(text() === 'Saving', 'silent/composed/legacy-role flag rearms');
  clear();
  await a.drop();
  initial.remove();
  const exposed = root('exposed');
  const exposedOwner = await mount(exposed, 'Exposed status');
  settle(); await exposedOwner.drop();
  check(text() === 'Exposed status' && timers.size === 1, 'drop preserves exposed status and its shared clear');
  clear(); check(!text() && !timers.size, 'shared clear removes exposed status after 2000ms');
  exposed.remove();

  const rich = root('rich', '<span><strong>Fetch</strong>ing records<span aria-hidden="true"> decoration</span><span hidden> hidden</span></span>');
  const richOwner = await mount(rich, 'Please wait');
  settle();
  check(text() === 'Fetching records', 'nested rich text and hidden descendants');
  clear();
  rich.innerHTML = '<div style="display:flex;flex-direction:column"><span>Saving</span><span>records</span></div>';
  await richOwner.render('Please wait');
  settle();
  check(text() === 'Saving records', 'equal arguments still read changed rich text and flex blocks');
  clear();
  rich.innerHTML = '<span aria-label="not rendered">Visible</span><span>block</span> bare';
  await richOwner.render();
  settle();
  check(text() === 'Visible block bare', 'direct children and bare text are blocks; descendant aria-label ignored');
  clear();
  rich.innerHTML = '<span aria-hidden="true">Decoration</span>';
  rich.setAttribute('aria-labelledby', 'external-label');
  const label = root('external-label', 'Uploading <span hidden>skip</span> files');
  await richOwner.render(); settle();
  check(text() === 'Uploading files', 'aria-labelledby fallback');
  clear(); label.textContent = 'Saved files';
  await richOwner.render(); settle();
  check(text() === 'Saved files', 'external name update');
  clear(); await richOwner.drop(); rich.remove(); label.remove();

  readyState = 'loading';
  const page = root('page');
  const pageOwner = await mount(page, 'Loading page');
  check(!frames.size, 'page-load silence');
  readyState = 'complete';
  await pageOwner.render(); check(!frames.size, 'page-load remembered, not delayed announcement');
  await pageOwner.render('Page changed'); settle();
  check(text() === 'Page changed', 'later page change speaks');
  clear(); await pageOwner.drop(); page.remove();
  const hydrated = root('hydrated');
  hydrated.setAttribute('data-node-hydration', '0,mounted:0');
  const hydratedOwner = await mount(hydrated);
  check(!frames.size, 'late hydration silence');
  await hydratedOwner.render(); check(!frames.size, 'hydration remembered');
  await hydratedOwner.render('Hydrated change'); settle();
  check(text() === 'Hydrated change', 'hydration later change speaks');
  clear(); await hydratedOwner.drop(); hydrated.remove();

  for (const attribute of ['hidden', 'aria-hidden', 'style']) {
    const host = root(`host-${attribute}`);
    if (attribute === 'style') host.style.display = 'none';
    else host.setAttribute(attribute, attribute === 'hidden' ? '' : 'true');
    const child = root(`child-${attribute}`, '', host);
    const owner = await mount(child);
    check(!frames.size, `ancestor ${attribute} silence`);
    if (attribute === 'style') host.style.display = 'block'; else host.removeAttribute(attribute);
    await owner.render(); settle();
    check(text() === 'Loading', `mounted ${attribute} reveal on subsequent render`);
    clear(); await owner.drop(); host.remove();
  }
  for (const [attribute, value] of [['role', 'log'], ['role', 'status'], ['role', 'alert'], ['aria-live', 'polite'], ['aria-live', 'assertive']]) {
    const host = root('live-host'); host.setAttribute(attribute, value);
    const child = root('live-child', '', host);
    const owner = await mount(child);
    check(!frames.size, `live ancestor ${value} suppresses`);
    await owner.drop(); host.remove();
  }

  const one = root('one'), two = root('two'), three = root('three');
  const first = await mount(one, 'Joined');
  const duplicate = await mount(two, 'Joined');
  const third = await mount(three, 'Other');
  check(frames.size === 1, 'one shared write frame');
  await first.drop();
  frame(); check(text() === 'Joined Other', 'ordered joining and per-owner duplicate shares');
  await duplicate.drop(); check(text() === 'Other', 'unexposed share withdrawn after write, before settle');
  await third.drop(); check(text() === '' && !frames.size && !timers.size, 'last unexposed share cancels frames/timer');
  one.remove(); two.remove(); three.remove();
  const removed = root('removed');
  const removedOwner = await mount(removed, 'Never expose');
  removed.remove(); frame();
  check(text() === '' && !frames.size && !timers.size, 'disconnected queued owner is discarded');
  await removedOwner.drop();

  const dialog = document.createElement('dialog');
  const nested = document.createElement('dialog');
  document.body.append(dialog); dialog.append(nested);
  const inside = root('inside', '', dialog);
  const outside = root('outside');
  dialog.showModal();
  const insideOwner = await mount(inside, 'In modal');
  const outsideOwner = await mount(outside, 'Not in modal');
  const modalRegion = dialog.querySelector(':scope > [data-astryx-modal-live-region]');
  check(modalRegion && modalRegion.textContent === '' && !text(), 'native modal region initially empty, page silent');
  settle();
  check(modalRegion.textContent === 'In modal' && !text(), 'nearest native modal routing');
  clear();
  const nonModal = document.createElement('dialog'); dialog.append(nonModal); nonModal.show();
  const nonModalRoot = root('non-modal-root', '', nonModal);
  const nonModalOwner = await mount(nonModalRoot, 'In non-modal child'); settle();
  check(modalRegion.textContent === 'In non-modal child', 'non-modal dialog skips to enclosing native modal');
  clear(); await nonModalOwner.drop(); nonModal.close(); nonModal.remove();
  const nestedRoot = root('nested-root', '', nested);
  nested.showModal();
  await insideOwner.render('Covered modal');
  const nestedOwner = await mount(nestedRoot, 'Top modal');
  settle();
  check(!modalRegion.textContent && !text(), 'covered outer modal and page are silent');
  check(nested.querySelector('[data-astryx-modal-live-region]').textContent === 'Top modal', 'top nested modal speaks');
  clear(); await nestedOwner.drop(); nested.close();
  await insideOwner.render('Closes before write');
  dialog.close(); frame();
  check(!modalRegion.textContent && !text(), 'modal closing before write never leaks to page');
  await insideOwner.drop(); await outsideOwner.drop(); dialog.remove(); outside.remove();
  const raced = root('raced');
  const racedOwner = await mount(raced, 'Page before modal');
  const blocker = document.createElement('dialog'); document.body.append(blocker); blocker.showModal();
  frame(); check(!text(), 'modal opens before page write: page share discarded');
  await racedOwner.drop(); blocker.close(); blocker.remove(); raced.remove();
  check(!frames.size && !timers.size, 'no component frames, timers or pending shares remain');
  return 'announcement browser contracts passed';
}

try {
  await send('Runtime.enable');
  console.log(await evaluate(`(${contracts})(${JSON.stringify(`${writer}\n${reader}`)})`));
} finally {
  await send('Page.close').catch(() => {});
  socket.close();
}
