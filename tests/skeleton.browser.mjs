// Node 22+, isolated Chrome CDP. Offline source evidence; no npm packages.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {skeletonMetrics,contextMetrics,expandBlock,selectSkeletonMetrics} from './skeleton_metrics.mjs';
const endpoint=process.env.CDP_URL??'http://127.0.0.1:9222';
const target=await(await fetch(`${endpoint}/json/new?about:blank`,{method:'PUT'})).json();
const socket=new WebSocket(target.webSocketDebuggerUrl);await new Promise((resolve,reject)=>{socket.onopen=resolve;socket.onerror=reject});
let sequence=0;const pending=new Map(),events=new Map(),errors=[];
socket.onmessage=({data})=>{const m=JSON.parse(data);events.get(m.method)?.(m.params);if(m.method==='Runtime.exceptionThrown')errors.push(m.params.exceptionDetails);const p=pending.get(m.id);if(p){pending.delete(m.id);clearTimeout(p.timer);m.error?p.reject(Error(JSON.stringify(m.error))):p.resolve(m.result)}};
const send=(method,params={})=>new Promise((resolve,reject)=>{const id=++sequence,timer=setTimeout(()=>{pending.delete(id);reject(Error(`CDP timeout ${method}`))},15000);pending.set(id,{resolve,reject,timer});socket.send(JSON.stringify({id,method,params}))});
const evaluate=async expression=>{const r=await send('Runtime.evaluate',{expression,returnByValue:true,awaitPromise:true});assert.equal(r.exceptionDetails,undefined,JSON.stringify(r.exceptionDetails));return r.result.value};
const wait=ms=>new Promise(resolve=>setTimeout(resolve,ms));
async function waitFor(expression){const end=Date.now()+15000;while(Date.now()<end){if(await evaluate(expression))return;await wait(25)}assert.fail(`Timed out: ${expression}`)}
const click=async id=>{await evaluate(`document.getElementById(${JSON.stringify(id)}).click()`);await wait(60)};
const fixture=JSON.parse(await readFile(new URL('./fixtures/skeleton-neutral-reference.json',import.meta.url),'utf8'));
const frameIds=['sk-showcase','sk-card','sk-list','sk-table'];
function compare(actual,expected,path='root'){
 if(typeof expected==='number'){assert.ok(Math.abs(actual-expected)<=1/64+1e-6,`${path}: ${actual} != ${expected}`);return}
 if(expected&&typeof expected==='object'){assert.deepEqual(Object.keys(actual).sort(),Object.keys(expected).sort(),`${path} keys`);for(const key of Object.keys(expected))compare(actual[key],expected[key],`${path}/${key}`);return}
 assert.equal(actual,expected,path);
}
function frameView(frame,style){
 const properties=['display','flexDirection','flexWrap','rowGap','columnGap','padding','borderTopWidth','borderRightWidth','borderBottomWidth','borderLeftWidth','borderRadius','backgroundColor','boxShadow','overflowX','overflowY'];
 // Native example frames are deliberately NOT public Card/Stack components.
 const selected=Object.fromEntries(properties.map(p=>[p,style[p]]));
 // Source Card's two transparent zero-sized shadows paint exactly like none.
 if(selected.boxShadow==='rgba(0, 0, 0, 0) 0px 0px 0px 0px, rgba(0, 0, 0, 0) 0px 0px 0px 0px')selected.boxShadow='none';
 return {box:frame.box,parent:frame.parent,children:frame.children,style:selected};
}
function sourceBackdrop(context,localIndex){
 let frame=context.frames.findIndex(f=>f.children.some(c=>c.block===localIndex));
 while(frame!==null&&frame>=0){const f=context.frames[frame],color=fixture.profiles.frameStyles[f.style].backgroundColor;if(color!=='rgba(0, 0, 0, 0)')return color;frame=f.parent}
 for(const f of context.presentation){const color=fixture.profiles.frameStyles[f.style].backgroundColor;if(color!=='rgba(0, 0, 0, 0)')return color}
 assert.fail('Source presentation backdrop missing');
}
const media=async features=>send('Emulation.setEmulatedMedia',{features:Object.entries({'prefers-reduced-motion':'no-preference','prefers-contrast':'no-preference','forced-colors':'none',...features}).map(([name,value])=>({name,value}))});
const settle=()=>evaluate('new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(()=>resolve(true))))');
const freeze=async()=>{await evaluate('Array.from(document.querySelectorAll("[data-skeleton-named]")).forEach(e=>e.getAnimations().forEach(a=>{a.pause();a.currentTime=0}))');await settle()};
try{
 await send('Page.enable');await send('Runtime.enable');await send('Accessibility.enable');await send('Emulation.setDeviceMetricsOverride',{width:1200,height:1600,deviceScaleFactor:1,mobile:false});await media({});
 await send('Page.navigate',{url:process.env.SHOWCASE_URL??'http://127.0.0.1:8080'});
 await waitFor('document.querySelectorAll("[data-skeleton-named]").length===33&&document.getElementById("sk-motion").getAnimations().length===1');await evaluate('document.fonts.ready.then(()=>true)');
 for(const mode of ['light','dark']){
  if(mode==='dark')await click('sk-theme');await freeze();
  const actual=await evaluate(`Array.from(document.querySelectorAll('[data-skeleton-named]')).map(e=>(${selectSkeletonMetrics.toString()})((${skeletonMetrics.toString()})(e,e.closest('#sk-showcase,#sk-card,#sk-list,#sk-table'))))`);
  assert.equal(actual.length,33);actual.forEach((value,index)=>compare(value,selectSkeletonMetrics(expandBlock(fixture,mode,index)),`${mode}/block/${index}`));
  const contexts=await evaluate(`${JSON.stringify(frameIds)}.map(id=>{const root=document.getElementById(id);return (${contextMetrics.toString()})(document.getElementById('skeleton-examples'),root,[...root.querySelectorAll('[data-skeleton-named]')])})`);
  contexts.forEach((context,index)=>compare(context.frames.map(f=>frameView(f,f.style)),fixture.cases[mode].contexts[index].frames.map(f=>frameView(f,fixture.profiles.frameStyles[f.style])),`${mode}/native frame/${index}`));
  const backdrops=await evaluate('Array.from(document.querySelectorAll("[data-skeleton-named]")).map(e=>{for(let p=e.parentElement;p;p=p.parentElement){const color=getComputedStyle(p).backgroundColor;if(color!=="rgba(0, 0, 0, 0)")return color}return null})');
  const sourceBackdrops=fixture.cases[mode].contexts.flatMap((context,index)=>Array.from({length:[6,6,5,16][index]},(_,i)=>sourceBackdrop(context,i)));assert.deepEqual(backdrops,sourceBackdrops,`${mode} actual compositing backdrops`);
 }
 await click('sk-theme');await freeze();
 // Sample both alternating legs away from ramps and at one real linear ramp.
 const samples=await evaluate(`(()=>{const e=document.querySelector('[data-skeleton-named="showcase/0"]'),a=e.getAnimations()[0],t=a.effect.getTiming(),out=[];for(let leg=0;leg<2;leg++)for(let step=0;step<10;step++)for(const within of [.02,.5,.98]){a.currentTime=t.delay+t.duration*(leg+(step+within)/10);out.push({leg,step,opacity:+getComputedStyle(e).opacity})}a.currentTime=t.delay+t.duration*.09995;const ramp=+getComputedStyle(e).opacity;a.currentTime=0;return {out,ramp}})()`);
 for(const {leg,step,opacity} of samples.out)assert.ok(Math.abs(opacity-(.25+.075*(leg?9-step:step)))<1e-6,`ten held steps ${leg}/${step}: ${opacity}`);
 assert.ok(Math.abs(samples.ramp-.2875)<1e-5,'0.01% ramp is linear, not a steps() timing function');
 assert.deepEqual(await evaluate('(()=>{const es=[...document.querySelectorAll("[data-skeleton-named]")].slice(0,2);return es.map(e=>{e.getAnimations()[0].currentTime=1050;return +getComputedStyle(e).opacity})})()'),[.325,.25],'index staggers the initial start, not each iteration');
 const radii=await evaluate('["none","0","1","2","3","4","rounded"].map(n=>getComputedStyle(document.getElementById("sk-radius-"+n)).borderRadius)');assert.deepEqual(radii,['0px','0px','6px','10px','12px','12px','9999px']);
 await evaluate('document.getElementById("skeleton-examples").style.setProperty("--radius-none","7px")');assert.deepEqual(await evaluate('["none","0"].map(n=>getComputedStyle(document.getElementById("sk-radius-"+n)).borderRadius)'),['0px','7px']);await evaluate('document.getElementById("skeleton-examples").style.removeProperty("--radius-none")');
 assert.deepEqual(await evaluate('["sk-default","sk-inherit","sk-css","sk-themed"].map(id=>{const r=document.getElementById(id).getBoundingClientRect();return [r.width,r.height]})'),[[137,63],[137,63],[168,17],[55,23]]);
 assert.equal(await evaluate('getComputedStyle(document.getElementById("sk-themed")).opacity'),'0.6');assert.equal(await evaluate('getComputedStyle(document.getElementById("sk-pretempo")).animationDuration'),'0.7s');
 await evaluate('document.querySelector("main").setAttribute("data-astryx-theme","core");document.getElementById("skeleton-examples").removeAttribute("data-astryx-theme")');
 assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("sk-motion"),s=getComputedStyle(e);return [s.backgroundColor,s.animationDuration,getComputedStyle(document.getElementById("sk-radius-1")).borderRadius,getComputedStyle(document.getElementById("sk-radius-2")).borderRadius]})()'),['rgb(204, 211, 219)','0.55s','4px','8px']);
 await evaluate('document.querySelector("main").setAttribute("data-astryx-theme","astryx");document.getElementById("skeleton-examples").setAttribute("data-astryx-theme","neutral")');
 assert.equal(await evaluate('document.querySelectorAll("#skeleton-examples .astryx-skeleton[data-radius],#skeleton-examples .astryx-skeleton[data-index],#skeleton-examples .astryx-skeleton[aria-live],#skeleton-examples .astryx-skeleton[tabindex]").length'),0);
 assert.equal(await evaluate('document[Symbol.for("astryx.dioxus.button.runtime")].records.has(document.getElementById("sk-native"))'),false);
 assert.equal(await evaluate('document.getElementById("sk-events").textContent'),'Native events: 1');await click('sk-native');assert.equal(await evaluate('document.getElementById("sk-events").textContent'),'Native events: 2');
 assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("sk-native");return [e.tagName,e.lang,e.dataset.testid,e.style.marginInline,e.classList.contains("caller-skeleton")]})()'),['DIV','fr','native-skeleton','7px',true]);
 const doc=await send('DOM.getDocument');const node=await send('DOM.querySelector',{nodeId:doc.root.nodeId,selector:'[data-skeleton-named]'});const ax=await send('Accessibility.getPartialAXTree',{nodeId:node.nodeId,fetchRelatives:false});assert.equal(ax.nodes[0].ignored,true,'default placeholder absent from AX');
 const tree=await send('Accessibility.getFullAXTree');assert.ok(tree.nodes.some(n=>!n.ignored&&n.role?.value==='image'&&n.name?.value==='Explicit loading preview'),'caller may expose and name shape');
 await evaluate('window.__skRoot=document.getElementById("sk-changing")');await click('sk-change');assert.equal(await evaluate('document.getElementById("sk-changing")===window.__skRoot'),true);assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("sk-changing"),s=getComputedStyle(e);return [e.getBoundingClientRect().width,s.borderRadius,s.animationDelay,e.getAnimations()[0].effect.getTiming().delay]})()'),[240,'9999px','1.5s',1500]);
 await click('sk-toggle');await waitFor('document.getElementById("sk-late")?.getAnimations().length===1');assert.equal(await evaluate('getComputedStyle(document.getElementById("sk-late")).animationDelay'),'1.2s');await evaluate('window.__skDropped=document.getElementById("sk-late")');await click('sk-toggle');assert.equal(await evaluate('!window.__skDropped.isConnected&&window.__skDropped.getAnimations().length===0'),true);
 await click('sk-finish');assert.equal(await evaluate('document.getElementById("sk-region").getAttribute("aria-busy")'),'false');assert.equal(await evaluate('document.getElementById("sk-owned")'),null);assert.equal(await evaluate('document.getElementById("sk-region").textContent'),'Profile content is ready.');
 // Media precedence is observed against actual system colors, never a captured
 // host-specific GrayText RGB substituted for Windows High Contrast testing.
 await media({'prefers-contrast':'more'});await freeze();
 assert.equal(await evaluate('getComputedStyle(document.querySelector("[data-skeleton-named]")).backgroundColor'),fixture.mediaObservations.light.increasedContrast.backgroundColor);
 await media({'prefers-reduced-motion':'reduce'});await settle();
 assert.equal(await evaluate('Array.from(document.querySelectorAll("[data-skeleton-named]")).every(e=>getComputedStyle(e).animationName==="none"&&e.getAnimations().length===0&&getComputedStyle(e).opacity==="0.25")'),true);
 await media({'forced-colors':'active','prefers-reduced-motion':'reduce','prefers-contrast':'more'});await settle();
 assert.equal(await evaluate('(()=>{const p=document.createElement("div");p.style.backgroundColor="GrayText";document.body.append(p);const color=getComputedStyle(p).backgroundColor;p.remove();return [...document.querySelectorAll("[data-skeleton-named]")].every(e=>getComputedStyle(e).backgroundColor===color&&getComputedStyle(e).opacity==="1"&&e.getAnimations().length===0)})()'),true,'forced colors win over increased contrast and remain visible at rest');
 await media({'forced-colors':'active'});await freeze();
 assert.equal(await evaluate('getComputedStyle(document.querySelector("[data-skeleton-named]")).opacity'),'1');
 assert.equal(await evaluate('(()=>{const e=document.querySelector("[data-skeleton-named]"),a=e.getAnimations()[0],t=a.effect.getTiming();a.currentTime=t.delay+t.duration*.55;return getComputedStyle(e).opacity})()'),'0.625','forced-color pulse remains active when motion allowed');
 await media({});await settle();
 // RTL does not invent a different sequence; native layouts carry direction.
 await evaluate('document.getElementById("skeleton-examples").dir="rtl"');
 assert.equal(await evaluate('(()=>{const es=[...document.querySelectorAll("#sk-table > div:first-child > .astryx-skeleton")];return es.every((e,i)=>!i||e.getBoundingClientRect().x<es[i-1].getBoundingClientRect().x)})()'),true);await evaluate('document.getElementById("skeleton-examples").removeAttribute("dir")');
 await evaluate('document.documentElement.style.fontSize="32px"');assert.deepEqual(await evaluate('(()=>{const e=document.querySelector("[data-skeleton-named=\\"showcase/1\\"]"),s=getComputedStyle(e);return [e.getBoundingClientRect().width,e.getBoundingClientRect().height,s.borderRadius]})()'),[160,20,'24px'],'pixel placeholders keep known content geometry while radius token scales');await evaluate('document.documentElement.style.fontSize=""');
 await send('Emulation.setDeviceMetricsOverride',{width:320,height:714,deviceScaleFactor:1,mobile:true});assert.deepEqual(await evaluate('["sk-default","sk-css"].map(id=>{const r=document.getElementById(id).getBoundingClientRect();return [r.width,r.height]})'),[[137,63],[168,17]],'known CSS/pixel geometry remains caller-owned at narrow widths');
 assert.deepEqual(await evaluate(`${JSON.stringify(frameIds)}.map(id=>{const r=document.getElementById(id).getBoundingClientRect();return [r.width,r.height]})`),[[300,144],[320,142],[320,112],[458,88]],'scroll stages retain original known content geometry instead of shrinking table columns');
 assert.equal(await evaluate('(()=>{const stage=document.getElementById("sk-table").parentElement;return stage.scrollWidth>stage.clientWidth&&getComputedStyle(stage).overflowX==="auto"})()'),true,'wide source samples have caller-owned horizontal scroll stages');
 await send('Emulation.setDeviceMetricsOverride',{width:1200,height:1600,deviceScaleFactor:1,mobile:false});
 // A single isolated visible block proves idle rendering and compositor paint,
 // rather than measuring unrelated Spinner/Button/gallery work.
 await evaluate('(()=>{const s=document.createElement("style");s.id="sk-isolation";s.textContent="main > :not(#skeleton-examples){display:none!important} #skeleton-examples > :not(:has(#sk-motion)){display:none!important}";document.head.append(s);document.getElementById("sk-motion").scrollIntoView({block:"center"})})()');await wait(1300);
 let traceDone;const traced=new Promise(resolve=>traceDone=resolve);events.set('Tracing.tracingComplete',traceDone);await send('Tracing.start',{categories:'devtools.timeline',transferMode:'ReturnAsStream'});await wait(1200);await send('Tracing.end');const trace=await traced;let text='';while(true){const chunk=await send('IO.read',{handle:trace.stream});text+=chunk.data;if(chunk.eof)break}await send('IO.close',{handle:trace.stream});events.delete('Tracing.tracingComplete');
 const timeline=JSON.parse(text).traceEvents;assert.deepEqual(timeline.filter(e=>e.name==='Paint'||e.name==='Layout').map(e=>e.name),[],'idle pulse has no paint/layout per frame');assert.ok(timeline.filter(e=>e.name==='UpdateLayoutTree').length<=2,'no per-frame style loop');
 const crop=await evaluate('(()=>{const r=document.getElementById("sk-motion").getBoundingClientRect();return {x:r.x,y:r.y,width:r.width,height:r.height}})()');const frames=[];events.set('Page.screencastFrame',e=>{frames.push({data:e.data,t:e.metadata.timestamp*1000});void send('Page.screencastFrameAck',{sessionId:e.sessionId})});
 await send('Page.startScreencast',{format:'png',everyNthFrame:1});await wait(200);const busy=await evaluate('(()=>{const start=Date.now();while(Date.now()-start<700){}return {start,end:Date.now()}})()');await wait(150);await send('Page.stopScreencast');events.delete('Page.screencastFrame');const during=frames.filter(f=>f.t>busy.start+30&&f.t<busy.end-30);assert.ok(during.length>=3,`independent compositor frames: ${during.length}`);
 const pixels=await evaluate(`(async()=>{const box=${JSON.stringify(crop)},items=${JSON.stringify(during)},out=[];for(const frame of items){const image=await createImageBitmap(await(await fetch('data:image/png;base64,'+frame.data)).blob());const canvas=new OffscreenCanvas(1,1),ctx=canvas.getContext('2d');ctx.drawImage(image,Math.floor(box.x+box.width/2),Math.floor(box.y+box.height/2),1,1,0,0,1,1);image.close();out.push([...ctx.getImageData(0,0,1,1).data].slice(0,3).join(','))}return out})()`);assert.ok(new Set(pixels).size>=4,`painted opacity must change during JS block, got ${new Set(pixels).size} values`);
 assert.deepEqual(errors,[]);
 console.log('Skeleton browser checks passed: 33 original blocks/light-dark geometry, paint, compositing backdrops and native frames; exact ten-step alternating pulse/ramp/stagger; all radii/default/CSS/keyword sizes; core/Neutral/pre-tempo/style precedence; native refs/events and AX exclusion/override; stable updates/drop and caller busy state; increased contrast/forced colors/reduced motion; RTL/root text/narrow layout; no idle paint/layout and compositor opacity through 700ms JS block.');
}finally{socket.close();await fetch(`${endpoint}/json/close/${target.id}`)}
