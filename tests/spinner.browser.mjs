// Node 22+ and an isolated Chrome CDP endpoint; no npm dependencies.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
const endpoint=process.env.CDP_URL??'http://127.0.0.1:9222';
const target=await(await fetch(`${endpoint}/json/new?about:blank`,{method:'PUT'})).json();
const socket=new WebSocket(target.webSocketDebuggerUrl);await new Promise((resolve,reject)=>{socket.onopen=resolve;socket.onerror=reject});
let serial=0;const pending=new Map(),events=new Map(),errors=[];
socket.onmessage=({data})=>{const m=JSON.parse(data);if(m.method==='Runtime.exceptionThrown')errors.push(m.params);events.get(m.method)?.(m.params);const p=pending.get(m.id);if(p){pending.delete(m.id);clearTimeout(p.timer);m.error?p.reject(Error(JSON.stringify(m.error))):p.resolve(m.result)}};
const send=(method,params={})=>new Promise((resolve,reject)=>{const id=++serial,timer=setTimeout(()=>{pending.delete(id);reject(Error(`CDP timeout: ${method}`))},20000);pending.set(id,{resolve,reject,timer});socket.send(JSON.stringify({id,method,params}))});
const evaluate=async expression=>{const result=await send('Runtime.evaluate',{expression,returnByValue:true,awaitPromise:true});assert.equal(result.exceptionDetails,undefined,JSON.stringify(result.exceptionDetails));return result.result.value};
const wait=ms=>new Promise(r=>setTimeout(r,ms));
async function waitFor(expression){const deadline=Date.now()+10000;while(Date.now()<deadline){if(await evaluate(expression))return;await wait(25)}assert.fail(`Timed out: ${expression}`)}
const click=async id=>{await evaluate(`document.getElementById(${JSON.stringify(id)}).click()`);await wait(50)};
const reference=JSON.parse(await readFile(new URL('./fixtures/spinner-neutral-reference.json',import.meta.url),'utf8'));
const metrics=e=>{const s=getComputedStyle(e),r=e.getBoundingClientRect(),ring=e.matches('[role=progressbar]')?e:e.querySelector('[role=progressbar]'),b=ring.getBoundingClientRect(),[track,arc]=e.querySelectorAll('circle'),t=getComputedStyle(track),a=getComputedStyle(arc),animation=arc.closest('svg').parentElement.getAnimations()[0];return {tag:e.tagName,size:e.dataset.size,shade:e.dataset.shade,text:e.textContent,display:s.display,gap:s.gap,width:r.width,height:r.height,ringWidth:b.width,ringHeight:b.height,arc:a.stroke,track:t.stroke,opacity:t.strokeOpacity,radius:a.r,stroke:a.strokeWidth,dash:a.strokeDasharray,linecap:a.strokeLinecap,period:Number(animation.effect.getTiming().duration)/100,texts:[...e.querySelectorAll('.astryx-text')].map(n=>{const c=getComputedStyle(n),b=n.getBoundingClientRect();return {text:n.textContent,font:c.fontFamily,size:c.fontSize,weight:c.fontWeight,leading:c.lineHeight,color:c.color,width:b.width,height:b.height}})};};
try {
 await send('Page.enable');await send('Runtime.enable');await send('Accessibility.enable');
 await send('Emulation.setDeviceMetricsOverride',{width:1200,height:1600,deviceScaleFactor:1,mobile:false});
 await send('Page.navigate',{url:process.env.SHOWCASE_URL??'http://127.0.0.1:8080'});
 await waitFor('Boolean(document.getElementById("sp-showcase"))&&document.querySelector("#sp-sm [data-astryx-spinner-rotor]").getAnimations()[0]?.startTime===0');
 await evaluate('document.fonts.ready.then(()=>true)');
 for(const mode of ['light','dark']) {
  if(mode==='dark')await click('sp-theme');
  const actual=await evaluate(`Array.from(document.querySelectorAll('[data-spinner-named]')).map(${metrics})`);
  assert.equal(actual.length,9);
  actual.forEach((value,index)=>{const expected=reference.measurements[mode][index];for(const key of Object.keys(expected)){
   if(key==='texts'){assert.equal(value.texts.length,expected.texts.length);value.texts.forEach((t,i)=>{for(const field of Object.keys(t)){if(field==='font')assert.equal(t[field].split(',')[0],expected.texts[i][field].split(',')[0]);else if(field==='width')assert.ok(Math.abs(t[field]-expected.texts[i][field])<=1/32,`${mode}/${index}/label width`);else assert.equal(t[field],expected.texts[i][field],`${mode}/${index}/label/${field}`)}})}
   else if(key==='opacity')assert.ok(Math.abs(Number(value[key])-Number(expected[key]))<0.0001,'SVG full precision and StyleX rounded alpha composite to the same byte');
   else if(key==='width')assert.ok(Math.abs(value[key]-expected[key])<=1/32,`${mode}/${index}/${key}`);
   else assert.equal(value[key],expected[key],`${mode}/${index}/${key}`);
  }});
 }
 await click('sp-theme');
 const tree=await send('Accessibility.getFullAXTree');
 const bars=tree.nodes.filter(n=>!n.ignored&&n.role?.value==='progressbar');
 for(const name of ['Loading','Loading...','Fetching data','Chargement','Imported task'])assert.ok(bars.some(n=>n.name?.value===name),`AX name ${name}`);
 assert.equal(await evaluate('document.querySelector("#sp-label [role=progressbar]").hasAttribute("aria-label")'),false);
 assert.equal(await evaluate('document.querySelector("#sp-rich [role=progressbar]").getAttribute("aria-label")'),'Fetching data');
 assert.equal(await evaluate('document.getElementById("sp-showcase").hasAttribute("aria-live")'),false);
 assert.equal(await evaluate('document.querySelectorAll("[data-astryx-spinner-id][tabindex]").length'),0);
 const geometry=await evaluate('(()=>{const e=document.getElementById("sp-no-shrink"),s=getComputedStyle(e),r=e.getBoundingClientRect();return [r.width,r.height,s.opacity]})()');
 assert.deepEqual(geometry,[36,36,'0.8'],'fixed sibling and min-width:0 cannot compress ring; own width wins');
 await evaluate('(()=>{const s=document.createElement("style");s.id="sp-theme-test";s.textContent="@layer astryx-theme {.spinner-themed {--spinner-diameter:40px;--spinner-stroke-width:6px;--spinner-arc-fraction:0.5;--spinner-color:rgb(180,0,20);--spinner-track-color:color-mix(in srgb, currentColor 50%, transparent);color:rgb(0,60,120)}}";document.head.append(s)})()');
 assert.deepEqual(await evaluate('(()=>{const e=document.querySelector("#sp-themed .astryx-spinner__ring"),a=e.querySelector(".astryx-spinner__arc"),c=getComputedStyle(a);return [e.getBoundingClientRect().width,e.getBoundingClientRect().height,c.r,c.strokeWidth,c.stroke,c.strokeDasharray]})()'),[52,52,'20px','6px','rgb(180, 0, 20)','62.8319px, 62.8319px']);
 assert.equal(await evaluate('document.querySelector("#sp-themed .astryx-spinner__ring").classList.contains("astryx-spinner")'),false,'inner label ring never redeclares theme defaults');
 await evaluate('document.getElementById("sp-themed").style.setProperty("--spinner-stroke-width","0")');
 assert.deepEqual(await evaluate('(()=>{const e=document.querySelector("#sp-themed .astryx-spinner__ring");return [e.getBoundingClientRect().width,getComputedStyle(e.querySelector("circle")).strokeWidth]})()'),[40,'0px']);
 await evaluate('document.getElementById("sp-themed").style.setProperty("--spinner-color","rgb(10, 90, 40)")');
 assert.equal(await evaluate('getComputedStyle(document.querySelector("#sp-themed .astryx-spinner__arc")).stroke'),'rgb(10, 90, 40)');
 const live='document.querySelector(\'[data-astryx-live-region="polite"]\')';
 await wait(2300);await click('sp-toggle');await waitFor('Boolean(document.getElementById("sp-late"))');
 await waitFor(`${live}?.textContent.includes("Fetching records")`);
 assert.equal(await evaluate('document.getElementById("sp-late").tagName'),'SPAN');
 await click('sp-toggle-label');await waitFor('document.getElementById("sp-late").tagName==="DIV"');
 assert.equal(await evaluate('document.querySelector("#sp-late [role=progressbar]").getAttribute("aria-label")'),null);
 await click('sp-update');await waitFor(`${live}?.textContent.includes("Refreshing records")`);
 await click('sp-extra-toggle');await waitFor(`${live}?.textContent.includes("Uploading files")`);
 await waitFor('document.querySelector("#sp-late-extra [data-astryx-spinner-rotor]").getAnimations()[0].startTime===0');
 assert.deepEqual(await evaluate('Array.from(document.querySelectorAll("#sp-sm [data-astryx-spinner-rotor],#sp-md [data-astryx-spinner-rotor],#sp-lg [data-astryx-spinner-rotor],#sp-xl [data-astryx-spinner-rotor],#sp-late [data-astryx-spinner-rotor],#sp-late-extra [data-astryx-spinner-rotor]")).map(e=>e.getAnimations()[0].startTime)'),[0,0,0,0,0,0],'late and original rings share exact origin');
 await click('sp-toggle');await click('sp-extra-toggle');await waitFor('!document.getElementById("sp-late")&&!document.getElementById("sp-late-extra")');
 await wait(2300);assert.equal(await evaluate(`${live}.textContent`),'');
 assert.deepEqual(await evaluate('(()=>{const s=document[Symbol.for("astryx.dioxus.spinner.rotors")];return [s.pending.size,s.frame]})()'),[0,null],'no retained mounted rings or idle frame loop');
 // Sample one full turn at exact animation times, then restore the shared origin.
 const samples=await evaluate('(()=>{const a=document.querySelector("#sp-xl [data-astryx-spinner-rotor]").getAnimations()[0],period=a.effect.getTiming().duration/100,e=a.effect.target; a.pause(); const out=Array.from({length:8},(_,i)=>{a.currentTime=period*i/8;return getComputedStyle(e).transform});a.play();a.startTime=0;return {period,easing:a.effect.getTiming().easing,iterations:String(a.effect.getTiming().iterations),out}})()');
 assert.equal(samples.period,525);assert.equal(samples.easing,'linear');assert.equal(samples.iterations,'Infinity');
 samples.out.forEach((matrix,i)=>{const [a,b]=matrix.slice(7,-1).split(',').map(Number);const angle=(Math.atan2(b,a)*180/Math.PI+360)%360;assert.ok(Math.abs(angle-i*45)<0.01,`clockwise sample ${i}`)});
 await send('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:'reduce'}]});
 assert.equal(await evaluate('document.querySelector("#sp-xl [data-astryx-spinner-rotor]").getAnimations()[0].effect.getTiming().duration/100'),3000);
 assert.equal(await evaluate('document.querySelector("#sp-xl [data-astryx-spinner-rotor]").getAnimations()[0].playState'),'running');
 await send('Emulation.setEmulatedMedia',{features:[]});
 // Idle tracing is done after announcement clears and with unrelated examples hidden.
 await evaluate('(()=>{const s=document.createElement("style");s.id="sp-motion-isolation";s.textContent="main > :not(#spinner-examples) {display:none!important} #spinner-examples > :not(:has(#sp-sm)) {display:none!important}";document.head.append(s);document.getElementById("sp-sm").scrollIntoView({block:"center"})})()');
 await wait(300);let traceDone;const traced=new Promise(r=>traceDone=r);events.set('Tracing.tracingComplete',traceDone);
 await send('Tracing.start',{categories:'devtools.timeline',transferMode:'ReturnAsStream'});await wait(1200);await send('Tracing.end');const trace=await traced;let traceText='';while(true){const chunk=await send('IO.read',{handle:trace.stream});traceText+=chunk.data;if(chunk.eof)break}await send('IO.close',{handle:trace.stream});events.delete('Tracing.tracingComplete');
 const traceEvents=JSON.parse(traceText).traceEvents;
 const paints=traceEvents.filter(e=>e.name==='Paint'||e.name==='Layout');assert.deepEqual(paints.map(e=>e.name),[],'rotor motion does not paint or lay out per frame');
 assert.ok(traceEvents.filter(e=>e.name==='UpdateLayoutTree').length<=2,'no per-frame style-recalculation loop');
 // CSS remains compositor-driven while JavaScript blocks the renderer.
 const crop=await evaluate('(()=>{const r=document.getElementById("sp-xl").getBoundingClientRect();return {x:r.x,y:r.y,width:r.width,height:r.height}})()');
 const frames=[];events.set('Page.screencastFrame',e=>{frames.push({data:e.data,t:e.metadata.timestamp*1000});void send('Page.screencastFrameAck',{sessionId:e.sessionId})});
 await send('Page.startScreencast',{format:'png',everyNthFrame:1});await wait(200);
 const busy=await evaluate('(()=>{const start=Date.now();while(Date.now()-start<700){}return {start,end:Date.now()}})()');await wait(150);await send('Page.stopScreencast');events.delete('Page.screencastFrame');
 const during=frames.filter(f=>f.t>busy.start+30&&f.t<busy.end-30);assert.ok(during.length>=3,`need independent compositor frames during block, got ${during.length}`);
 const angles=await evaluate(`(async()=>{const box=${JSON.stringify(crop)},items=${JSON.stringify(during)};const result=[];for(const frame of items){const image=await createImageBitmap(await(await fetch('data:image/png;base64,'+frame.data)).blob());const c=new OffscreenCanvas(Math.ceil(box.width),Math.ceil(box.height)),ctx=c.getContext('2d');ctx.drawImage(image,Math.floor(box.x),Math.floor(box.y),c.width,c.height,0,0,c.width,c.height);image.close();const {data}=ctx.getImageData(0,0,c.width,c.height);let xSum=0,ySum=0;for(let y=0;y<c.height;y++)for(let x=0;x<c.width;x++){const p=(y*c.width+x)*4,w=Math.abs(data[p]-data[0])+Math.abs(data[p+1]-data[1])+Math.abs(data[p+2]-data[2]),dx=x+.5-c.width/2,dy=y+.5-c.height/2,d=Math.hypot(dx,dy);if(d){xSum+=w*dx/d;ySum+=w*dy/d}}result.push(Math.atan2(ySum,xSum)*180/Math.PI)}return result})()`);
 let travel=0;for(let i=1;i<angles.length;i++)travel+=((angles[i]-angles[i-1]+540)%360)-180;
 assert.ok(travel>120,`painted arc must turn while JS blocks, measured ${travel} degrees in ${during.length} frames`);
 await evaluate('document.getElementById("sp-motion-isolation").remove()');
 await send('Emulation.setEmulatedMedia',{features:[{name:'forced-colors',value:'active'}]});
 assert.ok(await evaluate('getComputedStyle(document.querySelector("#sp-showcase .astryx-spinner__arc")).stroke!=="rgba(0, 0, 0, 0)"'),'forced colors retain visible ring');
 await send('Emulation.setEmulatedMedia',{features:[]});
 // Default presentation attributes survive the absence of the component stylesheet.
 await evaluate('Array.from(document.styleSheets).find(s=>s.ownerNode.textContent.includes("@property --_spinner-ring-diameter")).disabled=true');
 assert.deepEqual(await evaluate('(()=>{const e=document.querySelector("#sp-xl .astryx-spinner__rotor svg"),c=e.querySelector("circle");return [e.getAttribute("width"),e.getAttribute("height"),c.getAttribute("r"),c.getAttribute("stroke-width"),c.getAttribute("fill")]})()'),['36','36','14','4','none']);
 assert.deepEqual(errors,[]);
 console.log('Spinner browser checks passed: all nine upstream light/dark controls, naming/native roots, five theme variables, zero stroke, non-shrinking geometry, dynamic announcements/labels/drop, exact late-mount phase, constant clockwise motion, reduced motion, idle paint/layout trace, compositor motion while JS blocks, forced colors, and no-CSS geometry.');
} finally {socket.close();await fetch(`${endpoint}/json/close/${target.id}`)}
