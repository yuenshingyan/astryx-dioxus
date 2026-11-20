// Run behind the shared browser_lock.py. Own target only; all requests offline.
import assert from 'node:assert/strict';
import {withTarget,cardMetrics} from './card_metrics.mjs';
import {patterns,quoteCSS,citeCSS,cardCSS,coreTokens,neutralTokens} from './blockquote_source.mjs';
function compare(actual,expected,path='root'){
 if(typeof expected==='number'){assert.ok(Math.abs(actual-expected)<1/64+1e-6,`${path}: ${actual} != ${expected}`);return;}
 if(expected&&typeof expected==='object'){assert.deepEqual(Object.keys(actual).sort(),Object.keys(expected).sort(),path);for(const k of Object.keys(expected))compare(actual[k],expected[k],path+'/'+k);return;}
 assert.equal(actual,expected,path);
}
await withTarget(async({send,evaluate,waitFor,pause,exceptions})=>{
 await send('Accessibility.enable');
 // Block all external requests without requiring the catalog/CDN to be available.
 await send('Network.enable');await send('Network.setBlockedURLs',{urls:['https://*']});
 await send('Page.navigate',{url:process.env.BLOCKQUOTE_URL??'http://127.0.0.1:8091'});
 await waitFor('document.querySelectorAll("[data-bq-pattern] blockquote").length===7');
 await evaluate('document.fonts.ready.then(()=>true)');
 await send('Network.emulateNetworkConditions',{offline:true,latency:0,downloadThroughput:0,uploadThroughput:0});
 await evaluate(`window.cardMetrics=(${cardMetrics});window.bqEvidence=${JSON.stringify({patterns,quoteCSS,citeCSS,cardCSS,coreTokens,neutralTokens})};true`);
 const measure=`(root)=>{const t=cardMetrics(root,root);return t.nodes.map(n=>({tag:n.tag,box:n.box,textRects:n.textRects,text:n.textRuns.map(t=>t.text).join(''),style:Object.fromEntries(['display','gridTemplateColumns','rowGap','columnGap','margin','padding','borderWidth','borderStyle','borderColor','borderRadius','backgroundColor','color','overflow','overflowWrap','fontFamily','fontSize','lineHeight','fontStyle','direction','boxShadow'].map(p=>[p,p==='fontStyle'?getComputedStyle(root.querySelector('cite')??root).fontStyle:n.style[p]]))}));}`;
 await evaluate(`window.measureBQ=${measure};true`);
 let comparisons=0;
 for(const theme of ['core','neutral'])for(const mode of ['light','dark'])for(const direction of ['ltr','rtl'])for(const width of [600,280]){
  await evaluate(`(()=>{const s=document.getElementById('blockquote-examples');s.dataset.astryxTheme=${JSON.stringify(theme)};s.style.colorScheme=${JSON.stringify(mode)};s.dir=${JSON.stringify(direction)};s.style.width=${JSON.stringify(width+'px')};})()`);
  for(const pattern of patterns){
   const result=await evaluate(`(()=>{
    const section=document.getElementById('blockquote-examples'),name=${JSON.stringify(pattern.name)},source=bqEvidence.patterns.find(p=>p.name===name),frame=document.querySelector('[data-bq-pattern="'+name+'"]');
    const actual=[...frame.querySelectorAll('blockquote')].map(q=>({text:[...q.childNodes].filter(n=>n.nodeType===3).map(n=>n.textContent).join(''),cite:q.querySelector('cite')?.textContent??null}));
    const ref=document.createElement('div');ref.style.cssText='position:absolute;visibility:hidden;box-sizing:border-box;width:'+frame.getBoundingClientRect().width+'px;'+(name==='Testimonials'?'display:grid;grid-template-columns:repeat(2,1fr);gap:var(--spacing-3);max-width:600px':'display:flex;flex-direction:column;gap:var(--spacing-4);max-width:500px');
    for(const [k,v]of Object.entries(bqEvidence[${JSON.stringify(theme==='core'?'coreTokens':'neutralTokens')}]))ref.style.setProperty(k,v);
    for(const [i,q]of source.quotes.entries()){
     const b=document.createElement('blockquote');b.style.cssText=bqEvidence.quoteCSS;b.append(document.createTextNode(q.text));if(q.cite!==null){const c=document.createElement('cite');c.style.cssText=bqEvidence.citeCSS;c.textContent=q.cite;b.append(c);}
     if(name==='Testimonials'){const card=document.createElement('div');card.style.cssText=bqEvidence.cardCSS;card.append(b);if(i===2){const span=document.createElement('div');span.style.gridColumn='span 2';span.append(card);ref.append(span);}else ref.append(card);}else ref.append(b);
    }
    section.append(ref);const out={actual,port:measureBQ(frame),reference:measureBQ(ref)};ref.remove();return out;
   })()`);
   assert.deepEqual(result.actual,pattern.quotes,pattern.name+' content and author');
   compare(result.port,result.reference,`${theme}/${mode}/${direction}/${width}/${pattern.name}`);comparisons++;
  }
 }
 await evaluate('(()=>{const s=document.getElementById("blockquote-examples");s.dataset.astryxTheme="neutral";s.style.colorScheme="light";s.style.width="";s.dir="ltr"})()');
 assert.deepEqual(await evaluate('[...document.querySelectorAll("#bq-scalars blockquote")].map(e=>e.querySelector("cite")?.textContent??null)'),[null,null,null,' ','0','']);
 assert.equal(await evaluate('document.querySelector("#bq-rich").lastElementChild.tagName'),'CITE');
 assert.equal(await evaluate('document.querySelectorAll("#blockquote-examples footer").length'),0);
 assert.deepEqual(await evaluate('["bq-class","bq-style"].map(id=>{const s=getComputedStyle(document.getElementById(id));return[s.paddingInlineStart,s.color]})'),[['23px','rgb(102, 51, 153)'],['31px','rgb(40, 80, 120)']]);
 assert.equal(await evaluate('(()=>{const e=document.getElementById("bq-long");return e.scrollWidth===e.clientWidth&&e.querySelector("cite").getBoundingClientRect().height>30})()'),true,'unbroken text and long author wrap');
 const click=async id=>{await evaluate(`document.getElementById(${JSON.stringify(id)}).click()`);await pause(75)};
 assert.equal(await evaluate('document.getElementById("bq-events").textContent'),'Mounts: 1; clicks: 0');
 await evaluate('document.getElementById("bq-native").scrollIntoView({block:"center"})');
 const point=await evaluate('(()=>{const r=document.getElementById("bq-native").getBoundingClientRect();return{x:r.x+r.width/2,y:r.y+r.height/2}})()');
 await send('Input.dispatchMouseEvent',{type:'mousePressed',...point,button:'left',clickCount:1});
 await send('Input.dispatchMouseEvent',{type:'mouseReleased',...point,button:'left',clickCount:1});await pause(75);assert.equal(await evaluate('document.getElementById("bq-events").textContent'),'Mounts: 1; clicks: 1');
 assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("bq-native");e.focus();window.savedBQ=e;return[e.tagName,e.lang,e.dataset.caller,document.activeElement===e]})()'),['BLOCKQUOTE','fr','forwarded',true]);
 await click('bq-cite-toggle');assert.equal(await evaluate('savedBQ===document.getElementById("bq-native")&&!savedBQ.querySelector("cite")'),true);
 await click('bq-cite-toggle');assert.equal(await evaluate('savedBQ.lastElementChild.textContent'),'Dynamic author');
 await evaluate('document.querySelector("#bq-rich a").focus()');
 await send('Input.dispatchKeyEvent',{type:'keyDown',key:'Enter',code:'Enter',windowsVirtualKeyCode:13});
 await send('Input.dispatchKeyEvent',{type:'keyUp',key:'Enter',code:'Enter',windowsVirtualKeyCode:13});
 await waitFor('location.hash==="#bq-source"');
 const doc=await send('DOM.getDocument'),node=await send('DOM.querySelector',{nodeId:doc.root.nodeId,selector:'#bq-native'});
 const ax=(await send('Accessibility.getPartialAXTree',{nodeId:node.nodeId,fetchRelatives:false})).nodes[0];assert.equal(ax.role.value,'blockquote');assert.equal(ax.name.value,'Caller named quote');
 const tree=(await send('Accessibility.getFullAXTree')).nodes;assert.equal(tree.filter(n=>n.role?.value==='contentinfo').length,0);
 await click('bq-toggle');assert.equal(await evaluate('document.getElementById("bq-native")'),null);await click('bq-toggle');assert.equal(await evaluate('document.getElementById("bq-events").textContent'),'Mounts: 2; clicks: 1');
 for(const features of [[{name:'prefers-reduced-motion',value:'reduce'}],[{name:'forced-colors',value:'active'}]]){
  await send('Emulation.setEmulatedMedia',{features});
  assert.equal(await evaluate('[...document.querySelectorAll("[data-bq-pattern] blockquote")].every(e=>getComputedStyle(e).borderInlineStartWidth==="2px"&&getComputedStyle(e).borderInlineStartStyle==="solid"&&e.getAnimations().length===0)'),true);
 }
 assert.equal(exceptions.length,0,JSON.stringify(exceptions));
 console.log(`Blockquote browser passed: ${comparisons} offline source-derived pattern comparisons (7 quotes; core/Neutral, light/dark, LTR/RTL, 600/280px), Card framing, wrapping, shallow slots, native AX/ref/events, stable updates/remount and media.`);
});
