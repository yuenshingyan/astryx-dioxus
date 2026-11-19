// Node 22+, isolated Chrome CDP. Original fixtures are offline evidence.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {statusDotMetrics,nativeFrameMetrics,expandControl,expandContext} from './status_dot_metrics.mjs';
const endpoint=process.env.CDP_URL??'http://127.0.0.1:9222';
const target=await(await fetch(`${endpoint}/json/new?about:blank`,{method:'PUT'})).json();
const ws=new WebSocket(target.webSocketDebuggerUrl);await new Promise((resolve,reject)=>{ws.onopen=resolve;ws.onerror=reject});
let sequence=0;const pending=new Map(),events=new Map(),errors=[];
ws.onmessage=({data})=>{const m=JSON.parse(data);events.get(m.method)?.(m.params);if(m.method==='Runtime.exceptionThrown')errors.push(m.params.exceptionDetails);const p=pending.get(m.id);if(p){pending.delete(m.id);clearTimeout(p.timer);m.error?p.reject(Error(JSON.stringify(m.error))):p.resolve(m.result)}};
const send=(method,params={})=>new Promise((resolve,reject)=>{const id=++sequence,timer=setTimeout(()=>{pending.delete(id);reject(Error(`CDP timeout ${method}`))},15000);pending.set(id,{resolve,reject,timer});ws.send(JSON.stringify({id,method,params}))});
const evaluate=async expression=>{const r=await send('Runtime.evaluate',{expression,returnByValue:true,awaitPromise:true});assert.equal(r.exceptionDetails,undefined,JSON.stringify(r.exceptionDetails));return r.result.value};
const wait=ms=>new Promise(resolve=>setTimeout(resolve,ms));
async function waitFor(expression){const end=Date.now()+15000;while(Date.now()<end){if(await evaluate(expression))return;await wait(25)}assert.fail(`Timed out: ${expression}`)}
const click=async id=>{await evaluate(`document.getElementById(${JSON.stringify(id)}).click()`);await wait(70)};
const rect=id=>evaluate(`(()=>{const r=document.getElementById(${JSON.stringify(id)}).getBoundingClientRect();return {x:r.x,y:r.y,width:r.width,height:r.height}})()`);
const key=async(key,code,windowsVirtualKeyCode)=>{await send('Input.dispatchKeyEvent',{type:'keyDown',key,code,windowsVirtualKeyCode});await send('Input.dispatchKeyEvent',{type:'keyUp',key,code,windowsVirtualKeyCode});await wait(70)};
const point=async id=>{await evaluate(`document.getElementById(${JSON.stringify(id)}).scrollIntoView({block:'center'})`);await wait(100);const r=await rect(id);return{x:r.x+r.width/2,y:r.y+r.height/2}};
const mouse=async(type,position)=>send('Input.dispatchMouseEvent',{type,...position,button:'left',buttons:type==='mouseReleased'?0:1,clickCount:1});
const pointerClick=async id=>{const p=await point(id);await mouse('mouseMoved',p);await mouse('mousePressed',p);await mouse('mouseReleased',p);await wait(100)};
const touch=async position=>{await send('Input.dispatchTouchEvent',{type:'touchStart',touchPoints:[{...position,id:1,radiusX:3,radiusY:3}]});await send('Input.dispatchTouchEvent',{type:'touchEnd',touchPoints:[]});await wait(120)};
const fixture=JSON.parse(await readFile(new URL('./fixtures/status-dot-neutral-reference.json',import.meta.url),'utf8'));
const ids=['sd-showcase','sd-pulsing','sd-indicators','sd-variants'];
function compare(actual,expected,path='root'){
 if(typeof expected==='number'){assert.ok(Math.abs(actual-expected)<=1/64+1e-6,`${path}: ${actual} != ${expected}`);return}
 if(expected&&typeof expected==='object'){assert.deepEqual(Object.keys(actual).sort(),Object.keys(expected).sort(),`${path} keys`);for(const key of Object.keys(expected))compare(actual[key],expected[key],`${path}/${key}`);return}
 assert.equal(actual,expected,path);
}
function controlView(value){
 const {rawStyle,dataAttributes,...root}=value.root;
 const style={...value.style};
 if(style.borderWidth==='0px')style.borderStyle='none';
 if(style.outlineStyle==='none'){style.outlineWidth='0px';style.outlineColor='unused';style.outlineOffset='0px'}
 const {name,...css}=value.animation.css;
 return {root:{...root,dataAttributes:{'data-variant':dataAttributes['data-variant']}},geometry:value.geometry,style,
  backdrop:value.parentPaint.find(p=>p.backgroundColor!=='rgba(0, 0, 0, 0)')?.backgroundColor,
  animation:{css,effects:value.animation.effects.map(({name,...effect})=>effect)}};
}
function frameView(frame){
 const style={};for(const property of ['display','flexDirection','flexWrap','rowGap','columnGap','padding','margin','backgroundColor'])style[property]=frame.style[property];
 if(frame.tag==='span')for(const property of ['fontSize','fontWeight','lineHeight','letterSpacing','color','textAlign','whiteSpace'])style[property]=frame.style[property];
 return {tag:frame.tag,box:frame.box,textBoxes:frame.textBoxes,parent:frame.parent,children:frame.children,style};
}
const media=features=>send('Emulation.setEmulatedMedia',{features:Object.entries({'prefers-reduced-motion':'no-preference','forced-colors':'none',...features}).map(([name,value])=>({name,value}))});
const focus=id=>evaluate(`document.getElementById(${JSON.stringify(id)}).focus()`);
const pinned=id=>`document.getElementById(${JSON.stringify(id)}).getAttribute('data-astryx-tooltip-pinned')==='true'`;
const open=id=>`(()=>{const e=document.getElementById(${JSON.stringify(id)}),t=document.getElementById(e.dataset.astryxStatusTooltipId+'-tooltip');return !!t?.matches(':popover-open')})()`;
const ax=async id=>{const doc=await send('DOM.getDocument'),node=await send('DOM.querySelector',{nodeId:doc.root.nodeId,selector:'#'+id});return(await send('Accessibility.getPartialAXTree',{nodeId:node.nodeId,fetchRelatives:false})).nodes[0]};
try{
 await send('Page.enable');await send('Runtime.enable');await send('Accessibility.enable');await send('Page.bringToFront');await send('Emulation.setDeviceMetricsOverride',{width:1200,height:1600,deviceScaleFactor:1,mobile:false});await media({});
 await send('Page.navigate',{url:process.env.SHOWCASE_URL??'http://127.0.0.1:8080'});
 await waitFor('document.querySelectorAll("[data-status-dot-named]").length===19&&document.getElementById("sd-tip").dataset.astryxTooltipPlacement==="standalone"');await evaluate('document.fonts.ready.then(()=>true)');
 for(const mode of ['light','dark']){
  if(mode==='dark')await click('sd-theme');
  await evaluate('Array.from(document.querySelectorAll("[data-status-dot-named]")).forEach(e=>e.getAnimations().forEach(a=>{a.pause();a.currentTime=0}))');await wait(60);
  const actual=await evaluate(`Array.from(document.querySelectorAll('[data-status-dot-named]')).map(e=>(${statusDotMetrics.toString()})(e,e.closest('#sd-showcase,#sd-pulsing,#sd-indicators,#sd-variants')))`);
  actual.forEach((value,index)=>compare(controlView(value),controlView(expandControl(fixture,mode,index)),`${mode}/original/${index}`));
  const contexts=await evaluate(`${JSON.stringify(ids)}.map(id=>{const r=document.getElementById(id);return (${nativeFrameMetrics.toString()})(r,[...r.querySelectorAll('[data-status-dot-named]')])})`);
  contexts.forEach((context,index)=>compare(context.frames.map(frameView),expandContext(fixture,mode,index).frames.map(frameView),`${mode}/native frames/${index}`));
  const samples=await evaluate('(()=>{const e=document.querySelector("#sd-pulsing .astryx-status-dot"),a=e.getAnimations()[0];return [0,250,500,750,1000,1250,1500,1750,2000].map(currentTime=>{a.currentTime=currentTime;return {currentTime,opacity:+getComputedStyle(e).opacity}})})()');
  samples.forEach((value,index)=>assert.ok(Math.abs(value.opacity-Number(fixture.cases[mode].pulseSamples[index].opacity))<1e-5,`${mode} actual easing sample ${index}`));
 }
 await click('sd-theme');
 assert.equal(await evaluate('Array.from(document.querySelectorAll("[data-status-dot-named]")).some(e=>document[Symbol.for("astryx.dioxus.button.runtime")].records.has(e))'),false,'no tooltip means no runtime registration');
 assert.deepEqual(await evaluate('["sd-empty-text","sd-false","sd-true","sd-zero"].map(id=>document.getElementById(id).childElementCount)'),[0,0,0,1]);
 assert.equal(await evaluate('Array.from(document.querySelectorAll("[id^=sd-icon-]")).every(e=>{const s=e.firstElementChild,r=s.getBoundingClientRect(),v=s.firstElementChild.getBoundingClientRect(),p=e.getBoundingClientRect();return s.getAttribute("aria-hidden")==="true"&&r.width===8&&r.height===8&&v.width===8&&v.height===8&&Math.abs(v.x-p.x)<.01&&Math.abs(v.y-p.y)<.01&&getComputedStyle(s).color===getComputedStyle(e).color})'),true,'caller artwork fits hidden centered field and inherits ink');
 const first=await ax('sd-icon-0');assert.equal(first.role.value,'image');assert.equal(first.name.value,'Marked status 0');
 assert.equal(await evaluate('getComputedStyle(document.getElementById("sd-fallback")).backgroundColor'),await evaluate('getComputedStyle(document.querySelector("#sd-variants [data-variant=neutral]")).backgroundColor'),'approved safe neutral custom fallback');
 await evaluate('document.querySelector("main").setAttribute("data-astryx-theme","core");document.getElementById("status-dot-examples").removeAttribute("data-astryx-theme")');
 const corePairs=await evaluate('Array.from(document.querySelectorAll("#sd-variants .astryx-status-dot")).map(e=>{const s=getComputedStyle(e);return [s.backgroundColor,s.color]})');
 const rgb=hex=>`rgb(${[1,3,5].map(i=>parseInt(hex.slice(i,i+2),16)).join(', ')})`,core=fixture.pinnedContract.palettes.core.resolved.light;
 assert.deepEqual(corePairs,Object.keys(fixture.pinnedContract.plateTokens).map(variant=>[rgb(core[fixture.pinnedContract.plateTokens[variant]]),rgb(core[fixture.pinnedContract.inkTokens[variant]])]),'core roles remain distinct from Neutral palette');
 await evaluate('document.querySelector("main").setAttribute("data-astryx-theme","astryx");document.getElementById("status-dot-examples").setAttribute("data-astryx-theme","neutral")');
 assert.equal(await evaluate('document.getElementById("sd-fallback").dataset.variant'),'queued');assert.equal(await evaluate('getComputedStyle(document.getElementById("sd-custom")).backgroundColor'),'rgb(100, 40, 160)','caller theme overrides baseline');
 assert.equal(await evaluate('document.getElementById("sd-events").textContent'),'Native events: 1');await click('sd-native');assert.equal(await evaluate('document.getElementById("sd-events").textContent'),'Native events: 2');
 assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("sd-native");return [e.tagName,e.lang,e.dataset.testid,e.style.marginInline,e.classList.contains("caller-dot")]})()'),['SPAN','fr','native-status-dot','7px',true]);
 assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("sd-override");return[e.getAttribute("role"),e.getAttribute("aria-label"),e.getAttribute("tabindex")]})()'),['note','Caller-owned name','-1']);
 await evaluate('window.__sdRoot=document.getElementById("sd-changing")');await click('sd-change');assert.equal(await evaluate('window.__sdRoot===document.getElementById("sd-changing")'),true);assert.equal(await evaluate('document.getElementById("sd-changing").getAnimations().length'),1);await click('sd-change');assert.equal(await evaluate('document.getElementById("sd-changing").getAnimations().length'),0);
 // Standalone preview, persistent pin, shared dismissal and renderer propagation.
 assert.equal((await ax('sd-tip')).role.value,'button');assert.equal((await ax('sd-tip')).name.value,'Online');
 await focus('sd-tip');await waitFor(open('sd-tip'));assert.equal(await evaluate(pinned('sd-tip')),false);await key('Enter','Enter',13);await waitFor(pinned('sd-tip'));await waitFor('document.querySelector("[data-astryx-live-region=polite]")?.textContent.includes("Connected to the workspace")');assert.equal(await evaluate('document.querySelectorAll("[data-astryx-live-region=polite]").length'),1,'pin uses the existing hook-owned polite channel');await wait(350);assert.equal(await evaluate(open('sd-tip')),true);await key('Escape','Escape',27);await waitFor(`!(${open('sd-tip')})`);assert.equal(await evaluate('document.activeElement.id'),'sd-tip');
 await key(' ','Space',32);await waitFor(pinned('sd-tip'));await key(' ','Space',32);await waitFor(`!(${pinned('sd-tip')})`);
 await pointerClick('sd-tip');await waitFor(pinned('sd-tip'));await mouse('mouseMoved',{x:1,y:1});await wait(350);assert.equal(await evaluate(open('sd-tip')),true,'pin survives pointer leave');await pointerClick('sd-native');await waitFor(`!(${pinned('sd-tip')})`);
 await click('sd-cancel');assert.equal(await evaluate(pinned('sd-cancel')),false,'caller native click cancellation respected');await focus('sd-cancel');await key('Enter','Enter',13);assert.equal(await evaluate(pinned('sd-cancel')),false,'caller native key cancellation respected');
 await click('sd-contained');await waitFor(pinned('sd-contained'));assert.equal(await evaluate('document.getElementById("sd-host-actions").textContent'),'Host actions: 0','standalone control press does not activate clickable ancestor');await key('Escape','Escape',27);
 // In real native and ARIA hosts, tooltip is hover-only and host owns its name.
 for(const suffix of ['button','link','menu','option','role-button','role-link','labelled']){
  const id='sd-in-'+suffix,hostId=suffix==='labelled'?'sd-labelled-host':'sd-host-'+suffix;
  await waitFor(`document.getElementById(${JSON.stringify(id)}).dataset.astryxTooltipPlacement==='nested'`);
  assert.deepEqual(await evaluate(`(()=>{const e=document.getElementById(${JSON.stringify(id)});return[e.getAttribute('role'),e.getAttribute('tabindex')]})()`),['img',null]);
  const name=(await ax(hostId)).name.value;if(suffix==='labelled')assert.equal(name,'Manage users — Online');else assert.ok(name.includes(suffix==='button'?'Connection verified':suffix==='link'?'Data is updating':'Status explanation'),`${suffix} host accessible name: ${name}`);
  await focus(hostId);await wait(240);assert.equal(await evaluate(open(id)),false,`${suffix} host focus must not open nested tip`);
 }
 await pointerClick('sd-in-button');assert.equal(await evaluate('document.getElementById("sd-host-actions").textContent'),'Host actions: 1');assert.equal(await evaluate(pinned('sd-in-button')),false,'nested image press never pins');
 await mouse('mouseMoved',{x:1,y:1});await mouse('mouseMoved',await point('sd-in-button'));await waitFor(open('sd-in-button'));assert.equal((await ax('sd-host-button')).name.value.split('Connection verified').length-1,1,'opening surface cannot duplicate host name');await mouse('mouseMoved',{x:1,y:1});await wait(220);
 assert.equal(await evaluate('(()=>{const e=document.getElementById(document.getElementById("sd-in-button").dataset.astryxStatusTooltipId+"-tooltip");return !document.getElementById("sd-host-button").contains(e)})()'),true,'lazy tooltip surface stays outside the interactive host');
 // Config, placement and drop repair without replacing a root or owned caller name.
 await evaluate('window.__sdDynamic=document.getElementById("sd-dynamic")');await click('sd-place');await waitFor('document.getElementById("sd-dynamic").dataset.astryxTooltipPlacement==="nested"');assert.equal(await evaluate('window.__sdDynamic===document.getElementById("sd-dynamic")'),true);assert.ok((await ax('sd-dynamic-host')).name.value.includes('Dynamic status detail'));await click('sd-place');await waitFor('document.getElementById("sd-dynamic").dataset.astryxTooltipPlacement==="standalone"');assert.equal(await evaluate('document.getElementById("sd-dynamic-host").querySelector("[data-astryx-status-host-name]")'),null);
 assert.ok((await ax('sd-updating-host')).name.value.includes('Original connection detail'));await click('sd-update');await waitFor('document.getElementById("sd-updating-host").textContent.includes("Updated connection detail")');const updated=(await ax('sd-updating-host')).name.value;assert.ok(updated.includes('Updated connection detail')&&!updated.includes('Original connection detail'));
 await click('sd-toggle-tip');await waitFor('!document.getElementById("sd-optional").hasAttribute("data-astryx-status-tooltip-id")');assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("sd-optional");return[e.getAttribute("role"),e.getAttribute("tabindex"),e.getAttribute("aria-expanded")]})()'),['img',null,null]);await click('sd-toggle-tip');await waitFor('document.getElementById("sd-optional").dataset.astryxTooltipPlacement==="standalone"');
 await click('sd-drop');await waitFor(pinned('sd-drop'));const dropped=await evaluate('document.getElementById("sd-drop").dataset.astryxStatusTooltipId');await click('sd-toggle');await waitFor(`!document.getElementById(${JSON.stringify(dropped+'-tooltip')})`);assert.equal(await evaluate('document.getElementById("sd-drop")'),null);await click('sd-toggle');await waitFor('document.getElementById("sd-drop").dataset.astryxTooltipPlacement==="standalone"');
 // Approved outlines are asserted separately, not against the source's known gap.
 await media({'forced-colors':'active','prefers-reduced-motion':'reduce'});await wait(80);
 assert.equal(await evaluate('Array.from(document.querySelectorAll("[data-status-dot-named],#sd-fallback")).every(e=>{const s=getComputedStyle(e);return s.outlineStyle==="solid"&&s.outlineWidth==="1px"&&s.outlineOffset==="-1px"&&s.animationName==="none"&&s.opacity==="1"})'),true,'all builtins and custom fallback have approved forced-color boundary');await media({});
 await evaluate('document.documentElement.style.fontSize="32px";document.getElementById("sd-indicators").dir="rtl"');assert.equal(await evaluate('Array.from(document.querySelectorAll("[data-status-dot-named]")).every(e=>{const r=e.getBoundingClientRect();return r.width===8&&r.height===8})'),true,'field is fixed physical pixels, not typography size');assert.equal(await evaluate('(()=>{const row=document.querySelector("#sd-indicators > div"),a=row.children[0].getBoundingClientRect(),b=row.children[1].getBoundingClientRect();return a.x>b.x})()'),true);await evaluate('document.documentElement.style.fontSize="";document.getElementById("sd-indicators").removeAttribute("dir")');
 // Coarse-pointer bands preserve paint geometry and stop at neighbor midpoints.
 await send('Emulation.setTouchEmulationEnabled',{enabled:true,maxTouchPoints:1});await send('Emulation.setDeviceMetricsOverride',{width:720,height:1100,deviceScaleFactor:1,mobile:true});
 await waitFor('document.getElementById("sd-hit-a").dataset.astryxHitArea==="true"');
 const hit=await evaluate('(()=>{const e=document.getElementById("sd-hit-a"),s=getComputedStyle(e,"::after"),r=e.getBoundingClientRect();return{width:r.width,height:r.height,top:parseFloat(s.top),right:parseFloat(s.right),bottom:parseFloat(s.bottom),left:parseFloat(s.left)}})()');assert.equal(hit.width,8);assert.equal(hit.height,8);assert.ok(hit.top<=-18&&hit.left<=-18&&hit.right<=-18&&hit.bottom<=-18,JSON.stringify(hit));
 const hp=await point('sd-hit-a');await touch({x:hp.x-16,y:hp.y});await waitFor(pinned('sd-hit-a'));await touch({x:hp.x-16,y:hp.y});await waitFor(`!(${pinned('sd-hit-a')})`);
 await evaluate('document.getElementById("sd-hit-a").style.border="2px solid transparent"');await waitFor('document.getElementById("sd-hit-a").style.getPropertyValue("--_hit-border-left")==="2px"');
 assert.deepEqual(await evaluate('(()=>{const s=getComputedStyle(document.getElementById("sd-hit-a"),"::after");return [parseFloat(s.width),parseFloat(s.height)]})()'),[44,44],'caller border is compensated at the padding-box origin');
 await evaluate('document.getElementById("sd-hit-a").style.transform="translateX(40px)"');await waitFor('parseFloat(document.getElementById("sd-hit-a").style.getPropertyValue("--astryx-hit-inset-right"))<=4');
 await evaluate('document.getElementById("sd-hit-a").style.transform="translateX(56px)"');await waitFor('parseFloat(document.getElementById("sd-hit-a").style.getPropertyValue("--astryx-hit-inset-right"))===0');
 await evaluate('const e=document.getElementById("sd-hit-a");e.style.removeProperty("transform");e.style.removeProperty("border")');await waitFor('parseFloat(document.getElementById("sd-hit-a").style.getPropertyValue("--astryx-hit-inset-right"))===18');
 const cp=await point('sd-close-a');const close=await evaluate('(()=>{const a=document.getElementById("sd-close-a"),b=document.getElementById("sd-close-b"),x=a.getBoundingClientRect(),y=b.getBoundingClientRect(),sa=getComputedStyle(a,"::after"),sb=getComputedStyle(b,"::after");return{aEnd:x.right-parseFloat(sa.right),bStart:y.left+parseFloat(sb.left),gap:y.left-x.right}})()');assert.ok(close.aEnd<=close.bStart+.02,JSON.stringify(close));assert.equal(close.gap,8);await touch(cp);await waitFor(pinned('sd-close-a'));assert.equal(await evaluate(pinned('sd-close-b')),false,'nearby target never steals the press');
 const nestedPoint=await point('sd-in-button');await touch(nestedPoint);assert.equal(await evaluate(pinned('sd-in-button')),false);assert.equal(await evaluate(open('sd-in-button')),false,'nested status tooltip does not open on touch');
 assert.deepEqual(errors,[]);
 console.log('StatusDot browser checks passed: all 19 originals/light-dark native geometry/paint/ink/labels/pulse, hidden caller icons, safe custom fallback and themed override, native refs/events and caller semantics, standalone keyboard/mouse/touch preview and pin/dismiss/cancellation, nested hover-only and host AX naming, lazy external surface, dynamic placement/text/option/drop cleanup, universal forced-color outline, reduced motion, fixed field/RTL/text growth and bounded coarse-pointer hit areas.');
}finally{ws.close();await fetch(`${endpoint}/json/close/${target.id}`)}
