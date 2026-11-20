// Real StatusDot/Tooltip portals exercise Layer-owned neutral ancestry, not
// an arbitrary popover heuristic or a mock-only layer implementation.
import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';
await withTarget(async({send,evaluate,waitFor,pause,exceptions})=>{
 await send('Page.bringToFront');await send('Emulation.setTouchEmulationEnabled',{enabled:false});await send('Page.navigate',{url:process.env.SHOWCASE_URL??'http://127.0.0.1:8082'});await waitFor('!!document.getElementById("sc-tooltip-trigger")');
 const click=async id=>{await evaluate(`document.getElementById('${id}').click()`);await pause(80);};const clear=()=>click('sc-clear');const log=()=>evaluate('document.getElementById("sc-log").textContent');
 const show=async id=>{await evaluate(`document.getElementById('${id}').scrollIntoView({block:'center'});document.getElementById('${id}').focus()`);await waitFor(`(()=>{const e=document.getElementById('${id}'),tip=document.getElementById(e.getAttribute('aria-controls'));return tip?.matches(':popover-open')})()`);return evaluate(`document.getElementById('${id}').getAttribute('aria-controls')`);};
 const tip=await show('sc-tooltip-trigger');assert.equal(await evaluate(`document.getElementById('${tip}').parentElement===document.body`),true);
 await clear();await click(tip);assert.equal(await log(),'click|capture','released caller receives actual body-portaled tooltip press before its native stop');assert.equal(await evaluate('document.querySelector("#sc-controlled > input").checked'),false);
 await click('sc-unrelated-on-click');await clear();await click(tip);assert.equal(await log(),'click|unrelated|capture','internal document forwarding must reject unrelated recursive native dispatch before any FnMut callback bottleneck');await click('sc-unrelated-on-click');
 await click('sc-layer-cancel');await clear();await click(tip);assert.equal(await log(),'click|capture','caller stopPropagation cannot starve actual tooltip surface');
 await click('sc-disable');await clear();await click(tip);assert.equal(await log(),'capture','disabled removes caller click forwarding but preserves caller capture');await click('sc-disable');await click('sc-layer-cancel');
 await evaluate(`window.oldTip=document.getElementById('${tip}');document.getElementById('sc-clear').focus()`);await waitFor('!oldTip.matches(":popover-open")');await clear();await evaluate('oldTip.dispatchEvent(new MouseEvent("click",{bubbles:true,cancelable:true}))');await pause(80);assert.equal(await log(),'','hidden portal no longer retains a logical owner');
 const nested=await show('sc-inner-tooltip-trigger');await clear();await click(nested);assert.equal(await log(),'click|capture|nested-card-click','logical parent capture precedes nested card even if native mount order differs');assert.equal(await evaluate('document.querySelector("#sc-controlled > input").checked'),false);assert.equal(await evaluate('document.querySelector("#sc-nested-card > input").checked'),false);
 await click('sc-stop-capture');await clear();await click(nested);assert.equal(await log(),'click|capture','explicit parent capture stop blocks descendant logical capture');await click('sc-stop-capture');
 await evaluate('document.getElementById("sc-clear").focus()');await waitFor('!document.querySelector(".astryx-tooltip:popover-open")');
 await evaluate('document.getElementById("sc-native-modal").showModal()');const modal=await show('sc-modal-trigger');assert.equal(await evaluate(`document.getElementById('sc-native-modal').contains(document.getElementById('${modal}'))&&!document.getElementById('sc-modal-card').contains(document.getElementById('${modal}'))`),true);
 // Clear via the application state handler, without trying to move native
 // focus outside a modal or clicking through its top-layer backdrop.
 await clear();await click(modal);assert.equal(await log(),'modal-click|modal-capture');assert.equal(await evaluate('document.querySelector("#sc-modal-card > input").checked'),false);
 await evaluate('document.getElementById("sc-native-modal").close()');await pause(100);
 await show('sc-tooltip-trigger');await evaluate('window.removedTip=document.querySelector(".astryx-tooltip:popover-open");window.removedTrigger=document.getElementById("sc-tooltip-trigger")');await click('sc-drop');await waitFor('!document.getElementById("sc-controlled")&&!removedTip.isConnected');await clear();await evaluate('removedTip.dispatchEvent(new MouseEvent("click",{bubbles:true,cancelable:true}))');await pause(80);assert.equal(await log(),'');
 assert.equal(exceptions.length,0,JSON.stringify(exceptions));console.log('SelectableCard actual layer capture: body portals, caller isolation, disabled capture, hidden/removal cleanup, nested parent-before-child logical order and native-modal Tooltip passed.');
});
