// Offline localhost network-control and source-derived native presentation proof.
import assert from 'node:assert/strict';
import {createServer} from 'node:http';
import {readFile} from 'node:fs/promises';
import {openTooltipPage} from './internal/tooltip_engine.mjs';
const files = ['src/private_attributes/layer_root.js','src/layer/layer_surface_press.js','src/utils/trigger_focus.js','src/layer/gesture_counter.js','src/layer/use_touch_trigger.js','src/text/truncation_tooltip.js','src/tooltip/use_tooltip.js','src/motion/theme_timing.js','src/hooks/use_press_feedback/press_feedback.js'];
const scripts = (await Promise.all(files.map(p => readFile(p,'utf8')))).join('\n');
const css = await readFile('assets/astryx.css','utf8');
const bodyText = 'Original tooltip body content with enough words to wrap across multiple lines.';
let mode = 'hold', held = [], requests = [];
const server = createServer(async (req,res) => {
  res.setHeader('Cache-Control','no-store');
  const path = req.url.split('?')[0];
  if (path === '/tooltip.js' || path === '/layer.js') {
    requests.push(req.url);
    const complete = async () => {
      res.setHeader('Content-Type','text/javascript');
      if (mode === 'fail') {res.writeHead(503); res.end('unavailable'); return;}
      res.end(await readFile(path === '/tooltip.js' ? 'src/tooltip/truncation_presentation.js' : 'src/layer/truncation_presentation.js'));
    };
    if (mode === 'hold') held.push(complete); else await complete();
    return;
  }
  res.setHeader('Content-Type','text/html');
  if (path === '/reference') { res.end(await readFile('tests/fixtures/truncation-tooltip-source-reference.html')); return; }
  res.end(`<!doctype html><style>${css}\nbody{margin:0}#e{position:fixed;left:240px;top:220px;width:80px;height:24px;overflow:hidden;white-space:nowrap;--font-family-body:Arial;--text-body-size:14px;--text-body-leading:1.4286}</style><span id="e">${bodyText}</span><button id="outside">Outside</button><script>
    const truncationPresentationUrls=['/tooltip.js','/layer.js'];${scripts}
    if (location.search.includes('stale-build')) document[Symbol.for('astryx.dioxus.truncation-presentation.v1')]={urls:'obsolete-build',loaded:{createTooltipPresentation(){throw Error('stale presentation reused')}},pending:null,attempt:0};
    installPressFeedback();window.runtime=document[runtimeKey];window.e=document.getElementById('e');
    window.attach=(cut=false)=>{installPressFeedback();runtime=document[runtimeKey];e[Symbol.for('astryx.dioxus.truncation')]={isTruncated:cut,fullText:e.textContent};window.handle=runtime.attach(e,{id:'probe',label:'',tooltip:null,isTruncation:true})};attach();
    window.cut=value=>{e[Symbol.for('astryx.dioxus.truncation')].isTruncated=value;e.dispatchEvent(new Event('astryxtruncationchange'))};
    window.pointer=(type)=>e.dispatchEvent(new PointerEvent(type,{bubbles:true,pointerType:'mouse',pointerId:1}));
    window.tip=()=>document.getElementById('probe-tooltip');window.open=()=>!!tip()?.matches(':popover-open');
    window.loaded=()=>!!document[Symbol.for('astryx.dioxus.truncation-presentation.v1')]?.loaded;
    window.installExplicitTooltip=()=>{
      const truncationPresentationUrls=undefined;
      ${scripts}
      createTooltipController({records:new Map(),listen(){},later(){},cancelTimer(){}});
    };
  </script>`);
});
await new Promise(resolve => server.listen(Number(process.env.TOOLTIP_PORT || 8099),'127.0.0.1',resolve));
const origin = `http://127.0.0.1:${server.address().port}`;
const page = await openTooltipPage();
const {send,evaluate,waitFor} = page;
const wait = ms => new Promise(resolve => setTimeout(resolve,ms));
const navigate = async path => { await send('Page.navigate',{url:origin+path}); await waitFor('typeof window.cut === "function"'); };
const release = async () => { mode='ok'; const tasks=held; held=[]; await Promise.all(tasks.map(fn=>fn())); };
const rect = selector => `(()=>{const b=${selector}.getBoundingClientRect();const c=getComputedStyle(${selector});return {x:b.x,y:b.y,w:b.width,h:b.height,font:c.fontFamily,size:c.fontSize,leading:c.lineHeight,weight:c.fontWeight,color:c.color,background:c.backgroundColor,radius:c.borderRadius}})()`;
try {
  await send('Network.enable');
  await send('Network.setBlockedURLs',{urls:['https://*','http://example.*']});
  await navigate('/'); await wait(100);
  assert.equal(requests.length,0,'not downloaded for nonoverflowing text');
  await evaluate('cut(true)');
  while (requests.length<2) await wait(10);
  assert.equal(await evaluate('tip()'),null,'overflow preloads code, not DOM');
  await evaluate('installExplicitTooltip()');
  await evaluate('pointer("pointerover")'); await wait(250);
  assert.equal(await evaluate('tip()'),null,'held actual network request prevents presentation');
  await release(); await waitFor('!!tip()');
  assert.equal(await evaluate('loaded()'),true,'an explicit tooltip mounted during preload must not replace the shared module entry');
  assert.equal(await evaluate('open()'),false,'pending hover starts its delay on module resolution');
  await evaluate('installExplicitTooltip()');
  assert.equal(await evaluate('loaded()'),true,'an explicit tooltip must not evict the loaded modules');
  await waitFor('open()'); await wait(300);
  const actual=await evaluate(rect('tip()'));
  await send('Page.navigate',{url:origin+'/reference'}); await waitFor('document.getElementById("tip")?.matches(":popover-open")');
  const original=await evaluate(rect('document.getElementById("tip")'));
  for (const key of ['x','y','w','h']) assert.ok(Math.abs(actual[key]-original[key])<0.6,`${key}: ${JSON.stringify({actual,original})}`);
  for (const key of ['font','size','leading','weight','color','background','radius']) assert.equal(actual[key],original[key],key);
  await evaluate('document.documentElement.style.colorScheme="dark"');
  const originalDark=await evaluate(rect('document.getElementById("tip")'));
  await navigate('/?dark'); await evaluate('document.documentElement.style.colorScheme="dark";cut(true);pointer("pointerover")'); await waitFor('open()'); await wait(300);
  const actualDark=await evaluate(rect('tip()'));
  for (const key of ['font','size','leading','weight','color','background','radius']) assert.equal(actualDark[key],originalDark[key],`dark ${key}`);
  const viewportWidth=await evaluate('innerWidth');
  for (const sample of [{left:240,top:1},{left:1,top:220},{left:viewportWidth-90,top:220}]) {
    await send('Page.navigate',{url:`${origin}/reference?left=${sample.left}&top=${sample.top}`});
    await waitFor('document.getElementById("tip")?.matches(":popover-open")');
    const expected=await evaluate(rect('document.getElementById("tip")'));
    await navigate('/?source-fallback');
    await evaluate(`e.style.left='${sample.left}px';e.style.top='${sample.top}px';cut(true);pointer('pointerover')`); await waitFor('open()'); await wait(300);
    const observed=await evaluate(rect('tip()'));
    for (const key of ['x','y','w','h']) assert.ok(Math.abs(observed[key]-expected[key])<0.6,`source fallback ${JSON.stringify(sample)} ${key}: ${JSON.stringify({observed,expected})}`);
  }
  // Content taller than both block rooms exercises beside/fit, not just flip.
  await send('Page.navigate',{url:origin+'/reference?repeat=100'});
  await waitFor('document.getElementById("tip")?.matches(":popover-open")');
  const tallReference = await evaluate(rect('document.getElementById("tip")'));
  await navigate('/?tall');
  await evaluate('handle.dispose();e.textContent=e.textContent.repeat(100);attach(true);pointer("pointerover")');
  await waitFor('open()'); await wait(300);
  const tallActual = await evaluate(rect('tip()'));
  for (const key of ['x','y','w','h']) assert.ok(Math.abs(tallActual[key]-tallReference[key])<0.6,`tall source ${key}: ${JSON.stringify({tallActual,tallReference})}`);
  assert.equal(await evaluate('(()=>{const a=e.getBoundingClientRect(),b=tip().getBoundingClientRect();return (b.right<=a.left || b.left>=a.right || b.bottom<=a.top || b.top>=a.bottom) && b.top>=0 && b.bottom<=innerHeight && tip().scrollHeight>tip().clientHeight})()'),true,'tall content stays clear and scrolls within viewport');
  mode='ok'; requests=[]; await navigate('/?stale-build'); await evaluate('cut(true);pointer("pointerover")'); await waitFor('open()');
  assert.equal(requests.length,2,'stale cached code from different asset URLs is not reused');
  // Leave/Escape while loading retains first-interest DOM, but never opens stale hover.
  mode='hold'; requests=[]; await navigate('/?dismissed'); await evaluate('cut(true);pointer("pointerover");pointer("pointerout")');
  while (requests.length<2) await wait(10);
  await release(); await waitFor('loaded() && !!tip()'); await wait(250);
  assert.equal(await evaluate('open()'),false);
  mode='hold'; requests=[]; await navigate('/?escape'); await evaluate('cut(true);pointer("pointerover");document.dispatchEvent(new KeyboardEvent("keydown",{key:"Escape",bubbles:true}))');
  while (requests.length<2) await wait(10);
  await release(); await waitFor('loaded() && !!tip()'); await wait(250); assert.equal(await evaluate('open()'),false,'Escape cancels unresolved interest');
  mode='hold'; requests=[]; await navigate('/?focus');
  await send('Input.dispatchKeyEvent',{type:'keyDown',key:'Tab',code:'Tab'});
  await evaluate('cut(true);e.tabIndex=0;e.focus()');
  while (requests.length<2) await wait(10);
  await release(); await waitFor('open()');
  assert.equal(await evaluate('document.activeElement === e'),true,'held focus opens after resolution without replacing trigger');
  assert.equal(await evaluate('e.getAttribute("aria-describedby")'),'probe-tooltip','loaded focus attaches description');
  mode='hold'; requests=[]; await navigate('/?blur-during-load');
  await send('Input.dispatchKeyEvent',{type:'keyDown',key:'Tab',code:'Tab'});
  await evaluate('cut(true);e.tabIndex=0;e.focus();document.getElementById("outside").focus()');
  while (requests.length<2) await wait(10);
  await release(); await waitFor('loaded() && !!tip()'); await wait(250);
  assert.equal(await evaluate('open()'),false,'blur invalidates unresolved focus');
  assert.equal(await evaluate('e.getAttribute("aria-describedby")'),'probe-tooltip','blur retains first-interest description');
  mode='hold'; requests=[]; await navigate('/?tap');
  await evaluate('cut(true);e.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,isPrimary:true,pointerType:"touch",pointerId:7,button:0}));e.dispatchEvent(new PointerEvent("pointerup",{bubbles:true,isPrimary:true,pointerType:"touch",pointerId:7,button:0}));e.click()');
  while (requests.length<2) await wait(10);
  await release(); await waitFor('open()');
  mode='hold'; requests=[]; await navigate('/?tap-dismissed');
  await evaluate('cut(true);e.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,isPrimary:true,pointerType:"touch",pointerId:7,button:0}));e.dispatchEvent(new PointerEvent("pointerup",{bubbles:true,isPrimary:true,pointerType:"touch",pointerId:7,button:0}));e.click();document.getElementById("outside").dispatchEvent(new PointerEvent("pointerup",{bubbles:true,pointerType:"touch"}))');
  while (requests.length<2) await wait(10);
  await release(); await waitFor('loaded() && !!tip()'); await wait(250); assert.equal(await evaluate('open()'),false,'outside release cancels pending completed tap');
  // Removal and overflow loss invalidate all pending code resolutions.
  mode='hold'; requests=[]; await navigate('/?removed'); await evaluate('cut(true);pointer("pointerover");handle.dispose();e.remove()');
  while (requests.length<2) await wait(10);
  await release(); await wait(300); assert.equal(await evaluate('tip()'),null);
  mode='hold'; requests=[]; await navigate('/?uncut'); await evaluate('cut(true);pointer("pointerover");cut(false)');
  while (requests.length<2) await wait(10);
  await release(); await wait(300); assert.equal(await evaluate('tip()'),null);
  mode='hold'; requests=[]; await navigate('/?replacement'); await evaluate('cut(true);pointer("pointerover");handle.dispose();attach(true)');
  while (requests.length<2) await wait(10);
  await release(); await waitFor('loaded() && !!tip()'); await wait(250);
  assert.equal(await evaluate('open()'),false,'stale old record cannot replay hover onto a reattached same root');
  // Failed real import does not attach anything. A later interest retries a fresh
  // URL because browsers cache failed module fetches for the old URL.
  mode='fail'; requests=[]; await navigate('/?failure'); await evaluate('cut(true)');
  await waitFor('document[Symbol.for("astryx.dioxus.truncation-presentation.v1")]?.attempt === 1 && !document[Symbol.for("astryx.dioxus.truncation-presentation.v1")]?.pending');
  assert.equal(await evaluate('tip()'),null); mode='ok'; await evaluate('pointer("pointerover")'); await waitFor('open()');
  assert.ok(requests.some(url=>url.includes('recovery=')),'recovery makes a real fresh module request');
  await wait(300);
  await evaluate('document.body.style.cssText="font-style:italic;text-transform:uppercase;letter-spacing:3px;font-weight:700"');
  assert.deepEqual(await evaluate('(()=>{const c=getComputedStyle(tip());return [c.fontStyle,c.textTransform,c.letterSpacing,c.fontWeight]})()'),['normal','none','normal','400'],'Layer text reset isolates ancestor formatting');
  await evaluate('document.body.style.cssText=""');
  assert.equal(await evaluate('getComputedStyle(tip()).positionAnchor === getComputedStyle(e).anchorName'),true);
  // Browser layout responds directly to resize, scroll, RTL and native modal hosts.
  for (const direction of ['ltr','rtl']) {
    await evaluate(`e.style.direction='${direction}';e.setAttribute('data-astryx-truncation-placement','end');handle.dispose();attach(true);pointer('pointerover')`);
    await waitFor('open()'); await wait(300);
    assert.equal(await evaluate(`(()=>{const a=e.getBoundingClientRect(),b=tip().getBoundingClientRect();return ${direction === 'ltr' ? 'b.left >= a.right' : 'b.right <= a.left'}})()`),true,direction);
  }
  await evaluate('handle.dispose();e.style.direction="ltr";e.setAttribute("data-astryx-truncation-placement","above");e.style.top="1px";attach(true);pointer("pointerover")');
  await waitFor('open()'); await wait(300);
  assert.equal(await evaluate('tip().getBoundingClientRect().top >= e.getBoundingClientRect().bottom'),true,'native flip below near viewport edge');
  await send('Emulation.setDeviceMetricsOverride',{width:360,height:400,deviceScaleFactor:1,mobile:false}); await wait(100);
  assert.equal(await evaluate('tip().getBoundingClientRect().right <= innerWidth - 15'),true,'native viewport gutter on resize');
  await evaluate('handle.dispose();window.panel=document.createElement("div");panel.style="position:absolute;top:100px;left:40px;width:200px;height:100px;overflow:auto";panel.innerHTML="<div style=height:600px></div>";document.body.append(panel);e.style="display:block;position:relative;top:40px;width:80px;height:24px;overflow:hidden;white-space:nowrap";panel.firstChild.prepend(e);attach(true);pointer("pointerover")');
  await waitFor('open()'); await wait(300);
  const before=await evaluate('tip().getBoundingClientRect().y'); await evaluate('panel.scrollTop=20'); await wait(100);
  assert.ok(Math.abs((await evaluate('tip().getBoundingClientRect().y'))-before+20)<1,'native anchor tracks nested scrolling');
  await evaluate('handle.dispose();window.dialog=document.createElement("dialog");document.body.append(dialog);dialog.append(e);dialog.showModal();attach(true);pointer("pointerover")'); await waitFor('open()');
  assert.equal(await evaluate('tip().parentElement === dialog'),true,'modal-local portal');
  await send('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:'reduce'},{name:'pointer',value:'fine'}]});
  assert.equal(await evaluate('getComputedStyle(tip()).animationName'),'none','fine-pointer reduced motion appears immediately');
  if (!process.env.TOOLTIP_ENGINE) {
    await send('Emulation.setTouchEmulationEnabled',{enabled:true});
    await send('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:'reduce'}]});
    assert.equal(await evaluate('matchMedia("(pointer: coarse)").matches'),true);
    assert.equal(await evaluate('getComputedStyle(tip()).animationName'),'astryx-truncation-fade');
    assert.equal(await evaluate('getComputedStyle(tip()).animationDuration'),'0.2s');
    await send('Emulation.setTouchEmulationEnabled',{enabled:false});
  } else console.log('UNRUN: coarse-pointer media emulation (CDP-only); synthetic completed taps above are not device evidence.');
  await send('Emulation.setEmulatedMedia',{features:[]});
  await evaluate('document.dispatchEvent(new KeyboardEvent("keydown",{key:"Escape",bubbles:true}))');
  assert.equal(await evaluate('!!tip() && !open()'),true,'source default exit is immediate with retained DOM');
  assert.equal(await evaluate('e.style.getPropertyValue("anchor-name")'),'','default close releases anchor');
  // Hidden WAAPI content may keep running without a display exit. It must
  // not keep the trigger registered as an anchor (source closed-style gate).
  await evaluate('pointer("pointerover")'); await waitFor('open()'); await wait(300);
  await evaluate('window.unrelated=tip().firstElementChild.animate([{opacity:1},{opacity:0.5}],{duration:2000});document.dispatchEvent(new KeyboardEvent("keydown",{key:"Escape",bubbles:true}))');
  assert.equal(await evaluate('unrelated.playState'),'running','unrelated finite animation really survives hide');
  assert.equal(await evaluate('e.style.getPropertyValue("anchor-name")'),'','non-display animation cannot retain hidden anchor');
  await evaluate('unrelated.cancel()');
  // Calibrate an actual native popover, not CSS.supports(): Firefox/WebKit
  // can parse allow-discrete without keeping a hidden popover painting.
  await evaluate('window.style=document.createElement("style");style.textContent=".astryx-truncation-tooltip{transition:display 600ms allow-discrete,overlay 600ms allow-discrete,opacity 600ms}.astryx-truncation-tooltip:not(:popover-open){opacity:0}";document.head.append(style);window.nativeExit=tip().cloneNode(true);nativeExit.id="native-exit";dialog.append(nativeExit);nativeExit.showPopover()');
  await wait(300);
  const nativeExitPaints = await evaluate('nativeExit.hidePopover();getComputedStyle(nativeExit).display !== "none" && nativeExit.getAnimations().some(a=>a.playState === "running")');
  await evaluate('nativeExit.remove();pointer("pointerover")');
  await waitFor('open()'); await wait(300);
  await evaluate('document.dispatchEvent(new KeyboardEvent("keydown",{key:"Escape",bubbles:true}))');
  assert.equal(await evaluate('!!e.style.getPropertyValue("anchor-name")'),nativeExitPaints,'retain anchor exactly when native discrete exit paints');
  console.log(`Native discrete popover exit paints: ${nativeExitPaints} (false means immediate native hide, not an animated-exit pass)`);
  if (nativeExitPaints) {
    await evaluate('pointer("pointerover")'); await waitFor('open()');
    await wait(750);
    assert.equal(await evaluate('!!e.style.getPropertyValue("anchor-name")'),true,'old exit completion cannot detach reopened anchor');
    await evaluate('document.dispatchEvent(new KeyboardEvent("keydown",{key:"Escape",bubbles:true}))');
  }
  await wait(750); assert.equal(await evaluate('e.style.getPropertyValue("anchor-name")'),'','exit completion releases anchor');
  assert.deepEqual(page.errors,[],'no unhandled asynchronous module or presentation exceptions');
  console.log('Tooltip separate-module network hold/failure/recovery/stale resolution, source geometry/typography, native layout/modal and reduced/exit: PASS');
} finally { await page.close(); await new Promise(resolve=>server.close(resolve)); }
