// Run under browser_lock.py. Node 22+, no npm packages; isolated CDP target only.
import assert from 'node:assert/strict';
import {readFile,writeFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
const endpoint=process.env.CDP_URL??'http://127.0.0.1:9222';
const target=await(await fetch(`${endpoint}/json/new?about:blank`,{method:'PUT'})).json();
const ws=new WebSocket(target.webSocketDebuggerUrl);
await new Promise((resolve,reject)=>{ws.onopen=resolve;ws.onerror=reject});
let serial=0;const pending=new Map(),errors=[],requests=[],blocked=new Set();
ws.onmessage=({data})=>{const m=JSON.parse(data);if(m.method==='Runtime.exceptionThrown')errors.push(m.params);if(m.method==='Network.requestWillBeSent')requests.push({id:m.params.requestId,url:m.params.request.url});if(m.method==='Network.loadingFailed'&&m.params.blockedReason)blocked.add(m.params.requestId);const p=pending.get(m.id);if(p){pending.delete(m.id);m.error?p.reject(Error(JSON.stringify(m.error))):p.resolve(m.result)}};
const send=(method,params={})=>new Promise((resolve,reject)=>{const id=++serial;pending.set(id,{resolve,reject});ws.send(JSON.stringify({id,method,params}))});
const evaluate=async expression=>{const r=await send('Runtime.evaluate',{expression,returnByValue:true,awaitPromise:true});assert.equal(r.exceptionDetails,undefined,JSON.stringify(r.exceptionDetails));return r.result.value};
const wait=ms=>new Promise(r=>setTimeout(r,ms));
async function until(expression){for(let n=0;n<300;n++){if(await evaluate(expression))return;await wait(100)}assert.fail(`Timed out: ${expression}`)}
function metrics(){return [...document.querySelectorAll('#code-showcase code,#code-inline code,#code-content code,#code-sizes code')].map(e=>{const s=getComputedStyle(e),r=e.getBoundingClientRect(),p=e.parentElement.getBoundingClientRect();return {text:e.textContent,css:Object.fromEntries(['display','fontFamily','fontSize','fontWeight','lineHeight','color','backgroundColor','paddingInlineStart','paddingInlineEnd','paddingBlockStart','paddingBlockEnd','borderRadius','overflowWrap','wordBreak','direction','unicodeBidi'].map(k=>[k,s[k]])),width:r.width,height:r.height,fragments:[...e.getClientRects()].map(r=>[r.x-p.x,r.y-p.y,r.width,r.height])}})}
function compare(a,b,path='root'){if(typeof b==='number'){assert.ok(Math.abs(a-b)<0.025,`${path}: ${a} != ${b}`)}else if(b&&typeof b==='object'){assert.deepEqual(Object.keys(a),Object.keys(b),path);for(const k of Object.keys(b))compare(a[k],b[k],`${path}/${k}`)}else assert.equal(a,b,path)}
try {
 await send('Page.enable');await send('Runtime.enable');await send('Accessibility.enable');await send('Network.enable');
 await send('Network.setBlockedURLs',{urls:['https://*','http://*.com/*']});
 const reference=await readFile(new URL('./fixtures/code-source-reference.html',import.meta.url),'utf8');
 const manifest=JSON.parse(await readFile(new URL('./fixtures/code-source-manifest.json',import.meta.url),'utf8'));
 assert.equal(manifest.pin,'476a3153417dbf75c565f7df05b0784e94ddd59e');
 assert.equal(createHash('sha256').update(reference).digest('hex'),manifest.projection_sha256);
 const cases={};
 for(const width of [1000,320])for(const mode of ['light','dark']){
  await send('Emulation.setDeviceMetricsOverride',{width,height:1400,deviceScaleFactor:1,mobile:false});
  await send('Page.navigate',{url:'data:text/html;charset=utf-8,'+encodeURIComponent(reference)});
  await until('document.querySelectorAll("code").length===17');
  await evaluate(`document.documentElement.style.colorScheme='${mode}'`);
  cases[`${width}/${mode}`]=await evaluate(`(${metrics})()`);
 }
 await send('Page.navigate',{url:process.env.CODE_URL??'http://127.0.0.1:8087'});
 await until('document.getElementById("code-events")?.textContent==="0:true"');
 await evaluate('document.fonts.ready.then(()=>true)');
 for(const width of [1000,320])for(const mode of ['light','dark']){
  await send('Emulation.setDeviceMetricsOverride',{width,height:1400,deviceScaleFactor:1,mobile:false});
  await evaluate(`document.getElementById('code-patterns').style.colorScheme='${mode}'`);
  const actual=await evaluate(`(${metrics})()`);assert.equal(actual.length,17);
  compare(actual,cases[`${width}/${mode}`],`${width}/${mode}`);
  assert.equal(await evaluate('document.getElementById("code-long").scrollWidth<=180'),true,'unbroken identifier wraps');
  assert.equal(await evaluate('document.getElementById("code-rtl").dir'),'ltr');
  assert.equal(await evaluate('document.getElementById("code-dir").dir'),'rtl');
 }
 assert.deepEqual(await evaluate(`(()=>{const e=document.getElementById('code-inherit'),p=e.parentElement,s=getComputedStyle(e),ps=getComputedStyle(p);return [s.color===ps.color,s.fontSize===ps.fontSize,s.lineHeight===ps.lineHeight]})()`),[true,true,true]);
 assert.deepEqual(await evaluate(`Array.from(document.querySelectorAll('#code-extra p > code[data-size=inherit]')).map(e=>getComputedStyle(e).fontSize)`),['24px','12px','14px','18px','24px']);
 assert.equal(await evaluate('getComputedStyle(document.getElementById("code-secondary")).color===getComputedStyle(document.getElementById("code-primary")).color'),false);
 assert.deepEqual(await evaluate(`(()=>{const e=document.getElementById('code-native');return [e.tagName,e.classList.contains('consumer-class'),e.style.opacity,e.dataset.testid,e.textContent]})()`),['CODE',true,'0.75','native-code','const count = 0']);
 await evaluate('document.getElementById("code-native").click()');await until('document.getElementById("code-events").textContent==="1:true"');
 const point=await evaluate(`(()=>{const e=document.querySelector('#code-native strong');e.scrollIntoView({block:'center'});const r=e.getBoundingClientRect();return {x:r.x+r.width/2,y:r.y+r.height/2}})()`);
 await send('Input.dispatchMouseEvent',{type:'mousePressed',button:'left',clickCount:1,...point});
 await send('Input.dispatchMouseEvent',{type:'mouseReleased',button:'left',clickCount:1,...point});
 await until('document.getElementById("code-events").textContent==="2:true"');
 assert.equal(await evaluate(`(()=>{const e=document.getElementById('code-native'),r=document.createRange();r.selectNodeContents(e);const s=getSelection();s.removeAllRanges();s.addRange(r);return s.toString()})()`),'const count = 0','native text selection preserves caller content');
 assert.equal(await evaluate('document.querySelectorAll(".astryx-code[role],.astryx-code[tabindex],.astryx-code[aria-label],.astryx-code[aria-hidden]").length'),0);
 const tree=await send('Accessibility.getFullAXTree');assert.ok(tree.nodes.some(n=>!n.ignored&&n.name?.value==='npm install @astryxdesign/core'),'caller code text reaches AX tree');
 // Native code does not consume keyboard events or turn itself into a button.
 assert.equal(await evaluate(`(()=>{const e=new KeyboardEvent('keydown',{key:'Enter',bubbles:true,cancelable:true});document.getElementById('code-native').dispatchEvent(e);return e.defaultPrevented})()`),false);
 await until('document.getElementById("code-key-events").textContent==="1"');
 await evaluate('document.documentElement.style.fontSize="20px"');
 assert.equal(await evaluate('getComputedStyle(document.getElementById("code-primary")).fontSize'),'17.5px','default code scale follows rem tokens');
 assert.equal(await evaluate('getComputedStyle(document.getElementById("code-primary")).paddingInlineStart'),'5px');
 // Consumer theme token override, including viewport-responsive theme changes.
 await evaluate(`{const s=document.createElement('style');s.textContent='@media(min-width:800px){#code-patterns {--text-code-size:21px;--spacing-1:7px}}';document.head.append(s)}`);
 await send('Emulation.setDeviceMetricsOverride',{width:1000,height:1400,deviceScaleFactor:1,mobile:false});
 assert.equal(await evaluate('getComputedStyle(document.getElementById("code-primary")).fontSize'),'21px');
 assert.equal(await evaluate('getComputedStyle(document.getElementById("code-inherit")).fontSize'),'24px');
 await send('Emulation.setDeviceMetricsOverride',{width:320,height:1400,deviceScaleFactor:1,mobile:false});
 assert.equal(await evaluate('getComputedStyle(document.getElementById("code-primary")).fontSize'),'17.5px');
 await send('Emulation.setEmulatedMedia',{features:[{name:'forced-colors',value:'active'}]});
 assert.equal(await evaluate('getComputedStyle(document.getElementById("code-primary")).forcedColorAdjust'),'auto');
 assert.equal(await evaluate('document.getElementById("code-primary").tagName'),'CODE');
 assert.deepEqual(errors,[]);
 const remote=requests.filter(({url})=>/^https?:/.test(url)&&!/^http:\/\/(127\.0\.0\.1|localhost):\d+(\/|$)/.test(url));
 assert.equal(remote.every(({id})=>blocked.has(id)),true,'all remote requests (including dx dev-overlay font import) blocked, no remote dependency');
 if(process.env.CODE_EVIDENCE){const image=await send('Page.captureScreenshot',{format:'png'});await writeFile(process.env.CODE_EVIDENCE,Buffer.from(image.data,'base64'))}
 console.log('Code browser passed: 17 original inline samples × 2 widths × 2 color schemes vs offline source projection; native root/ref/click/AX, RTL override, inheritance, wrapping, rem and responsive theme tokens, forced colors.');
}finally{ws.close();await fetch(`${endpoint}/json/close/${target.id}`)}
