// Offline fixture; every CDP session must be serialized by browser_lock.py.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {cardMetrics,withTarget} from './card_metrics.mjs';
const fixture=JSON.parse(await readFile(new URL('./fixtures/selectable-card-neutral-reference.json',import.meta.url),'utf8'));
function compare(a,b,path='root'){
 if(typeof b==='number'){assert.ok(Math.abs(a-b)<=1/64+1e-6,`${path}: ${a} != ${b}`);return;}
 if(b&&typeof b==='object'){assert.deepEqual(Object.keys(a).sort(),Object.keys(b).sort(),`${path} keys`);for(const k of Object.keys(b))compare(a[k],b[k],`${path}/${k}`);return;}assert.equal(a,b,path);
}
function view(tree,is_fixture=false){return tree.nodes.map(n=>{
 const style=is_fixture?fixture.profiles.styles[n.style]:n.style,paint={};
 for(const p of ['display','flexDirection','alignItems','justifyContent','rowGap','columnGap','padding','margin','backgroundColor','color','opacity','borderRadius','borderWidth','boxShadow','overflowX','overflowY','fontFamily','fontSize','fontWeight','lineHeight','whiteSpace'])paint[p]=style[p];
 if(n.tag==='input'){for(const p of ['fontFamily','fontSize','fontWeight','lineHeight','color'])delete paint[p];}
 if(style.borderWidth!=='0px'){paint.borderStyle=style.borderStyle;paint.borderColor=style.borderColor;}
 if(paint.alignItems==='normal'&&['flex','grid'].includes(paint.display))paint.alignItems='stretch';
 return {tag:n.tag,box:n.box,text:n.textRuns.map(r=>r.text).join(''),textRects:n.textRects,paint};
});}
await withTarget(async({send,evaluate,pause,waitFor,exceptions})=>{
 await send('Page.bringToFront');await send('Accessibility.enable');
 await send('Page.navigate',{url:process.env.SHOWCASE_URL??'http://127.0.0.1:8082'});
 await waitFor('document.querySelectorAll("[data-selectable-named]").length===11');await evaluate('document.fonts.ready');
 const click=async selector=>{await evaluate(`document.querySelector(${JSON.stringify(selector)}).click()`);await pause(80);};
 const clear=()=>click('#sc-clear');const log=()=>evaluate('document.getElementById("sc-log").textContent');
 const check=()=>evaluate('document.querySelector("#sc-controlled > input").checked');
 await evaluate(`window.cardMetrics=(${cardMetrics});true`);
 let originals=0;
 for(const mode of ['light','dark']){
  if(mode==='dark'){await click('#sc-theme');await pause(250);}
  for(const [i,id] of ['sc-showcase','sc-elevated','sc-multi'].entries()){
   const actual=await evaluate(`cardMetrics(document.getElementById('${id}'))`);compare(view(actual),view(fixture.cases[mode][i].frame,true),`${mode}/${id}`);originals+=fixture.cases[mode][i].cards.length;
  }
 }
 await send('Emulation.setDeviceMetricsOverride',{width:390,height:1200,deviceScaleFactor:1,mobile:true});
 await send('Emulation.setTouchEmulationEnabled',{enabled:true,maxTouchPoints:1});
 for(const mode of ['light','dark']){
  await click('#sc-theme');await pause(250);
  for(const [i,id] of ['sc-showcase','sc-elevated','sc-multi'].entries()){
   const actual=await evaluate(`cardMetrics(document.getElementById('${id}'))`);compare(view(actual),view(fixture.mobile[mode][i],true),`${mode}/mobile/${id}`);originals+=fixture.cases[mode][i].cards.length;
  }
 }
 await send('Emulation.setDeviceMetricsOverride',{width:1200,height:1600,deviceScaleFactor:1,mobile:false});await send('Emulation.setTouchEmulationEnabled',{enabled:false});
 await click('#sc-theme');await pause(250);
 // Native surface click has change before click, with original native downcast.
 await clear();await click('#sc-surface');assert.equal(await log(),'capture|change:true|click');assert.equal(await check(),true);assert.equal(await evaluate('getComputedStyle(document.getElementById("sc-nested-card")).boxShadow.includes("2px inset")'),false,'unselected nested card must not inherit parent selection ring');assert.equal(await evaluate('getComputedStyle(document.getElementById("sc-disabled-card")).boxShadow.includes("2px inset")'),false);
 assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("sc-controlled");return[e.dataset.nativeType,e.dataset.nativeTrusted,e.tagName,e.lang,e.dataset.caller,e.hasAttribute("role"),e.hasAttribute("tabindex")]})()'),['click','false','DIV','fr','kept',false,false]);
 await clear();await click('#sc-controlled > input');assert.equal(await log(),'capture|click|change:false');assert.equal(await check(),false);
 // A native Space click runs caller before change; Enter has no click.
 await evaluate('document.querySelector("#sc-controlled > input").focus()');
 await clear();await send('Input.dispatchKeyEvent',{type:'keyDown',key:' ',code:'Space',windowsVirtualKeyCode:32});await send('Input.dispatchKeyEvent',{type:'keyUp',key:' ',code:'Space',windowsVirtualKeyCode:32});await pause(100);assert.equal(await log(),'capture|click|change:true');assert.equal(await check(),true);
 await clear();await send('Input.dispatchKeyEvent',{type:'keyDown',key:'Enter',code:'Enter',windowsVirtualKeyCode:13});await send('Input.dispatchKeyEvent',{type:'keyUp',key:'Enter',code:'Enter',windowsVirtualKeyCode:13});await pause(80);assert.equal(await log(),'change:false');assert.equal(await check(),false);
 await clear();await evaluate('document.querySelector("#sc-controlled > input").dispatchEvent(new KeyboardEvent("keydown",{key:"Enter",keyCode:229,bubbles:true,cancelable:true}))');await evaluate('document.querySelector("#sc-controlled > input").dispatchEvent(new KeyboardEvent("keydown",{key:"Enter",isComposing:true,bubbles:true,cancelable:true}))');await pause(50);assert.equal(await log(),'');
 // A rejected request must repair the checked PROPERTY, not only its attribute.
 await clear();await click('#sc-rejected > input');assert.equal(await evaluate('document.querySelector("#sc-rejected > input").checked'),false);assert.equal(await log(),'rejected:true');
 await clear();await click('#sc-nested');assert.equal(await log(),'capture|nested|click|nested-action');assert.equal(await check(),false);
 await clear();await click('#sc-nested-input');assert.equal(await log(),'capture|click');assert.equal(await check(),false);
 await clear();await click('#sc-nested-link');assert.equal(await log(),'capture|link|click');assert.equal(await check(),false);assert.equal(await evaluate('location.hash'),'#sc-destination');
 await clear();await click('#sc-nested-card');assert.equal(await log(),'capture|nested-card-change:true|nested-card-click|click');assert.equal(await check(),false);
 await clear();await click('#sc-disabled-card');assert.equal(await log(),'capture|click');assert.equal(await check(),false);
 await evaluate('(()=>{const e=document.getElementById("sc-surface"),r=document.createRange();r.selectNodeContents(e);getSelection().removeAllRanges();getSelection().addRange(r)})()');await clear();await click('#sc-surface');assert.equal(await log(),'capture|click');assert.equal(await check(),false);await evaluate('getSelection().removeAllRanges()');
 // Released capture order: caller before capture callback and layer action;
 // its stopPropagation is isolated, defaultPrevented deferred until document.
 await clear();await click('#sc-layer-button');assert.equal(await log(),'click|capture|layer:true|layer-boundary');assert.equal(await check(),false);
 await click('#sc-layer-cancel');await clear();await click('#sc-nested');assert.equal(await log(),'capture|nested|click|nested-action','root preventDefault cannot revoke nested action admission');await clear();await click('#sc-layer-button');assert.equal(await log(),'click|capture|layer:true|layer-boundary');
 await evaluate('location.hash="sc-before"');await clear();await click('#sc-layer-link');assert.equal(await evaluate('location.hash'),'#sc-before');assert.equal(await log(),'click|capture|layer-boundary');
 await click('#sc-layer-cancel');await clear();await click('#sc-layer-link');assert.equal(await evaluate('location.hash'),'#sc-native-default');
 await clear();await click('#sc-surface');assert.equal(await log(),'capture|change:true|click');
 const before=await evaluate('(()=>{const e=document.getElementById("sc-controlled"),r=e.getBoundingClientRect();return[r.width,r.height,getComputedStyle(e).boxShadow]})()');await click('#sc-surface');await pause(180);const after=await evaluate('(()=>{const e=document.getElementById("sc-controlled"),r=e.getBoundingClientRect();return[r.width,r.height,getComputedStyle(e).boxShadow]})()');assert.deepEqual(before.slice(0,2),after.slice(0,2));assert.notEqual(before[2],after[2]);assert.ok(before[2].includes('2px inset')&&before[2].split(',').length>2);
 // Native AX role/name/state and disabled focus reachability.
 const dom=await send('DOM.getDocument'),node=await send('DOM.querySelector',{nodeId:dom.root.nodeId,selector:'#sc-controlled > input'});const ax=(await send('Accessibility.getPartialAXTree',{nodeId:node.nodeId,fetchRelatives:false})).nodes[0];assert.equal(ax.role.value,'checkbox');assert.equal(ax.name.value,'Controlled option');
 await click('#sc-disable');await clear();await click('#sc-surface');await click('#sc-controlled > input');assert.equal(await log(),'capture|capture');
 await clear();await click('#sc-layer-button');assert.equal(await log(),'capture|layer:true|layer-boundary','disabled removes layer onclick forwarding but not child operations');await clear();await evaluate('document.getElementById("sc-controlled").dispatchEvent(new MouseEvent("mouseup",{bubbles:true}))');assert.equal(await log(),'');await evaluate('document.getElementById("sc-controlled").dispatchEvent(new PointerEvent("pointerdown",{pointerType:"mouse",bubbles:true}))');await pause(30);assert.equal(await log(),'pointerdown');await clear();await click('#sc-disabled-selected > input');assert.equal(await evaluate('document.querySelector("#sc-disabled-selected > input").checked'),true);assert.equal(await log(),'');await clear();await click('#sc-raw');assert.equal(await log(),'');await evaluate('document.getElementById("sc-raw").dispatchEvent(new KeyboardEvent("keydown",{key:"a",bubbles:true}))');await pause(80);assert.equal(await log(),'raw-keydown');await clear();assert.equal(await check(),false);
 await evaluate('document.querySelector("#sc-controlled > input").focus()');await clear();for(const [key,code,n] of [[' ','Space',32],['Enter','Enter',13]]){await send('Input.dispatchKeyEvent',{type:'keyDown',key,code,windowsVirtualKeyCode:n});await send('Input.dispatchKeyEvent',{type:'keyUp',key,code,windowsVirtualKeyCode:n});}await pause(50);assert.equal(await log(),'');assert.equal(await check(),false);assert.equal(await evaluate('document.activeElement===document.querySelector("#sc-controlled > input")'),true);assert.equal(await evaluate('getComputedStyle(document.getElementById("sc-controlled")).opacity'),'0.5');
 await click('#sc-disable');
 // Only the primary input delegates a focus-visible indicator to the root.
 await evaluate('document.querySelector("#sc-controlled > input").focus()');assert.equal(await evaluate('getComputedStyle(document.getElementById("sc-controlled")).outlineWidth'),'2px');await evaluate('document.getElementById("sc-nested").focus()');assert.equal(await evaluate('getComputedStyle(document.getElementById("sc-controlled")).outlineWidth'),'0px');
 await click('#sc-drop');assert.equal(await evaluate('document.getElementById("sc-controlled")'),null);await click('#sc-drop');await clear();await click('#sc-surface');assert.equal(await log(),'capture|change:true|click');
 // The canonical ownership walk covers every source interactive role, not
 // only native buttons or the nearest aria-readonly match.
 for(const role of ['button','link','checkbox','radio','switch','tab','menuitem','option','combobox','listbox','slider','spinbutton']){
  await evaluate(`(()=>{const e=document.createElement('div');e.id='sc-extra';e.setAttribute('role',${JSON.stringify(role)});e.innerHTML='<span aria-readonly="true">role child</span>';document.getElementById('sc-controlled').append(e)})()`);await clear();await click('#sc-extra > span');assert.equal(await log(),'capture|click',role);assert.equal(await check(),true);await evaluate('document.getElementById("sc-extra").remove()');
 }
 await evaluate('(()=>{const e=document.createElement("div");e.id="sc-extra";e.setAttribute("role","button");e.setAttribute("aria-readonly","true");e.textContent="Read-only role";document.getElementById("sc-controlled").append(e)})()');await clear();await click('#sc-extra');assert.equal(await log(),'capture|change:false|click');await evaluate('document.getElementById("sc-extra").remove()');await click('#sc-surface');
 // Original framework metadata survives child cancellation and ordinary raw builders.
 await clear();await click('#sc-child-stop');assert.equal(await log(),'capture|span-stop');assert.equal(await check(),true);
 await clear();await click('#sc-child-prevent');assert.equal(await log(),'capture|span-prevent|change:false|click');assert.equal(await check(),false);assert.equal(await evaluate('document.getElementById("sc-controlled").dataset.callerDefault'),'false');
 await click('#sc-child-cancel');await clear();await click('#sc-nested');assert.equal(await log(),'capture|nested|click');assert.equal(await evaluate('document.getElementById("sc-controlled").dataset.callerDefault'),'false');await click('#sc-child-cancel');
 await clear();await click('#sc-raw');assert.equal(await log(),'raw-change:true','reserved raw click is not a component callback alias');await clear();await click('#sc-raw > input');assert.equal(await log(),'raw-change:true');assert.equal(await evaluate('document.querySelector("#sc-raw > input").checked'),false);
 await clear();await evaluate('document.getElementById("sc-raw").dispatchEvent(new MouseEvent("mouseup",{bubbles:true}))');await pause(80);assert.equal(await log(),'','reserved raw mouseup is ignored');await evaluate('document.getElementById("sc-raw").dispatchEvent(new KeyboardEvent("keydown",{key:"a",bubbles:true}))');await pause(80);assert.equal(await log(),'raw-keydown');
 // Removing/restoring native builders or unrelated attributes must not starve
 // retained owned callbacks, remount the native input, or emit a fake ref mount.
 await evaluate('window.rawRoot=document.getElementById("sc-raw");window.rawInput=rawRoot.firstElementChild');
 assert.equal(await evaluate('document.getElementById("sc-raw-mounts").textContent'),'1');
 let hasKeydown=true,hasExtra=true;
 for(const toggle of ['sc-raw-keydown-toggle','sc-raw-attribute-toggle','sc-raw-keydown-toggle','sc-raw-attribute-toggle','sc-raw-attribute-toggle','sc-raw-keydown-toggle','sc-raw-attribute-toggle','sc-raw-keydown-toggle']) {
  await click(`#${toggle}`);if(toggle==='sc-raw-keydown-toggle')hasKeydown=!hasKeydown;else hasExtra=!hasExtra;
  assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("sc-raw");return[e===rawRoot,e.firstElementChild===rawInput,e.hasAttribute("data-raw-extra"),document.getElementById("sc-raw-mounts").textContent]})()'),[true,true,hasExtra,'1']);
  await clear();await click('#sc-raw');assert.equal(await log(),'raw-change:true');
  await clear();await evaluate('document.getElementById("sc-raw").dispatchEvent(new KeyboardEvent("keydown",{key:"a",bubbles:true}))');await pause(40);assert.equal(await log(),hasKeydown?'raw-keydown':'');
 }
 await evaluate('window.sameRoot=document.getElementById("sc-controlled");window.sameInput=sameRoot.firstElementChild');await click('#sc-change-props');assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("sc-controlled");return[e===sameRoot,e.firstElementChild===sameInput,e.firstElementChild.getAttribute("aria-label"),e.dataset.variant,e.dataset.elevation,e.getBoundingClientRect().width,e.firstElementChild.checked]})()'),[true,true,'Renamed option','green','high',320,false]);await click('#sc-change-props');await click('#sc-surface');
 // An unmarked popover is not silently promoted to an open-layer boundary.
 await evaluate('document.getElementById("sc-unmarked").showPopover()');await clear();await click('#sc-unmarked');assert.equal(await log(),'capture|change:false|click');await evaluate('document.getElementById("sc-unmarked").hidePopover()');
 assert.equal(exceptions.length,0,JSON.stringify(exceptions));
 console.log(`SelectableCard: ${originals} source-observed cards/three patterns, native surface/input/Space/Enter/IME/order, actual checked property, nested/selection/layer isolation/default deferral, disabled/AX/focus, ring/elevation geometry and disposal passed.`);
});
