// Node 22+, offline isolated Chrome target. Serve text_patterns on 8085.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {openTextPage} from './internal/text_cdp.mjs';
const page = await openTextPage();
const {send, evaluate, waitFor} = page;
const wait = ms => new Promise(resolve => setTimeout(resolve, ms));
const fixture = await readFile(new URL('./fixtures/text-source-reference.html', import.meta.url), 'utf8');
const font = await readFile(new URL('../assets/fonts/Figtree-latin.woff2', import.meta.url));
const fontStyle = `@font-face{font-family:Figtree;src:url(data:font/woff2;base64,${font.toString('base64')}) format('woff2');font-weight:300 900}`;
const measure = doc => [...doc.querySelectorAll('[data-text-pattern]')].map(e => {
  const group = e.closest('[id]'), r = e.getBoundingClientRect(), parent = group.getBoundingClientRect(), s = doc.defaultView.getComputedStyle(e);
  const range = doc.createRange(); range.selectNodeContents(e);
  return {pattern:e.dataset.textPattern, text:e.textContent, box:[r.x-parent.x,r.y-parent.y,r.width,r.height], style:Object.fromEntries(['fontFamily','fontSize','fontWeight','lineHeight','color','display','textWrap','whiteSpace','wordBreak','overflowWrap','textDecorationLine','fontVariantNumeric','webkitLineClamp'].map(p=>[p,s[p]])), lines:[...range.getClientRects()].map(r=>[r.x-parent.x,r.y-parent.y,r.width,r.height])};
});
function compare(actual, expected, path='root') {
  if (typeof expected === 'number') { assert.ok(Math.abs(actual-expected)<=1/64+1e-6,`${path}: ${actual} != ${expected}`); return; }
  if (expected && typeof expected === 'object') { assert.deepEqual(Object.keys(actual),Object.keys(expected),`${path} keys`); for (const key of Object.keys(expected)) compare(actual[key],expected[key],`${path}/${key}`); return; }
  assert.equal(actual, expected, path);
}
const click = async id => { await evaluate(`document.getElementById(${JSON.stringify(id)}).click()`); await wait(60); };
const hover = async id => {
  await send('Input.dispatchMouseEvent',{type:'mouseMoved',x:1190,y:10});
  const box = await evaluate(`(()=>{const e=document.getElementById(${JSON.stringify(id)});e.scrollIntoView({block:'center'});const r=e.getBoundingClientRect();return {x:r.x+r.width/2,y:r.y+r.height/2}})()`);
  await send('Input.dispatchMouseEvent',{type:'mouseMoved',...box});
};
try {
  await send('Page.bringToFront');
  await send('Emulation.setDeviceMetricsOverride',{width:1200,height:1000,deviceScaleFactor:1,mobile:false});
  await send('Page.navigate',{url:process.env.TEXT_URL ?? 'http://127.0.0.1:8085'});
  await waitFor('document.querySelectorAll("[data-text-pattern]").length === 31 && document.getElementById("tx-events")?.textContent === "1"');
  await evaluate('document.fonts.ready.then(()=>true)');
  await waitFor('document.getElementById("tx-hook-build").nextElementSibling?.dataset.hookState === "true"');
  // Reference fixture is reconstructed directly from the authority, not a
  // published-catalog capture or independently executed React/StyleX build.
  await evaluate(`(()=>{const iframe=document.createElement('iframe');iframe.id='tx-reference';iframe.style.cssText='position:absolute;left:-10000px;top:0;border:0;width:'+(document.getElementById('text-examples').clientWidth-48)+'px;height:5000px';iframe.srcdoc=${JSON.stringify(fixture.replace('</head>', `<style>${fontStyle}</style></head>`))};document.body.append(iframe);return true})()`);
  await waitFor('document.getElementById("tx-reference").contentDocument.querySelectorAll("[data-text-pattern]").length===31');
  await evaluate('document.getElementById("tx-reference").contentDocument.fonts.ready.then(()=>true)');
  for (const mode of ['light','dark']) {
    if (mode === 'dark') await click('tx-theme');
    await evaluate(`document.getElementById('tx-reference').contentDocument.documentElement.style.colorScheme=${JSON.stringify(mode)}`);
    const actual = await evaluate(`(${measure.toString()})(document)`);
    const reference = await evaluate(`(${measure.toString()})(document.getElementById('tx-reference').contentDocument)`);
    compare(actual, reference, mode);
  }
  await click('tx-theme');
  assert.deepEqual(await evaluate('["tx-hook-build","tx-hook-bundle","tx-hook-readme"].map(id=>document.getElementById(id).nextElementSibling.dataset.hookState)'), ['true','true','false']);
  assert.equal(await evaluate('document.querySelectorAll("#text-examples [title],#text-examples [role=tooltip],#text-examples .astryx-text[tabindex],#text-examples .astryx-text[aria-describedby]").length'),0,'no eager layers/title/tabstop/duplicate descriptions');
  for (const direction of ['ltr','rtl']) {
    await evaluate(`document.getElementById('text-examples').dir=${JSON.stringify(direction)}`);
    for (const placement of ['above','below','start','end']) {
      await hover('tx-'+placement);
      await waitFor('document.querySelector("[role=tooltip]:popover-open")');
      const geometry = await evaluate(`(()=>{const e=document.getElementById('tx-${placement}'),t=document.querySelector('[role=tooltip]:popover-open'),a=e.getBoundingClientRect(),b=t.getBoundingClientRect();return {above:b.bottom<=a.top,below:b.top>=a.bottom,left:b.right<=a.left,right:b.left>=a.right,text:t.textContent,description:e.getAttribute('aria-describedby')}})()`);
      const side = placement === 'start' ? (direction==='rtl'?'right':'left') : placement === 'end' ? (direction==='rtl'?'left':'right') : placement;
      assert.equal(geometry[side],true,`${direction}/${placement}: ${JSON.stringify(geometry)}`);
      assert.ok(geometry.text.startsWith('Placement '+placement)); assert.ok(geometry.description?.endsWith('-tooltip'));
      await send('Input.dispatchMouseEvent',{type:'mouseMoved',x:1190,y:10});
      await send('Input.dispatchKeyEvent',{type:'keyDown',key:'Escape',code:'Escape'});
      await waitFor('!document.querySelector("[role=tooltip]:popover-open")');
    }
  }
  await evaluate('document.getElementById("text-examples").removeAttribute("dir")');
  await hover('tx-dynamic'); await waitFor('document.querySelector("[role=tooltip]:popover-open")');
  await click('tx-content');
  await waitFor('document.querySelector("[role=tooltip]:popover-open")?.textContent.startsWith("Changed content")');
  await click('tx-toggle-tooltip');
  await waitFor('!document.querySelector("[role=tooltip]:popover-open")');
  assert.equal(await evaluate('document[Symbol.for("astryx.dioxus.sharedResizeObserver")].callbacks.has(document.getElementById("tx-dynamic"))'),false);
  await click('tx-toggle-tooltip'); await hover('tx-dynamic'); await waitFor('document.querySelector("[role=tooltip]:popover-open")');
  await evaluate('window.dropped=document.getElementById("tx-dynamic")'); await click('tx-drop');
  await waitFor('!document.querySelector("[role=tooltip]:popover-open")');
  assert.equal(await evaluate('document[Symbol.for("astryx.dioxus.sharedResizeObserver")].callbacks.has(dropped)'),false);
  await click('tx-drop');
  await evaluate('window.stable=document.getElementById("tx-dynamic");window.keptField=document.getElementById("tx-kept-field");keptField.value="Typed value";keptField.focus()');
  await click('tx-lines');
  assert.equal(await evaluate('stable===document.getElementById("tx-dynamic")'),true,'clamp updates preserve root');
  assert.equal(await evaluate('keptField===document.getElementById("tx-kept-field") && keptField.value==="Typed value" && document.activeElement===keptField'),true,'clamp updates preserve native input, focus and uncontrolled value');
  assert.deepEqual(await evaluate('(()=>{const s=getComputedStyle(document.getElementById("tx-override"));return [s.fontSize,s.fontWeight,s.lineHeight,s.fontFamily]})()'),['20px','700','28.572px','"SF Mono", Monaco, Consolas, monospace']);
  assert.equal(await evaluate('getComputedStyle(document.getElementById("tx-capsize")).display'),'block');
  assert.equal(await evaluate('getComputedStyle(document.getElementById("tx-align")).textAlign'),'end');
  assert.equal(await evaluate('getComputedStyle(document.getElementById("tx-rtl")).textAlign'),'end');
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#tx-nested a")).display'),'inline');
  const beforeNative = await evaluate('+document.getElementById("tx-events").textContent');
  await click('tx-native'); assert.equal(await evaluate('+document.getElementById("tx-events").textContent'),beforeNative+1);
  const beforeEvents = await evaluate('+document.getElementById("tx-events").textContent');
  await evaluate(`(()=>{const e=document.getElementById('tx-native');e.dispatchEvent(new WheelEvent('wheel',{bubbles:true}));e.dispatchEvent(new ClipboardEvent('copy',{bubbles:true}));e.dispatchEvent(new AnimationEvent('animationstart',{bubbles:true}));e.dispatchEvent(new TransitionEvent('transitionend',{bubbles:true}));e.dispatchEvent(new DragEvent('dragstart',{bubbles:true}));return true})()`);
  await waitFor(`+document.getElementById('tx-events').textContent===${beforeEvents+111110}`);
  assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("tx-native");return [e.tagName,e.htmlFor,e.lang,e.dataset.testid,e.style.marginInline,e.classList.contains("caller-text")]})()'),['LABEL','tx-field','en','native-text','7px',true]);
  await evaluate('document.getElementById("tx-focus-host").scrollIntoView({block:"center"});document.getElementById("tx-focus-host").focus()');
  await wait(250);
  assert.equal(await evaluate('!!document.querySelector("[role=tooltip]:popover-open")'),false,'focus on an enclosing link is not focus inside Text');
  // Synthetic touch/pen sequences exercise shared native capture ownership;
  // this is not a claim of real-device gesture verification.
  await evaluate(`window.touchSequence=(target,mode='tap',pointerType='touch')=>{const e=document.getElementById(target),opts={bubbles:true,isPrimary:true,pointerId:17,pointerType,button:0,clientX:20,clientY:20};e.dispatchEvent(new PointerEvent('pointerdown',opts));if(mode==='move')e.dispatchEvent(new PointerEvent('pointermove',{...opts,clientY:50}));if(mode==='scroll')document.dispatchEvent(new Event('scroll'));if(mode==='focus')e.focus();e.dispatchEvent(new PointerEvent('pointerup',opts));e.dispatchEvent(new MouseEvent('click',{bubbles:true}));}`);
  for (const mode of ['move','scroll']) {
    await evaluate(`touchSequence('tx-touch',${JSON.stringify(mode)})`); await wait(250);
    assert.equal(await evaluate('!!document.querySelector("[role=tooltip]:popover-open")'),false,mode+' is not a tap');
  }
  await evaluate("touchSequence('tx-touch-field','focus')"); await wait(250);
  assert.equal(await evaluate('!!document.querySelector("[role=tooltip]:popover-open")'),false,'entry tap only focuses');
  for (const pointerType of ['touch','pen']) {
    await evaluate(`touchSequence('tx-touch-child','focus',${JSON.stringify(pointerType)})`);
    await waitFor('document.querySelector("[role=tooltip]:popover-open")');
    await send('Input.dispatchKeyEvent',{type:'keyDown',key:'Escape',code:'Escape'});
    await waitFor('!document.querySelector("[role=tooltip]:popover-open")');
  }
  // A standalone Text remains unfocusable: the authority documents this gap.
  await send('Accessibility.enable'); const ax = await send('Accessibility.getFullAXTree');
  assert.ok(ax.nodes.some(n=>n.name?.value?.includes('Keyboard focus can reach this clipped text')));
  await send('Emulation.setDeviceMetricsOverride',{width:320,height:714,deviceScaleFactor:1,mobile:false});
  await evaluate('document.getElementById("tx-reference").remove();document.getElementById("tx-unbroken").scrollIntoView()');
  assert.equal(await evaluate('(()=>{const e=document.getElementById("tx-unbroken");return e.scrollWidth<=e.clientWidth})()'),true,'unbroken ordinary flow rewraps at 320px');
  await evaluate('document.documentElement.style.fontSize="32px"');
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#tx-showcase .astryx-text")).fontSize'),'28px');
  assert.deepEqual(page.errors, []);
  console.log('Text browser checks passed: 31 original-pattern source-derived offline fixtures/light-dark paint and line geometry; logical placements; reactive content/clamp/tooltips/drop; native attributes/events/ref; nested clamp, leading/size override, AX/no tabstop and narrow/root-text reflow.');
} finally { await page.close(); }
