// Node 22+ and Chrome CDP. Frozen source evidence; no live network dependency.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {dividerMetrics,selectDividerMetrics} from './divider_metrics.mjs';
const endpoint=process.env.CDP_URL??'http://127.0.0.1:9222';
const target=await(await fetch(`${endpoint}/json/new?about:blank`,{method:'PUT'})).json();
const ws=new WebSocket(target.webSocketDebuggerUrl);await new Promise((resolve,reject)=>{ws.onopen=resolve;ws.onerror=reject});
let serial=0;const pending=new Map(),errors=[];ws.onmessage=({data})=>{const m=JSON.parse(data);if(m.method==='Runtime.exceptionThrown')errors.push(m.params);const p=pending.get(m.id);if(p){pending.delete(m.id);clearTimeout(p.timer);m.error?p.reject(Error(JSON.stringify(m.error))):p.resolve(m.result)}};
const send=(method,params={})=>new Promise((resolve,reject)=>{const id=++serial,timer=setTimeout(()=>{pending.delete(id);reject(Error(`CDP timeout: ${method}`))},15000);pending.set(id,{resolve,reject,timer});ws.send(JSON.stringify({id,method,params}))});
const evaluate=async expression=>{const r=await send('Runtime.evaluate',{expression,returnByValue:true,awaitPromise:true});assert.equal(r.exceptionDetails,undefined,JSON.stringify(r.exceptionDetails));return r.result.value};
const wait=ms=>new Promise(r=>setTimeout(r,ms));
async function waitFor(expression){const end=Date.now()+10000;while(Date.now()<end){if(await evaluate(expression))return;await wait(25)}assert.fail(`Timed out: ${expression}`)}
const click=async id=>{await evaluate(`document.getElementById(${JSON.stringify(id)}).click()`);await wait(60)};
const reference=JSON.parse(await readFile(new URL('./fixtures/divider-neutral-reference.json',import.meta.url),'utf8'));
function compare(actual,expected,path='root'){
 if(typeof expected==='number'){assert.ok(Math.abs(actual-expected)<=1/64+1e-6,`${path}: ${actual} != ${expected}`);return}
 if(expected&&typeof expected==='object'){assert.deepEqual(Object.keys(actual),Object.keys(expected),`${path} keys`);for(const key of Object.keys(expected))compare(actual[key],expected[key],`${path}/${key}`);return}
 assert.equal(actual,expected,path);
}
const edges=e=>{const parent=e.parentElement.getBoundingClientRect(),box=e.getBoundingClientRect(),s=getComputedStyle(e);return {left:box.left-parent.left,right:parent.right-box.right,top:box.top-parent.top,bottom:parent.bottom-box.bottom,width:box.width,height:box.height,start:s.marginInlineStart,end:s.marginInlineEnd,blockStart:s.marginBlockStart,blockEnd:s.marginBlockEnd}};
try {
 await send('Page.enable');await send('Runtime.enable');await send('Accessibility.enable');await send('Emulation.setDeviceMetricsOverride',{width:1200,height:1600,deviceScaleFactor:1,mobile:false});
 await send('Page.navigate',{url:process.env.SHOWCASE_URL??'http://127.0.0.1:8080'});await waitFor('Boolean(document.getElementById("dv-zero"))');await evaluate('document.fonts.ready.then(()=>true)');
 for(const mode of ['light','dark']){
  if(mode==='dark')await click('dv-theme');
  const actual=await evaluate(`Array.from(document.querySelectorAll('[data-divider-named]')).map((element,index)=>(${selectDividerMetrics.toString()})((${dividerMetrics.toString()})(element,${JSON.stringify(reference.identities)}[index])))`);
  assert.equal(actual.length,9);compare(actual,reference.measurements[mode],mode);
 }
 await click('dv-theme');
 let tree=await send('Accessibility.getFullAXTree');
 for(const name of ['or','0','Rich label','Caller name','External reference','Native root','Stable label'])assert.ok(tree.nodes.some(n=>!n.ignored&&n.role?.value==='separator'&&n.name?.value===name),`AX separator name ${name}`);
 assert.ok(tree.nodes.some(n=>!n.ignored&&n.role?.value==='separator'&&n.properties?.some(p=>p.name==='orientation'&&p.value.value==='vertical')),'vertical AX separator');
 assert.equal(await evaluate('document.getElementById("dv-zero").children.length'),3);
 for(const id of ['dv-empty','dv-false','dv-true'])assert.deepEqual(await evaluate(`(()=>{const e=document.getElementById(${JSON.stringify(id)});return [e.children.length,e.hasAttribute('aria-labelledby')]})()`),[1,false]);
 assert.equal(await evaluate('document.getElementById("dv-named").hasAttribute("aria-labelledby")'),false);
 assert.equal(await evaluate('document.getElementById("dv-referenced").getAttribute("aria-labelledby")'),'dv-external-name');
 assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("dv-overridden");return [e.getAttribute("role"),e.getAttribute("aria-orientation"),e.dataset.orientation,e.dataset.variant]})()'),['note','vertical','horizontal','subtle'],'explicit caller semantics follow authoritative FR2/FR6, not the source implementation bug');
 assert.equal(await evaluate('document.querySelectorAll("#divider-examples .astryx-divider[tabindex],#divider-examples .astryx-divider[aria-live]").length'),0);
 assert.equal(await evaluate('document.getElementById("dv-zero").tabIndex'),-1);
 assert.equal(await evaluate('Array.from(document.querySelectorAll("#divider-examples .astryx-divider__line")).some(e=>e.hasAttribute("role")||e.hasAttribute("tabindex")||e.hasAttribute("aria-label"))'),false,'internal rules add no separate semantics');
 assert.equal(await evaluate('document[Symbol.for("astryx.dioxus.button.runtime")].records.has(document.getElementById("dv-zero"))'),false,'no press/tooltip controller');
 assert.equal(await evaluate('document.getElementById("dv-events").textContent'),'Native events: 1');
 await click('dv-native');assert.equal(await evaluate('document.getElementById("dv-events").textContent'),'Native events: 2');
 assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("dv-native");return [e.tagName,e.lang,e.dataset.testid,e.getAttribute("aria-describedby")]})()'),['DIV','fr','divider-native','dv-native-note']);
 await evaluate('window.__dividerRoot=document.getElementById("dv-changing");window.__dividerLabel=window.__dividerRoot.getAttribute("aria-labelledby")');
 await click('dv-toggle-label');assert.equal(await evaluate('document.getElementById("dv-changing")===window.__dividerRoot&&!window.__dividerRoot.hasAttribute("aria-labelledby")&&window.__dividerRoot.children.length===1'),true);
 await click('dv-toggle-label');assert.equal(await evaluate('document.getElementById("dv-changing")===window.__dividerRoot&&window.__dividerRoot.getAttribute("aria-labelledby")===window.__dividerLabel'),true);
 for(const direction of ['ltr','rtl']){
  const inline=await evaluate(`(${edges.toString()})(document.querySelector('#dv-bleed-${direction} .astryx-divider'))`);
  compare(inline.left,0);compare(inline.right,0);assert.equal(inline.width,320);assert.equal(inline.start,'-13px');assert.equal(inline.end,'-27px');
  const block=await evaluate(`(${edges.toString()})(document.querySelector('#dv-block-${direction} .astryx-divider'))`);
  compare(block.top,0);compare(block.bottom,0);assert.equal(block.height,160);assert.equal(block.blockStart,'-11px');assert.equal(block.blockEnd,'-23px');
 }
 assert.equal(await evaluate('document.querySelector("#dv-zero-inset .astryx-divider").getBoundingClientRect().width'),200);
 assert.equal(await evaluate('document.querySelector("#dv-zero-block .astryx-divider").getBoundingClientRect().height'),90);
 await evaluate('(()=>{const p=document.getElementById("dv-bleed-rtl");p.style.paddingInlineStart="21px";p.style.setProperty("--container-padding-inline-start","21px")})()');
 const updated=await evaluate(`(${edges.toString()})(document.querySelector('#dv-bleed-rtl .astryx-divider'))`);compare(updated.left,0);compare(updated.right,0);assert.equal(updated.start,'-21px');assert.equal(updated.width,320,'CSS inset change needs no component measurement');
 for(const axis of ['horizontal','vertical'])for(const weight of ['subtle','strong']){
  const value=await evaluate(`(()=>{const e=document.getElementById('dv-${axis}-${weight}'),r=[e.firstElementChild,e.lastElementChild].map(n=>n.getBoundingClientRect()),l=getComputedStyle(e.children[1]);return {first:${JSON.stringify(axis)}==='horizontal'?r[0].width:r[0].height,last:${JSON.stringify(axis)}==='horizontal'?r[1].width:r[1].height,inline:l.paddingInline,block:l.paddingBlock}})()`);
  compare(value.first,value.last,`${axis}/${weight}/symmetric rules`);assert.equal(value.inline,axis==='horizontal'?'12px':'0px');assert.equal(value.block,axis==='horizontal'?'0px':'12px');
 }
 assert.equal(await evaluate('document.getElementById("dv-custom").dataset.variant'),'custom-rule');
 assert.deepEqual(await evaluate('Array.from(document.querySelectorAll("#dv-custom .astryx-divider__line")).map(e=>getComputedStyle(e).backgroundColor)'),['rgba(0, 0, 0, 0)','rgba(0, 0, 0, 0)'],'unmatched custom paint remains the documented original OQ2 gap, not a silently chosen fallback');
 await evaluate('(()=>{const s=document.createElement("style");s.id="dv-theme-test";s.textContent="@layer astryx-theme {.astryx-divider {--border-width:3px;--color-border:rgb(20,80,140)}}";document.head.append(s)})()');
 assert.deepEqual(await evaluate('(()=>{const h=document.querySelector("[data-divider-named=showcase-subtle] .astryx-divider__line"),v=document.querySelector("[data-divider-named=vertical-revenue] .astryx-divider__line");return [h.getBoundingClientRect().height,v.getBoundingClientRect().width,getComputedStyle(h).backgroundColor]})()'),[3,3,'rgb(20, 80, 140)']);
 await evaluate('document.getElementById("dv-theme-test").remove()');
 await send('Emulation.setDeviceMetricsOverride',{width:402,height:714,deviceScaleFactor:1,mobile:true});
 const mobile=await evaluate('Array.from(document.querySelectorAll("[data-divider-named=full-items],[data-divider-named=full-tax]")).map(e=>{const a=e.getBoundingClientRect(),c=e.closest(".demo-divider-card").getBoundingClientRect();return [a.left-c.left,c.right-a.right]})');for(const pair of mobile)pair.forEach(value=>compare(value,0,'mobile full bleed meets Card edges'));
 await send('Emulation.setEmulatedMedia',{features:[{name:'forced-colors',value:'active'}]});
 tree=await send('Accessibility.getFullAXTree');assert.ok(tree.nodes.some(n=>!n.ignored&&n.role?.value==='separator'&&n.name?.value==='Rich label'),'forced colors retain separator semantics and full label');
 assert.deepEqual(errors,[]);
 console.log('Divider browser checks passed: all nine original controls/light-dark native frame metrics, rule/label anatomy and AX names, caller semantic precedence, native refs/events, stable label updates, both logical full-bleed axes in LTR/RTL, absent/changed insets, custom-variant source gap, theme thickness/paint, narrow layouts and forced-color semantics.');
}finally{ws.close();await fetch(`${endpoint}/json/close/${target.id}`)}
