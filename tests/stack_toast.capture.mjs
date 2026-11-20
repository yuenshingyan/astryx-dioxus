// Opt-in published observation. Run only under browser_lock.py; owns one CDP target.
import assert from 'node:assert/strict';
import {mkdir,writeFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {withTarget,cardMetrics} from './card_metrics.mjs';
const out=new URL('./fixtures/stack-toast/',import.meta.url);
await mkdir(out,{recursive:true});
await withTarget(async({send,evaluate,waitFor,pause,browser})=>{
 await send('Page.navigate',{url:'https://astryx.atmeta.com/components/Stack'});
 await waitFor(`!!document.querySelector('#example-StackAlignment button')`);
 await evaluate('document.fonts.ready.then(()=>true)');
 const identity=await evaluate(`({url:location.href,title:document.title,version:document.body.innerText.match(/@astryxdesign\\/core v[^ ·\\n]+/)?.[0]})`);
 assert.equal(identity.title,'Stack · Astryx');
 const report={...identity,capturedAt:new Date().toISOString(),browser:browser.Browser,viewport:[1200,1600,1],deploymentCommit:'unknown',sourcePin:'476a3153417dbf75c565f7df05b0784e94ddd59e',actions:[],interactions:{}};
 for(const [owner,label]of [['StackAlignment','Cancel'],['StackAlignment','Save'],['StackFillItem','View']]){
  await evaluate(`(()=>{const b=[...document.querySelectorAll('#example-${owner} button')].find(e=>e.textContent==='${label}');b.click()})()`);
  await waitFor(`!!document.querySelector('.astryx-toast')`);await pause(300);
  const metrics=await evaluate(`(${cardMetrics.toString()})(document.querySelector('.astryx-toast'),document.querySelector('.astryx-toast'))`);
  const extra=await evaluate(`(()=>{const e=document.querySelector('.astryx-toast'),r=e.getBoundingClientRect();return {body:e.textContent,clip:{x:r.x,y:r.y,width:r.width,height:r.height,scale:1},viewport:e.closest('[role=region]')?.outerHTML,rootTheme:document.documentElement.outerHTML.slice(0,500)}})()`);
  report.actions.push({owner,label,...extra,metrics});
  const shot=await send('Page.captureScreenshot',{format:'png',clip:extra.clip});const bytes=Buffer.from(shot.data,'base64');
  report.actions.at(-1).screenshot={path:label.toLowerCase()+'.png',sha256:createHash('sha256').update(bytes).digest('hex')};
  await writeFile(new URL(label.toLowerCase()+'.png',out),bytes);
  await evaluate(`document.querySelector('.astryx-toast button').click()`);await waitFor(`!document.querySelector('.astryx-toast')`);
 }
 // Compare real published keyboard routing, timing/hover and cap behavior too.
 await send('Page.bringToFront');
 await evaluate(`window.trigger=[...document.querySelectorAll('#example-StackAlignment button')].find(e=>e.textContent==='Cancel');trigger.focus()`);
 for(const type of ['keyDown','keyUp'])await send('Input.dispatchKeyEvent',{type,key:'Enter',code:'Enter',windowsVirtualKeyCode:13,...(type==='keyDown'?{text:'\r'}:{})});
 await waitFor(`!!document.querySelector('.astryx-toast')`);
 for(const type of ['keyDown','keyUp'])await send('Input.dispatchKeyEvent',{type,key:'F6',code:'F6',windowsVirtualKeyCode:117});
 report.interactions.f6DismissName=await evaluate(`document.activeElement.getAttribute('aria-label')`);
 for(const type of ['keyDown','keyUp'])await send('Input.dispatchKeyEvent',{type,key:'Enter',code:'Enter',windowsVirtualKeyCode:13,...(type==='keyDown'?{text:'\r'}:{})});
 await waitFor(`!document.querySelector('.astryx-toast')`);
 report.interactions.focusReturnsToCaller=await evaluate(`document.activeElement===trigger`);
 assert.equal(report.interactions.f6DismissName,'Dismiss notification');assert.equal(report.interactions.focusReturnsToCaller,true);
 await evaluate(`trigger.click()`);await waitFor(`!!document.querySelector('.astryx-toast')`);
 const point=await evaluate(`(()=>{const r=document.querySelector('.astryx-toast').getBoundingClientRect();return{x:r.x+20,y:r.y+20}})()`);
 await send('Input.dispatchMouseEvent',{type:'mouseMoved',...point});await pause(5300);
 report.interactions.hoverHeldBeyondFiveSeconds=await evaluate(`!!document.querySelector('.astryx-toast')`);
 assert.equal(report.interactions.hoverHeldBeyondFiveSeconds,true);
 await send('Input.dispatchMouseEvent',{type:'mouseMoved',x:10,y:10});
 await waitFor(`!document.querySelector('.astryx-toast')`);
 const started=Date.now();await evaluate(`trigger.click()`);await waitFor(`!!document.querySelector('.astryx-toast')`);
 await waitFor(`!document.querySelector('.astryx-toast')`);
 report.interactions.unheldAutoHideIncludingExitMs=Date.now()-started;
 assert.ok(report.interactions.unheldAutoHideIncludingExitMs>=4900&&report.interactions.unheldAutoHideIncludingExitMs<6000);
 await evaluate(`for(let i=0;i<6;i++)trigger.click()`);await pause(300);
 report.interactions.sixShowsVisibleCount=await evaluate(`document.querySelectorAll('.astryx-toast').length`);
 report.interactions.idsBefore=await evaluate(`[...document.querySelectorAll('[data-toast-id]')].map(e=>e.dataset.toastId)`);
 await evaluate(`document.querySelector('[data-toast-id]:last-child button').click()`);await pause(300);
 report.interactions.idsAfterNewestDismiss=await evaluate(`[...document.querySelectorAll('[data-toast-id]')].map(e=>e.dataset.toastId)`);
 assert.equal(report.interactions.sixShowsVisibleCount,5);assert.equal(report.interactions.idsAfterNewestDismiss.length,5);
 await evaluate(`document.querySelectorAll('.astryx-toast button').forEach(e=>e.click())`);await pause(300);
 await evaluate(`document.querySelectorAll('.astryx-toast button').forEach(e=>e.click())`);await waitFor(`!document.querySelector('.astryx-toast')`);
 for(const name of ['StackAlignment','StackFillItem']){
  await evaluate(`document.querySelector('#example-${name} [data-tab-value="code"]').click()`);await pause(250);
  report[name]=await evaluate(`document.querySelector('#example-${name}').innerText`);
 }
 await writeFile(new URL('published.json',out),JSON.stringify(report,null,2)+'\n');
 assert.equal(report.actions.length,3);console.log(report.actions.map(a=>[a.label,a.body,a.clip]));
});
