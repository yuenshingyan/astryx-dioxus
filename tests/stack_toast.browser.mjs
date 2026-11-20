// Actual open private Toasts, not replay frames. Serialized isolated CDP target.
import assert from 'node:assert/strict';
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {withTarget,cardMetrics} from './card_metrics.mjs';
const published=JSON.parse(await readFile(new URL('./fixtures/stack-toast/published.json',import.meta.url),'utf8'));
const out=process.env.TOAST_EVIDENCE_DIR??'/tmp/astryx-parity/toast-actual';
await mkdir(out,{recursive:true});
const near=(a,b,label)=>assert.ok(Math.abs(a-b)<=1/64+1e-6,`${label}: ${a} != ${b}`);
await withTarget(async({send,evaluate,waitFor,pause,exceptions,browser,target})=>{
 await send('Network.enable');await send('Network.setBlockedURLs',{urls:['https://*']});
 await send('Page.navigate',{url:process.env.STACK_URL??'http://127.0.0.1:8098'});
 await send('Page.bringToFront');
 await waitFor(`!!document.getElementById('st-alignment')`);
 await evaluate(`document.fonts.load('400 14px Figtree').then(()=>document.fonts.ready).then(()=>true)`);
 await evaluate(`document.documentElement.style.colorScheme='dark';document.querySelector('main').dataset.astryxTheme='neutral';document.querySelector('main').style.color='var(--color-text-primary)'`);
 await send('Network.emulateNetworkConditions',{offline:true,latency:0,downloadThroughput:0,uploadThroughput:0});
 const report={kind:'Actual Rust-rendered Toast nodes, published comparison',browser:browser.Browser,publishedAt:published.capturedAt,viewport:[1200,1600,1],cases:[],tests:[]};
 const click=async selector=>{
  await evaluate(`document.querySelector(${JSON.stringify(selector)}).scrollIntoView({block:'center'})`);
  const point=await evaluate(`(()=>{const r=document.querySelector(${JSON.stringify(selector)}).getBoundingClientRect();return{x:r.x+r.width/2,y:r.y+r.height/2}})()`);
  for(const type of ['mousePressed','mouseReleased'])await send('Input.dispatchMouseEvent',{type,button:'left',clickCount:1,...point});
 };
 const key=async(key,code,vk)=>{for(const type of ['keyDown','keyUp'])await send('Input.dispatchKeyEvent',{type,key,code,windowsVirtualKeyCode:vk,...(key==='Enter'&&type==='keyDown'?{text:'\r'}:{})});};
 const drain=async()=>{
  await evaluate(`document.querySelectorAll('.astryx-toast button').forEach(b=>b.click())`);
  await waitFor(`!document.querySelector('.astryx-toast')`);
 };
 for(const [i,selector]of ['#st-alignment button','#st-alignment button:nth-of-type(2)','#st-fill button'].entries()){
  await click(selector);await waitFor(`document.querySelectorAll('.astryx-toast').length===1`);await pause(300);
  const actual=await evaluate(`(${cardMetrics.toString()})(document.querySelector('.astryx-toast'),document.querySelector('.astryx-toast'))`);
  const original=published.actions[i].metrics;
  const selectors=['.astryx-toast','.astryx-demo-toast-inner','.astryx-demo-toast-body','.astryx-demo-toast-end','.astryx-toast-close','.astryx-icon','svg'];
  const originalIndices=[0,2,3,4,5,8,9];
  const measured=await evaluate(`(()=>{const root=document.querySelector('.astryx-toast'),r=root.getBoundingClientRect();return ${JSON.stringify(selectors)}.map(sel=>{const e=sel==='.astryx-toast'?root:root.querySelector(sel),b=e.getBoundingClientRect(),s=getComputedStyle(e);return{selector:sel,box:[b.x-r.x,b.y-r.y,b.width,b.height],paint:[s.backgroundColor,s.color,s.borderRadius,s.boxShadow],font:[s.fontSize,s.fontWeight,s.lineHeight]}})})()`);
  measured.forEach((node,index)=>{
   const reference=original.nodes[originalIndices[index]];
   node.box.forEach((v,j)=>near(v,reference.box[j],`${i}/${node.selector}/box${j}`));
   assert.deepEqual(node.paint,[reference.style.backgroundColor,reference.style.color,reference.style.borderRadius,reference.style.boxShadow],`${i}/${node.selector}/paint`);
   assert.deepEqual(node.font,[reference.style.fontSize,reference.style.fontWeight,reference.style.lineHeight],`${i}/${node.selector}/font`);
  });
  assert.equal(await evaluate(`document.querySelector('.astryx-demo-toast-body').textContent`),published.actions[i].body);
  assert.deepEqual(await evaluate(`(()=>{const r=document.querySelector('.astryx-toast').getBoundingClientRect();return [innerWidth-r.right,innerHeight-r.bottom,r.width,r.height]})()`),[16,16,400,52]);
  assert.equal(await evaluate(`document.querySelector('.astryx-toast').hasAttribute('role')`),false,'viewport is sole announcement owner');
  assert.equal(await evaluate(`document.querySelector('.astryx-demo-toast-media').dataset.astryxMedia`),'light');
  assert.deepEqual(await evaluate(`(()=>{const e=document.querySelector('.astryx-demo-toast-viewport');return[e.getAttribute('role'),e.getAttribute('aria-label'),e.matches(':popover-open')]})()`),['region','Notifications',true]);
  assert.equal(await evaluate(`document.querySelector('[data-astryx-live-region]').textContent`),published.actions[i].body);
  const clip=await evaluate(`(()=>{const r=document.querySelector('.astryx-toast').getBoundingClientRect();return{x:r.x,y:r.y,width:r.width,height:r.height,scale:1}})()`);
  const shot=await send('Page.captureScreenshot',{format:'png',clip});await writeFile(`${out}/${published.actions[i].label.toLowerCase()}.png`,Buffer.from(shot.data,'base64'));
  report.cases.push({action:published.actions[i].label,actual,measured,clip});
  // F6 reaches the dismiss control; Enter dismisses and restores caller focus.
  await evaluate(`document.querySelector(${JSON.stringify(selector)}).focus()`);
  await key('F6','F6',117);
  assert.equal(await evaluate(`document.activeElement===document.querySelector('.astryx-toast button')`),true);
  await key('Enter','Enter',13);await waitFor(`!document.querySelector('.astryx-toast')`);
  assert.equal(await evaluate(`document.activeElement===document.querySelector(${JSON.stringify(selector)})`),true);
 }
 report.tests.push('Three native pointer actions, published geometry/paint, one polite writer, F6/Enter dismiss and focus return');
 // Every show is a new entry, including repeated identical messages; cap is five.
 await evaluate(`for(let i=0;i<6;i++)document.querySelector('#st-fill button').click()`);
 await waitFor(`document.querySelectorAll('.astryx-toast').length===5`);await pause(250);
 assert.deepEqual(await evaluate(`[...document.querySelectorAll('[data-toast-id]')].map(e=>Number(e.dataset.toastId))`),[5,6,7,8,9]);
 const ys=await evaluate(`[...document.querySelectorAll('.astryx-toast')].map(e=>{const r=e.getBoundingClientRect();return[r.y,r.height]})`);
 for(let i=1;i<ys.length;i++)near(ys[i][0]-ys[i-1][0]-ys[i-1][1],8,'stack gap');
 await evaluate(`document.querySelector('.astryx-demo-toast-row:last-child button').click()`);
 await waitFor(`document.querySelectorAll('.astryx-toast').length===5&&document.querySelector('[data-toast-id="4"]')`);
 assert.equal(await evaluate(`document.querySelector('.astryx-demo-toast-row:first-child .astryx-demo-toast-body').textContent`),published.actions[2].body);
 // Drain through cap resurfacing, not just currently visible entries.
 await drain();await drain();
 report.tests.push('Six repeated shows, five-visible cap, hidden oldest resurfaces, 8px gap, independent identities');
 await evaluate(`window.announcementWrites=[];window.announcementObserver=new MutationObserver(()=>{const text=document.querySelector('[data-astryx-live-region]').textContent;if(text)announcementWrites.push(text)});announcementObserver.observe(document.querySelector('[data-astryx-live-region]'),{childList:true,subtree:true,characterData:true})`);
 for(let i=0;i<2;i++){
  await evaluate(`document.querySelector('#st-fill button').click()`);await pause(300);
 }
 await evaluate(`document.getElementById('st-change').click()`);await pause(300);
 assert.deepEqual(await evaluate(`announcementWrites`),[published.actions[2].body,published.actions[2].body]);
 await evaluate(`announcementObserver.disconnect()`);await drain();
 report.tests.push('Identical messages shown in separate turns each produce one polite DOM write; unrelated render produces none (not screen-reader speech testing)');
 // Hover + focus overlap: leaving the pointer cannot release a focus hold.
 await click('#st-fill button');await waitFor(`!!document.querySelector('.astryx-toast')`);
 const point=await evaluate(`(()=>{const r=document.querySelector('.astryx-toast').getBoundingClientRect();return{x:r.x+20,y:r.y+20}})()`);
 await send('Input.dispatchMouseEvent',{type:'mouseMoved',...point});
 await evaluate(`document.querySelector('.astryx-toast button').focus()`);
 await pause(5200);assert.equal(await evaluate(`document.querySelectorAll('.astryx-toast').length`),1);
 await send('Input.dispatchMouseEvent',{type:'mouseMoved',x:10,y:10});await pause(1100);
 assert.equal(await evaluate(`document.querySelectorAll('.astryx-toast').length`),1);
 await evaluate(`document.querySelector('#st-fill button').focus()`);
 await waitFor(`!document.querySelector('.astryx-toast')`);
 report.tests.push('Real pointer hover + overlapping focus holds exceed five seconds; remaining-time resume auto-hides');
 // Resume floor: pause late in the five-second timer, then receive >=1s.
 await click('#st-fill button');await waitFor(`!!document.querySelector('.astryx-toast')`);await pause(4350);
 await evaluate(`document.querySelector('.astryx-toast button').focus()`);await pause(900);
 await evaluate(`document.querySelector('#st-fill button').focus()`);await pause(650);
 assert.equal(await evaluate(`document.querySelectorAll('.astryx-toast').length`),1);
 await waitFor(`!document.querySelector('.astryx-toast')`);
 report.tests.push('Late focus hold uses >=1000ms resume floor');
 // A second owned target supplies a real tab/window blur, including a show
 // while already blurred. It is closed independently in finally.
 const auxiliary=await send('Target.createTarget',{url:'about:blank'});
 try {
  await send('Target.activateTarget',{targetId:auxiliary.targetId});
  await waitFor(`!document.hasFocus()`);
  await evaluate(`document.querySelector('#st-fill button').click()`);
  await waitFor(`!!document.querySelector('.astryx-toast')`);await pause(5300);
  assert.equal(await evaluate(`document.querySelectorAll('.astryx-toast').length`),1);
  await send('Target.activateTarget',{targetId:target.id});
  await waitFor(`document.hasFocus()`);
  await waitFor(`!document.querySelector('.astryx-toast')`);
 } finally { await send('Target.closeTarget',{targetId:auxiliary.targetId}); }
 report.tests.push('Real owned-tab blur, show while blurred, >5s hold and native window-focus resume');
 // Touch events are CDP-dispatched to the actual surface. Mouse dragging is
 // intentionally inert at this pin; short touch returns, long touch dismisses.
 await evaluate(`document.querySelector('#st-fill button').click()`);await waitFor(`!!document.querySelector('.astryx-toast')`);await pause(200);
 await send('Emulation.setTouchEmulationEnabled',{enabled:true,maxTouchPoints:1});
 const touchPoint=await evaluate(`(()=>{const r=document.querySelector('.astryx-toast').getBoundingClientRect();return{x:r.x+30,y:r.y+10}})()`);
 for(const [distance,shouldDismiss]of [[18,false],[52,true]]){
  await send('Input.dispatchTouchEvent',{type:'touchStart',touchPoints:[{...touchPoint,id:1}]});
  await send('Input.dispatchTouchEvent',{type:'touchMove',touchPoints:[{x:touchPoint.x,y:touchPoint.y+12,id:1}]});
  await pause(150);
  await send('Input.dispatchTouchEvent',{type:'touchMove',touchPoints:[{x:touchPoint.x,y:touchPoint.y+distance,id:1}]});
  await pause(150);
  await send('Input.dispatchTouchEvent',{type:'touchEnd',touchPoints:[]});
  if(shouldDismiss)await waitFor(`!document.querySelector('.astryx-toast')`);
  else {await pause(250);assert.equal(await evaluate(`document.querySelectorAll('.astryx-toast').length`),1);}
 }
 await send('Emulation.setTouchEmulationEnabled',{enabled:false});
 report.tests.push('CDP touch: slow short swipe snaps back; >=48px toward bottom dismisses');
 // MediaTheme reacts to inherited mode without changing caller message/control.
 await evaluate(`document.querySelector('#st-fill button').click()`);await waitFor(`!!document.querySelector('.astryx-toast')`);
 await evaluate(`document.documentElement.style.colorScheme='light'`);await pause(200);
 assert.equal(await evaluate(`document.querySelector('.astryx-demo-toast-media').dataset.astryxMedia`),'dark');
 assert.equal(await evaluate(`getComputedStyle(document.querySelector('.astryx-demo-toast-body')).color`),'rgb(255, 255, 255)');
 await evaluate(`document.documentElement.style.colorScheme='dark'`);await pause(200);
 assert.equal(await evaluate(`document.querySelector('.astryx-demo-toast-media').dataset.astryxMedia`),'light');
 await drain();
 report.tests.push('Inherited light/dark mode retargets private measured media text side');
 // Narrow/RTL/reduced-motion geometry and cleanup are observable on actual nodes.
 await send('Emulation.setDeviceMetricsOverride',{width:320,height:640,deviceScaleFactor:1,mobile:false});
 await send('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:'reduce'}]});
 await evaluate(`document.querySelector('main').dir='rtl';document.querySelector('#st-fill button').click()`);
 await waitFor(`!!document.querySelector('.astryx-toast')`);await pause(200);
 assert.deepEqual(await evaluate(`(()=>{const r=document.querySelector('.astryx-toast').getBoundingClientRect();return[r.x,r.width,innerHeight-r.bottom]})()`),[16,288,16]);
 assert.equal(await evaluate(`getComputedStyle(document.querySelector('.astryx-toast')).transitionDuration`),'1e-05s');
 await drain();
 await evaluate(`document.querySelector('#st-fill button').click()`);await waitFor(`!!document.querySelector('.astryx-toast')`);
 await evaluate(`document.getElementById('st-remove-patterns').click()`);
 await waitFor(`!document.querySelector('.astryx-demo-toast-viewport')`);await pause(5300);
 assert.equal(await evaluate(`document.querySelectorAll('.astryx-toast').length`),0);
 assert.equal(await evaluate(`document.querySelector('[data-astryx-live-region]')?.textContent??''`),'');
 await key('F6','F6',117);
 assert.equal(exceptions.length,0,JSON.stringify(exceptions));
 report.tests.push('320px RTL/reduced-motion paint bounds, reduced dismissal, unmount while timed; no orphan Toast or exception after 5.3s');
 await writeFile(`${out}/measurements.json`,JSON.stringify(report,null,2)+'\n');
 console.log('PASS private Stack Toast:',report.tests);
});
