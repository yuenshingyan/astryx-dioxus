// ButtonGroup browser contracts. Node.js 22+, Chrome CDP, rebuilt showcase.
// See docs/button-group-port.md. No npm dependencies.
import assert from 'node:assert/strict';
import { readFile, writeFile } from 'node:fs/promises';

const cdpUrl = process.env.CDP_URL ?? 'http://127.0.0.1:9222';
const showcaseUrl = process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8080';
const response = await fetch(`${cdpUrl}/json/new?about:blank`, { method: 'PUT' });
assert.ok(response.ok, 'Chrome must expose its debugging endpoint');
const target = await response.json();
const socket = new WebSocket(target.webSocketDebuggerUrl);
await new Promise((resolve, reject) => {
  socket.addEventListener('open', resolve, { once: true });
  socket.addEventListener('error', reject, { once: true });
});
let nextId = 0;
const requests = new Map();
const browserErrors = [];
socket.addEventListener('message', ({ data }) => {
  const message = JSON.parse(data);
  if (message.method === 'Runtime.exceptionThrown') {
    browserErrors.push(message.params.exceptionDetails.exception?.description ?? message.params.exceptionDetails.text);
  }
  const request = requests.get(message.id);
  if (!request) return;
  requests.delete(message.id);
  clearTimeout(request.timer);
  if (message.error) request.reject(new Error(message.error.message));
  else request.resolve(message.result);
});
function send(method, params = {}) {
  return new Promise((resolve, reject) => {
    const id = ++nextId;
    const timer = setTimeout(() => {
      requests.delete(id);
      reject(new Error(`CDP timeout: ${method}`));
    }, 15000);
    requests.set(id, { resolve, reject, timer });
    socket.send(JSON.stringify({ id, method, params }));
  });
}
async function evaluate(expression) {
  const result = await send('Runtime.evaluate', {
    expression, returnByValue: true, awaitPromise: true,
  });
  assert.equal(result.exceptionDetails, undefined, JSON.stringify(result.exceptionDetails));
  return result.result.value;
}
async function waitFor(expression) {
  const deadline = Date.now() + 15000;
  while (Date.now() < deadline) {
    if (await evaluate(expression)) return;
    await new Promise(resolve => setTimeout(resolve, 25));
  }
  assert.fail(`Timed out waiting for: ${expression}`);
}
async function key(key, code, windowsVirtualKeyCode, modifiers = 0) {
  await send('Input.dispatchKeyEvent', {
    type: 'keyDown', key, code, windowsVirtualKeyCode, modifiers,
    text: key === 'Enter' ? '\r' : key === ' ' ? ' ' : undefined,
  });
  await send('Input.dispatchKeyEvent', { type: 'keyUp', key, code, windowsVirtualKeyCode, modifiers });
}
const click = id => evaluate(`document.getElementById(${JSON.stringify(id)}).click()`);
const focus = id => evaluate(`document.getElementById(${JSON.stringify(id)}).focus()`);
const active = () => evaluate('document.activeElement.id');
async function point(id) {
  // A cancelled vertical touch can leave a compositor fling. Let it settle
  // before scrolling the target into view and measuring a new gesture.
  return evaluate(`new Promise((resolve,reject)=>{const element=document.getElementById(${JSON.stringify(id)}); let previous=null,stable=0,frames=0,isPositioned=false; const sample=()=>{const r=element.getBoundingClientRect(); const position=[r.x,r.y,scrollX,scrollY].join(','); stable=position===previous?stable+1:0; previous=position; if(stable>=4&&!isPositioned){element.scrollIntoView({block:'center'});isPositioned=true;stable=0;previous=null;requestAnimationFrame(sample);} else if(stable>=4) resolve({x:r.x+r.width/2,y:r.y+r.height/2}); else if(++frames>240) reject(new Error('Target did not stop scrolling')); else requestAnimationFrame(sample);}; requestAnimationFrame(sample);})`);
}
async function mouse(type, position, pointerType = 'mouse') {
  await send('Input.dispatchMouseEvent', {type,...position,button:'left',buttons:type==='mouseReleased'?0:1,clickCount:1,pointerType});
}
async function touch(type, position) {
  await send('Input.dispatchTouchEvent',{type,touchPoints:position?[{...position,id:1,radiusX:3,radiusY:3}]:[]});
}
const metrics = group => {
  const css=getComputedStyle(group);
  return {label:group.getAttribute('aria-label'),orientation:group.dataset.orientation||'horizontal',elevation:group.dataset.elevation||'none',display:css.display,alignItems:css.alignItems,flexDirection:css.flexDirection,radius:css.borderRadius,shadow:css.boxShadow,width:group.getBoundingClientRect().width,height:group.getBoundingClientRect().height,members:[...group.querySelectorAll(':scope > .astryx-button')].map(button=>{const c=getComputedStyle(button),r=button.getBoundingClientRect();return {label:button.getAttribute('aria-label')||button.textContent.trim(),width:r.width,height:r.height,background:c.backgroundColor,color:c.color,radii:[c.borderStartStartRadius,c.borderStartEndRadius,c.borderEndStartRadius,c.borderEndEndRadius],dividerWidth:c.borderInlineStartWidth,dividerColor:c.borderInlineStartColor,font:c.fontSize,fontWeight:c.fontWeight,padding:[c.paddingInlineStart,c.paddingInlineEnd],gap:c.gap,shadow:c.boxShadow};})};
};
async function oneStop(id, expected) {
  await waitFor(`(() => {const list=window.bgMembers(${JSON.stringify(id)});return list.length>0&&list.filter(e=>e.tabIndex===0).length===1${expected?`&&list.find(e=>e.tabIndex===0).id===${JSON.stringify(expected)}`:''};})()`);
}
try {
  await send('Page.enable'); await send('Runtime.enable'); await send('Page.bringToFront');
  await send('Emulation.setTouchEmulationEnabled',{enabled:false});
  await send('Emulation.setDeviceMetricsOverride',{width:1200,height:1400,deviceScaleFactor:1,mobile:false});
  await send('Page.navigate',{url:showcaseUrl});
  await waitFor('Boolean(document.querySelector("#bg-basic")) && Boolean(document[Symbol.for("astryx.dioxus.button-group.runtime")])');
  await evaluate('document.fonts.ready.then(()=>true)');
  await evaluate(`window.bgMembers=id=>{const group=document.getElementById(id);return [...group.querySelectorAll('button,a[href],[tabindex]')].filter(element=>element.closest('[role="group"],[popover],dialog')===group&&element.getClientRects().length&&!element.matches(':disabled'));};true`);
  await oneStop('bg-basic','bg-copy');
  await oneStop('bg-keyboard','bg-key-first');
  const reference=JSON.parse(await readFile(new URL('./fixtures/button-group-neutral-reference.json',import.meta.url),'utf8'));
  assert.equal(reference.glyphPaths.length,4,'record all original showcase glyphs');
  assert.deepEqual(await evaluate('["bg-clipboard","bg-save-options"].flatMap(id=>[...document.getElementById(id).querySelectorAll("svg path")].map(path=>path.getAttribute("d")))'),reference.glyphPaths,'named showcase glyphs match the actual upstream page');
  const ids=['bg-basic','bg-floating','bg-clipboard','bg-save-options'];
  for(const mode of ['light','dark']) {
    await evaluate(`document.getElementById('button-group-examples').style.colorScheme=${JSON.stringify(mode)}`);
    await new Promise(resolve=>setTimeout(resolve,600));
    assert.equal(reference.measurements[mode].length,4,'reference must contain all named patterns');
    const actual=await evaluate(`${JSON.stringify(ids)}.map(id=>(${metrics})(document.getElementById(id)))`);
    for(const expected of reference.measurements[mode]) {
      const found=actual.find(group=>group.label===expected.label);
      assert.ok(found,`missing original pattern ${expected.label}`);
      assert.ok(Math.abs(found.width-expected.width)<=expected.members.length/32,`${mode} ${expected.label}: width ${found.width} vs ${expected.width}`);
      found.width=expected.width;
      assert.equal(found.members.length,expected.members.length);
      found.members.forEach((member,index)=>{
        assert.ok(Math.abs(member.width-expected.members[index].width)<=1/32,`${mode} ${expected.label} member width: ${JSON.stringify(member)}`);
        member.width=expected.members[index].width;
      });
      assert.deepEqual(found,expected,`${mode} original ${expected.label} appearance`);
    }
  }
  await evaluate('document.getElementById("button-group-examples").style.colorScheme="light"');
  assert.deepEqual(await evaluate('["sm","md","lg"].map(size=>document.getElementById("bg-size-"+size).getBoundingClientRect().height)'),[28,32,36]);
  assert.deepEqual(await evaluate('["bg-inherited-size","bg-explicit-size"].map(id=>document.getElementById(id).getBoundingClientRect().height)'),[28,36]);
  const elevations=await evaluate('["none","low","med","high"].map(tier=>{const g=document.getElementById("bg-elevation-"+tier);return {shadow:getComputedStyle(g).boxShadow,members:[...g.querySelectorAll(".astryx-button")].map(e=>[e.dataset.elevation,getComputedStyle(e).boxShadow])};})');
  assert.equal(elevations[0].shadow,'none');
  assert.equal(new Set(elevations.map(value=>value.shadow)).size,4);
  elevations.forEach(value=>value.members.forEach(member=>assert.deepEqual(member,['none','none'])));
  const verticalPaint=await evaluate('[...document.querySelectorAll("#bg-vertical > .astryx-button")].map(e=>{const c=getComputedStyle(e),r=e.getBoundingClientRect();return {corners:[c.borderStartStartRadius,c.borderStartEndRadius,c.borderEndStartRadius,c.borderEndEndRadius],divider:c.borderBlockStartWidth,top:r.top,bottom:r.bottom,width:r.width};})');
  assert.deepEqual(verticalPaint.map(value=>value.corners),[['10px','10px','0px','0px'],['0px','0px','0px','0px'],['0px','0px','10px','10px']]);
  assert.deepEqual(verticalPaint.map(value=>value.divider),['0px','1px','1px']);
  assert.equal(verticalPaint[1].top-verticalPaint[0].bottom,0);
  assert.equal(verticalPaint[2].top-verticalPaint[1].bottom,0);
  assert.equal(new Set(verticalPaint.map(value=>value.width)).size,1,'vertical members stretch across the cross axis');
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#bg-tiny-action")).borderRadius'),'10px','one member keeps all outer corners');
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#bg-key-link")).borderStartEndRadius'),'0px','a native following member still occupies the trailing slot');
  await focus('bg-before'); await key('Tab','Tab',9); assert.equal(await active(),'bg-key-first');
  await key('ArrowRight','ArrowRight',39); assert.equal(await active(),'bg-key-link','disabled member must be skipped');
  await key('End','End',35); assert.equal(await active(),'bg-key-native','native focusable member participates');
  await key('ArrowRight','ArrowRight',39); assert.equal(await active(),'bg-key-first','arrows wrap');
  await key('Tab','Tab',9); assert.equal(await active(),'bg-after','one Tab leaves group');
  await key('Tab','Tab',9,8); assert.equal(await active(),'bg-key-first','Shift+Tab reenters at holder');
  const nativePoint=await point('bg-key-native');
  await mouse('mousePressed',nativePoint); await mouse('mouseReleased',nativePoint);
  await oneStop('bg-keyboard','bg-key-native');
  await key('Tab','Tab',9); assert.equal(await active(),'bg-after','pointer focus moves the roving stop');
  await focus('bg-key-link'); await oneStop('bg-keyboard','bg-key-link');
  await key('Home','Home',36); assert.equal(await active(),'bg-key-first');
  await click('bg-cancel-keys'); await key('ArrowRight','ArrowRight',39);
  assert.equal(await active(),'bg-key-first','caller preventDefault must cancel group navigation');
  await click('bg-cancel-keys'); await key('ArrowRight','ArrowRight',39); assert.equal(await active(),'bg-key-link');
  assert.match(await evaluate('document.querySelector("#bg-callbacks").textContent'),/Focus: [1-9]\d*; keys: [1-9]\d*/);
  await focus('bg-up'); await key('ArrowDown','ArrowDown',40); assert.equal(await active(),'bg-middle');
  await key('End','End',35); assert.equal(await active(),'bg-down');
  await key('ArrowDown','ArrowDown',40); assert.equal(await active(),'bg-up');
  await key('ArrowRight','ArrowRight',39); assert.equal(await active(),'bg-up','cross-axis keys remain native');
  await focus('bg-rtl-first'); await key('ArrowLeft','ArrowLeft',37); assert.equal(await active(),'bg-rtl-last');
  await key('ArrowRight','ArrowRight',39); assert.equal(await active(),'bg-rtl-first');

  await focus('bg-dyn-first'); await click('bg-dyn-first');
  await waitFor('document.getElementById("bg-dyn-first").getAttribute("aria-busy")==="true"');
  await oneStop('bg-dynamic','bg-dyn-first');
  assert.equal(await active(),'bg-dyn-first','a pending holder must retain focus');
  await key('ArrowRight','ArrowRight',39); assert.equal(await active(),'bg-dyn-link');
  await click('bg-reset'); await focus('bg-dyn-first'); await click('bg-remove-first');
  await waitFor('!document.getElementById("bg-dyn-first")'); await oneStop('bg-dynamic','bg-dyn-link');
  await click('bg-insert'); await waitFor('Boolean(document.getElementById("bg-dyn-extra"))');
  await oneStop('bg-dynamic','bg-dyn-link');
  await focus('bg-dyn-link'); await click('bg-toggle-disabled');
  await waitFor('document.getElementById("bg-dyn-link").tagName==="BUTTON"');
  await oneStop('bg-dynamic','bg-dyn-link'); assert.equal(await active(),'bg-dyn-link');
  assert.equal(await evaluate('document.getElementById("bg-dyn-link").disabled'),false);
  await evaluate('document.addEventListener("keydown",event=>window.bgDisabledKeyPrevented=event.defaultPrevented,{once:true});true');
  await key('ArrowRight','ArrowRight',39);
  assert.equal(await active(),'bg-dyn-link','an all-disabled group keeps its holder');
  assert.equal(await evaluate('window.bgDisabledKeyPrevented'),true,'owned keys must not scroll the page when all members are disabled');
  await click('bg-toggle-disabled'); await waitFor('document.getElementById("bg-dyn-link").tagName==="A"');
  await oneStop('bg-dynamic','bg-dyn-link'); assert.equal(await active(),'bg-dyn-link');

  const wrapped=await evaluate(`['bg-wrap-first','bg-wrap-middle','bg-wrap-last'].map(id=>{const c=getComputedStyle(document.getElementById(id));return [c.borderStartStartRadius,c.borderStartEndRadius,c.borderEndStartRadius,c.borderEndEndRadius,c.borderInlineStartWidth];})`);
  assert.deepEqual(wrapped,[['10px','0px','10px','0px','0px'],['0px','0px','0px','0px','1px'],['0px','10px','0px','10px','1px']]);
  assert.equal(await evaluate('document.getElementById("bg-key-native").hasAttribute("data-astryx-group-position")'),false,'foreign children retain their own paint');
  const gap=await evaluate('(() => {const e=[...document.querySelectorAll("#bg-basic > button")].map(e=>e.getBoundingClientRect());return [e[1].left-e[0].right,e[2].left-e[1].right];})()');
  assert.deepEqual(gap,[0,0],'members must meet without gaps');
  await focus('bg-key-first');
  assert.equal(await evaluate('getComputedStyle(document.getElementById("bg-key-first")).outlineWidth'),'2px');

  await click('bg-menu-trigger'); await waitFor('document.getElementById("bg-member-layer").matches(":popover-open")');
  assert.deepEqual(await evaluate('(() => {const e=document.getElementById("bg-layer-action");return [e.dataset.size,e.dataset.elevation,e.hasAttribute("data-astryx-grouped"),e.hasAttribute("aria-disabled")];})()'),['md','high',false,false]);
  await focus('bg-layer-action'); await key('ArrowRight','ArrowRight',39); assert.equal(await active(),'bg-layer-action','outer group must not steal layer keys');
  await oneStop('bg-menu-group');
  await key('Escape','Escape',27); await waitFor('!document.getElementById("bg-member-layer").matches(":popover-open")');
  assert.equal(await active(),'bg-menu-trigger');
  await click('bg-inner-layer-open'); await waitFor('document.getElementById("bg-inner-layer").matches(":popover-open")');
  await oneStop('bg-inner-group','bg-inner-first');
  await focus('bg-inner-first'); await key('ArrowRight','ArrowRight',39); assert.equal(await active(),'bg-inner-last','group inside layer owns its arrows');
  await key('Escape','Escape',27); await waitFor('!document.getElementById("bg-inner-layer").matches(":popover-open")');
  assert.equal(await active(),'bg-inner-layer-open');
  const doc=await send('DOM.getDocument');
  const {nodeId}=await send('DOM.querySelector',{nodeId:doc.root.nodeId,selector:'#bg-basic'});
  const ax=await send('Accessibility.getPartialAXTree',{nodeId});
  assert.ok(ax.nodes.some(node=>node.role?.value==='group'&&node.name?.value==='Text editing actions'),'native accessibility tree must expose group name');

  await evaluate('(() => {const e=document.createElement("button");e.id="bg-external-native";e.tabIndex=7;e.textContent="External";document.querySelector("#bg-basic").append(e);})()');
  await oneStop('bg-basic'); await focus('bg-external-native'); await oneStop('bg-basic','bg-external-native');
  await evaluate('document.body.append(document.getElementById("bg-external-native"))');
  await waitFor('document.getElementById("bg-external-native").tabIndex===7');
  await evaluate('document.getElementById("bg-external-native").remove()'); await oneStop('bg-basic');
  const countGroups=await evaluate('document[Symbol.for("astryx.dioxus.button-group.runtime")].groups.size');
  await click('bg-mount-toggle');
  await waitFor(`document[Symbol.for('astryx.dioxus.button-group.runtime')].groups.size===${countGroups-1}`);
  await click('bg-mount-toggle'); await oneStop('bg-disposable');
  assert.equal(await evaluate('document[Symbol.for("astryx.dioxus.button-group.runtime")].groups.size'),countGroups);

  const desktopA=await point('bg-copy'),desktopB=await point('bg-cut');
  await mouse('mousePressed',desktopA); await mouse('mouseMoved',desktopB); await mouse('mouseReleased',desktopB);
  assert.equal(await evaluate('document.querySelector("#bg-activations").textContent'),'Activations: 0','desktop dragging does not select release member');
  await mouse('mousePressed',desktopA); await mouse('mouseReleased',desktopA);
  await waitFor('document.querySelector("#bg-activations").textContent==="Activations: 1"');
  await send('Emulation.setDeviceMetricsOverride',{width:390,height:844,deviceScaleFactor:1,mobile:true});
  await send('Emulation.setTouchEmulationEnabled',{enabled:true,maxTouchPoints:1});
  await waitFor('document.getElementById("bg-basic").dataset.astryxTouchAction==="pan-y"');
  const a=await point('bg-copy'),b=await point('bg-cut');
  const beforeBubbles=await evaluate('Number(document.querySelector("#bg-bubbles").textContent.split(":")[1])');
  await touch('touchStart',a);
  await waitFor('document.getElementById("bg-copy").dataset.astryxPress==="on"');
  assert.equal(await evaluate('getComputedStyle(document.getElementById("bg-copy")).transform'),'none','grouped members never scale');
  await touch('touchMove',b); await waitFor('document.getElementById("bg-cut").dataset.astryxPress==="on"');
  await touch('touchEnd');
  await waitFor('document.querySelector("#bg-activations").textContent==="Activations: 2"');
  assert.equal(await evaluate('document.querySelector("#bg-action").textContent'),'Last group action: Cut');
  assert.equal(await evaluate('Number(document.querySelector("#bg-bubbles").textContent.split(":")[1])'),beforeBubbles+1,'tracked activation bubbles once, without a native tail');
  const touchA=await point('bg-copy');
  await touch('touchStart',touchA); await touch('touchMove',{x:touchA.x,y:touchA.y+90}); await touch('touchEnd');
  assert.equal(await evaluate('document.querySelector("#bg-activations").textContent'),'Activations: 2','outside release does nothing');
  const penA=await point('bg-copy'),penB=await point('bg-paste');
  await mouse('mousePressed',penA,'pen'); await mouse('mouseMoved',penB,'pen'); await mouse('mouseReleased',penB,'pen');
  await waitFor('document.querySelector("#bg-activations").textContent==="Activations: 3"');
  assert.equal(await evaluate('document.querySelector("#bg-action").textContent'),'Last group action: Paste');
  const mouseA=await point('bg-copy'),mouseB=await point('bg-cut');
  await mouse('mousePressed',mouseA); await mouse('mouseMoved',mouseB); await mouse('mouseReleased',mouseB);
  assert.equal(await evaluate('document.querySelector("#bg-activations").textContent'),'Activations: 3','a mouse stays native even on a coarse-primary device');
  await click('bg-reset');
  const pendingDisable=await point('bg-dyn-first');
  await touch('touchStart',pendingDisable);
  await waitFor('document.getElementById("bg-dyn-first").dataset.astryxPress==="on"');
  await click('bg-toggle-disabled');
  await waitFor('document.getElementById("bg-dyn-first").getAttribute("aria-disabled")==="true"&&!document.getElementById("bg-dyn-first").hasAttribute("data-astryx-press")');
  await touch('touchEnd');
  assert.equal(await evaluate('document.getElementById("bg-dyn-first").getAttribute("aria-busy")'),null,'disabling a held group cancels activation and feedback');
  await click('bg-reset');
  const menu=await point('bg-menu-trigger'),sibling=await point('bg-menu-first');
  await touch('touchStart',menu); await touch('touchMove',sibling); await touch('touchEnd');
  assert.equal(await evaluate('document.getElementById("bg-member-layer").matches(":popover-open")'),false,'a menu trigger does not join group sliding');
  const menuTap=await point('bg-menu-trigger');
  await touch('touchStart',menuTap); await touch('touchEnd');
  await waitFor('document.getElementById("bg-member-layer").matches(":popover-open")');
  await click('bg-menu-trigger'); await waitFor('!document.getElementById("bg-member-layer").matches(":popover-open")');
  await point('bg-tiny-action');
  const band=await evaluate('(() => {const g=document.getElementById("bg-tiny"),r=g.getBoundingClientRect();return {x:r.right+1,y:r.y+r.height/2,clippedX:r.right+3,top:parseFloat(g.style.getPropertyValue("--_group-hit-top")),bottom:parseFloat(g.style.getPropertyValue("--_group-hit-bottom")),right:parseFloat(g.style.getPropertyValue("--_group-hit-right")),height:r.height};})()');
  assert.ok(band.height+band.top+band.bottom>=44,'28px member gets a 44px vertical target');
  assert.ok(band.right<=2,'growth stops at half the 4px neighbour gap');
  await evaluate('document.getElementById("bg-neighbour").style.marginInlineStart="-2px"');
  await waitFor('parseFloat(document.getElementById("bg-tiny").style.getPropertyValue("--_group-hit-right"))<=1');
  await evaluate('document.getElementById("bg-neighbour").style.marginInlineStart=""');
  await waitFor('parseFloat(document.getElementById("bg-tiny").style.getPropertyValue("--_group-hit-right"))===2');
  await touch('touchStart',{x:band.x,y:band.y}); await touch('touchEnd');
  await waitFor('document.querySelector("#bg-action").textContent==="Last group action: Tiny action"');
  await click('bg-clear-action');
  await touch('touchStart',{x:band.clippedX,y:band.y}); await touch('touchEnd');
  assert.ok(['Last group action: None','Last group action: Neighbour'].includes(await evaluate('document.querySelector("#bg-action").textContent')), 'the group must not steal its neighbour half-gap; native touch adjustment may choose the neighbour');
  await click('bg-clear-action');
  await point('bg-wide-action');
  const empty=await evaluate('(() => {const r=document.getElementById("bg-wide").getBoundingClientRect();return {x:r.left+200,y:r.top+r.height/2};})()');
  await touch('touchStart',empty); await touch('touchEnd');
  assert.equal(await evaluate('document.querySelector("#bg-action").textContent'),'Last group action: None','blank wide-group space is not an action');
  await send('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:'reduce'},{name:'forced-colors',value:'active'}]});
  const forcedPoint=await point('bg-copy'); await touch('touchStart',forcedPoint);
  assert.equal(await evaluate('getComputedStyle(document.getElementById("bg-copy")).transform'),'none');
  await touch('touchCancel');
  assert.equal(await evaluate('getComputedStyle(document.getElementById("bg-copy")).borderInlineStartStyle'),'solid');
  if(process.env.GROUP_SCREENSHOT_PATH) {
    const screenshot=await send('Page.captureScreenshot',{format:'png',captureBeyondViewport:false});
    await writeFile(process.env.GROUP_SCREENSHOT_PATH,Buffer.from(screenshot.data,'base64'));
  }
  assert.deepEqual(browserErrors,[],'no uncaught browser exceptions');
  console.log('ButtonGroup browser checks passed: upstream light/dark named-pattern metrics; single Tab stop, native members, caller cancellation, RTL/vertical keys, dynamic busy/disabled/removal/replacement, wrapped geometry, layer isolation, cleanup, touch/pen/native mouse, bounded hit targets, reduced motion and forced colors.');
} finally {
  await send('Emulation.setEmulatedMedia',{features:[]});
  await send('Emulation.setTouchEmulationEnabled',{enabled:false});
  socket.close(); await fetch(`${cdpUrl}/json/close/${target.id}`);
}
