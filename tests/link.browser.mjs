// Link contracts; Node 22+, Chrome CDP, rebuilt showcase. No npm dependencies.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
const cdp=process.env.CDP_URL??'http://127.0.0.1:9222',url=process.env.SHOWCASE_URL??'http://127.0.0.1:8080';
const target=await(await fetch(`${cdp}/json/new?about:blank`,{method:'PUT'})).json();
const socket=new WebSocket(target.webSocketDebuggerUrl);await new Promise((resolve,reject)=>{socket.addEventListener('open',resolve,{once:true});socket.addEventListener('error',reject,{once:true});});
let serial=0;const pending=new Map(),errors=[];
socket.addEventListener('message',({data})=>{const message=JSON.parse(data);if(message.method==='Runtime.exceptionThrown')errors.push(message.params.exceptionDetails.exception?.description??message.params.exceptionDetails.text);const request=pending.get(message.id);if(!request)return;pending.delete(message.id);clearTimeout(request.timer);if(message.error)request.reject(new Error(message.error.message));else request.resolve(message.result);});
function send(method,params={}){return new Promise((resolve,reject)=>{const id=++serial,timer=setTimeout(()=>{pending.delete(id);reject(new Error(`CDP timeout ${method}`));},15000);pending.set(id,{resolve,reject,timer});socket.send(JSON.stringify({id,method,params}));});}
async function evaluate(expression){const result=await send('Runtime.evaluate',{expression,returnByValue:true,awaitPromise:true});assert.equal(result.exceptionDetails,undefined,JSON.stringify(result.exceptionDetails));return result.result.value;}
async function waitFor(expression){const deadline=Date.now()+15000;while(Date.now()<deadline){if(await evaluate(expression))return;await new Promise(resolve=>setTimeout(resolve,25));}assert.fail(`Timed out: ${expression}`);}
const click=id=>evaluate(`document.getElementById(${JSON.stringify(id)}).click()`);
const focus=id=>evaluate(`document.getElementById(${JSON.stringify(id)}).focus()`);
const active=()=>evaluate('document.activeElement.id');
const count=()=>evaluate('Number(document.getElementById("ln-actions").textContent.split(":")[1])');
async function key(key,code,windowsVirtualKeyCode){await send('Input.dispatchKeyEvent',{type:'keyDown',key,code,windowsVirtualKeyCode,text:key==='Enter'?'\r':key===' '?' ':undefined});await send('Input.dispatchKeyEvent',{type:'keyUp',key,code,windowsVirtualKeyCode});}
async function point(id){return evaluate(`new Promise((resolve,reject)=>{const e=document.getElementById(${JSON.stringify(id)});let last=null,stable=0,frames=0,positioned=false;const tick=()=>{const r=e.getBoundingClientRect(),p=[r.x,r.y,scrollX,scrollY].join(',');stable=p===last?stable+1:0;last=p;if(stable>=4&&!positioned){e.scrollIntoView({block:'center'});positioned=true;stable=0;last=null;requestAnimationFrame(tick);}else if(stable>=4)resolve({x:r.x+r.width/2,y:r.y+r.height/2});else if(++frames>240)reject(new Error('Layout/scroll did not settle'));else requestAnimationFrame(tick);};requestAnimationFrame(tick);})`);}
async function mouse(type,position){await send('Input.dispatchMouseEvent',{type,...position,button:'left',buttons:type==='mouseReleased'?0:1,clickCount:1});}
async function touch(type,position){await send('Input.dispatchTouchEvent',{type,touchPoints:position?[{...position,id:1,radiusX:3,radiusY:3}]:[]});}
const metrics=link=>{const c=getComputedStyle(link),label=link.querySelector('.astryx-text'),s=getComputedStyle(label),r=link.getBoundingClientRect(),svg=link.querySelector('svg');return {text:label.textContent,href:link.getAttribute('href'),target:link.getAttribute('target'),rel:link.getAttribute('rel'),color:link.dataset.color,rootColor:c.color,labelColor:s.color,display:c.display,font:s.fontSize,weight:s.fontWeight,leading:s.lineHeight,decoration:c.textDecorationLine,gap:c.gap,width:r.width,height:r.height,iconWidth:svg?.getBoundingClientRect().width??null,iconHeight:svg?.getBoundingClientRect().height??null};};
async function ax(id){const root=await send('DOM.getDocument');const node=await send('DOM.querySelector',{nodeId:root.root.nodeId,selector:'#'+id});return (await send('Accessibility.getPartialAXTree',{nodeId:node.nodeId})).nodes.find(node=>node.role?.value==='link'||node.role?.value==='button');}
try{
  await send('Page.enable');await send('Runtime.enable');await send('Page.bringToFront');await send('Emulation.setTouchEmulationEnabled',{enabled:false});await send('Emulation.setDeviceMetricsOverride',{width:1200,height:1400,deviceScaleFactor:1,mobile:false});await send('Page.navigate',{url});
  await waitFor('Boolean(document.getElementById("ln-documentation"))&&Boolean(document[Symbol.for("astryx.dioxus.button.runtime")]?.records.has(document.getElementById("ln-documentation")))');await evaluate('document.fonts.ready.then(()=>true)');
  const reference=JSON.parse(await readFile(new URL('./fixtures/link-neutral-reference.json',import.meta.url),'utf8'));
  const ids=['ln-documentation','ln-github','ln-mdn','ln-react','ln-inline-docs','ln-settings','ln-profile','ln-help'];
  for(const mode of ['light','dark']){
    await evaluate(`document.getElementById('link-examples').style.colorScheme=${JSON.stringify(mode)}`);await new Promise(resolve=>setTimeout(resolve,600));
    const actual=await evaluate(`${JSON.stringify(ids)}.map(id=>(${metrics})(document.getElementById(id)))`);assert.equal(reference.measurements[mode].length,8);
    actual.forEach((link,index)=>{const expected=reference.measurements[mode][index];assert.ok(Math.abs(link.width-expected.width)<=1/32,`${mode} ${link.text} width ${link.width} vs ${expected.width}`);link.width=expected.width;assert.deepEqual(link,expected,`${mode} ${link.text}`);});
  }
  await evaluate('document.getElementById("link-examples").style.colorScheme="light"');
  assert.deepEqual(await evaluate(`(()=>{const actual=document.querySelector('#ln-github svg'),expected=new DOMParser().parseFromString(${JSON.stringify(reference.externalArtwork)},'image/svg+xml').documentElement;const signature=e=>({viewBox:e.getAttribute('viewBox'),stroke:e.getAttribute('stroke-width'),paths:[...e.querySelectorAll('path')].map(p=>p.getAttribute('d'))});return [signature(actual),signature(expected)]})()` ).then(values=>values[0]),await evaluate(`(()=>{const e=new DOMParser().parseFromString(${JSON.stringify(reference.externalArtwork)},'image/svg+xml').documentElement;return {viewBox:e.getAttribute('viewBox'),stroke:e.getAttribute('stroke-width'),paths:[...e.querySelectorAll('path')].map(p=>p.getAttribute('d'))}})()`),'registered Neutral glyph matches upstream, not the core fallback');
  await evaluate('(()=>{const s=document.createElement("style");s.id="link-text-theme-test";s.textContent=".astryx-text { color: rgb(255, 0, 0); }";document.head.append(s)})()');
  assert.equal(await evaluate('getComputedStyle(document.querySelector("#ln-documentation .astryx-text")).color'),await evaluate('getComputedStyle(document.getElementById("ln-documentation")).color'),'a Text theme cannot steal builtin Link paint');
  await evaluate('document.getElementById("link-text-theme-test").remove()');
  assert.equal(await evaluate('document[Symbol.for("astryx.dioxus.button.runtime")].records.has(document.getElementById("ln-parent-clamp"))'),false,'disabled truncation tooltip adds no registration');
  assert.equal(await evaluate('document[Symbol.for("astryx.dioxus.button.runtime")].records.has(document.querySelector("#ln-flow .astryx-text"))'),false,'unclamped label adds no tooltip registration');
  assert.deepEqual(await evaluate('["ln-documentation","ln-action","ln-disabled","ln-disabled-action","ln-rejected"].map(id=>document.getElementById(id).tagName)'),['A','BUTTON','A','BUTTON','A']);
  const start=await count();await focus('ln-action');await key('Enter','Enter',13);await key(' ','Space',32);const action=await point('ln-action');await mouse('mousePressed',action);await mouse('mouseReleased',action);await waitFor(`document.getElementById('ln-actions').textContent==='Actions: ${start+3}'`);
  for(const id of ['ln-disabled','ln-disabled-action','ln-rejected']){await focus(id);assert.equal(await active(),id);await key('Enter','Enter',13);await key(' ','Space',32);await click(id);assert.equal(await count(),start+3);assert.equal(await evaluate(`document.getElementById('${id}').hasAttribute('href')`),false);}
  await focus('ln-callbacks');await key('ArrowRight','ArrowRight',39);await waitFor('document.getElementById("ln-callback-count").textContent==="Mounts: 1; Keys: 1"');
  assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("ln-blank");return [e.target,e.rel]})()'),['_blank','help noopener noreferrer']);
  assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("ln-download");return [e.download,e.referrerPolicy]})()'),['report.txt','no-referrer']);
  assert.ok((await ax('ln-github')).name.value.includes('opens in new tab'));
  assert.equal((await ax('ln-labelled')).name.value,'Open documentation');
  const disabledAx=await ax('ln-disabled');assert.equal(disabledAx.role.value,'link');assert.ok(disabledAx.properties.some(property=>property.name==='disabled'&&property.value.value===true));
  await click('ln-article-router');await waitFor('document.getElementById("ln-route").textContent==="Router location: /articles/getting-started"');
  await click('ln-article-native');await waitFor('location.hash==="#style-guide"');
  assert.equal(await evaluate('document.getElementById("ln-route").textContent'),'Router location: /articles/getting-started');
  assert.equal(await evaluate('document.getElementById("ln-article-native").dataset.adapter'),'native');
  assert.equal(await evaluate('document.getElementById("ln-article-fallback").dataset.adapter'),'native');
  assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("ln-absent");return [e.dataset.adapter,e.dataset.hasDestination,e.hasAttribute("href")]})()'),['router','false',false],'None is not rewritten to href=""');
  await focus('ln-router');await click('ln-toggle-disabled');await waitFor('document.getElementById("ln-router").tagName==="BUTTON"');assert.equal(await active(),'ln-router');
  const beforeDisabled=await count();await click('ln-router');await key('Enter','Enter',13);assert.equal(await count(),beforeDisabled);
  await click('ln-toggle-disabled');await waitFor('document.getElementById("ln-router").tagName==="A"');assert.equal(await active(),'ln-router');
  await focus('ln-router-passive');await click('ln-toggle-disabled');await waitFor('document.getElementById("ln-router-passive").getAttribute("aria-disabled")==="true"');
  assert.equal(await active(),'ln-router-passive','router-to-disabled-anchor replacement preserves focus');
  assert.equal(await evaluate('document.getElementById("ln-router-passive").hasAttribute("href")'),false);
  await click('ln-toggle-disabled');await waitFor('document.getElementById("ln-router-passive").getAttribute("href")==="/details"');assert.equal(await active(),'ln-router-passive');
  const commentLinks=await evaluate('[...document.querySelectorAll("#ln-comments .astryx-link")].map(e=>[e.querySelector(".astryx-text").textContent,e.getAttribute("href"),e.target])');
  assert.deepEqual(commentLinks,[['https://example.com/spec','https://example.com/spec','_blank'],['support@example.com','mailto:support@example.com',''],['#123','https://example.com/issues/123',''],['#456','https://example.com/issues/456','']]);
  assert.ok(await evaluate('document.getElementById("ln-comments").textContent.endsWith("No links here, just a plain note.")'));
  for(const id of ['ln-long','ln-long-button','ln-long-external'])assert.ok(await evaluate(`(()=>{const e=document.getElementById('${id}'),r=e.getBoundingClientRect(),p=e.parentElement.getBoundingClientRect();return r.width<=200&&r.left>=p.left&&r.right<=p.right+0.02&&r.height>20})()`),`${id} wraps without overflowing`);
  assert.equal(await evaluate('getComputedStyle(document.getElementById("ln-flow")).display'),'inline');
  assert.notEqual(await evaluate('getComputedStyle(document.getElementById("ln-layer-link")).display'),'inline');
  await focus('ln-flow');assert.equal(await evaluate('getComputedStyle(document.getElementById("ln-flow")).outlineOffset'),'-2px');
  assert.equal(await evaluate('getComputedStyle(document.getElementById("ln-inherit")).fontSize'),'17px');
  assert.deepEqual(await evaluate('(()=>{const c=getComputedStyle(document.querySelector("#ln-size .astryx-text"));return [c.fontSize,c.fontWeight]})()'),['24px','700']);
  const hover=await point('ln-github');await send('Input.dispatchMouseEvent',{type:'mouseMoved',...hover});await new Promise(resolve=>setTimeout(resolve,250));
  assert.ok(await evaluate('(()=>{const e=document.getElementById("ln-github"),c=getComputedStyle(e);return c.textDecorationLine==="underline"&&c.color===getComputedStyle(e.querySelector(".astryx-text")).color&&c.color===getComputedStyle(e.querySelector("svg")).color})()'),'hover color is shared by root/label/icon');
  await point('ln-settings');await focus('ln-settings');await waitFor('Boolean(document.querySelector("#ln-settings + [role=tooltip]:popover-open"))');
  const hint=await ax('ln-settings');assert.equal(hint.name.value,'Settings');assert.equal(hint.description.value,'Configure your account settings');
  assert.ok(await evaluate('(()=>{const e=document.getElementById("ln-settings"),t=e.nextElementSibling;return t.getBoundingClientRect().bottom<=e.getBoundingClientRect().top})()'),'explicit hint opens above');
  await key('Escape','Escape',27);await waitFor('!document.querySelector("#ln-settings + [role=tooltip]:popover-open")');
  const cut=await point('ln-clamp-one');await send('Input.dispatchMouseEvent',{type:'mouseMoved',...cut});await waitFor('Boolean(document.querySelector("#ln-clamp-one + [role=tooltip]:popover-open"))');
  const full=await evaluate('document.querySelector("#ln-clamp-one .astryx-text").textContent');
  assert.equal((await ax('ln-clamp-one')).name.value,full,'automatic tip cannot duplicate the link name');
  assert.equal(await evaluate('document.getElementById("ln-clamp-one").hasAttribute("aria-describedby")'),false);
  await click('ln-widen');await waitFor('!document.querySelector("#ln-clamp-one + [role=tooltip]:popover-open")');
  await click('ln-widen');
  await focus('ln-dynamic-tip');await waitFor('Boolean(document.querySelector("#ln-dynamic-tip + [role=tooltip]:popover-open"))');
  await evaluate('window.oldLink=document.getElementById("ln-dynamic-tip");window.oldTip=oldLink.nextElementSibling.id');await click('ln-remove-tip');
  await waitFor('!document.getElementById("ln-dynamic-tip")&&!document.getElementById(window.oldTip)&&!document[Symbol.for("astryx.dioxus.button.runtime")].records.has(window.oldLink)');
  await send('Emulation.setTouchEmulationEnabled',{enabled:true,maxTouchPoints:1});await send('Emulation.setDeviceMetricsOverride',{width:390,height:844,deviceScaleFactor:1,mobile:true});await waitFor('matchMedia("(pointer: coarse)").matches');
  const mobile=await point('ln-action'),beforeTouch=await count();await touch('touchStart',mobile);await waitFor('document.getElementById("ln-action").dataset.astryxPress==="on"');
  assert.equal(await evaluate('getComputedStyle(document.getElementById("ln-action")).transform'),'none','text links never take Button scale');await touch('touchEnd');await waitFor(`document.getElementById('ln-actions').textContent==='Actions: ${beforeTouch+1}'`);
  const cancel=await point('ln-action');await touch('touchStart',cancel);await touch('touchCancel');assert.equal(await count(),beforeTouch+1);
  await send('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:'reduce'},{name:'forced-colors',value:'active'}]});
  await key('Tab','Tab',9);await focus('ln-action');assert.equal(await evaluate('getComputedStyle(document.getElementById("ln-action")).outlineStyle'),'solid');
  assert.deepEqual(errors,[],'no uncaught browser exceptions');
  console.log('Link browser checks passed: live-source light/dark named metrics, optional destinations, routing/safety, all element forms and disabled focus, names/new-tab hints, callbacks, hover/press, typography/wrapping/clamps/layers, lazy tooltips/live resize/cleanup, touch, forced colors.');
}finally{await send('Emulation.setEmulatedMedia',{features:[]});await send('Emulation.setTouchEmulationEnabled',{enabled:false});socket.close();await fetch(`${cdp}/json/close/${target.id}`);}
