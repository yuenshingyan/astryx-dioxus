// Standalone IconButton contracts. Node 22+, Chrome CDP, rebuilt showcase.
// See docs/icon-button-port.md. No npm dependencies.
import assert from 'node:assert/strict';
import {readFile,writeFile} from 'node:fs/promises';

const cdpUrl=process.env.CDP_URL??'http://127.0.0.1:9222';
const showcaseUrl=process.env.SHOWCASE_URL??'http://127.0.0.1:8080';
const target=await(await fetch(`${cdpUrl}/json/new?about:blank`,{method:'PUT'})).json();
const socket=new WebSocket(target.webSocketDebuggerUrl);
await new Promise((resolve,reject)=>{socket.addEventListener('open',resolve,{once:true});socket.addEventListener('error',reject,{once:true});});
let nextId=0;
const requests=new Map(),browserErrors=[];
socket.addEventListener('message',({data})=>{
  const message=JSON.parse(data);
  if(message.method==='Runtime.exceptionThrown') browserErrors.push(message.params.exceptionDetails.exception?.description??message.params.exceptionDetails.text);
  const request=requests.get(message.id); if(!request)return;
  requests.delete(message.id);clearTimeout(request.timer);
  if(message.error)request.reject(new Error(message.error.message));else request.resolve(message.result);
});
function send(method,params={}) {
  return new Promise((resolve,reject)=>{const id=++nextId;const timer=setTimeout(()=>{requests.delete(id);reject(new Error(`CDP timeout: ${method}`));},15000);requests.set(id,{resolve,reject,timer});socket.send(JSON.stringify({id,method,params}));});
}
async function evaluate(expression) {
  const result=await send('Runtime.evaluate',{expression,returnByValue:true,awaitPromise:true});
  assert.equal(result.exceptionDetails,undefined,JSON.stringify(result.exceptionDetails));return result.result.value;
}
async function waitFor(expression) {
  const deadline=Date.now()+15000;
  while(Date.now()<deadline){if(await evaluate(expression))return;await new Promise(resolve=>setTimeout(resolve,25));}
  assert.fail(`Timed out waiting for: ${expression}`);
}
const click=id=>evaluate(`document.getElementById(${JSON.stringify(id)}).click()`);
const focus=id=>evaluate(`document.getElementById(${JSON.stringify(id)}).focus()`);
const active=()=>evaluate('document.activeElement.id');
const count=id=>evaluate(`Number(document.getElementById(${JSON.stringify(id)}).textContent.split(':')[1])`);
async function key(key,code,windowsVirtualKeyCode) {
  await send('Input.dispatchKeyEvent',{type:'keyDown',key,code,windowsVirtualKeyCode,text:key==='Enter'?'\r':key===' '?' ':undefined});
  await send('Input.dispatchKeyEvent',{type:'keyUp',key,code,windowsVirtualKeyCode});
}
async function point(id) {
  return evaluate(`new Promise((resolve,reject)=>{const element=document.getElementById(${JSON.stringify(id)});let previous=null,stable=0,frames=0,isPositioned=false;const sample=()=>{const r=element.getBoundingClientRect(),position=[r.x,r.y,scrollX,scrollY].join(',');stable=position===previous?stable+1:0;previous=position;if(stable>=4&&!isPositioned){element.scrollIntoView({block:'center'});isPositioned=true;stable=0;previous=null;requestAnimationFrame(sample);}else if(stable>=4)resolve({x:r.x+r.width/2,y:r.y+r.height/2});else if(++frames>240)reject(new Error('Target did not stop scrolling'));else requestAnimationFrame(sample);};requestAnimationFrame(sample);})`);
}
async function mouse(type,position) {await send('Input.dispatchMouseEvent',{type,...position,button:'left',buttons:type==='mouseReleased'?0:1,clickCount:1});}
async function touch(type,position) {await send('Input.dispatchTouchEvent',{type,touchPoints:position?[{...position,id:1,radiusX:3,radiusY:3}]:[]});}
const metrics=button=>{
  const c=getComputedStyle(button),r=button.getBoundingClientRect(),svg=button.querySelector('svg'),s=svg.getBoundingClientRect();
  const keys=['d','cx','cy','r','x','y','width','height','rx','ry'];
  return {label:button.getAttribute('aria-label'),variant:button.dataset.variant,size:button.dataset.size,elevation:button.dataset.elevation||'none',width:r.width,height:r.height,padding:[c.paddingTop,c.paddingRight,c.paddingBottom,c.paddingLeft],radius:c.borderRadius,background:c.backgroundColor,color:c.color,shadow:c.boxShadow,font:c.fontSize,fontWeight:c.fontWeight,viewBox:svg.getAttribute('viewBox'),iconWidth:s.width,iconHeight:s.height,centerX:+((s.left+s.width/2)-(r.left+r.width/2)).toFixed(4),centerY:+((s.top+s.height/2)-(r.top+r.height/2)).toFixed(4),shapes:[...svg.children].map(shape=>{const style=getComputedStyle(shape);return {tag:shape.tagName,geometry:Object.fromEntries(keys.filter(key=>shape.hasAttribute(key)).map(key=>[key,shape.getAttribute(key)])),fill:style.fill,stroke:style.stroke,strokeWidth:style.strokeWidth,cap:style.strokeLinecap,join:style.strokeLinejoin};})};
};
try {
  await send('Page.enable');await send('Runtime.enable');await send('Page.bringToFront');
  await send('Emulation.setTouchEmulationEnabled',{enabled:false});
  await send('Emulation.setDeviceMetricsOverride',{width:1200,height:1400,deviceScaleFactor:1,mobile:false});
  await send('Page.navigate',{url:showcaseUrl});
  await waitFor('Boolean(document.getElementById("ib-settings"))&&Boolean(document[Symbol.for("astryx.dioxus.button.runtime")]?.records.has(document.getElementById("ib-settings")))');
  await evaluate('document.fonts.ready.then(()=>true)');
  const reference=JSON.parse(await readFile(new URL('./fixtures/icon-button-neutral-reference.json',import.meta.url),'utf8'));
  for(const mode of ['light','dark']) {
    await evaluate(`document.getElementById('icon-button-examples').style.colorScheme=${JSON.stringify(mode)}`);
    await new Promise(resolve=>setTimeout(resolve,600));
    const actual=await evaluate(`[...document.querySelectorAll('#ib-showcase .astryx-button,#ib-action-bar .astryx-button,#ib-floating .astryx-button,#ib-loading-toggle .astryx-button,#ib-tooltips .astryx-button')].map(${metrics})`);
    assert.equal(actual.length,16);assert.equal(reference.measurements[mode].length,16);
    actual.forEach((button,index)=>{
      const expected=reference.measurements[mode][index];
      assert.ok(Math.abs(button.width-expected.width)<=1/32,`${mode} ${button.label} width`);
      button.width=expected.width;
      assert.deepEqual(button,expected,`${mode} named control ${index}: ${button.label}`);
    });
  }
  await evaluate('document.getElementById("icon-button-examples").style.colorScheme="light"');
  const matrix=await evaluate(`Array.from(document.querySelectorAll('[id^="ib-matrix-"]')).map(${metrics})`);
  assert.equal(matrix.length,12);
  matrix.forEach(button=>{
    const height={sm:28,md:32,lg:36}[button.size],glyph=button.size==='lg'?20:16;
    assert.equal(button.width,height);assert.equal(button.height,height);assert.deepEqual(button.padding,['0px','0px','0px','0px']);
    assert.equal(button.iconWidth,glyph);assert.equal(button.iconHeight,glyph);assert.equal(button.centerX,0);assert.equal(button.centerY,0);
  });
  const extras=await evaluate('["ib-inherited","ib-override","ib-width","ib-aspect","ib-aspect-width"].map(id=>{const e=document.getElementById(id),r=e.getBoundingClientRect();return [r.width,r.height];})');
  assert.deepEqual(extras,[[28,28],[36,36],[96,32],[64,32],[96,32]],'inherited sizes, explicit width, inherited theme aspect, and width precedence');
  assert.equal(await evaluate('document.querySelector("#ib-explicit-icon svg").getBoundingClientRect().width'),24);
  const shadows=await evaluate('["none","low","med","high"].map(tier=>getComputedStyle(document.getElementById("ib-elevation-"+tier)).boxShadow)');
  assert.equal(shadows[0],'none');assert.equal(new Set(shadows).size,4);
  await key('Tab','Tab',9);await focus('ib-settings');
  await new Promise(resolve=>setTimeout(resolve,400));
  assert.equal(await evaluate('Boolean(document.querySelector("#ib-settings + [role=tooltip]"))'),false,'no automatic label tooltip');
  assert.equal(await evaluate('document.getElementById("ib-settings").textContent.includes("Settings")'),false,'no visible label');
  const before=await count('ib-activations');
  await key('Enter','Enter',13);await key(' ','Space',32);
  const settingsPoint=await point('ib-settings');await mouse('mousePressed',settingsPoint);await mouse('mouseReleased',settingsPoint);
  await waitFor(`document.getElementById('ib-activations').textContent==='Activations: ${before+3}'`);
  await focus('ib-mounted');await key('ArrowRight','ArrowRight',39);
  await waitFor('document.getElementById("ib-callbacks").textContent==="Mounts: 1; Focuses: 1; Keys: 1"');
  for(const [state,blocked,busy] of [['disabled',true,false],['loading',true,true],['interruptible',false,true],['disabled-loading',true,true],['rest',false,false]]) {
    await click('ib-next-state');await waitFor(`document.getElementById('ib-state').textContent===${JSON.stringify(state)}`);
    await waitFor(`document.getElementById('ib-matrix-primary-sm').hasAttribute('aria-disabled')===${blocked}&&document.getElementById('ib-matrix-primary-sm').hasAttribute('aria-busy')===${busy}`);
    await focus('ib-matrix-primary-sm');assert.equal(await active(),'ib-matrix-primary-sm');
    const value=await count('ib-activations');await key('Enter','Enter',13);await click('ib-matrix-primary-sm');
    assert.equal(await count('ib-activations'),value+(blocked?0:2),state);
    assert.equal(await evaluate('document.getElementById("ib-matrix-primary-sm").disabled'),false);
  }
  await focus('ib-tip-search');await waitFor('Boolean(document.querySelector("#ib-tip-search + [role=tooltip]:popover-open"))');
  const documentNode=await send('DOM.getDocument');
  const {nodeId}=await send('DOM.querySelector',{nodeId:documentNode.root.nodeId,selector:'#ib-tip-search'});
  const ax=await send('Accessibility.getPartialAXTree',{nodeId});
  assert.ok(ax.nodes.some(node=>node.role?.value==='button'&&node.name?.value==='Search'&&node.description?.value==='Search items'));
  await key('Escape','Escape',27);assert.equal(await evaluate('Boolean(document.querySelector("#ib-tip-search + [role=tooltip]:popover-open"))'),false);
  await focus('ib-tip-more');await waitFor('Boolean(document.querySelector("#ib-tip-more + [role=tooltip]:popover-open"))');
  assert.equal(await evaluate('document.getElementById("ib-tip-more").hasAttribute("aria-describedby")'),false,'same-name tooltip does not duplicate description');
  await key('Escape','Escape',27);
  const hover=await point('ib-tip-copy');await send('Input.dispatchMouseEvent',{type:'mouseMoved',...hover});
  await waitFor('Boolean(document.querySelector("#ib-tip-copy + [role=tooltip]:popover-open"))');
  await key('Escape','Escape',27);
  await click('ib-load-copy');await waitFor('document.getElementById("ib-load-copy").getAttribute("aria-busy")==="true"');
  assert.equal(await evaluate('document.querySelector("#ib-load-copy [role=status]").textContent'),'Loading');
  await click('ib-load-search');await waitFor('document.getElementById("ib-load-search").getAttribute("aria-busy")==="true"&&!document.getElementById("ib-load-copy").hasAttribute("aria-busy")');
  await waitFor('!document.getElementById("ib-load-search").hasAttribute("aria-busy")');
  await click('ib-owned-action');await click('ib-owned-action');
  await waitFor('document.getElementById("ib-owned-action").getAttribute("aria-busy")==="true"');
  assert.equal(await count('ib-starts'),1);await focus('ib-owned-action');assert.equal(await active(),'ib-owned-action');
  await waitFor('document.getElementById("ib-completions").textContent==="Completions: 1"');
  await click('ib-interrupt');await click('ib-interrupt');await waitFor('document.getElementById("ib-interrupts").textContent==="Interrupts: 2"');
  assert.equal(await evaluate('document.getElementById("ib-interrupt").hasAttribute("aria-disabled")'),false);
  await waitFor('!document.getElementById("ib-interrupt").hasAttribute("aria-busy")');
  await focus('ib-router');await click('ib-toggle-link');
  await waitFor('document.getElementById("ib-router").tagName==="BUTTON"');assert.equal(await active(),'ib-router');
  await click('ib-toggle-link');await waitFor('document.getElementById("ib-router").tagName==="A"');assert.equal(await active(),'ib-router');
  assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("ib-router");return [e.dataset.adapter,e.getAttribute("href"),e.target,e.rel]})()'),['icon-router','#ib-destination','_self','help']);
  await click('ib-native-link');await waitFor('location.hash==="#ib-destination"');
  await evaluate('window.iconSubmitters=[];document.getElementById("ib-form").addEventListener("submit",event=>window.iconSubmitters.push([event.submitter.name,event.submitter.value]));');
  await click('ib-submit');await waitFor('document.getElementById("ib-submits").textContent==="Submits: 1"');
  assert.deepEqual(await evaluate('window.iconSubmitters'),[['action','icon-submit']]);
  await click('ib-blocked-submit');assert.equal(await count('ib-submits'),1);
  await evaluate('document.getElementById("ib-form-input").value="Edited"');await click('ib-reset');
  assert.equal(await evaluate('document.getElementById("ib-form-input").value'),'Original');
  await send('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:'reduce'}]});
  const finePoint=await point('ib-settings');await mouse('mousePressed',finePoint);
  assert.equal(await evaluate('getComputedStyle(document.getElementById("ib-settings")).transform'),'none');await mouse('mouseReleased',finePoint);
  await click('ib-owned-action');await waitFor('document.getElementById("ib-owned-action").getAttribute("aria-busy")==="true"');
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#ib-owned-action .astryx-button__spinner-overlay")).opacity'),'1','loading must appear at once under reduced motion');
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#ib-self-colored .astryx-button__icon svg")).visibility'),'hidden','self-colored artwork cannot leak through spinner');
  await evaluate('window.removedIcon=document.getElementById("ib-owned-action")');await click('ib-remove-action');
  await waitFor('!document.getElementById("ib-owned-action")&&!document[Symbol.for("astryx.dioxus.button.runtime")].records.has(window.removedIcon)');
  await new Promise(resolve=>setTimeout(resolve,1600));assert.equal(await count('ib-completions'),1,'unmount cancels the owned future');
  await send('Emulation.setTouchEmulationEnabled',{enabled:true,maxTouchPoints:1});
  await send('Emulation.setDeviceMetricsOverride',{width:390,height:844,deviceScaleFactor:1,mobile:true});
  await waitFor('matchMedia("(pointer: coarse)").matches');
  const touchPoint=await point('ib-settings');const touchBefore=await count('ib-activations');
  await touch('touchStart',touchPoint);assert.equal(await evaluate('getComputedStyle(document.getElementById("ib-settings")).transform'),'none');
  await touch('touchEnd');await waitFor(`document.getElementById('ib-activations').textContent==='Activations: ${touchBefore+1}'`);
  const cancelPoint=await point('ib-settings');await touch('touchStart',cancelPoint);await touch('touchCancel');
  assert.equal(await count('ib-activations'),touchBefore+1);
  const disabledPoint=await point('ib-disabled-tip');await touch('touchStart',disabledPoint);await touch('touchEnd');
  await waitFor('Boolean(document.querySelector("#ib-disabled-tip + [role=tooltip]:popover-open"))');assert.equal(await count('ib-activations'),touchBefore+1);
  await send('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:'reduce'},{name:'forced-colors',value:'active'}]});
  assert.equal(await evaluate('getComputedStyle(document.getElementById("ib-settings")).borderTopStyle'),'solid');
  if(process.env.ICON_SCREENSHOT_PATH){await point('ib-settings');const shot=await send('Page.captureScreenshot',{format:'png'});await writeFile(process.env.ICON_SCREENSHOT_PATH,Buffer.from(shot.data,'base64'));}
  assert.deepEqual(browserErrors,[],'no uncaught browser exceptions');
  console.log('IconButton browser checks passed: all 16 original controls in light/dark, exact glyphs, sizes/centering/aspect/width, keyboard/mouse/touch, callbacks, tooltips/AX names, all availability policies, loading/async/unmount, links/focus, forms, reduced motion and forced colors.');
} finally {
  await send('Emulation.setEmulatedMedia',{features:[]});await send('Emulation.setTouchEmulationEnabled',{enabled:false});
  socket.close();await fetch(`${cdpUrl}/json/close/${target.id}`);
}
