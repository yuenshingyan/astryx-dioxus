// Actual Dioxus nodes: no computed-DOM replay, clones or frozen supporting sizes.
import assert from 'node:assert/strict';
import {readFile, mkdir, writeFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {withTarget} from './card_metrics.mjs';
const published=JSON.parse(await readFile(new URL('./fixtures/stack-published.json',import.meta.url),'utf8'));
// Preserve stack-actual as the historical transformed capture; corrected captures are additive.
const evidenceDir=process.env.STACK_EVIDENCE_DIR
 ? pathToFileURL(resolve(process.env.STACK_EVIDENCE_DIR)+'/')
 : new URL('./fixtures/stack-raster/',import.meta.url);
const near=(a,b,label)=>assert.ok(Math.abs(a-b)<=1/64+1e-6,`${label}: ${a} != ${b}`);
await withTarget(async({send,evaluate,waitFor,pause,exceptions,browser})=>{
 await send('Network.enable');await send('Network.setBlockedURLs',{urls:['https://*']});
 await send('Page.navigate',{url:process.env.STACK_URL??'http://127.0.0.1:8097'});
 await waitFor(`document.querySelectorAll('#st-fill .astryx-avatar').length===3`);
 await evaluate(`document.fonts.load('500 12px Figtree').then(()=>document.fonts.load('700 14px Figtree')).then(()=>document.fonts.ready).then(()=>true)`);
 await send('Network.emulateNetworkConditions',{offline:true,latency:0,downloadThroughput:0,uploadThroughput:0});
 await evaluate(`(()=>{
  document.documentElement.style.cssText='font-size:16px;color-scheme:dark';
  const main=document.querySelector('main');main.dataset.astryxTheme='neutral';
  const stage=document.createElement('div');stage.id='actual-stage';stage.style.cssText='position:absolute;z-index:1;left:0;top:0;width:100%;display:flex;flex-direction:column;align-items:center;background:var(--color-background-card);color:var(--color-text-primary);font-family:var(--font-family-body);font-size:14px;line-height:1.4286;-webkit-font-smoothing:antialiased';main.prepend(stage);
  for(const id of ['st-directions','st-alignment','st-fill']){const e=document.getElementById(id);const host=document.createElement('div');host.style.cssText='display:inline-flex;background:var(--color-background-card)';stage.append(host);host.append(e)}
 })()`);
 await evaluate(`document.fonts.load('500 12px Figtree').then(()=>document.fonts.load('700 14px Figtree')).then(()=>document.fonts.ready).then(()=>true)`);
 const report={kind:'Actual rendered Rust nodes; intrinsic supporting sizing and paint; layout-positioned at original capture coordinates without transforms',browser:browser.Browser,viewport:[1200,1600,1],mode:'Neutral dark, LTR, root16',sourceCapture:published.capturedAt,cases:{}};
 await mkdir(evidenceDir,{recursive:true});
 for(const [name,id]of [['directions','st-directions'],['alignment','st-alignment'],['fill','st-fill']]){
  // Match original coordinates through layout, not transform: transforms preserve
  // geometry but rasterize already pixel-snapped child paint at a different phase.
  // The host surface is the original composited catalog background, not Stack ink.
  const origin=published.cases[name].clip;
  await evaluate(`(()=>{const host=document.getElementById('${id}').parentElement;host.style.position='absolute';host.style.left='${origin.x}px';host.style.top='${origin.y}px'})()`);
  const metrics=await evaluate(`(()=>{const root=document.getElementById('${id}'),rb=root.getBoundingClientRect();return {clip:{x:rb.x+scrollX,y:rb.y+scrollY,width:rb.width,height:rb.height,scale:1},nodes:[root,...root.querySelectorAll('*')].filter(e=>!e.matches('[role=tooltip]')).map(e=>{const b=e.getBoundingClientRect(),s=getComputedStyle(e);return{targets:[...e.classList].filter(c=>c.startsWith('astryx-')),box:[b.x-rb.x,b.y-rb.y,b.width,b.height],paint:[s.backgroundColor,s.color,s.borderRadius],border:[s.borderWidth,s.borderStyle,s.borderColor],font:[s.fontFamily,s.fontSize,s.fontWeight,s.lineHeight],role:e.getAttribute('role'),name:e.getAttribute('aria-label'),tabIndex:e.getAttribute('tabindex'),before:getComputedStyle(e,'::before').content}})}})()`);
  report.cases[name]=metrics;
  const owners=['astryx-stack','astryx-token','astryx-stack-item','astryx-avatar','astryx-avatar-fallback','astryx-card','astryx-button','astryx-text'];
  const selected=nodes=>nodes.filter(n=>n.targets.some(t=>owners.includes(t)));
  const actual=selected(metrics.nodes),original=selected(published.cases[name].nodes);
  assert.equal(actual.length,original.length,`${name} owner node count`);
  actual.forEach((n,i)=>{
   const o=original[i];n.box.forEach((v,j)=>near(v,o.box[j],`${name}/${i}/box${j}`));
   assert.deepEqual(n.paint,[o.style.backgroundColor,o.style.color,o.style.borderRadius],`${name}/${i} actual paint`);
   if(parseFloat(o.style.borderWidth)>0)assert.deepEqual(n.border,[o.style.borderWidth,o.style.borderStyle,o.style.borderColor],`${name}/${i} visible border`);
   assert.equal(n.font[1],o.style.fontSize,`${name}/${i} font size`);
   assert.equal(n.font[2],o.style.fontWeight,`${name}/${i} font weight`);
   assert.equal(n.font[3],o.style.lineHeight,`${name}/${i} line height`);
  });
  console.log(name,actual.length,'actual owner nodes match captured geometry/paint within 1/64px');
  const shot=await send('Page.captureScreenshot',{format:'png',captureBeyondViewport:true,clip:metrics.clip});const bytes=Buffer.from(shot.data,'base64');
  metrics.screenshot={path:`${name}.png`,sha256:createHash('sha256').update(bytes).digest('hex')};await writeFile(new URL(`${name}.png`,evidenceDir),bytes);
 }
 await writeFile(new URL('measurements.json',evidenceDir),JSON.stringify(report,null,2)+'\n');
 await evaluate(`(()=>{for(const id of ['st-directions','st-alignment','st-fill'])document.getElementById('stack-examples').append(document.getElementById(id));document.getElementById('actual-stage').remove()})()`);
 // Exact original Token usage is label-only: no invented interaction or tab stop.
 assert.equal(await evaluate(`document.querySelectorAll('#st-directions button,#st-directions a,#st-directions [tabindex]').length`),0);
 assert.equal(await evaluate(`document.querySelectorAll('#st-fill img').length`),0);
 for(const [i,name]of ['Olivia Chen','Marcus Rivera','Aisha Patel'].entries()){
  await evaluate(`window.avatar=document.querySelectorAll('#st-fill .astryx-avatar')[${i}];avatar.scrollIntoView({block:'center'});avatar.focus()`);
  // Keyboard Tab, not synthetic focus, establishes focus-visible tooltip interest.
  await send('Input.dispatchKeyEvent',{type:'keyDown',key:'Tab',code:'Tab',windowsVirtualKeyCode:9,modifiers:8});
  await send('Input.dispatchKeyEvent',{type:'keyUp',key:'Tab',code:'Tab',windowsVirtualKeyCode:9,modifiers:8});
  await send('Input.dispatchKeyEvent',{type:'keyDown',key:'Tab',code:'Tab',windowsVirtualKeyCode:9});
  await send('Input.dispatchKeyEvent',{type:'keyUp',key:'Tab',code:'Tab',windowsVirtualKeyCode:9});
  await waitFor(`!!document.querySelector('[role=tooltip]:popover-open')`);
  assert.equal(await evaluate(`document.activeElement===avatar&&avatar.matches(':focus-visible')`),true);
  assert.equal(await evaluate(`document.querySelector('[role=tooltip]:popover-open').getBoundingClientRect().bottom<=avatar.getBoundingClientRect().top`),true,'Tooltip above Avatar');
  assert.deepEqual(await evaluate(`[avatar.getAttribute('role'),avatar.getAttribute('aria-label'),avatar.getAttribute('tabindex'),avatar.hasAttribute('aria-describedby'),document.querySelector('[role=tooltip]:popover-open').textContent]`),['img',name,'0',false,name]);
  await send('Input.dispatchKeyEvent',{type:'keyDown',key:'Escape',code:'Escape',windowsVirtualKeyCode:27});
  await send('Input.dispatchKeyEvent',{type:'keyUp',key:'Escape',code:'Escape',windowsVirtualKeyCode:27});
  await waitFor(`!document.querySelector('[role=tooltip]:popover-open')`);
  const style=await evaluate(`(()=>{const s=getComputedStyle(avatar);return[s.outlineStyle,s.outlineWidth]})()`);assert.equal(style[0],'solid');assert.ok(parseFloat(style[1])>0);
 }
 // Native accessibility tree, mouse hover and hoverable tooltip persistence.
 await send('Accessibility.enable');
 const doc=await send('DOM.getDocument');const node=await send('DOM.querySelector',{nodeId:doc.root.nodeId,selector:'#st-fill .astryx-avatar'});
 const ax=(await send('Accessibility.getPartialAXTree',{nodeId:node.nodeId,fetchRelatives:false})).nodes[0];
 assert.equal(ax.role.value,'image');assert.equal(ax.name.value,'Olivia Chen');
 await evaluate(`document.activeElement.blur();window.avatar=document.querySelector('#st-fill .astryx-avatar');avatar.scrollIntoView({block:'center'})`);
 const point=await evaluate(`(()=>{const r=avatar.getBoundingClientRect();return{x:r.x+r.width/2,y:r.y+r.height/2}})()`);
 await send('Input.dispatchMouseEvent',{type:'mouseMoved',...point});
 await waitFor(`document.querySelector('[role=tooltip]:popover-open')?.textContent==='Olivia Chen'`);
 const tipPoint=await evaluate(`(()=>{const r=document.querySelector('[role=tooltip]:popover-open').getBoundingClientRect();return{x:r.x+r.width/2,y:r.y+r.height/2}})()`);
 await send('Input.dispatchMouseEvent',{type:'mouseMoved',...tipPoint});await pause(500);
 assert.equal(await evaluate(`document.querySelector('[role=tooltip]:popover-open')?.textContent`),'Olivia Chen');
 await send('Input.dispatchMouseEvent',{type:'mouseMoved',x:1190,y:10});
 await waitFor(`!document.querySelector('[role=tooltip]:popover-open')`);
 // Published actions are caller-owned demo messages; not a fake Toast API.
 for(const [selector,message,key]of [
  ['#st-fill button','Demo: this profile would open here','Enter'],
  ['#st-alignment button','Demo: your edits would be discarded here',' '],
  ['#st-alignment button:nth-of-type(2)','Demo: your changes would be saved here','Enter']]){
  await evaluate(`document.querySelector('${selector}').focus()`);
  const code=key===' '?'Space':'Enter',vk=key===' '?32:13;
  await send('Input.dispatchKeyEvent',{type:'keyDown',key,code,windowsVirtualKeyCode:vk,...(key==='Enter'?{text:'\r'}:{})});
  await send('Input.dispatchKeyEvent',{type:'keyUp',key,code,windowsVirtualKeyCode:vk});
  await waitFor(`document.getElementById('st-pattern-action').textContent===${JSON.stringify(message)}`);
 }
 // Pointer activation is observable too, not just keyboard/programmatic clicks.
 await evaluate(`document.querySelector('#st-fill button').scrollIntoView({block:'center'})`);
 const clickPoint=await evaluate(`(()=>{const r=document.querySelector('#st-fill button').getBoundingClientRect();return{x:r.x+r.width/2,y:r.y+r.height/2}})()`);
 const before=await evaluate(`Number(document.getElementById('st-pattern-action').dataset.count)`);
 for(const type of ['mousePressed','mouseReleased'])await send('Input.dispatchMouseEvent',{type,button:'left',clickCount:1,...clickPoint});
 await waitFor(`Number(document.getElementById('st-pattern-action').dataset.count)===${before+1}`);
 await evaluate(`window.avatar=document.querySelector('#st-fill .astryx-avatar');avatar.focus()`);
 for(const modifiers of [8,0])for(const type of ['keyDown','keyUp'])await send('Input.dispatchKeyEvent',{type,key:'Tab',code:'Tab',windowsVirtualKeyCode:9,modifiers});
 await waitFor(`!!document.querySelector('[role=tooltip]:popover-open')`);
 await evaluate(`document.getElementById('st-remove-patterns').click()`);
 await waitFor(`!document.getElementById('st-fill')&&!document.querySelector('[role=tooltip]:popover-open')`);
 assert.equal(await evaluate(`document.querySelectorAll('[id^="stack-avatar-"]').length`),0);
 assert.equal(exceptions.length,0,JSON.stringify(exceptions));
 console.log('Actual Stack patterns: keyboard tooltips and demo messages passed; screenshot and intrinsic geometry/paint evidence recorded.');
});
