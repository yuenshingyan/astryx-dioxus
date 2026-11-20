// Run only under the shared browser lock. Opt-in network capture, never a test dependency.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {mkdir, writeFile} from 'node:fs/promises';
import {withTarget} from './card_metrics.mjs';
const url='https://astryx.atmeta.com/components/Stack';
await withTarget(async({send,evaluate,waitFor,browser,pause})=>{
 await send('Network.enable');
 const response=await send('Page.navigate',{url});assert.ok(!response.errorText);
 await waitFor(`!!document.querySelector('#example-StackFillItem .astryx-avatar')`);
 await evaluate('document.fonts.ready.then(()=>true)');
 await mkdir(new URL('./fixtures/stack-published/',import.meta.url),{recursive:true});
 const evidence=await evaluate(`(()=>{
 const roots={directions:[...document.querySelectorAll('.astryx-stack')].find(e=>e.textContent==='HorizontalHorizontalHorizontalVerticalVerticalVertical'),alignment:document.querySelector('#example-StackAlignment [style*="max-width: 500px"]'),fill:document.querySelector('#example-StackFillItem [style*="max-width: 300px"]')};
 const props=['display','position','boxSizing','flexDirection','flexWrap','flexGrow','flexShrink','flexBasis','alignItems','justifyContent','rowGap','columnGap','paddingInlineStart','paddingInlineEnd','paddingBlockStart','paddingBlockEnd','backgroundColor','color','borderRadius','borderWidth','borderStyle','borderColor','fontFamily','fontSize','fontWeight','lineHeight','overflow','minWidth','minHeight'];
 const cases={};for(const [name,root] of Object.entries(roots)){
  if(!root)throw Error('Missing '+name);const rb=root.getBoundingClientRect();const nodes=[root,...root.querySelectorAll('*')].map(e=>{const s=getComputedStyle(e),b=e.getBoundingClientRect();return{tag:e.tagName,text:e.textContent,targets:[...e.classList].filter(c=>c.startsWith('astryx-')),data:{...e.dataset},box:[b.x-rb.x,b.y-rb.y,b.width,b.height],style:Object.fromEntries(props.map(p=>[p,s[p]])),before:getComputedStyle(e,'::before').content,after:getComputedStyle(e,'::after').content}});
  // Frozen original DOM and all computed declarations: a static render, not a React port.
  const clone=root.cloneNode(true),original=[root,...root.querySelectorAll('*')],copies=[clone,...clone.querySelectorAll('*')];
  copies.forEach((e,i)=>{const s=getComputedStyle(original[i]);e.removeAttribute('id');e.removeAttribute('tabindex');e.removeAttribute('popover');e.style.cssText=[...s].filter(p=>!p.startsWith('--')).map(p=>p+':'+s.getPropertyValue(p)).join(';');e.removeAttribute('class');
   // CSSOM rounds serialized used widths to six significant digits; recover
   // Chrome's 1/64px layout grid to avoid cumulative replay quantization.
   // A sub-grid 0.001px bias prevents CSSOM serialization rounding below
   // that grid again; it is not a geometry tolerance or measured adjustment.
   for(const p of ['width','height','inlineSize','blockSize'])if(s[p].endsWith('px'))e.style[p]=(Math.round(parseFloat(s[p])*64)/64+0.001)+'px';
   if(original[i].classList.contains('astryx-stack')){e.dataset.stackReplay='';e.dataset.stackIndex=String(original.slice(0,i).filter(n=>n.classList.contains('astryx-stack')).length)}
   if(original[i].classList.contains('astryx-avatar-fallback'))e.textContent=original[i].closest('[aria-label]').getAttribute('aria-label').split(' ').map(n=>n[0]).join('');
  });
  cases[name]={nodes,html:clone.outerHTML,rootSize:[rb.width,rb.height],clip:{x:rb.x+scrollX,y:rb.y+scrollY,width:rb.width,height:rb.height,scale:1}};
 }
 return{url:location.href,title:document.title,version:document.body.innerText.match(/@astryxdesign\\/core v[^ ·\\n]+/)?.[0],rootFont:getComputedStyle(document.documentElement).fontSize,theme:[...document.querySelectorAll('[data-theme]')].map(e=>e.dataset.theme),stylesheets:[...document.styleSheets].map(s=>s.href),cases};
 })()`);
 assert.equal(evidence.url,url);assert.equal(evidence.title,'Stack · Astryx');assert.ok(evidence.version);
 evidence.capturedAt=new Date().toISOString();evidence.browser=browser.Browser;evidence.viewport=[1200,1600,1];
 for(const [name,entry] of Object.entries(evidence.cases)){
  const shot=await send('Page.captureScreenshot',{format:'png',captureBeyondViewport:true,clip:entry.clip});
  const bytes=Buffer.from(shot.data,'base64');entry.screenshot={path:`stack-published/${name}.png`,sha256:createHash('sha256').update(bytes).digest('hex')};
  await writeFile(new URL('./fixtures/'+entry.screenshot.path,import.meta.url),bytes);
 }
 for(const name of ['StackAlignment','StackFillItem']){
  await evaluate(`document.querySelector('#example-${name} [data-tab-value="code"]').click()`);await pause(300);
  evidence.cases[name==='StackAlignment'?'alignment':'fill'].publishedCode=await evaluate(`document.querySelector('#example-${name}').innerText`);
 }
 evidence.provenance={kind:'Observed published DOM/computed styles, not commit or release equivalence',sourcePin:'476a3153417dbf75c565f7df05b0784e94ddd59e',limits:'Displayed version only; deployment commit unknown. Static computed replay does not execute React, tooltip, pseudo paint, or guarantee screenshot parity.'};
 // Content-address the served document and stylesheets independently of the displayed version.
 evidence.resources=[];
 for(const resource of [url,...new Set(evidence.stylesheets.filter(Boolean))]){const r=await fetch(resource),body=Buffer.from(await r.arrayBuffer());assert.ok(r.ok);evidence.resources.push({url:resource,status:r.status,etag:r.headers.get('etag'),sha256:createHash('sha256').update(body).digest('hex'),bytes:body.length});}
 await writeFile(new URL('./fixtures/stack-published.json',import.meta.url),JSON.stringify(evidence,null,2)+'\n');
 console.log('Captured published',evidence.version,evidence.capturedAt,Object.keys(evidence.cases));
});
