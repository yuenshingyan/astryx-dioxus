// Node 22+, isolated Chrome CDP, no npm packages or live source requests.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {kbdMetrics,selectKbdMetrics,expandControl} from './kbd_metrics.mjs';
const endpoint=process.env.CDP_URL??'http://127.0.0.1:9222';
const target=await(await fetch(`${endpoint}/json/new?about:blank`,{method:'PUT'})).json();
const ws=new WebSocket(target.webSocketDebuggerUrl);await new Promise((resolve,reject)=>{ws.onopen=resolve;ws.onerror=reject});
let serial=0;const pending=new Map(),errors=[];ws.onmessage=({data})=>{const m=JSON.parse(data);if(m.method==='Runtime.exceptionThrown')errors.push(m.params);const p=pending.get(m.id);if(p){pending.delete(m.id);clearTimeout(p.timer);m.error?p.reject(Error(JSON.stringify(m.error))):p.resolve(m.result)}};
const send=(method,params={})=>new Promise((resolve,reject)=>{const id=++serial,timer=setTimeout(()=>{pending.delete(id);reject(Error(`CDP timeout: ${method}`))},15000);pending.set(id,{resolve,reject,timer});ws.send(JSON.stringify({id,method,params}))});
const evaluate=async expression=>{const r=await send('Runtime.evaluate',{expression,returnByValue:true,awaitPromise:true});assert.equal(r.exceptionDetails,undefined,JSON.stringify(r.exceptionDetails));return r.result.value};
const wait=ms=>new Promise(r=>setTimeout(r,ms));
async function waitFor(expression){const end=Date.now()+15000;while(Date.now()<end){if(await evaluate(expression))return;await wait(25)}assert.fail(`Timed out: ${expression}`)}
const click=async id=>{await evaluate(`document.getElementById(${JSON.stringify(id)}).click()`);await wait(60)};
const reference=JSON.parse(await readFile(new URL('./fixtures/kbd-neutral-reference.json',import.meta.url),'utf8'));
function compare(actual,expected,path='root'){
 if(typeof expected==='number'){assert.ok(Math.abs(actual-expected)<=1/64+1e-6,`${path}: ${actual} != ${expected}`);return}
 if(expected&&typeof expected==='object'){assert.deepEqual(Object.keys(actual),Object.keys(expected),`${path} keys`);for(const key of Object.keys(expected))compare(actual[key],expected[key],`${path}/${key}`);return}
 assert.equal(actual,expected,path);
}
// Platform selection is an external-browser boundary, not a component key map.
const detector=await readFile(new URL('../src/utils/is_apple_platform.js',import.meta.url),'utf8');
const detect=new Function('navigator',`${detector};return isApplePlatform()`);
for(const [navigator,expected] of [[undefined,false],[{platform:'MacIntel'},true],[{platform:'iPhone'},true],[{platform:'iPad'},true],[{platform:'iPod'},true],[{platform:'Win32'},false],[{platform:'Linux x86_64'},false],[{platform:'MacIntel',userAgentData:{platform:'Windows'}},false],[{platform:'Win32',userAgentData:{platform:' macOS '}},true],[{platform:'iPhone',userAgentData:{platform:'Unknown'}},true],[{platform:'iPad',userAgentData:{platform:'  '}},true],[{platform:'MacIntel',userAgentData:{platform:42}},true],[{platform:'iPad',userAgentData:{platform:'iOS'}},false]])assert.equal(detect(navigator),expected,JSON.stringify(navigator));
try {
 await send('Page.enable');await send('Runtime.enable');await send('Accessibility.enable');await send('Emulation.setDeviceMetricsOverride',{width:1200,height:1600,deviceScaleFactor:1,mobile:false});
 // Observe the first committed non-Apple render without changing detector values.
 await send('Page.addScriptToEvaluateOnNewDocument',{source:`(()=>{const d=Object.getOwnPropertyDescriptor(Navigator.prototype,'userAgentData');if(!d?.get)throw Error('UAData observation unavailable');Object.defineProperty(Navigator.prototype,'userAgentData',{configurable:true,get(){window.__kbdPlatformReads=(window.__kbdPlatformReads??0)+1;const e=document.querySelector('[data-kbd-named=showcase]');if(e&&!window.__kbdFirstGlyphs)window.__kbdFirstGlyphs=[...e.querySelectorAll('kbd')].map(k=>k.textContent);if(window.__kbdRemoveDuringRead){window.__kbdRemoveDuringRead=false;document.getElementById('kb-toggle').click()}return d.get.call(this)}})})()`});
 for(const platform of ['apple','nonApple']){
  const apple=platform==='apple';
  await send('Emulation.setUserAgentOverride',{userAgent:`Mozilla/5.0 (${apple?'Macintosh; Intel Mac OS X 10_15_7':'Windows NT 10.0; Win64; x64'}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/157.0.0.0 Safari/537.36`,platform:apple?'MacIntel':'Win32',acceptLanguage:'en-US,en;q=0.9',userAgentMetadata:{brands:[{brand:'Chromium',version:'157'}],fullVersionList:[{brand:'Chromium',version:'157.0.8103.79'}],fullVersion:'157.0.8103.79',platform:apple?'macOS':'Windows',platformVersion:apple?'10.15.7':'10.0.0',architecture:'x86',model:'',mobile:false,bitness:'64',wow64:false}});
  await send('Page.navigate',{url:process.env.SHOWCASE_URL??'http://127.0.0.1:8080'});
  await waitFor(`document.querySelector('[data-kbd-named=showcase]')?.getAttribute('aria-label')===${JSON.stringify(apple?'Command + K':'Control + K')}`);await evaluate('document.fonts.ready.then(()=>true)');await wait(100);
  assert.deepEqual(await evaluate('window.__kbdFirstGlyphs'),['Ctrl','K'],'first committed client render uses server-safe snapshot');
  assert.deepEqual(await evaluate('[navigator.platform,navigator.userAgentData.platform]'),[apple?'MacIntel':'Win32',apple?'macOS':'Windows']);
  for(const mode of ['light','dark']){
   if(mode==='dark')await click('kb-theme');
   const actual=await evaluate(`Array.from(document.querySelectorAll('[data-kbd-named]')).map(element=>(${selectKbdMetrics.toString()})((${kbdMetrics.toString()})(element)))`);
   assert.equal(actual.length,26);assert.equal(actual.reduce((n,v)=>n+v.keys.length,0),46);
   actual.forEach((value,index)=>compare(value,selectKbdMetrics(expandControl(reference,`${platform}/${mode}`,index)),`${platform}/${mode}/${index}`));
   assert.equal(await evaluate('Array.from(document.querySelectorAll("[data-kbd-named] > kbd")).some(e=>[...e.classList].some(c=>c.startsWith("astryx-")))'),false,'pinned keycaps have no public target despite published live key classes');
   const contexts=reference.cases[`${platform}/${mode}`].contexts;
   const sizes=await evaluate('["kb-inline","kb-menu","kb-modifiers"].map(id=>{const r=document.getElementById(id).getBoundingClientRect();return [r.width,r.height]})');
   compare(sizes,contexts.slice(1).map(c=>[c.content.width,c.content.height]),`${platform}/${mode}/native contexts`);
  }
  await click('kb-theme');
  assert.deepEqual(await evaluate('Array.from(document.querySelectorAll("#kb-duplicates > kbd")).map(e=>e.textContent)'),['Esc','Esc','Esc','↵','↵','↵']);
  assert.deepEqual(await evaluate('Array.from(document.querySelectorAll("#kb-prototype > kbd")).map(e=>e.textContent)'),['CONSTRUCTOR','__PROTO__','SS']);
  assert.equal(await evaluate('document.querySelectorAll("#kb-all > kbd").length'),14);
  assert.equal(await evaluate('document.querySelector("#kb-owned").getAttribute("role")'),'img');
  assert.equal(await evaluate('document.querySelector("#kb-owned").getAttribute("aria-label")'),apple?'Command + K':'Control + K');
  assert.equal(await evaluate('document.getElementById("kb-menu").querySelectorAll("button,a[href],input,[role=button]").length'),0,'source menu example rows are readonly, not fake menu controls');
  const tree=await send('Accessibility.getFullAXTree');
  const imageNames=tree.nodes.filter(n=>!n.ignored&&['image','img'].includes(n.role?.value)).map(n=>n.name?.value);
  for(const name of [apple?'Command + K':'Control + K','Visible shortcut description','CONSTRUCTOR + __PROTO__ + SS',apple?'Shift + Plus':'Shift + Plus'])assert.ok(imageNames.includes(name),`AX name ${name}`);
  const document=await send('DOM.getDocument');const firstKey=await send('DOM.querySelector',{nodeId:document.root.nodeId,selector:'#kb-all > kbd'});const keyTree=await send('Accessibility.getPartialAXTree',{nodeId:firstKey.nodeId,fetchRelatives:false});assert.equal(keyTree.nodes[0].ignored,true,'glyphs are not separate announcements');
  assert.equal(await evaluate('document.querySelectorAll("#kbd-examples .astryx-kbd[tabindex],#kbd-examples .astryx-kbd[aria-live]").length'),0);
  assert.equal(await evaluate('document[Symbol.for("astryx.dioxus.button.runtime")].records.has(document.getElementById("kb-plus"))'),false,'no shared press/tooltip interest');
  assert.equal(await evaluate('document.getElementById("kb-events").textContent'),'Native events: 1');await click('kb-native');assert.equal(await evaluate('document.getElementById("kb-events").textContent'),'Native events: 2');
  assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("kb-native");return [e.tagName,e.lang,e.dataset.testid,e.style.marginInline,e.classList.contains("caller-kbd")]})()'),['SPAN','fr','kbd-native','7px',true]);
  assert.equal(await evaluate('(()=>{const e=new KeyboardEvent("keydown",{key:"k",ctrlKey:true,bubbles:true,cancelable:true});document.getElementById("kb-native").dispatchEvent(e);return e.defaultPrevented})()'),false,'Kbd registers no shortcut activation');
  await evaluate('window.__kbdLocaleRoot=document.getElementById("kb-localized");window.__kbdReadsBefore=window.__kbdPlatformReads');
  await click('kb-locale');assert.equal(await evaluate('document.getElementById("kb-localized")===window.__kbdLocaleRoot'),true);
  assert.equal(await evaluate('document.getElementById("kb-localized").getAttribute("aria-label")'),apple?'Abbrechen mit Befehl':'Abbrechen mit Steuerung');
  assert.deepEqual(await evaluate('Array.from(document.querySelectorAll("#kb-localized > kbd")).map(e=>e.textContent)'),[apple?'⌘':'Strg','Abbr']);
  await click('kb-change');assert.equal(await evaluate('document.getElementById("kb-changing").getAttribute("aria-label")'),'Shift + Plus');
  assert.equal(await evaluate('window.__kbdPlatformReads===window.__kbdReadsBefore'),true,'locale/key updates create no platform subscription or polling');
  await click('kb-toggle');await waitFor(`document.getElementById('kb-late')?.getAttribute('aria-label')===${JSON.stringify(apple?'Command + Shift + META + K':'Control + Shift + META + K')}`);
  await click('kb-toggle');await waitFor('!document.getElementById("kb-late")');
  await evaluate('window.__kbdRemoveDuringRead=true');await click('kb-toggle');await waitFor('!document.getElementById("kb-late")&&!window.__kbdRemoveDuringRead');
  assert.equal(await evaluate('Array.from(document.querySelectorAll("#kb-larger kbd")).every(e=>{const b=e.getBoundingClientRect(),r=document.createRange();r.selectNodeContents(e);const g=r.getBoundingClientRect();return b.height>20&&b.height+1/64>=parseFloat(getComputedStyle(e).lineHeight)&&g.top>=b.top-.01&&g.bottom<=b.bottom+.01})'),true,'larger key badges contain glyphs, not a frozen 20px height');
  const rtl=await evaluate('Array.from(document.querySelectorAll("#kb-rtl kbd")).map(e=>e.getBoundingClientRect().x)');assert.ok(rtl[0]>rtl[1]&&rtl[1]>rtl[2],'RTL preserves source logical flex order');
  await evaluate('document.getElementById("kb-plus").style.color="rgb(200,0,0)"');
  assert.notEqual(await evaluate('getComputedStyle(document.querySelector("#kb-plus kbd")).color'),'rgb(200, 0, 0)','root target is not an invented per-key color target');
 }
 await send('Emulation.setDeviceMetricsOverride',{width:320,height:714,deviceScaleFactor:1,mobile:true});
 assert.equal(await evaluate('Array.from(document.querySelectorAll("#kb-inline .astryx-kbd > kbd")).every(e=>e.getBoundingClientRect().height>=20)'),true,'narrow text preserves readable key geometry');
 await send('Emulation.setEmulatedMedia',{features:[{name:'forced-colors',value:'active'}]});
 assert.equal(await evaluate('getComputedStyle(document.querySelector("[data-kbd-named=showcase] kbd")).borderBottomStyle'),'solid');
 assert.equal(await evaluate('document.querySelector("[data-kbd-named=showcase]").getAttribute("role")'),'img');
 assert.deepEqual(errors,[]);
 console.log('Kbd browser checks passed: all 26 original groups/46 keycaps across Apple/non-Apple and light/dark, source geometry/paint/native contexts, first-client fallback, AX one-name/hidden glyphs, pinned target ownership, aliases/unknown/repeated keys, native refs/events, locale/key updates without polling, late/drop-during-platform-read, text growth, RTL, narrow layout and forced colors.');
}finally{ws.close();await fetch(`${endpoint}/json/close/${target.id}`)}
