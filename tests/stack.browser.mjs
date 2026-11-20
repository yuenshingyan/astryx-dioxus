// Offline SOURCE-DERIVED comparisons, not published site or upstream runtime measurements.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {withTarget} from './card_metrics.mjs';
const fixture=JSON.parse(await readFile(new URL('./fixtures/stack-source-reference.json',import.meta.url),'utf8'));
const near=(a,b,label)=>assert.ok(Math.abs(a-b)<1/64+1e-6,`${label}: ${a} != ${b}`);
await withTarget(async({send,evaluate,waitFor,pause,exceptions})=>{
 await send('Network.enable');
 await send('Network.setBlockedURLs',{urls:['https://*','http://astryx.atmeta.com/*']});
 await send('Page.navigate',{url:process.env.STACK_URL??'http://127.0.0.1:8093'});
 await waitFor('document.querySelectorAll("#st-matrix > .astryx-stack").length===48');
 await evaluate('document.fonts.ready.then(()=>true)');
 await send('Network.emulateNetworkConditions',{offline:true,latency:0,downloadThroughput:0,uploadThroughput:0});
 await evaluate(`window.stackFixture=${JSON.stringify(fixture)};true`);
 // Lower the verified original declarations independently of Dioxus's classes.
 await evaluate(`window.originalStack=(element,options={})=>{
  const styles=stackFixture.styles;
  const apply=rules=>{for(const [property,value] of Object.entries(rules??{}))element.style.setProperty(property,value)};
  apply(styles.baseStyles.stack);apply(styles.directionStyles[options.direction??'vertical']);
  if(options.gap!=null)apply(styles.gapStyles[options.gap]);
  if(options.wrap!=null)apply(styles.wrapStyles[options.wrap]);
  const horizontal=options.direction==='horizontal';
  const h=options.hAlign??(horizontal?options.justify:options.align);
  const v=options.vAlign??(horizontal?options.align:options.justify);
  apply(styles.justifyContentStyles[horizontal?h:v]);apply(styles.alignItemsStyles[horizontal?v:h]);
  for(const [edge,axis,name] of [['paddingInlineStart','paddingInline','paddingInlineStartStyles'],['paddingInlineEnd','paddingInline','paddingInlineEndStyles'],['paddingBlockStart','paddingBlock','paddingBlockStartStyles'],['paddingBlockEnd','paddingBlock','paddingBlockEndStyles']]){
   const value=options[edge]??options[axis]??options.padding;if(value!=null)apply(styles[name][value]);
  }
  if(options.isScrollable)element.style.overflow='auto';return element;
 };window.stackMeasure=e=>{const s=getComputedStyle(e),r=e.getBoundingClientRect();return{
  style:Object.fromEntries(['display','flexDirection','justifyContent','alignItems','rowGap','columnGap','flexWrap','paddingInlineStart','paddingInlineEnd','paddingBlockStart','paddingBlockEnd','overflowX','overflowY'].map(p=>[p,s[p]])),
  box:[r.width,r.height],children:[...e.children].map(c=>{const b=c.getBoundingClientRect();return[b.x-r.x,b.y-r.y,b.width,b.height]})
 }};true`);
 // 48 axis combinations in LTR/RTL, two color schemes, and enlarged root text.
 for(const rootSize of [16,20])for(const direction of ['ltr','rtl'])for(const scheme of ['light','dark']){
  await evaluate(`document.documentElement.style.fontSize='${rootSize}px';document.getElementById('stack-examples').dir='${direction}';document.documentElement.style.colorScheme='${scheme}'`);
  const comparisons=await evaluate(`(()=>{const output=[];for(const live of document.querySelectorAll('#st-matrix > .astryx-stack')){
   const ref=document.createElement('div');ref.className='stack-probe';ref.style.width='180px';ref.style.height='100px';for(let i=0;i<2;i++)ref.append(document.createElement('span'));
   live.after(ref);originalStack(ref,{direction:live.dataset.direction,justify:live.dataset.main,align:live.dataset.cross,gap:2});output.push([stackMeasure(live),stackMeasure(ref)]);ref.remove();
  }return output})()`);
  for(const [i,[actual,expected]]of comparisons.entries()){
   assert.deepEqual(actual.style,expected.style,`${rootSize}/${direction}/${scheme}/${i} styles`);
   actual.children.flat().forEach((n,j)=>near(n,expected.children.flat()[j],`${rootSize}/${direction}/${scheme}/${i}/${j}`));
  }
  const steps=await evaluate(`[...document.querySelectorAll('#st-steps > .astryx-stack')].map(e=>[e.dataset.gap,stackMeasure(e)])`);
  for(const [step,m]of steps){const px=Number(step)*rootSize/4;for(const p of ['rowGap','columnGap','paddingInlineStart','paddingInlineEnd','paddingBlockStart','paddingBlockEnd'])near(parseFloat(m.style[p]),px,`${step}/${p}`)}
 }
 await evaluate(`document.documentElement.style.fontSize='16px';document.getElementById('stack-examples').dir='ltr';document.documentElement.style.colorScheme='light'`);
 // Original named-pattern Stack topology. Supporting content deliberately remains
 // main's existing components or demo-local source-derived static framing.
 for(const [id,overrides]of [['st-alignment',{2:{hAlign:'start'},4:{hAlign:'center'},6:{hAlign:'end'}}],['st-directions',{0:{hAlign:'center'},1:{vAlign:'center'}}],['st-fill',{1:{vAlign:'center'},3:{vAlign:'center'},5:{vAlign:'center'}}]]){
  const result=await evaluate(`(()=>{const live=document.getElementById(${JSON.stringify(id)}),ref=live.cloneNode(true);ref.removeAttribute('id');live.after(ref);
   const actual=[live,...live.querySelectorAll('.astryx-stack')],original=[ref,...ref.querySelectorAll('.astryx-stack')];
   const overrides=${JSON.stringify(overrides)};
   original.forEach((e,i)=>{const options={direction:e.dataset.direction,gap:e.dataset.gap,wrap:e.dataset.wrap,...overrides[i]};
    for(const c of [...e.classList])if(c==='astryx-stack'||c.startsWith('astryx-stack--'))e.classList.remove(c);
    for(const p of [...e.style])if(p.startsWith('--_stack-'))e.style.removeProperty(p);originalStack(e,options);
   });const out=actual.map((e,i)=>[stackMeasure(e),stackMeasure(original[i])]);ref.remove();return out})()`);
  for(const [i,[a,b]]of result.entries()){
   assert.deepEqual(a.style,b.style,`${id}/${i} original-pattern styles`);a.children.flat().forEach((v,j)=>near(v,b.children.flat()[j],`${id}/${i}/${j}`));
  }
 }
 assert.deepEqual(await evaluate(`(()=>{const e=document.getElementById('st-default'),s=getComputedStyle(e);return[e.tagName,e.dataset.direction,e.hasAttribute('data-gap'),e.hasAttribute('data-wrap'),s.alignItems,s.justifyContent,s.rowGap,s.flexWrap,e.hasAttribute('role'),e.hasAttribute('tabindex')]})()`),['DIV','vertical',false,false,'normal','normal','normal','nowrap',false,false]);
 for(const id of ['st-explicit-h','st-explicit-v'])assert.deepEqual(await evaluate(`(()=>{const s=getComputedStyle(document.getElementById('${id}'));return[s.justifyContent,s.alignItems]})()`),['flex-end','flex-start']);
 assert.deepEqual(await evaluate(`(()=>{const s=getComputedStyle(document.getElementById('st-invalid'));return[s.justifyContent,s.alignItems]})()`),['normal','normal']);
 for(const dir of ['ltr','rtl']){
  const edges=await evaluate(`(()=>{const e=document.getElementById('st-padding');e.dir='${dir}';const s=getComputedStyle(e);return[s.paddingInlineStart,s.paddingInlineEnd,s.paddingBlockStart,s.paddingBlockEnd,s.paddingLeft,s.paddingRight]})()`);
  assert.deepEqual(edges,['0px','16px','12px','2px',...(dir==='ltr'?['0px','16px']:['16px','0px'])]);
 }
 assert.equal(await evaluate(`document.getElementById('st-padding').style.cssText.includes('container-padding')`),false);
 assert.equal(await evaluate(`(()=>{const e=document.getElementById('st-padding'),p=e.parentElement;p.style.setProperty('--container-padding-inline-start','99px');const value=getComputedStyle(e).getPropertyValue('--container-padding-inline-start').trim();p.style.removeProperty('--container-padding-inline-start');return value})()`),'99px','local Stack padding neither publishes nor resets an ancestor inset');
 assert.equal(await evaluate(`[...document.querySelectorAll('#st-matrix > .astryx-stack')].every(e=>!e.hasAttribute('role')&&!e.hasAttribute('tabindex')&&!e.hasAttribute('aria-label')&&e.getAnimations().length===0)`),true);
 assert.deepEqual(await evaluate(`(()=>{const s=getComputedStyle(document.getElementById('st-inner'));return[s.paddingInlineStart,s.paddingBlockStart,s.gap]})()`),['0px','0px','normal']);
 assert.deepEqual(await evaluate(`(()=>{const s=getComputedStyle(document.getElementById('st-style'));return[s.width,s.height,s.maxWidth,s.minHeight,s.gap,s.paddingInlineStart]})()`),['200px','120px','240px','80px','19px','17px']);
 assert.deepEqual(await evaluate(`(()=>{const s=getComputedStyle(document.getElementById('st-inherit'));return[s.width,s.height]})()`),['137px','63px']);
 assert.deepEqual(await evaluate(`['st-scroll','st-height'].map(id=>getComputedStyle(document.getElementById(id)).overflow)`),['auto','visible']);
 const wraps=await evaluate(`[...document.querySelectorAll('.st-wrap')].map(e=>stackMeasure(e))`);
 assert.deepEqual(wraps.map(x=>x.style.flexWrap),['nowrap','wrap','wrap-reverse']);
 assert.equal(wraps[0].children[0][1],wraps[0].children[2][1]);assert.ok(wraps[1].children[0][1]<wraps[1].children[2][1]);assert.ok(wraps[2].children[0][1]>wraps[2].children[2][1]);
 assert.equal(await evaluate(`document.getElementById('st-events').textContent`),'0:2:0:true');
 await evaluate(`document.getElementById('st-action').click()`);await pause(100);
 assert.equal(await evaluate(`document.getElementById('st-events').textContent`),'2:2:0:true');
 await evaluate(`document.getElementById('st-action').focus()`);
 await send('Input.dispatchKeyEvent',{type:'keyDown',key:'Enter',code:'Enter',windowsVirtualKeyCode:13,text:'\r'});
 await send('Input.dispatchKeyEvent',{type:'keyUp',key:'Enter',code:'Enter',windowsVirtualKeyCode:13});await pause(100);
 assert.equal(await evaluate(`document.getElementById('st-events').textContent`),'4:2:1:true');
 await evaluate(`document.getElementById('st-action').scrollIntoView({block:'center'})`);
 const clickPoint=await evaluate(`(()=>{const r=document.getElementById('st-action').getBoundingClientRect();return{x:r.x+r.width/2,y:r.y+r.height/2}})()`);
 await send('Input.dispatchMouseEvent',{type:'mousePressed',button:'left',clickCount:1,...clickPoint});
 await send('Input.dispatchMouseEvent',{type:'mouseReleased',button:'left',clickCount:1,...clickPoint});await pause(100);
 assert.equal(await evaluate(`document.getElementById('st-events').textContent`),'6:2:1:true');
 assert.deepEqual(await evaluate(`(()=>{const e=document.getElementById('st-native');return[e.tagName,e.dataset.caller,e.getAttribute('role'),e.getAttribute('aria-label'),e.dir]})()`),['NAV','yes','navigation','Caller navigation','rtl']);
 await send('Accessibility.enable');const doc=await send('DOM.getDocument');const node=await send('DOM.querySelector',{nodeId:doc.root.nodeId,selector:'#st-native'});const ax=(await send('Accessibility.getPartialAXTree',{nodeId:node.nodeId,fetchRelatives:false})).nodes[0];assert.equal(ax.role.value,'navigation');assert.equal(ax.name.value,'Caller navigation');
 await evaluate(`window.oldStack=document.getElementById('st-changing');window.oldChild=oldStack.firstElementChild;document.getElementById('st-change').click()`);await pause(100);
 assert.equal(await evaluate(`oldStack===document.getElementById('st-changing')&&oldChild===oldStack.firstElementChild&&oldStack.dataset.direction==='horizontal'`),true);
 await evaluate(`document.getElementById('st-toggle').click()`);await pause(100);assert.equal(await evaluate(`!!document.getElementById('st-changing')`),false);
 await evaluate(`document.getElementById('st-toggle').click()`);await pause(100);assert.equal(await evaluate(`!!document.getElementById('st-changing')`),true);
 for(const feature of [{'forced-colors':'active'},{'prefers-reduced-motion':'reduce'}]){
  await send('Emulation.setEmulatedMedia',{features:Object.entries(feature).map(([name,value])=>({name,value}))});
  assert.equal(await evaluate(`document.getElementById('st-default').getAnimations().length`),0);
 }
 assert.equal(exceptions.length,0,JSON.stringify(exceptions));
 console.log('Stack passed: offline source-derived 48-axis matrix × 8 environments, 11 gap/padding steps, three pattern Stack topologies (demo-only static framing; supporting interaction gaps disclosed), logical padding, invalid/explicit aliases, sizing/CSS precedence, wrap/scroll, native root/ref/events/AX and updates/media. Not published measurements.');
});
