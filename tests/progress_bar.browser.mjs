// Node 22+, isolated Chrome CDP; all original-source evidence is offline.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {progressBarMetrics,nativeFrameMetrics,expandControl,expandContext} from './progress_bar_metrics.mjs';
const endpoint=process.env.CDP_URL??'http://127.0.0.1:9222';
const target=await(await fetch(`${endpoint}/json/new?about:blank`,{method:'PUT'})).json();
const ws=new WebSocket(target.webSocketDebuggerUrl);await new Promise((resolve,reject)=>{ws.onopen=resolve;ws.onerror=reject});
let sequence=0;const pending=new Map(),events=new Map(),errors=[];
ws.onmessage=({data})=>{const m=JSON.parse(data);events.get(m.method)?.(m.params);if(m.method==='Runtime.exceptionThrown')errors.push(m.params.exceptionDetails);const p=pending.get(m.id);if(p){pending.delete(m.id);clearTimeout(p.timer);m.error?p.reject(Error(JSON.stringify(m.error))):p.resolve(m.result)}};
const send=(method,params={})=>new Promise((resolve,reject)=>{const id=++sequence,timer=setTimeout(()=>{pending.delete(id);reject(Error(`CDP timeout ${method}`))},20000);pending.set(id,{resolve,reject,timer});ws.send(JSON.stringify({id,method,params}))});
const evaluate=async expression=>{const r=await send('Runtime.evaluate',{expression,returnByValue:true,awaitPromise:true});assert.equal(r.exceptionDetails,undefined,JSON.stringify(r.exceptionDetails));return r.result.value};
const wait=ms=>new Promise(resolve=>setTimeout(resolve,ms));
async function waitFor(expression){const end=Date.now()+20000;while(Date.now()<end){if(await evaluate(expression))return;await wait(30)}assert.fail(`Timed out: ${expression}`)}
const click=async id=>{await evaluate(`document.getElementById(${JSON.stringify(id)}).click()`);await wait(80)};
const media=features=>send('Emulation.setEmulatedMedia',{features:Object.entries({'prefers-reduced-motion':'no-preference','forced-colors':'none',...features}).map(([name,value])=>({name,value}))});
const key=async(key,code,windowsVirtualKeyCode)=>{await send('Input.dispatchKeyEvent',{type:'keyDown',key,code,windowsVirtualKeyCode});await send('Input.dispatchKeyEvent',{type:'keyUp',key,code,windowsVirtualKeyCode});await wait(90)};
const fixture=JSON.parse(await readFile(new URL('./fixtures/progress-bar-neutral-reference.json',import.meta.url),'utf8'));
const frames=['#pg-custom-format','[data-progress-bar-named="indeterminate"]','#pg-variants','[data-progress-bar-named="showcase"]','[data-progress-bar-named="value-label"]'];
function compare(a,b,path='root'){
 if(typeof b==='number'){assert.ok(Math.abs(a-b)<=1/64+1e-6,`${path}: ${a} != ${b}`);return}
 if(b&&typeof b==='object'){assert.deepEqual(Object.keys(a).sort(),Object.keys(b).sort(),`${path} keys`);for(const k of Object.keys(b))compare(a[k],b[k],`${path}/${k}`);return}assert.equal(a,b,path);
}
const partNames=['root','header','label','value','trackContainer','track','fill'];
function view(c){
 const parts=Object.fromEntries(partNames.map(name=>{const p=c[name];if(!p)return[name,null];const style={};
  for(const k of ['display','position','flexDirection','alignItems','justifyContent','rowGap','columnGap','padding','margin','backgroundColor','color','opacity','borderRadius','overflow','direction','fontFamily','fontSize','fontWeight','lineHeight','letterSpacing','fontVariantNumeric','whiteSpace'])style[k]=p.style[k];
  if(name==='fill'||name==='track')style.boxSizing=p.style.boxSizing;
  const attributes={};for(const k of ['role','tabindex','aria-valuenow','aria-valuemin','aria-valuemax','aria-valuetext'])if(k in p.attributes)attributes[k]=p.attributes[k];
  return[name,{tag:p.tag,text:p.text,box:p.box,textRects:p.textRects,style,attributes}];}));
 return {...parts,geometry:c.geometry,tokens:Object.fromEntries(['--color-background-muted','--color-accent','--color-success','--color-warning','--color-error','--color-text-primary','--color-text-secondary','--color-text-disabled','--color-on-accent','--spacing-1','--radius-full','--duration-medium','--ease-standard'].map(k=>[k,c.tokens[k].replace(/\s+/g,'').replace(/#([0-9a-f]{3})(?![0-9a-f])/gi,(_,s)=>'#'+[...s].map(c=>c+c).join(''))])),backdrop:c.paintAncestry.find(p=>p.backgroundColor!=='rgba(0, 0, 0, 0)')?.backgroundColor};
}
function frameView(f){const style={};for(const k of ['display','flexDirection','rowGap','columnGap','padding','margin','backgroundColor'])style[k]=f.style[k];if(f.tag==='span')for(const k of ['fontSize','fontWeight','lineHeight','color'])style[k]=f.style[k];return{tag:f.tag,box:f.box,textRects:f.textRects,children:f.children,style};}
try{
 await send('Page.enable');await send('Runtime.enable');await send('Accessibility.enable');await send('Page.bringToFront');await send('Emulation.setDeviceMetricsOverride',{width:1200,height:1600,deviceScaleFactor:1,mobile:false});await media({});
 await send('Page.navigate',{url:process.env.SHOWCASE_URL??'http://127.0.0.1:8080'});
 await waitFor('document.querySelectorAll("[data-progress-bar-named]").length===9&&document.getElementById("pg-changing").dataset.astryxProgressBarMotion==="active"');
 await evaluate('document.fonts.ready.then(()=>true)');
 await waitFor('document[Symbol.for("astryx.dioxus.button.runtime")]?.records.has(document.querySelector("#pg-changing .astryx-progress-bar-mark"))');
 for(const mode of ['light','dark']){
  if(mode==='dark')await click('pg-theme');
  await evaluate('Array.from(document.querySelectorAll("[data-progress-bar-named] .astryx-progress-bar-fill")).forEach(e=>e.getAnimations().forEach(a=>{a.pause();a.currentTime=750}))');await wait(50);
  const controls=await evaluate(`(()=>{const selectors=${JSON.stringify(frames)};return selectors.flatMap(s=>{const f=document.querySelector(s),roots=f.matches('.astryx-progress-bar')?[f]:[...f.querySelectorAll('.astryx-progress-bar')];return roots.map(e=>(${progressBarMetrics.toString()})(e,f))})})()`);
  controls.forEach((c,i)=>compare(view(c),view(expandControl(fixture,mode,i)),`${mode}/original/${i}`));
  const contexts=await evaluate(`${JSON.stringify(frames)}.map(s=>{const f=document.querySelector(s);return (${nativeFrameMetrics.toString()})(f,f.matches('.astryx-progress-bar')?[f]:[...f.querySelectorAll('.astryx-progress-bar')])})`);
  contexts.forEach((c,i)=>compare(c.frames.map(frameView),expandContext(fixture,mode,i).frames.map(frameView),`${mode}/native frame/${i}`));
  assert.equal(controls.length,9);for(const c of controls){assert.equal(c.track.style.boxShadow,c.fill.style.boxShadow);assert.notEqual(c.fill.style.boxShadow,'none','approved complete/remaining inset boundaries');}
 }
 await click('pg-theme');
 assert.equal(await evaluate('document.getElementById("pg-events").textContent'),'Native events: 2');await click('pg-native');assert.equal(await evaluate('document.getElementById("pg-events").textContent'),'Native events: 3');
 assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("pg-native");return [e.tagName,e.lang,e.dataset.testid,e.classList.contains("caller-progress"),e.style.marginInline,e.dataset.variant,e.getAttribute("role")]})()'),['DIV','fr','native-progress',true,'7px','accent','group']);
 const ax=async selector=>{const doc=await send('DOM.getDocument'),node=await send('DOM.querySelector',{nodeId:doc.root.nodeId,selector});return(await send('Accessibility.getPartialAXTree',{nodeId:node.nodeId,fetchRelatives:false})).nodes[0]};
 const known=await ax('#pg-changing [role=progressbar]');assert.equal(known.role.value,'progressbar');assert.equal(known.name.value,'Controlled progress');assert.equal(known.value.value,10);
 assert.equal((await ax('#pg-hidden [role=progressbar]')).name.value,'Accessible hidden label');
 await click('pg-to-90');
 const moving=await evaluate('(()=>{const f=document.querySelector("#pg-changing .astryx-progress-bar-fill"),a=f.getAnimations()[0];a.pause();a.currentTime=150;return {duration:a.effect.getTiming().duration,easing:a.effect.getTiming().easing,x:new DOMMatrix(getComputedStyle(f).transform).m41,value:f.parentElement.getAttribute("aria-valuenow")}})()');
 assert.equal(moving.duration,300);assert.equal(moving.easing,'cubic-bezier(0.24, 1, 0.4, 1)');assert.ok(moving.x>-270&&moving.x< -30);assert.equal(moving.value,'90');
 await click('pg-to-10');const reverse=await evaluate('(()=>{const a=document.querySelector("#pg-changing .astryx-progress-bar-fill").getAnimations()[0];return{duration:a.effect.getTiming().duration,from:new DOMMatrix(a.effect.getKeyframes()[0].transform).m41}})()');assert.equal(reverse.duration,300);assert.ok(Math.abs(reverse.from-moving.x)<1/64,'retarget from paused drawn position, without CSS reversal shortening');await wait(350);
 await evaluate('window.stableMark=document.querySelectorAll("#pg-changing .astryx-progress-bar-mark")[2];stableMark.focus()');await click('pg-range');assert.equal(await evaluate('document.activeElement===stableMark&&stableMark.dataset.astryxMarkPercent==="40"'),true,'range updates preserve focused target identity');await click('pg-reorder');assert.deepEqual(await evaluate('[document.activeElement===stableMark,document.querySelectorAll("#pg-changing .astryx-progress-bar-mark")[0]===stableMark,stableMark.isConnected]'),[true,true,true],'target reorder preserves focus and native identity');await click('pg-reorder');await click('pg-range');assert.equal(await evaluate('stableMark.blur();Promise.resolve().then(()=>document.activeElement!==stableMark)'),true,'deliberate blur is not restored');await evaluate('stableMark.focus();document.getElementById("pg-to-90").focus()');await wait(30);assert.equal(await evaluate('document.activeElement.id'),'pg-to-90','explicit focus transfer is not stolen');
 const unknown=await ax('[data-progress-bar-named=indeterminate] [role=progressbar]');assert.equal(unknown.name.value,'Loading...');assert.equal(unknown.value,undefined);
 assert.equal(await evaluate('document.querySelectorAll("#pg-changing [role=progressbar] .astryx-progress-bar-mark").length'),0);
 assert.equal((await ax('#pg-changing .astryx-progress-bar-mark')).role.value,'image');
 assert.deepEqual(await evaluate('["pg-nonfinite","pg-negative-max"].map(id=>{const e=document.querySelector("#"+id+" [role=progressbar]");return[e.getAttribute("aria-valuenow"),e.getAttribute("aria-valuemax"),e.getAttribute("aria-valuetext")]})'),[['0','0','0%'],['0','0','0%']]);
 assert.equal(await evaluate('getComputedStyle(document.querySelector("#pg-fallback .astryx-progress-bar-fill")).backgroundColor'),await evaluate('getComputedStyle(document.querySelector("[data-progress-bar-named=showcase] .astryx-progress-bar-fill")).backgroundColor'));
 assert.equal(await evaluate('getComputedStyle(document.querySelector("#pg-custom .astryx-progress-bar-fill")).backgroundColor'),'rgb(100, 40, 160)');
 await click('pg-locale');assert.equal(await evaluate('document.querySelector("#pg-localized [role=progressbar]").getAttribute("aria-valuetext")'),'42\u00a0٪');
 await click('pg-mode');assert.deepEqual(await evaluate('(()=>{const r=document.getElementById("pg-mode-changing"),t=r.querySelector("[role=progressbar]");return[r.dataset.mode,t.hasAttribute("aria-valuenow"),r.querySelectorAll(".astryx-progress-bar-mark").length,r.querySelectorAll(".astryx-progress-bar__value").length]})()'),['indeterminate',false,0,0]);await click('pg-mode');
 // Keyboard focus always opens the source mark tooltip, even after pointer use.
 await evaluate('window.pgMark=document.querySelector("#pg-changing .astryx-progress-bar-mark");pgMark.scrollIntoView({block:"center"});pgMark.focus()');
 await waitFor('!!document.querySelector(".astryx-tooltip:popover-open")');
 assert.equal(await evaluate('document.querySelector(".astryx-tooltip:popover-open").textContent'),'Start');
 assert.equal(await evaluate('pgMark.getAttribute("aria-describedby")'),null,'identical marker name is not repeated');
 await key('Escape','Escape',27);await waitFor('!document.querySelector(".astryx-tooltip:popover-open")');
 await evaluate('pgMark.blur();pgMark.click()');await waitFor('!!document.querySelector(".astryx-tooltip:popover-open")');
 await evaluate('pgMark.blur();document.body.click()');await wait(150);
 const markPoint=await evaluate('(()=>{pgMark.scrollIntoView({block:"center"});const r=pgMark.getBoundingClientRect();return{x:r.x+r.width/2,y:r.y+r.height/2}})()');
 await send('Input.dispatchMouseEvent',{type:'mouseMoved',x:1,y:1});await wait(200);await send('Input.dispatchMouseEvent',{type:'mouseMoved',...markPoint});await waitFor('!!document.querySelector(".astryx-tooltip:popover-open")');
 await send('Input.dispatchMouseEvent',{type:'mouseMoved',x:1,y:1});await waitFor('!document.querySelector(".astryx-tooltip:popover-open")');
 await send('Emulation.setTouchEmulationEnabled',{enabled:true,maxTouchPoints:1});await send('Input.dispatchTouchEvent',{type:'touchStart',touchPoints:[{...markPoint,id:1,radiusX:2,radiusY:2}]});await send('Input.dispatchTouchEvent',{type:'touchEnd',touchPoints:[]});await waitFor('!!document.querySelector(".astryx-tooltip:popover-open")');await key('Escape','Escape',27);await send('Emulation.setTouchEmulationEnabled',{enabled:false,maxTouchPoints:1});
 await evaluate('window.modeMark=document.querySelector("#pg-mode-changing .astryx-progress-bar-mark");modeMark.scrollIntoView({block:"center"});modeMark.focus()');await waitFor('document.querySelector(".astryx-tooltip:popover-open")?.textContent==="Goal"');const removedTip=await evaluate('document.querySelector(".astryx-tooltip:popover-open").id');await click('pg-mode');await waitFor('!document[Symbol.for("astryx.dioxus.button.runtime")].records.has(modeMark)');assert.equal(await evaluate(`document.getElementById(${JSON.stringify(removedTip)})`),null,'mark removal destroys its tooltip');await click('pg-mode');
 await media({'prefers-reduced-motion':'reduce'});await wait(80);
 assert.deepEqual(await evaluate('["pg-ind-ltr-in-rtl","pg-ind-rtl","pg-tempo"].map(id=>{const e=document.querySelector("#"+id+" .astryx-progress-bar-fill"),a=e.getAnimations()[0],s=getComputedStyle(e);return[s.width,s.transform,a.effect.getTiming().duration,a.effect.getTiming().direction,a.effect.getKeyframes().map(k=>k.opacity)]})'),[['300px','none',2000,'alternate',['1','0.84']],['300px','none',2000,'alternate',['1','0.84']],['300px','none',2000,'alternate',['1','0.84']]]);
 await click('pg-to-90');assert.equal(await evaluate('document.querySelector("#pg-changing [role=progressbar]").getAttribute("aria-valuenow")'),'90');assert.equal(await evaluate('document.querySelector("#pg-changing .astryx-progress-bar-fill").getAnimations().length'),0);
 await send('Emulation.setTouchEmulationEnabled',{enabled:true,maxTouchPoints:1});await click('pg-to-one');await wait(80);
 const small=await evaluate('(()=>{const r=document.getElementById("pg-narrow"),f=r.querySelector(".astryx-progress-bar-fill"),t=r.querySelector(".astryx-progress-bar-track"),a=f.getBoundingClientRect(),b=t.getBoundingClientRect();return[b.width,a.right-b.left,[...r.querySelectorAll(".astryx-progress-bar-mark")].map(e=>e.dataset.placement)]})()');compare(small,[48,16,['fill','track']],'coarse minimum and tick classification');
 await click('pg-to-zero');assert.equal(await evaluate('getComputedStyle(document.querySelector("#pg-narrow .astryx-progress-bar-fill")).opacity'),'0');
 await media({'forced-colors':'active'});await click('pg-to-90');
 assert.deepEqual(await evaluate('["pg-changing","pg-rtl","pg-ltr-in-rtl"].map(id=>{const f=document.querySelector("#"+id+" .astryx-progress-bar-fill"),s=getComputedStyle(f);return[s.transform,s.outlineStyle,s.outlineWidth,s.outlineOffset]})'),[['none','solid','1px','-1px'],['none','solid','1px','-1px'],['none','solid','1px','-1px']]);
 await media({});await send('Emulation.setTouchEmulationEnabled',{enabled:false,maxTouchPoints:1});
 const before=await evaluate('document[Symbol.for("astryx.dioxus.button.runtime")].records.size');await click('pg-toggle');assert.equal(await evaluate('document.getElementById("pg-drop")'),null);await click('pg-toggle');await waitFor('document.getElementById("pg-drop")?.dataset.astryxProgressBarMotion==="active"');
 assert.equal(await evaluate('document[Symbol.for("astryx.dioxus.button.runtime")].records.size'),before);
 // Isolate an actual mounted bar for idle trace and blocked-main-thread pixels.
 await evaluate('(()=>{const s=document.createElement("style");s.textContent="main > :not(#progress-bar-examples){display:none!important} #progress-bar-examples > :not(#pg-indeterminate){display:none!important}";document.head.append(s);document.getElementById("pg-indeterminate").scrollIntoView({block:"center"});document.querySelector("#pg-indeterminate .astryx-progress-bar-fill").getAnimations().forEach(a=>a.play())})()');
 for(const reduced of [false,true]){
  await media(reduced?{'prefers-reduced-motion':'reduce'}:{});await wait(1300);
  let done;const complete=new Promise(resolve=>done=resolve);events.set('Tracing.tracingComplete',done);await send('Tracing.start',{categories:'devtools.timeline',transferMode:'ReturnAsStream'});await wait(1000);await send('Tracing.end');const trace=await complete;let text='';while(true){const c=await send('IO.read',{handle:trace.stream});text+=c.data;if(c.eof)break}await send('IO.close',{handle:trace.stream});events.delete('Tracing.tracingComplete');const timeline=JSON.parse(text).traceEvents;assert.deepEqual(timeline.filter(e=>e.name==='Paint'||e.name==='Layout').map(e=>e.name),[],`idle ${reduced?'pulse':'slide'} has no per-frame paint/layout`);assert.ok(timeline.filter(e=>e.name==='UpdateLayoutTree').length<=2);
  const box=await evaluate('(()=>{const r=document.querySelector("#pg-indeterminate .astryx-progress-bar-track").getBoundingClientRect();return{x:r.x,y:r.y,width:r.width,height:r.height}})()'),shots=[];events.set('Page.screencastFrame',p=>{shots.push({t:p.metadata.timestamp*1000,data:p.data});send('Page.screencastFrameAck',{sessionId:p.sessionId}).catch(()=>{})});await send('Page.startScreencast',{format:'png',everyNthFrame:1});await wait(200);const busy=await evaluate('(()=>{const start=Date.now();while(Date.now()-start<700){}return{start,end:Date.now()}})()');await wait(150);await send('Page.stopScreencast');events.delete('Page.screencastFrame');const during=shots.filter(f=>f.t>busy.start+30&&f.t<busy.end-30);assert.ok(during.length>=3,`independent compositor frames ${during.length}`);
  const pixels=await evaluate(`(async()=>{const box=${JSON.stringify(box)},items=${JSON.stringify(during)},out=[];for(const f of items){const image=await createImageBitmap(await(await fetch('data:image/png;base64,'+f.data)).blob()),canvas=new OffscreenCanvas(Math.round(box.width),1),c=canvas.getContext('2d');c.drawImage(image,Math.floor(box.x),Math.floor(box.y+box.height/2),Math.round(box.width),1,0,0,Math.round(box.width),1);image.close();out.push([...c.getImageData(0,0,Math.round(box.width),1).data].join(','))}return out})()`);assert.ok(new Set(pixels).size>=2,`${reduced?'pulse':'slide'} pixels change during main-thread blocking`);
 }
 assert.equal(errors.length,0,JSON.stringify(errors));console.log('ProgressBar: 18 source appearance observations, five native patterns, names/ranges/marks, fallback, native props, tooltip, media and lifetime passed.');
}finally{ws.close();await fetch(`${endpoint}/json/close/${target.id}`);for(let i=0;i<50;i++){if(!(await(await fetch(`${endpoint}/json/list`)).json()).some(t=>t.id===target.id))break;await wait(100)}assert.equal((await(await fetch(`${endpoint}/json/list`)).json()).some(t=>t.id===target.id),false)}
