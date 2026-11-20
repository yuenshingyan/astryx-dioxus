// Native identity and scoped source-owner adaptation, not synthetic transport.
import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';
await withTarget(async({send,evaluate,waitFor,pause,exceptions})=>{
 await send('Page.bringToFront');await send('Page.navigate',{url:process.env.SHOWCASE_URL??'http://127.0.0.1:8082'});await waitFor('!!document.getElementById("sc-controlled")');
 await evaluate(`(()=>{
  window.__selectableEvents=[];
  window.addEventListener('click',event=>{window.__selectableOriginalEvent=event;},{capture:true});
  document.addEventListener('click',event=>__selectableEvents.push({same:event===__selectableOriginalEvent,owner:event.currentTarget===document,hasOwn:Object.hasOwn(event,'currentTarget')}));
 })()`);
 const click=async id=>{await evaluate(`document.getElementById('${id}').click()`);await pause(80);};
 await click('sc-clear');
 await evaluate('window.testEvent=new MouseEvent("click",{bubbles:true,cancelable:true,ctrlKey:true,clientX:31,clientY:47});document.getElementById("sc-surface").dispatchEvent(testEvent)');await pause(80);
 assert.deepEqual(await evaluate('(()=>{const d=document.getElementById("sc-controlled").dataset;return[d.nativeIdentity,d.nativeOwner,d.nativeTrusted,d.nativePosition,d.wrappedPosition,d.nativeCtrl,d.wrappedCtrl]})()'),['true','sc-controlled','false','31,47','31,47','true','true']);
 assert.equal(await evaluate('Object.hasOwn(testEvent,"currentTarget")'),false);assert.equal(await evaluate('testEvent.currentTarget'),null);assert.deepEqual(await evaluate('__selectableEvents.at(-1)'),{same:true,owner:true,hasOwn:false});
 // A real trusted pointer click remains the original event, with exact data.
 await evaluate('document.getElementById("sc-surface").scrollIntoView({block:"center"})');await pause(100);const p=await evaluate('(()=>{const r=document.getElementById("sc-surface").getBoundingClientRect();return{x:r.x+r.width/2,y:r.y+r.height/2}})()');
 for(const type of ['mouseMoved','mousePressed','mouseReleased'])await send('Input.dispatchMouseEvent',{type,...p,button:type==='mouseMoved'?'none':'left',buttons:type==='mousePressed'?1:0,clickCount:type==='mouseMoved'?0:1});await pause(80);
 assert.deepEqual(await evaluate('(()=>{const d=document.getElementById("sc-controlled").dataset;return[d.nativeIdentity,d.nativeOwner,d.nativeTrusted,d.nativePosition===d.wrappedPosition]})()'),['true','sc-controlled','true',true]);
 // Sequential inner/outer owners share one original native event and restore
 // the renderer descriptor after each owner scope, not only at end of dispatch.
 await click('sc-nested-card');assert.equal(await evaluate('document.getElementById("sc-nested-card").dataset.nativeOwner'),'sc-nested-card');assert.equal(await evaluate('document.getElementById("sc-controlled").dataset.nativeOwner'),'sc-controlled');assert.equal(await evaluate('document.getElementById("sc-controlled").dataset.nativeIdentity'),'true');assert.deepEqual(await evaluate('__selectableEvents.at(-1)'),{same:true,owner:true,hasOwn:false});
 // Restore an existing configurable own descriptor exactly, including flags.
 await evaluate(`(()=>{window.describedEvent=new MouseEvent('click',{bubbles:true,cancelable:true});window.ownerGetter=()=>document;Object.defineProperty(describedEvent,'currentTarget',{get:ownerGetter,configurable:true,enumerable:true});document.getElementById('sc-surface').dispatchEvent(describedEvent);})()`);await pause(80);
 assert.equal(await evaluate('document.getElementById("sc-controlled").dataset.nativeOwner'),'sc-controlled');assert.deepEqual(await evaluate('(()=>{const d=Object.getOwnPropertyDescriptor(describedEvent,"currentTarget");return[d.get===ownerGetter,d.set===undefined,d.enumerable,d.configurable,describedEvent.currentTarget===document]})()'),[true,true,true,true,true]);
 // Before-layer forwarding retains the same original event and owner, while
 // high-level cancellation stays isolated until native document bubbling.
 await click('sc-layer-cancel');await click('sc-clear');await click('sc-layer-button');assert.equal(await evaluate('document.getElementById("sc-log").textContent'),'click|capture|layer:true|layer-boundary');assert.deepEqual(await evaluate('(()=>{const d=document.getElementById("sc-controlled").dataset;return[d.nativeIdentity,d.nativeOwner]})()'),['true','sc-controlled']);assert.equal(await evaluate('__selectableOriginalEvent.defaultPrevented'),true);assert.equal(await evaluate('Object.hasOwn(__selectableOriginalEvent,"currentTarget")'),false);
 await click('sc-remove-on-click');await evaluate('window.removedOwner=document.getElementById("sc-controlled")');await click('sc-surface');await waitFor('!document.getElementById("sc-controlled")&&!document[Symbol.for("astryx.dioxus.button.runtime")].records.has(removedOwner)');assert.equal(await evaluate('removedOwner.dataset.nativeOwner'),'sc-controlled');assert.equal(await evaluate('__selectableOriginalEvent.currentTarget'),null);assert.equal(await evaluate('Object.hasOwn(__selectableOriginalEvent,"currentTarget")'),false);await click('sc-clear');await evaluate('removedOwner.dispatchEvent(new MouseEvent("click",{bubbles:true,cancelable:true}))');await pause(50);assert.equal(await evaluate('document.getElementById("sc-log").textContent'),'','removed native capture listener is inert even on retained detached DOM');
 assert.equal(exceptions.length,0,JSON.stringify(exceptions));console.log('SelectableCard native event identity/trust/data, source owner scopes, exact own-descriptor restoration, nested owners and before-layer cancellation passed.');
});
