import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';
await withTarget(async({send,evaluate,waitFor,pause,exceptions})=>{
 await send('Page.bringToFront');await send('Page.navigate',{url:process.env.SHOWCASE_URL??'http://127.0.0.1:8082'});
 await waitFor('!!document.getElementById("sc-controlled")');
 await waitFor('document[Symbol.for("astryx.dioxus.button.runtime")]?.records.has(document.getElementById("sc-controlled"))');
 const click=async id=>{await evaluate(`document.getElementById(${JSON.stringify(id)}).click()`);await pause(80);};
 const point=async id=>{await evaluate(`document.getElementById('${id}').scrollIntoView({block:'center'})`);await pause(100);return evaluate(`(()=>{const r=document.getElementById('${id}').getBoundingClientRect();return{x:r.x+r.width/2,y:r.y+r.height/2}})()`);};
 const media=features=>send('Emulation.setEmulatedMedia',{features:Object.entries({'prefers-reduced-motion':'no-preference','forced-colors':'none',...features}).map(([name,value])=>({name,value}))});
 const state=()=>evaluate('document.getElementById("sc-controlled").getAttribute("data-astryx-press")');
 const color=()=>evaluate('getComputedStyle(document.getElementById("sc-controlled"),"::after").backgroundColor');
 const pixel=async()=>{const at=await evaluate('(()=>{const r=document.getElementById("sc-controlled").getBoundingClientRect();return{x:r.right-24,y:r.top+8}})()');const shot=await send('Page.captureScreenshot',{format:'png',captureBeyondViewport:false});return evaluate(`(async()=>{const image=await createImageBitmap(await(await fetch('data:image/png;base64,${shot.data}')).blob()),canvas=new OffscreenCanvas(2,2),ctx=canvas.getContext('2d');ctx.drawImage(image,${at.x},${at.y},2,2,0,0,2,2);image.close();const d=ctx.getImageData(0,0,2,2).data;return[0,1,2].map(c=>Math.round((d[c]+d[c+4]+d[c+8]+d[c+12])/4))})()`);};
 const pointer=async(type,p,pointerType='mouse')=>send('Input.dispatchMouseEvent',{type,...p,pointerType,button:type==='mouseMoved'?'none':'left',buttons:type==='mousePressed'?1:0,clickCount:type==='mouseMoved'?0:1});
 await media({});await send('Emulation.setTouchEmulationEnabled',{enabled:false});
 let p=await point('sc-surface');await pointer('mouseMoved',{x:1,y:1});await pause(250);const rest=await color(),restPixel=await pixel();
 await pointer('mouseMoved',p);await pause(250);const hover=await color(),hoverPixel=await pixel();assert.notEqual(hover,rest);assert.notDeepEqual(hoverPixel,restPixel);
 await pointer('mousePressed',p);assert.equal(await state(),'on','architecture press-feedback INV5 requires instant engine paint for fine mouse');assert.equal(await color(),'rgba(0, 0, 0, 0)');assert.deepEqual(await pixel(),[229,229,229],'original Neutral pressed token composite');
 await pointer('mouseReleased',p);assert.equal(await state(),'fading');await waitFor('!document.getElementById("sc-controlled").hasAttribute("data-astryx-press")');await pause(250);assert.equal(await color(),hover);assert.deepEqual(await pixel(),hoverPixel);
 for(const [id,pointerType] of [['sc-nested','mouse'],['sc-nested-input','mouse'],['sc-nested','pen'],['sc-nested-input','pen']]){
  p=await point(id);await pointer('mouseMoved',p,pointerType);await pointer('mousePressed',p,pointerType);await pause(200);
  assert.equal(await state(),null,`${id}/${pointerType} no root touch paint`);assert.equal(await color(),hover,`${id}/${pointerType} no root active paint`);await pointer('mouseReleased',p,pointerType);await pause(250);
 }
 // All built-in ring roles match source palette tokens; custom variants use
 // their own variable without recoloring the custom border.
 assert.equal(await evaluate('(()=>{const probe=document.createElement("i");document.body.append(probe);const ok=[...document.querySelectorAll("#sc-variants .astryx-selectable-card")].every(e=>{const s=getComputedStyle(e);probe.style.color=s.getPropertyValue("--_selectable-ring");return s.boxShadow.startsWith(getComputedStyle(probe).color+" 0px 0px 0px 2px inset,")});probe.remove();return ok})()'),true);
 assert.ok((await evaluate('getComputedStyle(document.getElementById("sc-custom")).boxShadow')).startsWith('rgb(255, 210, 90) 0px 0px 0px 2px inset,'));
 const geometry=await evaluate('(()=>{const r=document.getElementById("sc-controlled").getBoundingClientRect();return[r.width,r.height]})()');
 await evaluate('document.getElementById("sc-controlled").dir="rtl"');assert.deepEqual(await evaluate('(()=>{const r=document.getElementById("sc-controlled").getBoundingClientRect();return[r.width,r.height]})()'),geometry);
 // Touch and cancellation use shared resources, never a Selectable timer.
 await send('Emulation.setTouchEmulationEnabled',{enabled:true,maxTouchPoints:2});await pause(100);p=await point('sc-surface');
 await evaluate('document.getElementById("sc-controlled").style.setProperty("--duration-exit","650ms");document.getElementById("sc-controlled").style.setProperty("--ease-exit","linear")');
 await send('Input.dispatchTouchEvent',{type:'touchStart',touchPoints:[{...p,id:1}]});await pause(20);assert.equal(await state(),'on');assert.equal(await color(),'rgba(0, 0, 0, 0)','touch image replaces active color');
 assert.notEqual(await evaluate('getComputedStyle(document.getElementById("sc-controlled"),"::after").backgroundImage'),'none');
 await send('Input.dispatchTouchEvent',{type:'touchEnd',touchPoints:[]});await pause(30);assert.equal(await state(),'fading');
 const timing=await evaluate('document.getElementById("sc-controlled").getAnimations().map(a=>({duration:a.effect.getTiming().duration,easing:a.effect.getTiming().easing}))');assert.ok(timing.some(a=>a.duration===650&&a.easing==='linear'));
 await pause(180);assert.equal(await state(),'fading');await waitFor('!document.getElementById("sc-controlled").hasAttribute("data-astryx-press")');
 // Nested touch does not paint the card at any stage.
 p=await point('sc-nested-input');await send('Input.dispatchTouchEvent',{type:'touchStart',touchPoints:[{...p,id:2}]});await pause(200);assert.equal(await state(),null);await send('Input.dispatchTouchEvent',{type:'touchEnd',touchPoints:[]});
 // Slide-off and pointercancel never toggle or leave paint/resources behind.
 p=await point('sc-surface');await click('sc-clear');const checked=await evaluate('document.querySelector("#sc-controlled > input").checked');
 await send('Input.dispatchTouchEvent',{type:'touchStart',touchPoints:[{...p,id:3}]});await pause(20);await send('Input.dispatchTouchEvent',{type:'touchMove',touchPoints:[{x:p.x+400,y:p.y,id:3}]});await send('Input.dispatchTouchEvent',{type:'touchEnd',touchPoints:[]});await pause(250);assert.equal(await state(),null);assert.equal(await evaluate('document.querySelector("#sc-controlled > input").checked'),checked);
 await send('Input.dispatchTouchEvent',{type:'touchStart',touchPoints:[{...p,id:4}]});await pause(20);await send('Input.dispatchTouchEvent',{type:'touchCancel',touchPoints:[]});await pause(50);assert.equal(await state(),null);assert.equal(await evaluate('document.querySelector("#sc-controlled > input").checked'),checked);
 // Disable/drop while painted must dispose the old controller record.
 await send('Input.dispatchTouchEvent',{type:'touchStart',touchPoints:[{...p,id:5}]});await pause(20);await click('sc-disable');await pause(100);assert.equal(await state(),null);await send('Input.dispatchTouchEvent',{type:'touchCancel',touchPoints:[]});await click('sc-disable');
 await evaluate('window.oldSelectable=document.getElementById("sc-controlled")');await send('Input.dispatchTouchEvent',{type:'touchStart',touchPoints:[{...p,id:6}]});await pause(20);await click('sc-drop');await pause(100);assert.equal(await evaluate('document[Symbol.for("astryx.dioxus.button.runtime")].records.has(oldSelectable)'),false);await send('Input.dispatchTouchEvent',{type:'touchCancel',touchPoints:[]});await click('sc-drop');
 // Reduced motion: coarse opacity-only transitions and bounded touch release.
 await media({'prefers-reduced-motion':'reduce'});assert.equal(await evaluate('getComputedStyle(document.getElementById("sc-controlled")).transitionProperty'),'opacity');
 p=await point('sc-surface');await evaluate('document.getElementById("sc-controlled").style.setProperty("--duration-exit","650ms")');await send('Input.dispatchTouchEvent',{type:'touchStart',touchPoints:[{...p,id:7}]});await pause(20);await send('Input.dispatchTouchEvent',{type:'touchEnd',touchPoints:[]});await pause(10);assert.ok((await evaluate('document.getElementById("sc-controlled").getAnimations().map(a=>a.effect.getTiming().duration)')).every(duration=>duration<=200));await pause(250);assert.equal(await state(),null);
 await send('Emulation.setTouchEmulationEnabled',{enabled:false});assert.equal(await evaluate('getComputedStyle(document.getElementById("sc-controlled")).transitionDuration'),'0s');
 // A real finger beside a fine primary pointer retains the bounded release;
 // instant desktop mouse transitions must not erase that separate touch fade.
 assert.equal(await evaluate('matchMedia("(pointer:fine)").matches'),true);p=await point('sc-surface');
 await send('Input.dispatchTouchEvent',{type:'touchStart',touchPoints:[{...p,id:8}]});await pause(20);assert.equal(await state(),'on');
 await send('Input.dispatchTouchEvent',{type:'touchEnd',touchPoints:[]});await pause(10);assert.equal(await state(),'fading');
 assert.ok((await evaluate('document.getElementById("sc-controlled").getAnimations().map(a=>a.effect.getTiming().duration)')).includes(200));await pause(250);
 // Record (do not invent a fix for) source's forced-color persistent-state gap.
 await media({'forced-colors':'active'});assert.equal(await evaluate('getComputedStyle(document.getElementById("sc-custom")).boxShadow'),'none');assert.equal(await evaluate('getComputedStyle(document.getElementById("sc-custom")).borderWidth'),'0px');
 assert.equal(exceptions.length,0,JSON.stringify(exceptions));console.log('SelectableCard press/media: desktop engine hover/press, nested mouse/pen/touch veto, all ring roles/custom ring, RTL geometry, touch image/exit tokens, slide-off/cancel/disable/drop cleanup, reduced motion and preserved forced-color gap passed.');
});
