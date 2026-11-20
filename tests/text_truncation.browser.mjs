// Native shared-owner tests, offline; no server/npm packages required.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {openTextPage} from './internal/text_cdp.mjs';
const page = await openTextPage();
const {evaluate, waitFor} = page;
const scripts = await Promise.all(['src/utils/shared_resize_observer.js', 'src/utils/shared_mutation_observer.js', 'src/text/use_truncation.js'].map(path => readFile(path, 'utf8')));
try {
  await evaluate(`(() => {
    ${scripts.join('\n')}
    window.createTruncation = createTruncation; window.observeResize = observeResize;
    window.make = (id, text, lines = 1) => {
      const e = document.createElement('div'); e.id = id; e.textContent = text;
      e.style.cssText = 'font:14px/20px Arial;width:120px;overflow:hidden;';
      if (lines === 1) e.style.cssText += 'white-space:nowrap;';
      else e.style.cssText += 'display:-webkit-box;-webkit-box-orient:vertical;-webkit-line-clamp:'+lines+';word-break:break-all;';
      document.body.append(e); return e;
    };
    window.answers = {}; window.controllers = {};
    window.attach = (id, text, lines = 1) => {
      const e = make(id, text, lines);
      controllers[id] = createTruncation((cut, fullText) => answers[id] = [cut, fullText]);
      controllers[id].attach(e, lines, false); return e;
    };
    attach('single', 'a very long text that cannot fit into a one line status label');
    attach('multi', 'abcdefghijklmnopqrstuvwxyz'.repeat(8), 2);
    attach('short', 'Short');
    return true;
  })()`);
  await waitFor('answers.single?.[0] && answers.multi?.[0] && answers.short?.[1] === "Short"');
  assert.deepEqual(await evaluate('Object.keys(document[Symbol.for("astryx.dioxus.sharedResizeObserver")]).sort()'), ['callbacks','observer']);
  assert.equal(await evaluate('document[Symbol.for("astryx.dioxus.sharedResizeObserver")].callbacks.size'), 3);
  assert.equal(await evaluate('document[Symbol.for("astryx.dioxus.sharedContentObserver")].callbacks.size'), 4, 'three content roots and one shared removal root');
  // Same-size content mutation, then actual resize, both update the same answer.
  await evaluate('document.getElementById("single").textContent = "New text in fixed box still cut"');
  await waitFor('answers.single?.[1] === "New text in fixed box still cut"');
  await evaluate('document.getElementById("single").style.width = "900px"');
  await waitFor('answers.single?.[0] === false');
  // Range is authoritative even if scrollHeight lies about clamped content.
  await evaluate('Object.defineProperty(document.getElementById("multi"), "scrollHeight", {get:()=>40}); document.getElementById("multi").firstChild.textContent += " updated"');
  await waitFor('answers.multi?.[1].endsWith(" updated") && answers.multi[0]');
  // Expanded content keeps the last clamped answer and makes no subscriptions.
  await evaluate('controllers.multi.attach(document.getElementById("multi"), 2, true); document.getElementById("multi").style.webkitLineClamp="unset"; document.getElementById("multi").textContent="expanded content"');
  assert.equal(await evaluate('answers.multi[0] && !answers.multi[1].includes("expanded content")'), true);
  assert.equal(await evaluate('document[Symbol.for("astryx.dioxus.sharedResizeObserver")].callbacks.size'), 2);
  await evaluate('controllers.multi.attach(null,2,true);document.getElementById("multi").remove();window.replacement=make("multi","Other text",2);controllers.multi.attach(replacement,2,true);controllers.multi.attach(replacement,2,false)');
  await waitFor('answers.multi[1] === "Other text" && !answers.multi[0]');
  await evaluate('controllers.single.attach(document.getElementById("single"),0,false)');
  assert.deepEqual(await evaluate('answers.single'), [false,'']);
  await evaluate('document.getElementById("short").remove()');
  await waitFor('answers.short[0] === false && answers.short[1] === ""');
  await evaluate('Object.values(controllers).forEach(c=>c.dispose())');
  assert.equal(await evaluate('!document[Symbol.for("astryx.dioxus.sharedResizeObserver")] && !document[Symbol.for("astryx.dioxus.sharedContentObserver")]'), true);
  // Deterministic observer scheduling: no forced initial read; preobserved
  // elements get a guarded microtask; unsubscribing leaves peer callbacks alive.
  await evaluate(`(() => {
    window.NativeResizeObserver = ResizeObserver;
    window.notifications = null; window.reads = 0;
    window.ResizeObserver = class { constructor(cb) { notifications=cb; } observe(){} unobserve(){} disconnect(){} };
    window.probe = make('probe','test');
    Object.defineProperty(probe,'scrollWidth',{get(){reads++;return 300}});
    window.scheduled = createTruncation((cut,text)=>answers.probe=[cut,text]);
    scheduled.attach(probe,1,false); return true;
  })()`);
  assert.equal(await evaluate('reads'), 0, 'new root waits for ResizeObserver first notification');
  await evaluate('notifications([{target:probe}])');
  assert.equal(await evaluate('reads'), 1);
  await evaluate('scheduled.dispose();window.releasePeer=observeResize(probe,()=>{});scheduled.attach(probe,1,false);scheduled.attach(probe,0,false)');
  await evaluate('Promise.resolve(true)');
  assert.deepEqual(await evaluate('answers.probe'), [false,''], 'same-task disable cancels pending read');
  await evaluate('scheduled.attach(probe,1,false)');
  await waitFor('answers.probe[0]');
  await evaluate('scheduled.dispose()');
  assert.equal(await evaluate('document[Symbol.for("astryx.dioxus.sharedResizeObserver")].callbacks.get(probe).size'), 1);
  await evaluate('releasePeer();window.ResizeObserver=NativeResizeObserver');
  // No-ResizeObserver fallback measures synchronously and still tracks content.
  await evaluate('window.ResizeObserver=undefined;window.fallback=createTruncation((cut,text)=>answers.fallback=[cut,text]);fallback.attach(probe,1,false)');
  assert.equal(await evaluate('answers.fallback[0]'), true);
  await evaluate('fallback.dispose();window.ResizeObserver=NativeResizeObserver');
  assert.deepEqual(page.errors, []);
  console.log('Text truncation owner checks passed: shared resize/content/removal, Range, scheduling, expanded/replacement, disabled, peer cleanup and no-observer fallback.');
} finally { await page.close(); }
