// Offline source evidence; each run owns and closes one Chrome CDP target.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {cardMetrics,expandControl,expandContext,withTarget} from './card_metrics.mjs';
const fixture=JSON.parse(await readFile(new URL('./fixtures/card-neutral-reference.json',import.meta.url),'utf8'));
const frames=['#cd-showcase','#cd-callout','#cd-elevations','#cd-variants','#cd-layout','#cd-simple'];
function compare(a,b,path='root'){
 if(typeof b==='number'){assert.ok(Math.abs(a-b)<=1/64+1e-6,`${path}: ${a} != ${b}`);return}
 if(b&&typeof b==='object'){assert.deepEqual(Object.keys(a).sort(),Object.keys(b).sort(),`${path} keys`);for(const k of Object.keys(b))compare(a[k],b[k],`${path}/${k}`);return}assert.equal(a,b,path);
}
function textBoxes(n){const rows=[];for(const r of n.textRects){const row=rows.find(v=>Math.abs(v[1]-r[1])<1/64&&v[3]===r[3]&&Math.abs(v[0]+v[2]-r[0])<1/64);if(row)row[2]+=r[2];else rows.push([...r]);}return rows;}
function view(tree,isLayout=false){return tree.nodes.map((n,index)=>{
 const style={};for(const p of ['display','flexDirection','alignItems','justifyContent','rowGap','columnGap','padding','margin','backgroundColor','color','opacity','borderRadius','overflowX','overflowY','fontSize','fontWeight','lineHeight','fontFamily','fontVariantNumeric','whiteSpace','textOverflow','boxShadow'])style[p]=n.style[p];
 if(style.alignItems==='normal'&&['flex','grid'].includes(style.display))style.alignItems='stretch';
 if(n.style.borderWidth!=='0px')for(const p of ['borderWidth','borderStyle','borderColor'])style[p]=n.style[p];
 const children=[];for(const child of n.children){const last=children.at(-1);if('text'in child&&last&&'text'in last)last.text+=child.text;else children.push({...child});}
 return {tag:n.tag,box:n.box,textRects:textBoxes(n),text:n.textRuns.map(r=>r.text).join(''),style,children};
});}
await withTarget(async({send,evaluate,pause,waitFor,exceptions})=>{
 await send('Accessibility.enable');await send('Page.bringToFront');
 const media=features=>send('Emulation.setEmulatedMedia',{features:Object.entries({'prefers-reduced-motion':'no-preference','forced-colors':'none',...features}).map(([name,value])=>({name,value}))});
 const click=async id=>{await evaluate(`document.getElementById(${JSON.stringify(id)}).click()`);await pause(80)};
 await media({});await send('Page.navigate',{url:process.env.SHOWCASE_URL??'http://127.0.0.1:8080'});
 await waitFor('document.querySelectorAll("[data-card-named]").length===15');await evaluate('document.fonts.ready.then(()=>true)');
 await evaluate(`window.cardMetrics=(${cardMetrics});true`);
 for(const mode of ['light','dark']){
  if(mode==='dark')await click('cd-theme');
  const controls=await evaluate(`${JSON.stringify(frames)}.flatMap(s=>{const f=document.querySelector(s);return(f.matches('.astryx-card')?[f]:[...f.querySelectorAll('.astryx-card')]).map(e=>cardMetrics(e,f))})`);
  for(let i=0;i<controls.length;i++){
   const expected=expandControl(fixture,mode,i);
   if(i!==13)compare(view(controls[i]),view(expected),`${mode}/original/${i}`);
   else{
    // FR3 corrects the observed source overshoot. No blanket geometry waiver:
    // assert the changed layout's exact box and logical relationships below.
    const root=controls[i].nodes[0],old=expected.nodes[0];assert.equal(root.box[2],old.box[2]);assert.equal(root.box[3],166);assert.equal(root.style.padding,old.style.padding);assert.equal(root.style.backgroundColor,old.style.backgroundColor);
    assert.equal(controls[i].nodes.map(n=>n.textRuns.map(r=>r.text).join('')).join(''),expected.nodes.map(n=>n.textRuns.map(r=>r.text).join('')).join(''));
   }
  }
  const contexts=await evaluate(`${JSON.stringify(frames)}.map(s=>{const f=document.querySelector(s);return cardMetrics(f,f)})`);
  for(let i=0;i<contexts.length;i++)if(i!==4)compare(view(contexts[i]),view(expandContext(fixture,mode,i)),`${mode}/native frame/${i}`);
  assert.equal(await evaluate('Array.from(document.querySelectorAll("[data-card-named]")).every(e=>!e.hasAttribute("role")&&!e.hasAttribute("tabindex")&&!e.hasAttribute("aria-label")&&e.getAnimations().length===0&&!document[Symbol.for("astryx.dioxus.button.runtime")].records.has(e))'),true);
 }
 await click('cd-theme');
 // Verify every core role independently of the Neutral desktop observations.
 const corePaint=Object.fromEntries(['default','transparent','muted','blue','cyan','gray','green','orange','pink','purple','red','teal','yellow'].map(v=>[v,v==='transparent'?'transparent':fixture.pinnedTokens.core['--color-background-'+(v==='default'?'card':v)]]));
 await evaluate(`(()=>{const section=document.getElementById('card-examples');document.querySelector('main').dataset.astryxTheme='core';section.removeAttribute('data-astryx-theme');window.corePaint=${JSON.stringify(corePaint)};})()`);
 for(const mode of ['light','dark'])assert.equal(await evaluate(`(()=>{const section=document.getElementById('card-examples');section.style.colorScheme=${JSON.stringify(mode)};const probe=document.createElement('span');section.append(probe);const ok=[...document.querySelectorAll('#cd-all-variants .astryx-card')].every(e=>{probe.style.backgroundColor=corePaint[e.dataset.variant];return getComputedStyle(probe).backgroundColor===getComputedStyle(e).backgroundColor});probe.remove();return ok})()`),true,`${mode} all thirteen core fills`);
 assert.equal(await evaluate('getComputedStyle(document.getElementById("cd-showcase")).paddingInlineStart'),'15px','un-themed spacing-four padding minus border');
 await evaluate('document.querySelector("main").dataset.astryxTheme="astryx";document.getElementById("card-examples").dataset.astryxTheme="neutral";document.getElementById("card-examples").style.colorScheme="light"');
 // The corrected full-bleed layout stays INSIDE the border, with no phantom scroll.
 compare(await evaluate('(()=>{const c=document.getElementById("cd-layout"),l=c.querySelector(".demo-card-layout"),r=c.getBoundingClientRect(),b=l.getBoundingClientRect();return[b.x-r.x,b.y-r.y,b.width,b.height,c.scrollWidth,c.clientWidth,c.scrollHeight,c.clientHeight]})()'),[1,1,378,164,378,378,164,164],'FR3 corrected Layout');
 const expectedPadding=[0,2,4,6,8,12,16,20,24,32,40];
 const paddings=await evaluate('Array.from(document.querySelectorAll("#cd-paddings .astryx-card")).map(e=>{const s=getComputedStyle(e);return[parseFloat(s.paddingInlineStart),parseFloat(s.borderInlineStartWidth),e.firstElementChild.getBoundingClientRect().x-e.getBoundingClientRect().x]})');
 paddings.forEach((p,i)=>compare(p,[Math.max(0,expectedPadding[i]-2),2,Math.max(0,expectedPadding[i]-2)+2],`padding ${i}`));
 // Resolve publication through actual CSS lengths; custom-property strings retain calc().
 await evaluate('window.paddingValues=(e)=>{const s=getComputedStyle(e),p=document.createElement("div");e.append(p);const values=["inline-start","inline-end","block-start","block-end"].map(edge=>{p.style.width="var(--container-padding-"+edge+")";return parseFloat(getComputedStyle(p).width)});p.remove();return values};true');
 compare(await evaluate('paddingValues(document.getElementById("cd-asymmetric"))'),[18,6,4,8],'asymmetric actual publication');
 const bleed=await evaluate('(()=>{const c=document.getElementById("cd-asymmetric"),d=document.getElementById("cd-bleed"),a=c.getBoundingClientRect(),b=d.getBoundingClientRect();return[b.x-a.x,b.width,c.clientWidth,c.scrollWidth,c.clientHeight,c.scrollHeight]})()');compare(bleed,[2,236,236,236,96,96],'full-bleed divider ends at padding edges');
 await evaluate('document.getElementById("cd-asymmetric").dir="rtl"');compare(await evaluate('(()=>{const c=document.getElementById("cd-asymmetric"),d=document.getElementById("cd-bleed"),a=c.getBoundingClientRect(),b=d.getBoundingClientRect();return[b.x-a.x,b.width]})()'),[2,236],'RTL logical bleed');
 assert.deepEqual(await evaluate('["cd-corners","cd-nested-corner","cd-mixed-corner"].map(id=>{const e=document.querySelector("#"+id+" .demo-card-reader"),s=getComputedStyle(e);return[s.borderStartStartRadius,s.borderEndEndRadius,s.paddingBlockStart,s.paddingBlockEnd]})'),[['12px','12px','11px','11px'],['0px','0px','0px','0px'],['0px','0px','0px','0px']]);
 compare(await evaluate('paddingValues(document.getElementById("cd-nested-inner"))'),[11,11,11,11],'nested Card resets explicit ancestor padding');
 compare(await evaluate('document.getElementById("cd-preferred").getBoundingClientRect().width'),180,'preferred width yields');
 assert.equal(await evaluate('getComputedStyle(document.getElementById("cd-preferred")).overflow'),'clip');assert.equal(await evaluate('getComputedStyle(document.getElementById("cd-scroll")).overflow'),'auto');assert.equal(await evaluate('getComputedStyle(document.getElementById("cd-auto")).overflow'),'clip');
 compare(await evaluate('document.getElementById("cd-inherit").getBoundingClientRect().width'),137);compare(await evaluate('document.getElementById("cd-inherit").getBoundingClientRect().height'),63);compare(await evaluate('document.getElementById("cd-escaped-inherit").getBoundingClientRect().width'),137);
 compare(await evaluate('document.getElementById("cd-class").getBoundingClientRect().width'),211,'caller class beats preferred width');compare(await evaluate('document.getElementById("cd-style").getBoundingClientRect().width'),233,'caller style final');
 assert.ok((await evaluate('getComputedStyle(document.getElementById("cd-ring")).boxShadow')).startsWith('rgb(100, 40, 160) 0px 0px 0px 2px inset,'),'ring and elevation coexist');
 assert.equal(await evaluate('getComputedStyle(document.getElementById("cd-fallback")).backgroundColor'),'rgba(0, 0, 0, 0)');assert.equal(await evaluate('getComputedStyle(document.getElementById("cd-fallback")).borderWidth'),'0px');assert.equal(await evaluate('getComputedStyle(document.getElementById("cd-custom")).backgroundColor'),'rgb(30, 100, 180)');
 assert.equal(await evaluate('document.getElementById("cd-events").textContent'),'Native events: 2');await click('cd-native');assert.equal(await evaluate('document.getElementById("cd-events").textContent'),'Native events: 3');assert.deepEqual(await evaluate('(()=>{const e=document.getElementById("cd-native");return[e.tagName,e.lang,e.classList.contains("caller-card"),e.style.marginInline,e.dataset.variant]})()'),['DIV','fr',true,'7px','default']);
 const doc=await send('DOM.getDocument'),native=await send('DOM.querySelector',{nodeId:doc.root.nodeId,selector:'#cd-native'}),ax=(await send('Accessibility.getPartialAXTree',{nodeId:native.nodeId,fetchRelatives:false})).nodes[0];assert.equal(ax.role.value,'region');assert.equal(ax.name.value,'Caller named region');
 await evaluate('window.savedCard=document.getElementById("cd-changing")');await click('cd-change');assert.equal(await evaluate('document.getElementById("cd-changing")===savedCard'),true);assert.equal(await evaluate('savedCard.dataset.variant'),'muted');compare(await evaluate('savedCard.getBoundingClientRect().width'),240);await click('cd-toggle');assert.equal(await evaluate('document.getElementById("cd-late")'),null);await click('cd-toggle');
 await evaluate('document.querySelector("#cd-layout button:last-child").click()');assert.equal(await evaluate('document.getElementById("cd-notice").textContent'),'Demo: your profile changes would be saved here');
 await send('Emulation.setDeviceMetricsOverride',{width:390,height:1600,deviceScaleFactor:1,mobile:false});
 const phone=await evaluate('(()=>{const p=document.getElementById("cd-phone"),r=p.getBoundingClientRect();return{width:r.width,scroll:p.scrollWidth,client:p.clientWidth,cards:[...p.querySelectorAll(".astryx-card")].map(e=>{const b=e.getBoundingClientRect(),t=e.querySelector(".astryx-text--clamp-1");return[b.x-r.x,b.width,getComputedStyle(e).minWidth,getComputedStyle(e).overflow,!!t]})}})()');assert.equal(phone.scroll,phone.client);assert.equal(phone.cards.length,3);assert.ok(phone.cards.every(c=>c[1]<130&&c[2]==='0px'&&c[3]==='clip'));
 await send('Emulation.setTouchEmulationEnabled',{enabled:true,maxTouchPoints:1});await pause(150);assert.equal(await evaluate('getComputedStyle(document.querySelector("#cd-phone .astryx-text")).fontSize'),'16px');assert.deepEqual(await evaluate('(()=>{const h=getComputedStyle(document.querySelector("#cd-phone h3")),b=getComputedStyle(document.querySelector("#cd-phone .astryx-text")),s=getComputedStyle(document.querySelector("#cd-phone .astryx-text:last-child"));return[h.fontSize,h.lineHeight,b.fontSize,b.lineHeight,s.fontSize,s.lineHeight]})()'),['19px','28.0003px','16px','24px','14px','20.0004px']);
 // Text owns full-ID truncation tooltips; the Card itself remains inert.
 await evaluate('window.payoutId=document.querySelector("#cd-phone .astryx-card .astryx-text:last-child");payoutId.scrollIntoView({block:"center"})');const p=await evaluate('(()=>{const r=payoutId.getBoundingClientRect();return{x:r.x+r.width/2,y:r.y+r.height/2}})()');await send('Input.dispatchTouchEvent',{type:'touchStart',touchPoints:[{...p,id:1,radiusX:2,radiusY:2}]});await send('Input.dispatchTouchEvent',{type:'touchEnd',touchPoints:[]});await waitFor('!!document.querySelector(".astryx-tooltip:popover-open")');assert.equal(await evaluate('document.querySelector(".astryx-tooltip:popover-open").textContent'),'po_1Q8ZK4X7M2N5P3R6T9V1W4YB');
 await media({'forced-colors':'active'});assert.equal(await evaluate('getComputedStyle(document.getElementById("cd-ring")).boxShadow'),'none');assert.equal(await evaluate('getComputedStyle(document.querySelector("#cd-all-variants [data-variant=blue]")).borderWidth'),'0px','no invented forced-color category outlines');
 await media({'prefers-reduced-motion':'reduce'});assert.equal(await evaluate('document.getElementById("cd-showcase").getAnimations().length'),0);
 assert.equal(exceptions.length,0,JSON.stringify(exceptions));console.log('Card browser checks passed: 30 actual-source controls/six patterns (FR3 corrected Layout), source shadows/tints/text, all padding steps/asymmetric/RTL bleed/corners/nesting, preferred sizing and CSS precedence, clipping/scrolling, native refs/AX/events/stable updates, phone IDs/tooltips and media.');
});
