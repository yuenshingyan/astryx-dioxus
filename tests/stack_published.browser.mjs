// Offline isolated layout proof against actual published measurements.
// Frozen supporting dimensions/paint are intentional controls, not migrated components.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {withTarget} from './card_metrics.mjs';
const fixture=JSON.parse(await readFile(new URL('./fixtures/stack-published.json',import.meta.url),'utf8'));
const source=JSON.parse(await readFile(new URL('./fixtures/stack-source-reference.json',import.meta.url),'utf8'));
const near=(a,b,label)=>assert.ok(Math.abs(a-b)<=1/64+1e-6,`${label}: ${a} != ${b}`);
await withTarget(async({send,evaluate,waitFor})=>{
 await send('Network.enable');await send('Network.setBlockedURLs',{urls:['https://*']});
 await send('Page.navigate',{url:process.env.STACK_URL??'http://127.0.0.1:8093'});
 await waitFor(`document.querySelectorAll('#st-matrix > .astryx-stack').length===48`);
 await evaluate('document.fonts.ready.then(()=>true)');
 await send('Network.emulateNetworkConditions',{offline:true,latency:0,downloadThroughput:0,uploadThroughput:0});
 await evaluate(`window.published=${JSON.stringify(fixture)};true`);
 for(const [name,id]of [['directions','st-directions'],['alignment','st-alignment'],['fill','st-fill']]){
  const results=await evaluate(`(()=>{
   const entry=published.cases.${name},host=document.createElement('div');document.body.append(host);host.innerHTML=entry.html;const root=host.firstElementChild;
   const measured=()=>{const r=root.getBoundingClientRect();return [root,...root.querySelectorAll('*')].map(e=>{const b=e.getBoundingClientRect(),s=getComputedStyle(e);return{box:[b.x-r.x,b.y-r.y,b.width,b.height],paint:[s.backgroundColor,s.color,s.borderRadius]}})};
   const before=measured(),live=document.getElementById('${id}'),templates=[live,...live.querySelectorAll('.astryx-stack')];
   const layout=['display','flex-direction','flex-wrap','align-items','justify-content','row-gap','column-gap'];
   [root,...root.querySelectorAll('[data-stack-replay]')].forEach((e,i)=>{
    const t=templates[i];if(!t)throw Error('Missing live Stack '+i);
    for(const p of layout)e.style.removeProperty(p);
    // All supporting sizes/paint remain frozen. Only Stack layout comes from Dioxus.
    e.className=t.className;for(const p of [...t.style])if(p.startsWith('--_stack-'))e.style.setProperty(p,t.style.getPropertyValue(p));
    if('${name}'==='directions'&&i===0)e.style.flexWrap='wrap'; // Observed deployment differs from pinned template.
   });
   const after=measured();host.remove();return{before,after};
  })()`);
  const nodes=fixture.cases[name].nodes;
  for(const [i,n]of nodes.entries()){
   if(!n.targets.length||n.box[2]===0||n.box[3]===0)continue;
   for(const phase of ['before','after']){
    n.box.forEach((v,j)=>near(results[phase][i].box[j],v,`${name}/${phase}/${i}/box${j}`));
    assert.deepEqual(results[phase][i].paint,[n.style.backgroundColor,n.style.color,n.style.borderRadius],`${name}/${phase}/${i}/paint`);
   }
  }
 }
 // Independently lower verified original static declaration subsets, not local
 // demo CSS, and compare geometry/paint in the same inherited token environment.
 await evaluate(`window.demoStyles=${JSON.stringify(source.demoStyles)};true`);
 const framing=await evaluate(`(()=>{
  const output=[],apply=(e,rules)=>{for(const [p,v]of Object.entries(rules))e.style.setProperty(p,v)},measure=e=>{const s=getComputedStyle(e),b=e.getBoundingClientRect();return{box:[b.width,b.height],style:Object.fromEntries(['display','fontFamily','fontSize','fontWeight','lineHeight','backgroundColor','color','borderRadius','borderTopWidth','paddingInlineStart','paddingBlockStart','gap','minWidth','minHeight','flexGrow','flexShrink','flexBasis'].map(p=>[p,s[p]]))}};
  for(const [selector,names]of [['.astryx-token',['tokenBase','tokenMd','tokenDefault']],['.astryx-avatar',['avatarWrapper']],['.astryx-stack-item[data-size="static"]',['itemReset','itemStatic']],['.astryx-stack-item[data-size="fill"]',['itemReset','itemFill']]]){
   const live=document.querySelector(selector),ref=live.cloneNode(true);live.after(ref);ref.removeAttribute('class');
   for(const name of names)apply(ref,demoStyles[name]);
   if(selector==='.astryx-token'){ref.firstElementChild.removeAttribute('class');apply(ref.firstElementChild,demoStyles.tokenLabel)}
   if(selector==='.astryx-avatar'){
    ref.style.width='36px';ref.style.height='36px';ref.style.setProperty('--_avatar-radius','var(--radius-full)');
    const content=ref.firstElementChild,fallback=content.firstElementChild;content.removeAttribute('class');fallback.removeAttribute('class');apply(content,demoStyles.avatarContent);apply(fallback,demoStyles.avatarFallback);fallback.style.fontSize='14.4px';
    output.push([measure(live.firstElementChild),measure(content)],[measure(live.firstElementChild.firstElementChild),measure(fallback)]);
   }
   output.push([measure(live),measure(ref)]);ref.remove();
  }return output;
 })()`);
 for(const [i,[a,b]]of framing.entries()){assert.deepEqual(a.style,b.style,'source static framing '+i);a.box.forEach((n,j)=>near(n,b.box[j],'source static framing geometry '+i+'/'+j))}
 // Demo-only framing follows its own source tokens, not copied deployed colors.
 assert.deepEqual(await evaluate(`(()=>{const t=document.querySelector('.astryx-token'),s=getComputedStyle(t),a=getComputedStyle(document.querySelector('.astryx-avatar'));return[s.height,s.borderTopWidth,s.borderRadius,s.paddingInlineStart,s.fontSize,a.width,a.height,a.borderTopWidth]})()`),['24px','0px','4px','8px','12px','36px','36px','0px']);
 assert.equal(await evaluate(`document.querySelectorAll('#st-fill .astryx-avatar[role="img"][tabindex="0"]').length`),3,'Private pattern Avatar slices retain named-image keyboard semantics');
 console.log('Published Stack: three frozen original patterns reproduce measured targeted-node geometry and computed paint before/after actual Dioxus Stack layout transplantation. Supporting dimensions/paint held constant; no intrinsic/full-image/React parity claim.');
});
