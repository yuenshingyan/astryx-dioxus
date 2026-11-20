// Node 22+, no npm packages. Run through the shared browser_lock.py.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {referenceCss} from './aspect_ratio_source.mjs';
const css=await readFile(new URL('../assets/astryx.css',import.meta.url),'utf8');
const endpoint=process.env.CDP_URL??'http://127.0.0.1:9222';
const target=await(await fetch(`${endpoint}/json/new?about:blank`,{method:'PUT'})).json();
const socket=new WebSocket(target.webSocketDebuggerUrl);
await new Promise((resolve,reject)=>{socket.onopen=resolve;socket.onerror=reject});
let sequence=0;const pending=new Map();
socket.onmessage=({data})=>{const m=JSON.parse(data),p=pending.get(m.id);if(p){pending.delete(m.id);clearTimeout(p.timer);m.error?p.reject(Error(JSON.stringify(m.error))):p.resolve(m.result)}};
const send=(method,params={})=>new Promise((resolve,reject)=>{const id=++sequence,timer=setTimeout(()=>reject(Error(`CDP timeout ${method}`)),20000);pending.set(id,{resolve,reject,timer});socket.send(JSON.stringify({id,method,params}))});
const evaluate=async expression=>{const r=await send('Runtime.evaluate',{expression,returnByValue:true,awaitPromise:true});assert.equal(r.exceptionDetails,undefined,JSON.stringify(r.exceptionDetails));return r.result.value};
const wait=ms=>new Promise(resolve=>setTimeout(resolve,ms));
async function until(expression){for(let i=0;i<400;i++){if(await evaluate(expression))return;await wait(50)}assert.fail(expression)}
const box=id=>evaluate(`(()=>{const r=document.getElementById(${JSON.stringify(id)}).getBoundingClientRect();return [r.width,r.height]})()`);
const close=(actual,expected)=>actual.forEach((n,i)=>assert.ok(Math.abs(n-expected[i])<=1/64,`${actual} != ${expected}`));
try {
  await send('Page.enable');await send('Runtime.enable');await send('Network.enable');await send('Accessibility.enable');
  await send('Emulation.setDeviceMetricsOverride',{width:1200,height:900,deviceScaleFactor:1,mobile:false});
  await send('Page.navigate',{url:process.env.ASPECT_RATIO_URL??'http://127.0.0.1:8092'});
  await until('document.getElementById("ar-events")?.textContent==="1" && [...document.images].every(e=>e.complete&&e.naturalWidth>0)');
  // All subsequent fixtures and interactions run with the target offline.
  await send('Network.emulateNetworkConditions',{offline:true,latency:0,downloadThroughput:0,uploadThroughput:0});
  assert.equal(await evaluate('[...document.images].every(e=>e.src.startsWith("data:"))'),true);
  close(await box('ar-wide'),[600,337.5]);close(await box('ar-square'),[300,300]);close(await box('ar-circle'),[300,300]);close(await box('ar-skeleton'),[600,337.5]);close(await box('ar-oval'),[320,160]);
  const equal=await evaluate('[...document.querySelectorAll("#ar-equal .astryx-aspect-ratio")].map(e=>{const r=e.getBoundingClientRect();return [r.width,r.height]})');
  [[120,120],[160,120],[120*16/9,120]].forEach((expected,i)=>close(equal[i],expected));
  assert.equal(await evaluate('document.querySelectorAll("#ar-gallery > .astryx-aspect-ratio").length'),6);
  close(await evaluate('(()=>{const r=document.querySelector("#ar-gallery > div").getBoundingClientRect();return [r.width,r.height]})()'),[(600-32)/3,(600-32)/4]);
  assert.equal(await evaluate('getComputedStyle(document.getElementById("ar-circle")).borderRadius'),'50%');
  for(const fit of ['cover','contain','center','unset']) {
    const result=await evaluate(`(()=>{const e=document.querySelector('#ar-${fit} img'),r=e.getBoundingClientRect(),p=e.parentElement.getBoundingClientRect();return [r.width,r.height,r.x-p.x,r.y-p.y,getComputedStyle(e).objectFit]})()`);
    if(fit==='center')assert.deepEqual(result,[100,100,110,40,'fill']);
    else if(fit==='unset')assert.deepEqual(result,[100,100,0,0,'fill']);
    else assert.deepEqual(result,[320,180,0,0,fit]);
  }
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#ar-video video")).objectFit'),'contain');
  assert.equal(await evaluate('document.querySelector("#ar-wide").style.aspectRatio'),'');
  assert.equal(await evaluate('document.querySelector("#ar-wide").hasAttribute("role")'),false);
  // Source-derived independent rules vs actual component geometry/style for every named pattern.
  await evaluate(`(()=>{const s=document.createElement('style');s.textContent=${JSON.stringify(referenceCss)};document.head.append(s);const root=document.getElementById('aspect-ratio-examples'),ref=root.cloneNode(true);ref.id='ar-reference';ref.querySelectorAll('.astryx-aspect-ratio').forEach(e=>e.classList.replace('astryx-aspect-ratio','ref-ratio'));ref.querySelectorAll('.astryx-aspect-ratio__content').forEach(e=>e.classList.replace('astryx-aspect-ratio__content','ref-content'));root.after(ref)})()`);
  assert.equal(await evaluate(`(()=>{const get=(selector)=>[...document.querySelectorAll(selector)].slice(0,13).map(e=>{const r=e.getBoundingClientRect(),s=getComputedStyle(e),c=e.firstElementChild,t=getComputedStyle(c),m=c.firstElementChild,u=getComputedStyle(m);return [r.width,r.height,s.position,s.overflow,s.minHeight,s.flexShrink,s.borderRadius,t.position,t.insetInlineStart,t.width,t.height,u.width,u.height,u.objectFit]});return JSON.stringify(get('#aspect-ratio-examples .astryx-aspect-ratio'))===JSON.stringify(get('#ar-reference .ref-ratio'))})()`),true,'six patterns match declarations lowered from verified TS source');
  await evaluate('document.getElementById("ar-reference").remove()');
  // Preserve native root, child events, keyboard focus, caller labels and mounted ref.
  await evaluate('document.getElementById("ar-child").click()');await until('document.getElementById("ar-events").textContent==="2"');
  assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("ar-native");e.focus();return [e.tagName,e.lang,e.dataset.test,e.classList.contains("caller-ratio"),document.activeElement===e]})()'),['DIV','fr','native',true,true]);
  await send('Page.bringToFront');
  await evaluate('document.getElementById("ar-child").focus()');
  assert.equal(await evaluate('document.activeElement.id'),'ar-child');
  await send('Input.dispatchKeyEvent',{type:'keyDown',key:'Enter',code:'Enter',windowsVirtualKeyCode:13,text:'\r'});
  await send('Input.dispatchKeyEvent',{type:'keyUp',key:'Enter',code:'Enter',windowsVirtualKeyCode:13});
  await until('document.getElementById("ar-events").textContent==="3"');
  const ax=await send('Accessibility.getFullAXTree');assert.ok(ax.nodes.some(n=>!n.ignored&&n.role?.value==='image'&&n.name?.value==='Circular image'));
  const before=await box('ar-skeleton');await evaluate('document.getElementById("ar-load").click()');await until('!document.querySelector("#ar-skeleton .astryx-skeleton")');close(await box('ar-skeleton'),before);assert.equal(await evaluate('document.getElementById("ar-skeleton").parentElement.getAttribute("aria-busy")'),'false');
  close(await box('ar-inline'),[320,320*2/3]);close(await box('ar-responsive'),[600,200]);
  await evaluate('document.querySelector(".ar-responsive-stage").style.width="300px"');close(await box('ar-responsive'),[300,200]);
  await send('Emulation.setDeviceMetricsOverride',{width:400,height:900,deviceScaleFactor:1,mobile:false});close(await box('ar-media'),[320,320]);
  // Isolated iframe exercises each documented layer setup without page layer-order contamination.
  for(const [order,preflight] of [['reset, theme, base, astryx-base, astryx-theme, components, utilities','base'],['reset, tw-preflight, astryx-base, astryx-theme','tw-preflight']]) {
    for(const fit of ['cover','contain']) {
      const html=`<style>@layer ${order}; @layer ${preflight} {img,video {display:block;max-width:100%;height:auto}} ${css}</style><div style="width:320px"><div class="astryx-aspect-ratio" style="--astryx-aspect-ratio:1.7777777777777777"><div class="astryx-aspect-ratio__content" data-astryx-aspect-ratio-override="${fit}"><img width="100" height="100"></div></div></div>`;
      const measured=await evaluate(`new Promise(resolve=>{const frame=document.createElement('iframe');frame.onload=()=>{const d=frame.contentDocument,e=d.querySelector('img'),r=e.getBoundingClientRect();resolve([r.width,r.height,frame.contentWindow.getComputedStyle(e).objectFit]);frame.remove()};frame.srcdoc=${JSON.stringify(html)};document.body.append(frame)})`);
      assert.deepEqual(measured,[320,180,fit]);
    }
  }
  // Inline/unlayered/later-layer child overrides, no descendant or child prop mutation.
  assert.deepEqual(await evaluate(`(()=>{const e=document.querySelector('#ar-cover img'),s=document.createElement('style');s.textContent='@layer utilities { #ar-cover img {height:40px} }';document.head.append(s);const h=e.getBoundingClientRect().height;e.style.cssText='width:60px;height:30px;object-fit:contain';const r=e.getBoundingClientRect();return [h,r.width,r.height,getComputedStyle(e).objectFit]})()`),[40,60,30,'contain']);
  await evaluate('document.getElementById("ar-contain").dir="rtl"');assert.equal(await evaluate('getComputedStyle(document.querySelector("#ar-contain > div")).insetInlineStart'),'0px');
  assert.equal(await evaluate(`(()=>{const e=document.getElementById('ar-cover'),child=e.firstElementChild;child.style.width='640px';child.style.height='500px';const r=e.getBoundingClientRect();return getComputedStyle(e).overflow==='clip'&&r.width===320&&r.height===180})()`),true,'overflowing content cannot grow the ratio box');
  assert.deepEqual(await evaluate(`(()=>{const p=document.createElement('div');p.style.cssText='display:flex;width:100px';const e=document.getElementById('ar-square').cloneNode(true);e.style.width='300px';p.append(e);document.body.append(p);const r=e.getBoundingClientRect();p.remove();return [r.width,r.height]})()`),[300,300],'box does not flex shrink');
  // Explicitly retain upstream sizing gaps rather than masking them.
  assert.deepEqual(await evaluate(`(()=>{const root=document.getElementById('ar-unset'),p=root.parentElement,clone=root.cloneNode(true);clone.removeAttribute('id');clone.style.height='50px';p.append(clone);const r=clone.getBoundingClientRect();const shrink=document.createElement('div');shrink.style.display='inline-flex';shrink.append(root.cloneNode(true));document.body.append(shrink);const w=shrink.firstElementChild.getBoundingClientRect().width;clone.remove();shrink.remove();return [r.width,r.height,w]})()`),[320,50,0]);
  console.log('AspectRatio offline browser: six source-derived patterns, geometry, fits, ellipse, responsive/cascade, native ref/events/focus, AX and caller-owned Skeleton replacement passed.');
} finally {
  await fetch(`${endpoint}/json/close/${target.id}`);socket.close();
}
