// Node 22+ and Chrome CDP; no npm dependencies or live source requests.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
const endpoint=process.env.CDP_URL??'http://127.0.0.1:9222';
const target=await(await fetch(`${endpoint}/json/new?about:blank`,{method:'PUT'})).json();
const ws=new WebSocket(target.webSocketDebuggerUrl);await new Promise((resolve,reject)=>{ws.onopen=resolve;ws.onerror=reject});
let serial=0;const pending=new Map(),errors=[];ws.onmessage=({data})=>{const m=JSON.parse(data);if(m.method==='Runtime.exceptionThrown')errors.push(m.params);const p=pending.get(m.id);if(p){pending.delete(m.id);clearTimeout(p.timer);m.error?p.reject(Error(JSON.stringify(m.error))):p.resolve(m.result)}};
const send=(method,params={})=>new Promise((resolve,reject)=>{const id=++serial,timer=setTimeout(()=>{pending.delete(id);reject(Error(`CDP timeout: ${method}`))},15000);pending.set(id,{resolve,reject,timer});ws.send(JSON.stringify({id,method,params}))});
const evaluate=async expression=>{const r=await send('Runtime.evaluate',{expression,returnByValue:true,awaitPromise:true});assert.equal(r.exceptionDetails,undefined,JSON.stringify(r.exceptionDetails));return r.result.value};
const wait=ms=>new Promise(r=>setTimeout(r,ms));
async function waitFor(expression){const deadline=Date.now()+10000;while(Date.now()<deadline){if(await evaluate(expression))return;await wait(25)}assert.fail(`Timed out: ${expression}`)}
const click=async id=>{await evaluate(`document.getElementById(${JSON.stringify(id)}).click()`);await wait(60)};
const reference=JSON.parse(await readFile(new URL('./fixtures/badge-neutral-reference.json',import.meta.url),'utf8'));
const metric=e=>{const c=getComputedStyle(e),r=e.getBoundingClientRect(),label=e.lastElementChild,l=getComputedStyle(label);return {variant:e.dataset.variant,text:label.textContent,title:e.getAttribute('title'),dir:label.getAttribute('dir'),display:c.display,width:r.width,height:r.height,color:c.color,background:c.backgroundColor,font:c.fontFamily,size:c.fontSize,leading:c.lineHeight,weight:c.fontWeight,gap:c.gap,paddingInline:c.paddingInline,borderRadius:c.borderRadius,minHeight:c.minHeight,maxWidth:c.maxWidth,whiteSpace:c.whiteSpace,overflow:l.overflow,textOverflow:l.textOverflow,bidi:l.unicodeBidi};};
try {
 await send('Page.enable');await send('Runtime.enable');await send('Accessibility.enable');await send('Emulation.setDeviceMetricsOverride',{width:1200,height:1600,deviceScaleFactor:1,mobile:false});
 await send('Page.navigate',{url:process.env.SHOWCASE_URL??'http://127.0.0.1:8080'});await waitFor('Boolean(document.getElementById("bd-long"))');await evaluate('document.fonts.ready.then(()=>true)');
 for(const mode of ['light','dark']) {
  if(mode==='dark')await click('bd-theme');
  const actual=await evaluate(`Array.from(document.querySelectorAll('[data-badge-named]')).map(${metric})`);assert.equal(actual.length,28);
  for(const value of actual){const expected=reference.measurements[mode].find(e=>e.variant===value.variant&&e.text===value.text);assert.ok(expected,`${mode}: original named control ${value.text}`);for(const key of Object.keys(expected)){
   if(key==='width')assert.ok(Math.abs(value[key]-expected[key])<=1/32,`${mode}/${value.text}/${key}: ${value[key]} != ${expected[key]}`);
   else if(key==='font')assert.equal(value[key].split(',')[0],expected[key].split(',')[0]);
   else assert.equal(value[key],expected[key],`${mode}/${value.text}/${key}`);
  }}
 }
 await click('bd-theme');
 assert.equal(await evaluate('document.querySelectorAll("#bd-all .astryx-badge").length'),14);
 for(const id of ['bd-grid','bd-stack'])assert.equal(await evaluate(`(()=>{const p=document.getElementById(${JSON.stringify(id)}),e=p.querySelector('.astryx-badge'),l=e.lastElementChild;return e.getBoundingClientRect().width<p.getBoundingClientRect().width&&l.scrollWidth<=l.clientWidth})()`),true,`${id} stays content sized, not stretched`);
 assert.equal(await evaluate('(()=>{const p=document.getElementById("bd-row").getBoundingClientRect(),e=document.getElementById("bd-long"),b=e.getBoundingClientRect(),l=e.lastElementChild;return b.left>=p.left&&b.right<=p.right+.01&&l.scrollWidth>l.clientWidth&&getComputedStyle(l).textOverflow==="ellipsis"})()'),true,'crowded flex row clips only label, not neighbours');
 assert.equal(await evaluate('(()=>{const p=document.getElementById("bd-narrow").getBoundingClientRect(),b=p&&document.querySelector("#bd-narrow .astryx-badge").getBoundingClientRect();return b.left>=p.left&&b.right<=p.right+.01})()'),true,'cell narrower than a line stays bounded');
 assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("bd-large-text"),l=e.lastElementChild,b=e.getBoundingClientRect(),r=l.getBoundingClientRect();return [b.height+1/64>=parseFloat(getComputedStyle(e).lineHeight),r.top>=b.top-.01,r.bottom<=b.bottom+.01,document.getElementById("bd-large-empty").getBoundingClientRect().height]})()'),[true,true,true,20],'text-height floor grows only renderable labels');
 const tree=await send('Accessibility.getFullAXTree');
 const full='A very long label whose full tail remains available in the DOM';
 assert.ok(tree.nodes.some(n=>!n.ignored&&n.role?.value==='StaticText'&&n.name?.value===full),'clipped full text remains in AX tree');
 assert.equal(await evaluate('document.getElementById("bd-long").title'),full);
 assert.equal(await evaluate('document.querySelectorAll("#badge-examples .astryx-badge[role],#badge-examples .astryx-badge[tabindex],#badge-examples .astryx-badge[aria-live]").length'),0,'no invented role, focus stop or announcements');
 assert.equal(await evaluate('document.getElementById("bd-long").tabIndex'),-1);
 assert.equal(await evaluate('document[Symbol.for("astryx.dioxus.button.runtime")].records.has(document.getElementById("bd-long"))'),false,'static badge owns no press/tooltip controller');
 assert.deepEqual(await evaluate('["bd-number","bd-zero","bd-rich","bd-empty","bd-empty-string","bd-title"].map(id=>document.getElementById(id).getAttribute("title"))'),['42','0',null,null,null,'Caller title']);
 assert.equal(await evaluate('document.getElementById("bd-icon").firstElementChild.classList.contains("astryx-icon")'),true);
 assert.equal(await evaluate('document.querySelector("#bd-icon .astryx-icon").getAttribute("aria-hidden")'),'true');
 assert.equal(await evaluate('document.querySelector("#bd-icon-context .astryx-icon").getAttribute("aria-label")'),'Caller glyph');
 assert.equal(await evaluate('document.querySelector("#bd-icon-context .astryx-icon").dataset.size'),'lg','Badge must not override caller/context icon size');
 const start=await evaluate('document.getElementById("bd-events").textContent');assert.equal(start,'Native events: 1');
 await click('bd-native');assert.equal(await evaluate('document.getElementById("bd-events").textContent'),'Native events: 2');
 assert.equal(await evaluate('document.getElementById("bd-native").lang'),'fr');
 assert.equal(await evaluate('document.getElementById("bd-native").getAttribute("aria-describedby")'),'bd-note');
 // Original any-RTL rule, not first-strong or dir=auto. Check actual glyph positions.
 for(const direction of ['rtl','ltr']){
  const rows=await evaluate(`Array.from(document.querySelectorAll('#bd-${direction} .astryx-badge')).map(e=>{const l=e.lastElementChild,text=l.textContent,positions=[];if(l.firstChild?.nodeType===3){for(let i=0;i<text.length;i++){const r=document.createRange();r.setStart(l.firstChild,i);r.setEnd(l.firstChild,i+1);positions.push(r.getBoundingClientRect().x)}}return {text,dir:l.getAttribute('dir'),bidi:getComputedStyle(l).unicodeBidi,positions}})`);
  rows.slice(0,4).forEach(row=>{assert.equal(row.dir,'ltr');assert.equal(row.bidi,'isolate');assert.ok(row.positions[0]<row.positions.at(-1),`${direction}/${row.text}: weak characters retain order`)});
  rows.slice(4,6).forEach(row=>{assert.equal(row.dir,null);const arabic=row.text.indexOf(' ')+1;assert.equal(row.positions[0]>row.positions[arabic],direction==='rtl',`${direction}/${row.text}: inherit paragraph direction`)});
  assert.equal(rows[6].dir,null,'rich bdi stays caller controlled');
 }
 assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("bd-custom"),c=getComputedStyle(e);return [e.dataset.variant,c.backgroundColor,c.color]})()'),['premium','rgba(0, 0, 0, 0)','rgb(10, 60, 100)'],'custom name has geometry but no neutral paint');
 assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("bd-custom").cloneNode(true);e.removeAttribute("id");e.removeAttribute("style");e.dataset.variant="gray";document.getElementById("badge-examples").append(e);const c=getComputedStyle(e),result=[c.backgroundColor,c.color];e.remove();return result})()'),[reference.measurements.light[0].background,reference.measurements.light[0].color],'Neutral gray is a theme extension, not an invented fifteenth builtin');
 await evaluate('(()=>{const s=document.createElement("style");s.id="bd-test-theme";s.textContent="@layer astryx-theme {.astryx-badge[data-variant=premium] {background:rgb(200,220,240);height:30px;border-radius:7px}}";document.head.append(s)})()');
 assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("bd-custom"),c=getComputedStyle(e);return [c.backgroundColor,e.getBoundingClientRect().height,c.borderRadius]})()'),['rgb(200, 220, 240)',30,'7px']);
 await evaluate('document.getElementById("bd-custom").style.setProperty("--text-supporting-size","32px")');
 assert.equal(await evaluate('(()=>{const e=document.getElementById("bd-custom");return e.getBoundingClientRect().height+1/64>=parseFloat(getComputedStyle(e).lineHeight)})()'),true,'custom 30px height still grows with larger text');
 await wait(2200);await evaluate('window.__badgeWrites=[];window.__badgeObserver=new MutationObserver(records=>{for(const r of records){const e=r.target.nodeType===1?r.target:r.target.parentElement;if(e?.closest("[data-astryx-live-region],[data-astryx-modal-live-region]"))window.__badgeWrites.push(e.textContent)}});window.__badgeObserver.observe(document.body,{subtree:true,childList:true,characterData:true})');
 await click('bd-update');await waitFor('document.getElementById("bd-dynamic").textContent==="Updated state"');await wait(120);
 assert.deepEqual(await evaluate('window.__badgeWrites'),[],'static label update does not announce');await evaluate('window.__badgeObserver.disconnect()');
 await send('Emulation.setDeviceMetricsOverride',{width:402,height:714,deviceScaleFactor:1,mobile:true});
 assert.equal(await evaluate('(()=>{const p=document.getElementById("bd-row").getBoundingClientRect(),b=document.getElementById("bd-long").getBoundingClientRect();return b.right<=p.right+.01})()'),true,'mobile crowded row stays bounded');
 await send('Emulation.setEmulatedMedia',{features:[{name:'forced-colors',value:'active'}]});
 const forced=await evaluate('Array.from(document.querySelectorAll("#bd-all .astryx-badge")).map(e=>{const c=getComputedStyle(e);return {variant:e.dataset.variant,outline:c.outlineStyle,width:c.outlineWidth,offset:c.outlineOffset,color:c.color}})');
 for(const variant of ['info','success','warning','error']){const value=forced.find(e=>e.variant===variant);assert.equal(value.outline,'solid');assert.equal(value.width,'1px');assert.equal(value.offset,'-1px')}
 assert.equal(await evaluate('document.querySelector("#bd-all .astryx-badge").textContent'),'Neutral');
 assert.deepEqual(errors,[]);
 console.log('Badge browser checks passed: all 28 upstream controls in light/dark, 14 variants, titles/rich/empty content, AX full text, no own roles/focus/live/press machinery, native ref/events, caller icons, grid/stack/row bounds, larger text, custom theme overrides, actual RTL character order, static updates, mobile and forced colors.');
}finally{ws.close();await fetch(`${endpoint}/json/close/${target.id}`)}
